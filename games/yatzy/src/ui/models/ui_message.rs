use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::fl;
use crate::i18n;

/// A user-facing message displayed in the UI status area.
///
/// Wraps the message text in a newtype to avoid stringly-typed APIs
/// and allow future localization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiMessage(String);

impl UiMessage {
    /// Create a new UI message from any string-like value.
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    /// Get the message text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// A ready-state message.
    pub fn ready() -> Self {
        Self::new(fl!("status-ready"))
    }

    /// A scanning message.
    pub fn scanning() -> Self {
        Self::new(fl!("status-scanning"))
    }

    /// A no-devices-found message.
    pub fn no_devices() -> Self {
        Self::new(fl!("status-no-devices"))
    }

    /// A devices-found message.
    pub fn devices_found(count: usize) -> Self {
        Self::new(i18n::get_int("status-devices-found", "count", count as i64))
    }

    /// A dice-connected message.
    pub fn dice_connected(slot: u8, name: &str) -> Self {
        Self::new(i18n::get_int_str("status-dice-connected", "slot", slot as i64, "name", name))
    }

    /// An all-dice-connected message.
    pub fn all_dice_connected() -> Self {
        Self::new(fl!("status-all-dice-connected"))
    }

    /// A connection-failed message.
    pub fn connection_failed(name: &str, error: &str) -> Self {
        Self::new(i18n::get_str_str("status-connection-failed", "name", name, "error", error))
    }

    /// A scan-failed message.
    pub fn scan_failed(error: &str) -> Self {
        Self::new(i18n::get_str("status-scan-failed", "error", error))
    }

    /// A roll-started message.
    pub fn roll_started() -> Self {
        Self::new(fl!("status-roll-started"))
    }

    /// A roll-complete message.
    pub fn roll_complete() -> Self {
        Self::new(fl!("status-roll-complete"))
    }

    /// A roll-timed-out message.
    pub fn roll_timed_out() -> Self {
        Self::new(fl!("status-roll-timed-out"))
    }

    /// A dice-disconnected message.
    pub fn dice_disconnected(slot: u8) -> Self {
        Self::new(i18n::get_int("status-dice-disconnected", "slot", slot as i64))
    }

    /// A score-entered message.
    pub fn score_entered(player: &str, category: &str, score: u32) -> Self {
        Self::new(i18n::get_str_str_int("status-score-entered", "player", player, "category", category, "score", score as i64))
    }

    /// A game-over message.
    pub fn game_over() -> Self {
        Self::new(fl!("status-game-over"))
    }

    /// A waiting-for-roll message.
    pub fn waiting_for_roll(player: &str) -> Self {
        Self::new(i18n::get_str("status-waiting-for-roll", "player", player))
    }

    /// A hold-toggled message.
    pub fn hold_toggled(slot: u8, held: bool) -> Self {
        if held {
            Self::new(i18n::get_int("status-hold-toggled-held", "slot", slot as i64))
        } else {
            Self::new(i18n::get_int("status-hold-toggled-released", "slot", slot as i64))
        }
    }
}

impl Display for UiMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UiMessage {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for UiMessage {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_as_str() {
        let msg = UiMessage::new("Hello");
        assert_eq!(msg.as_str(), "Hello");
    }

    #[test]
    fn display() {
        let msg = UiMessage::new("Test message");
        assert_eq!(msg.to_string(), "Test message");
    }

    #[test]
    fn ready_message() {
        let msg = UiMessage::ready();
        assert!(!msg.as_str().is_empty());
    }

    #[test]
    fn devices_found_includes_count() {
        let msg = UiMessage::devices_found(3);
        assert!(msg.as_str().contains('3'));
    }

    #[test]
    fn dice_connected_includes_slot_and_name() {
        let msg = UiMessage::dice_connected(2, "GoDice_001234");
        assert!(msg.as_str().contains('2'));
        assert!(msg.as_str().contains("GoDice_001234"));
    }

    #[test]
    fn hold_toggled_held() {
        let msg = UiMessage::hold_toggled(1, true);
        assert!(msg.as_str().contains('1'));
        assert!(!msg.as_str().is_empty());
    }

    #[test]
    fn hold_toggled_released() {
        let msg = UiMessage::hold_toggled(1, false);
        assert!(msg.as_str().contains('1'));
        assert!(!msg.as_str().is_empty());
    }

    #[test]
    fn from_string() {
        let msg = UiMessage::from("test".to_string());
        assert_eq!(msg.as_str(), "test");
    }

    #[test]
    fn from_str() {
        let msg = UiMessage::from("test");
        assert_eq!(msg.as_str(), "test");
    }
}
