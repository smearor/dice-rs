use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use dice_rs::service::manager::DiceManager;
use gtk4::glib;
use gtk4::prelude::*;

use crate::config::app_settings::AppSettings;
use crate::info_dialog::InfoDialog;
use crate::platform::widget_container::WidgetContainer;
use crate::platform::window_mode::WindowMode;
use crate::services::connection_service::ConnectionEvent;
use crate::services::connection_service::ConnectionService;
use crate::settings_dialog::SettingsDialog;
use crate::widgets::dice_row::DiceRow;

/// Main application window.
pub struct MainWindow {
    window: gtk4::ApplicationWindow,
    scan_button: gtk4::Button,
    dice_list: gtk4::Box,
    status_label: gtk4::Label,
    connection_service: ConnectionService,
    settings: AppSettings,
    dice_rows: Rc<std::cell::RefCell<Vec<DiceRow>>>,
}

impl MainWindow {
    /// Create the main window.
    pub fn new(app: &gtk4::Application, manager: Arc<DiceManager>) -> Self {
        let scan_button = gtk4::Button::builder().label("Scan for GoDice").css_classes(vec!["suggested-action"]).build();

        let dice_list = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .css_classes(vec!["dice-list"])
            .build();

        let status_label = gtk4::Label::builder()
            .label("Ready")
            .css_classes(vec!["dim-label"])
            .halign(gtk4::Align::Start)
            .build();

        let header_bar = gtk4::HeaderBar::builder().build();
        header_bar.pack_start(&scan_button);

        // Menu button with Settings and Info actions.
        let menu_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(2)
            .margin_start(6)
            .margin_end(6)
            .margin_top(6)
            .margin_bottom(6)
            .build();
        let settings_action = gtk4::Button::builder().label("Settings").css_classes(vec!["flat"]).build();
        let info_action = gtk4::Button::builder().label("Info").css_classes(vec!["flat"]).build();
        menu_box.append(&settings_action);
        menu_box.append(&info_action);

        let popover = gtk4::Popover::builder().child(&menu_box).build();

        let menu_button = gtk4::MenuButton::builder().icon_name("open-menu-symbolic").popover(&popover).build();
        header_bar.pack_end(&menu_button);

        // Compact mode switch in the titlebar.
        let compact_switch = gtk4::Switch::builder().tooltip_text("Compact mode").valign(gtk4::Align::Center).build();
        header_bar.pack_end(&compact_switch);

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
            .default_width(1000)
            .default_height(1000)
            .child(&scrolled)
            .build();
        window.set_titlebar(Some(&header_bar));

        let settings = AppSettings::new();
        let dice_rows = Rc::new(std::cell::RefCell::new(Vec::<DiceRow>::new()));
        let connection_service = ConnectionService::new(manager);

        compact_switch.set_active(settings.get().compact_mode);
        window.set_size_request(-1, WindowMode::min_height());
        let mode = WindowMode::from(settings.get().compact_mode);
        if mode.is_compact() {
            dice_list.set_orientation(mode.orientation());
            window.set_default_size(1000, mode.window_height(0));
            status_label.set_visible(false);
        }

        let win = Self {
            window,
            scan_button,
            dice_list,
            status_label,
            connection_service,
            settings,
            dice_rows,
        };

        win.connect_signals();

        // Connect menu actions.
        {
            let settings_clone = win.settings.clone();
            let window_clone = win.window.clone();
            settings_action.connect_clicked(move |_| {
                let dialog = SettingsDialog::new(&window_clone, &settings_clone);
                dialog.present();
            });

            let window_clone = win.window.clone();
            info_action.connect_clicked(move |_| {
                let dialog = InfoDialog::new(&window_clone);
                dialog.present();
            });

            // Compact mode switch in titlebar.
            let settings_for_compact = win.settings.clone();
            compact_switch.connect_notify_local(Some("active"), move |switch, _| {
                let mut data = settings_for_compact.get();
                data.compact_mode = switch.is_active();
                settings_for_compact.set(data);
            });

            // Apply settings to all dice rows when settings change.
            let dice_rows_for_change = win.dice_rows.clone();
            let dice_list_for_change = win.dice_list.clone();
            let status_label_for_change = win.status_label.clone();
            let window_for_change = win.window.clone();
            win.settings.connect_changed(move |data| {
                let mode = WindowMode::from(data.compact_mode);
                dice_list_for_change.set_orientation(mode.orientation());
                status_label_for_change.set_visible(!mode.is_compact());
                let count = dice_rows_for_change.borrow().len();
                window_for_change.set_default_size(1000, mode.window_height(count));
                for row in dice_rows_for_change.borrow().iter() {
                    row.apply_settings(&data);
                }
            });

            // Auto-switch mode when window is resized.
            let settings_for_resize = win.settings.clone();
            let compact_switch_for_resize = compact_switch.clone();
            win.window.connect_notify_local(Some("default-height"), move |window, _| {
                let mode = WindowMode::from_height(window.default_height());
                let current = WindowMode::from(settings_for_resize.get().compact_mode);
                if mode != current {
                    compact_switch_for_resize.set_active(mode.is_compact());
                }
            });
        }

        win.start_auto_scan();
        win
    }

