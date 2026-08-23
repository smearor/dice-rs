use clap::Subcommand;

/// LED subcommand actions.
#[derive(Subcommand, Debug)]
pub enum LedAction {
    /// Set both LEDs to a color.
    Set {
        /// Color as hex (e.g. FF0000) or named (red, green, blue, white, off).
        color: String,
    },

    /// Set each LED independently.
    SetDual {
        /// LED 1 color.
        led1: String,
        /// LED 2 color.
        led2: String,
    },

    /// Pulse both LEDs.
    Pulse {
        /// Color as hex or named.
        color: String,
        /// Number of pulse cycles.
        #[arg(short, long, default_value = "3")]
        count: u8,
        /// On time in 10ms units.
        #[arg(short, long, default_value = "10")]
        on_time: u8,
        /// Off time in 10ms units.
        #[arg(long, default_value = "10")]
        off_time: u8,
    },

    /// Turn both LEDs off.
    Off,
}
