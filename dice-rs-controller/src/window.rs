use std::collections::HashSet;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use dice_rs::service::manager::DiceManager;
use glib::clone;
use gtk4::glib;
use gtk4::prelude::*;
use tracing::debug;

use crate::dice_row::DiceRow;

/// Interval for periodic auto-scan after the startup burst (seconds).
const AUTO_SCAN_INTERVAL_SECS: u64 = 15;

/// Fast auto-scan interval during startup burst (seconds).
const AUTO_SCAN_FAST_INTERVAL_SECS: u64 = 5;

/// Duration of the startup burst period (seconds).
const AUTO_SCAN_BURST_DURATION_SECS: u64 = 60;

/// Main application window.
pub struct MainWindow {
    window: gtk4::ApplicationWindow,
    scan_button: gtk4::Button,
    dice_list: gtk4::Box,
    status_label: gtk4::Label,
    manager: Arc<DiceManager>,
    connected_ids: Arc<std::sync::Mutex<HashSet<String>>>,
}

impl MainWindow {
    /// Create the main window.
    pub fn new(app: &gtk4::Application, manager: Arc<DiceManager>) -> Self {
        let scan_button = gtk4::Button::builder().label("Scan for GoDice").css_classes(vec!["suggested-action"]).build();

        let dice_list = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(6)
            .css_classes(vec!["dice-list"])
            .build();

        let status_label = gtk4::Label::builder()
            .label("Ready")
            .css_classes(vec!["dim-label"])
            .halign(gtk4::Align::Start)
            .build();

        let header_bar = gtk4::HeaderBar::builder().build();
        header_bar.pack_start(&scan_button);

        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .build();
        content.append(&dice_list);
        content.append(&status_label);

        let scrolled = gtk4::ScrolledWindow::builder().hexpand(true).vexpand(true).child(&content).build();

        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .title("dice-rs Controller")
            .default_width(800)
            .default_height(1000)
            .child(&scrolled)
            .build();
        window.set_titlebar(Some(&header_bar));

        let win = Self {
            window,
            scan_button,
            dice_list,
            status_label,
            manager,
            connected_ids: Arc::new(std::sync::Mutex::new(HashSet::new())),
        };

        win.connect_signals();
        win.start_auto_scan();
        win
    }

    /// Connect signal handlers.
    fn connect_signals(&self) {
        self.scan_button.connect_clicked(clone!(
            #[strong(rename_to = manager)]
            self.manager.clone(),
            #[strong(rename_to = dice_list)]
            self.dice_list.clone(),
            #[strong(rename_to = status_label)]
            self.status_label.clone(),
            #[strong(rename_to = connected_ids)]
            self.connected_ids.clone(),
            move |_| {
                let manager = manager.clone();
                let dice_list = dice_list.clone();
                let status_label = status_label.clone();
                let connected_ids = connected_ids.clone();

                // Clear existing dice rows and reset connected IDs.
                while let Some(child) = dice_list.first_child() {
                    dice_list.remove(&child);
                }
                if let Ok(mut ids) = connected_ids.lock() {
                    ids.clear();
                }

                status_label.set_text("Scanning...");
                let manager_scan = manager.clone();
                glib::spawn_future_local(async move {
                    let scan_result = tokio::spawn(async move { manager_scan.scan().await }).await;
                    match scan_result {
                        Ok(Ok(devices)) => {
                            if devices.is_empty() {
                                status_label.set_text("No GoDice devices found.");
                                return;
                            }
                            let count = devices.len();
                            status_label.set_text(&format!("Found {count} device(s), connecting..."));

                            let connected = Arc::new(AtomicUsize::new(0));
                            for device in devices {
                                let device_id = format!("{:?}", device.id);
                                if let Ok(mut ids) = connected_ids.lock() {
                                    ids.insert(device_id);
                                }

                                let device_name = device.name.clone();
                                let connect_manager = manager.clone();
                                let connect_result = tokio::spawn(async move { connect_manager.connect(&device).await }).await;
                                match connect_result {
                                    Ok(Ok(dice)) => {
                                        let row = DiceRow::new(dice, manager.clone());
                                        dice_list.append(row.widget());
                                        let n = connected.fetch_add(1, Ordering::Relaxed) + 1;
                                        status_label.set_text(&format!("Connected {n}/{count}"));
                                    }
                                    Ok(Err(error)) => {
                                        debug!(error = %error, device = %device_name, "connection failed");
                                        status_label.set_text(&format!("Connection failed for {device_name}: {error}"));
                                    }
                                    Err(error) => {
                                        debug!(error = %error, device = %device_name, "connect task join failed");
                                    }
                                }
                            }
                        }
                        Ok(Err(error)) => {
                            status_label.set_text(&format!("Scan failed: {error}"));
                            debug!(error = %error, "scan failed");
                        }
                        Err(error) => {
                            status_label.set_text(&format!("Scan task failed: {error}"));
                            debug!(error = %error, "scan task join failed");
                        }
                    }
                });
            }
        ));
    }