    /// Connect signal handlers.
    fn connect_signals(&self) {
        let connection_service = self.connection_service.clone();
        let dice_list = self.dice_list.clone();
        let status_label = self.status_label.clone();
        let dice_rows = self.dice_rows.clone();
        let settings = self.settings.clone();

        self.scan_button.connect_clicked(move |_| {
            // Clear existing dice rows and reset connected IDs.
            while let Some(child) = dice_list.first_child() {
                dice_list.remove(&child);
            }
            connection_service.clear_connected();
            dice_rows.borrow_mut().clear();

            status_label.set_text("Scanning...");

            let (sender, receiver) = std::sync::mpsc::channel::<ConnectionEvent>();
            connection_service.scan_once(sender);

            let dice_list = dice_list.clone();
            let status_label = status_label.clone();
            let dice_rows = dice_rows.clone();
            let settings = settings.clone();
            let connected_count = Rc::new(std::cell::Cell::new(0usize));
            let total_count = Rc::new(std::cell::Cell::new(0usize));
            glib::timeout_add_local(Duration::from_millis(50), move || {
                while let Ok(event) = receiver.try_recv() {
                    match event {
                        ConnectionEvent::ScanStarted => {}
                        ConnectionEvent::NoDevicesFound => {
                            status_label.set_text("No GoDice devices found.");
                        }
                        ConnectionEvent::DevicesFound(count) => {
                            total_count.set(count);
                            status_label.set_text(&format!("Found {count} device(s), connecting..."));
                        }
                        ConnectionEvent::DiceConnected { dice, manager } => {
                            let row = DiceRow::new(dice, manager);
                            row.apply_settings(&settings.get());
                            row.pack_into(&dice_list);
                            dice_list.append(row.compact_widget());
                            dice_rows.borrow_mut().push(row);
                            let n = connected_count.get() + 1;
                            connected_count.set(n);
                            let total = total_count.get();
                            status_label.set_text(&format!("Connected {n}/{total}"));
                        }
                        ConnectionEvent::DiceConnectionFailed { name, error } => {
                            status_label.set_text(&format!("Connection failed for {name}: {error}"));
                        }
                        ConnectionEvent::ScanFailed(error) => {
                            status_label.set_text(&format!("Scan failed: {error}"));
                        }
                        ConnectionEvent::AutoScanDiceConnected { .. } => {}
                    }
                }
                glib::ControlFlow::Continue
            });
        });
    }

    /// Start periodic auto-scan to discover and connect new dice.
    /// Existing dice rows are preserved - only newly discovered devices are added.
    fn start_auto_scan(&self) {
        let (sender, receiver) = std::sync::mpsc::channel::<ConnectionEvent>();
        self.connection_service.start_auto_scan(sender);

        let dice_list = self.dice_list.clone();
        let status_label = self.status_label.clone();
        let dice_rows = self.dice_rows.clone();
        let settings = self.settings.clone();
        glib::timeout_add_local(Duration::from_millis(50), move || {
            while let Ok(event) = receiver.try_recv() {
                if let ConnectionEvent::AutoScanDiceConnected { dice, manager } = event {
                    let row = DiceRow::new(dice, manager);
                    row.apply_settings(&settings.get());
                    row.pack_into(&dice_list);
                    dice_list.append(row.compact_widget());
                    dice_rows.borrow_mut().push(row);
                    status_label.set_text("New dice connected.");
                }
            }
            glib::ControlFlow::Continue
        });
    }

    /// Present the window.
    pub fn present(&self) {
        self.window.present();
    }
}
