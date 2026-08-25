//! dice-rs — Core library for GoDice BLE dice.
//!
//! Provides domain types, BLE transport abstraction, and a high-level
//! service API for scanning, connecting, and interacting with GoDice
//! devices over Bluetooth Low Energy.
//!
//! # Quick Start
//!
//! ```no_run
//! use dice_rs::{DiceManager, DiceEvent};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = DiceManager::new().await?;
//!     let devices = manager.scan().await?;
//!
//!     if devices.is_empty() {
//!         println!("No GoDice devices found");
//!         return Ok(());
//!     }
//!
//!     let dice = manager.connect(&devices[0]).await?;
//!     let mut receiver = dice.subscribe();
//!     while let Ok(event) = receiver.recv().await {
//!         match event {
//!             DiceEvent::Stable { face, .. } => println!("Rolled: {face}"),
//!             DiceEvent::RollStart => println!("Rolling..."),
//!             DiceEvent::Disconnected => break,
//!             _ => {}
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod ble;
pub mod error;
pub mod model;
pub mod service;

pub use model::acceleration::Acceleration;
pub use model::battery_level::BatteryLevel;
pub use model::charging_state::ChargingState;
pub use model::dice::DiceColor;
pub use model::dice::DiceType;
pub use model::face::FaceValue;
pub use model::led::LedColor;
pub use service::dice::Dice;
pub use service::dice::DiceDevice;
pub use service::dice::DiceEvent;
pub use service::manager::DiceManager;

#[cfg(test)]
mod tests {
    #[test]
    fn book_summary_has_all_chapters() {
        let summary = include_str!("../../book/src/SUMMARY.md");
        assert!(summary.contains("Introduction"));
        assert!(summary.contains("Getting Started"));
        assert!(summary.contains("Architecture"));
        assert!(summary.contains("BLE Protocol"));
        assert!(summary.contains("Scanning & Connecting"));
        assert!(summary.contains("Dice Events"));
        assert!(summary.contains("LED Control"));
        assert!(summary.contains("Battery & Status"));
        assert!(summary.contains("Calibration"));
        assert!(summary.contains("CLI Tool"));
        assert!(summary.contains("Controller"));
        assert!(summary.contains("WebSocket Server"));
        assert!(summary.contains("Platform Notes"));
    }

    #[test]
    fn changelog_has_unreleased_section() {
        let changelog = include_str!("../../CHANGELOG.md");
        assert!(changelog.contains("## Unreleased"));
        assert!(changelog.contains("### Added"));
        assert!(changelog.contains("### Changed"));
        assert!(changelog.contains("### Fixed"));
    }
}