    /// Start periodic auto-scan to discover and connect new dice.
    /// Existing dice rows are preserved — only newly discovered devices are added.
    /// During the first `AUTO_SCAN_BURST_DURATION_SECS` seconds, scans run every
    /// `AUTO_SCAN_FAST_INTERVAL_SECS` seconds; afterwards the interval reverts to
    /// `AUTO_SCAN_INTERVAL_SECS`.
    fn start_auto_scan(&self) {
        let manager = self.manager.clone();
        let dice_list = self.dice_list.clone();
        let status_label = self.status_label.clone();
        let connected_ids = self.connected_ids.clone();

        // UI updates from the tokio scan task are marshaled via std::sync::mpsc.
        enum ScanUiUpdate {
            NewDice { dice: dice_rs::service::dice::Dice, manager: Arc<DiceManager> },
        }

        let (sender, receiver) = std::sync::mpsc::channel::<ScanUiUpdate>();

        glib::timeout_add_local(Duration::from_millis(50), move || {
            while let Ok(update) = receiver.try_recv() {
                match update {
                    ScanUiUpdate::NewDice { dice, manager } => {
                        let row = DiceRow::new(dice, manager);
                        dice_list.append(row.widget());
                        status_label.set_text("New dice connected.");
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            loop {
                let manager_scan = manager.clone();
                let scan_result = manager_scan.scan().await;
                match scan_result {
                    Ok(devices) => {
                        for device in devices {
                            let device_id = format!("{:?}", device.id);
                            let already_connected = {
                                let mut ids = match connected_ids.lock() {
                                    Ok(guard) => guard,
                                    Err(error) => {
                                        debug!(error = %error, "connected_ids mutex poisoned");
                                        continue;
                                    }
                                };
                                if ids.contains(&device_id) {
                                    true
                                } else {
                                    ids.insert(device_id.clone());
                                    false
                                }
                            };
                            if already_connected {
                                continue;
                            }

                            let device_name = device.name.clone();
                            debug!(device = %device_name, "auto-scan: connecting new device");
                            let connect_manager = manager.clone();
                            let connect_sender = sender.clone();
                            let connect_ids = connected_ids.clone();
                            tokio::spawn(async move {
                                match connect_manager.connect(&device).await {
                                    Ok(dice) => {
                                        let _ = connect_sender.send(ScanUiUpdate::NewDice { dice, manager: connect_manager.clone() });
                                    }
                                    Err(error) => {
                                        debug!(error = %error, device = %device_name, "auto-scan: connection failed");
                                        if let Ok(mut ids) = connect_ids.lock() {
                                            ids.remove(&device_id);
                                        }
                                    }
                                }
                            });
                        }
                    }
                    Err(error) => {
                        debug!(error = %error, "auto-scan failed");
                    }
                }

                let elapsed = start.elapsed().as_secs();
                let interval = if elapsed < AUTO_SCAN_BURST_DURATION_SECS {
                    AUTO_SCAN_FAST_INTERVAL_SECS
                } else {
                    AUTO_SCAN_INTERVAL_SECS
                };
                tokio::time::sleep(Duration::from_secs(interval)).await;
            }
        });
    }

    /// Present the window.
    pub fn present(&self) {
        self.window.present();
    }
}
