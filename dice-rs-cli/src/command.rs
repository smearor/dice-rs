use clap::Subcommand;

use crate::led_action::LedAction;

/// Top-level subcommands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Scan for GoDice devices.
    Scan {
        /// Scan duration in seconds.
        #[arg(short, long, default_value = "5")]
        duration: u64,
    },

    /// Listen for dice events from a connected device.
    Listen {
        /// Device MAC address.
        address: String,
        /// Dice type (d6, d20, d10, d10x, d4, d8, d12).
        #[arg(short, long, default_value = "d6")]
        dice_type: String,
    },

    /// Query battery level of a dice.
    Battery {
        /// Device MAC address.
        address: String,
    },

    /// Control LEDs on a dice.
    Led {
        /// Device MAC address.
        address: String,
        #[command(subcommand)]
        action: LedAction,
    },

    /// Calibrate a dice on a flat surface.
    Calibrate {
        /// Device MAC address.
        address: String,
    },

    /// Get comprehensive system status of a dice.
    Status {
        /// Device MAC address.
        address: String,
    },

    /// Get the physical color of a dice.
    Color {
        /// Device MAC address.
        address: String,
    },

    /// Check if a dice is currently charging.
    Charging {
        /// Device MAC address.
        address: String,
    },

    /// Interactive REPL mode for exploratory use.
    Interactive,
}
