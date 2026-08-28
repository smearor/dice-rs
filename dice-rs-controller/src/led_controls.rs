use std::cell::RefCell;
use std::rc::Rc;

use dice_rs::model::led::LedColor;
use dice_rs::model::led::PulseBlinkMode;
use dice_rs::model::led::PulseLeds;
use dice_rs::service::dice::Dice;
use glib::clone;
use gtk4::prelude::*;
use tracing::debug;

/// LED control panel for a connected dice.
pub struct LedControls {
    container: gtk4::Box,
    color_button1: gtk4::ColorButton,
    color_button2: gtk4::ColorButton,
    set_button: gtk4::Button,
    pulse_button: gtk4::Button,
    off_button: gtk4::Button,
    blink_mode_dropdown: gtk4::DropDown,
    leds_dropdown: gtk4::DropDown,
    dice: Rc<RefCell<Option<Dice>>>,
    device_name: Rc<RefCell<Option<String>>>,
}

impl LedControls {
    /// Create a new LED control panel.
    pub fn new() -> Self {
        let color_button1 = gtk4::ColorButton::builder().tooltip_text("LED 1").build();
        let color_button2 = gtk4::ColorButton::builder().tooltip_text("LED 2").build();
        let set_button = gtk4::Button::builder().label("Set").build();
        let pulse_button = gtk4::Button::builder().label("Pulse").build();
        let off_button = gtk4::Button::builder().label("Off").build();
        let blink_mode_model = gtk4::StringList::new(&["Rainbow", "Color"]);
        let blink_mode_dropdown = gtk4::DropDown::builder()
            .model(&blink_mode_model)
            .tooltip_text("Blink mode")
            .selected(1)
            .build();

        let leds_model = gtk4::StringList::new(&["Both", "LED 1", "LED 2"]);
        let leds_dropdown = gtk4::DropDown::builder().model(&leds_model).tooltip_text("LEDs").selected(0).build();

        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(8)
            .css_classes(vec!["led-controls"])
            .build();
        container.append(&color_button1);
        container.append(&color_button2);
        container.append(&set_button);
        container.append(&pulse_button);
        container.append(&blink_mode_dropdown);
        container.append(&leds_dropdown);
        container.append(&off_button);

        let dice = Rc::new(RefCell::new(None::<Dice>));

        let device_name = Rc::new(RefCell::new(None::<String>));

        let widget = Self {
            container,
            color_button1,
            color_button2,
            set_button,
            pulse_button,
            off_button,
            blink_mode_dropdown,
            leds_dropdown,
            dice,
            device_name,
        };

        widget.connect_signals();
        widget
    }

    /// Set the dice to control.
    pub fn set_dice(&self, dice: Dice) {
        *self.dice.borrow_mut() = Some(dice);
    }

    /// Set the device name for per-dice config persistence.
    pub fn set_device_name(&self, name: String) {
        *self.device_name.borrow_mut() = Some(name);
    }

    /// Update color picker buttons from saved LED colors.
    pub fn set_colors(&self, color1: LedColor, color2: LedColor) {
        let rgba1 = gtk4::gdk::RGBA::new(
            color1.r as f32 / 255.0,
            color1.g as f32 / 255.0,
            color1.b as f32 / 255.0,
            1.0,
        );
        let rgba2 = gtk4::gdk::RGBA::new(
            color2.r as f32 / 255.0,
            color2.g as f32 / 255.0,
            color2.b as f32 / 255.0,
            1.0,
        );
        self.color_button1.set_rgba(&rgba1);
        self.color_button2.set_rgba(&rgba2);
    }

    /// Returns the root widget for packing.
    pub fn widget(&self) -> &gtk4::Box {
        &self.container
    }

