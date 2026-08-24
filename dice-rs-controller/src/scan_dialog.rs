use dice_rs::service::dice::DiceDevice;
use gtk4::prelude::*;

/// Device discovery and selection dialog.
pub struct ScanDialog {
    dialog: gtk4::Window,
    list: gtk4::ListBox,
    devices: Vec<DiceDevice>,
}

impl ScanDialog {
    /// Create a new scan dialog with discovered devices.
    pub fn new(devices: &[DiceDevice]) -> Self {
        let dialog = gtk4::Window::builder()
            .title("Select GoDice")
            .modal(true)
            .default_width(400)
            .default_height(300)
            .build();

        let list = gtk4::ListBox::builder().css_classes(vec!["scan-list"]).build();

        let mut device_list = Vec::new();
        for device in devices {
            let row = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .spacing(12)
                .margin_start(12)
                .margin_end(12)
                .margin_top(8)
                .margin_bottom(8)
                .build();

            let color_label = device.color().map(|c| c.to_string()).unwrap_or_else(|_| "Unknown".into());
            row.append(&gtk4::Label::builder().label(&color_label).width_chars(10).build());
            row.append(&gtk4::Label::builder().label(&device.name).hexpand(true).build());
            row.append(&gtk4::Label::builder().label(&device.address.to_string()).css_classes(vec!["dim-label"]).build());

            list.append(&row);
            device_list.push(device.clone());
        }

        let scrolled = gtk4::ScrolledWindow::builder().hexpand(true).vexpand(true).child(&list).build();

        let content = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).build();
        content.append(&scrolled);

        dialog.set_child(Some(&content));

        Self {
            dialog,
            list,
            devices: device_list,
        }
    }

    /// Connect to the row-activated signal with a callback.
    pub fn connect_response<F>(&self, callback: F)
    where
        F: Fn(Option<DiceDevice>) + 'static,
    {
        self.list.connect_row_activated({
            let devices = self.devices.clone();
            let dialog = self.dialog.clone();
            move |_, row| {
                let index = row.index() as usize;
                if let Some(device) = devices.get(index) {
                    callback(Some(device.clone()));
                } else {
                    callback(None);
                }
                dialog.close();
            }
        });
    }

    /// Present the dialog.
    pub fn present(&self) {
        self.dialog.present();
    }
}