    fn connect_signals(&self) {
        // LED 1 color picker - set LED 1 only (LED 2 stays off).
        let save_name1 = self.device_name.clone();
        self.color_button1.connect_color_set(clone!(
            #[strong(rename_to = dice_cell)]
            self.dice.clone(),
            move |button| {
                let rgba = button.rgba();
                let color = LedColor::new((rgba.red() * 255.0) as u8, (rgba.green() * 255.0) as u8, (rgba.blue() * 255.0) as u8);
                if let Some(name) = save_name1.borrow().as_ref() {
                    let mut settings = crate::config_dir::load_dice_settings(name).unwrap_or_default();
                    settings.led_color1 = color;
                    crate::config_dir::save_dice_settings(name, &settings);
                }
                if let Some(dice) = dice_cell.borrow().as_ref() {
                    let dice = dice.clone();
                    tokio::spawn(async move {
                        if let Err(error) = dice.set_leds_immediate(color, LedColor::OFF).await {
                            debug!(error = %error, "failed to set LED 1");
                        }
                    });
                }
            }
        ));

        // LED 2 color picker - set LED 2 only (LED 1 stays off).
        let save_name2 = self.device_name.clone();
        self.color_button2.connect_color_set(clone!(
            #[strong(rename_to = dice_cell)]
            self.dice.clone(),
            move |button| {
                let rgba = button.rgba();
                let color = LedColor::new((rgba.red() * 255.0) as u8, (rgba.green() * 255.0) as u8, (rgba.blue() * 255.0) as u8);
                if let Some(name) = save_name2.borrow().as_ref() {
                    let mut settings = crate::config_dir::load_dice_settings(name).unwrap_or_default();
                    settings.led_color2 = color;
                    crate::config_dir::save_dice_settings(name, &settings);
                }
                if let Some(dice) = dice_cell.borrow().as_ref() {
                    let dice = dice.clone();
                    tokio::spawn(async move {
                        if let Err(error) = dice.set_leds_immediate(LedColor::OFF, color).await {
                            debug!(error = %error, "failed to set LED 2");
                        }
                    });
                }
            }
        ));

        // Set button - set both LEDs to their respective picker colors.
        let save_name_set = self.device_name.clone();
        self.set_button.connect_clicked(clone!(
            #[strong(rename_to = dice_cell)]
            self.dice.clone(),
            #[strong(rename_to = color_button1)]
            self.color_button1.clone(),
            #[strong(rename_to = color_button2)]
            self.color_button2.clone(),
            move |_| {
                let rgba1 = color_button1.rgba();
                let color1 = LedColor::new((rgba1.red() * 255.0) as u8, (rgba1.green() * 255.0) as u8, (rgba1.blue() * 255.0) as u8);
                let rgba2 = color_button2.rgba();
                let color2 = LedColor::new((rgba2.red() * 255.0) as u8, (rgba2.green() * 255.0) as u8, (rgba2.blue() * 255.0) as u8);
                if let Some(name) = save_name_set.borrow().as_ref() {
                    let mut settings = crate::config_dir::load_dice_settings(name).unwrap_or_default();
                    settings.led_color1 = color1;
                    settings.led_color2 = color2;
                    crate::config_dir::save_dice_settings(name, &settings);
                }
                if let Some(dice) = dice_cell.borrow().as_ref() {
                    let dice = dice.clone();
                    tokio::spawn(async move {
                        if let Err(error) = dice.set_leds_immediate(color1, color2).await {
                            debug!(error = %error, "failed to set LEDs");
                        }
                    });
                }
            }
        ));

        // Pulse button - pulse LEDs with selected blink mode and LED selection.
        self.pulse_button.connect_clicked(clone!(
            #[strong(rename_to = dice_cell)]
            self.dice.clone(),
            #[strong(rename_to = color_button1)]
            self.color_button1.clone(),
            #[strong(rename_to = blink_mode_dropdown)]
            self.blink_mode_dropdown.clone(),
            #[strong(rename_to = leds_dropdown)]
            self.leds_dropdown.clone(),
            move |_| {
                let rgba = color_button1.rgba();
                let color = LedColor::new((rgba.red() * 255.0) as u8, (rgba.green() * 255.0) as u8, (rgba.blue() * 255.0) as u8);
                let blink_mode = match blink_mode_dropdown.selected() {
                    0 => PulseBlinkMode::Rainbow,
                    _ => PulseBlinkMode::Color,
                };
                let leds = match leds_dropdown.selected() {
                    1 => PulseLeds::Led1,
                    2 => PulseLeds::Led2,
                    _ => PulseLeds::Both,
                };
                if let Some(dice) = dice_cell.borrow().as_ref() {
                    let dice = dice.clone();
                    tokio::spawn(async move {
                        if let Err(error) = dice.pulse_leds(5, 10, 10, color, blink_mode, leds).await {
                            debug!(error = %error, "failed to pulse LEDs");
                        }
                    });
                }
            }
        ));

        // Off button - turn both LEDs off.
        let save_name_off = self.device_name.clone();
        self.off_button.connect_clicked(clone!(
            #[strong(rename_to = dice_cell)]
            self.dice.clone(),
            move |_| {
                if let Some(name) = save_name_off.borrow().as_ref() {
                    let mut settings = crate::config_dir::load_dice_settings(name).unwrap_or_default();
                    settings.led_color1 = LedColor::OFF;
                    settings.led_color2 = LedColor::OFF;
                    crate::config_dir::save_dice_settings(name, &settings);
                }
                if let Some(dice) = dice_cell.borrow().as_ref() {
                    let dice = dice.clone();
                    tokio::spawn(async move {
                        if let Err(error) = dice.set_leds_immediate(LedColor::OFF, LedColor::OFF).await {
                            debug!(error = %error, "failed to turn off LEDs");
                        }
                    });
                }
            }
        ));
    }
}

impl Default for LedControls {
    fn default() -> Self {
        Self::new()
    }
}
