use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

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
        Self::new("Bereit. Würfel scannen, um zu beginnen.")
    }

    /// A scanning message.
    pub fn scanning() -> Self {
        Self::new("Scanne nach GoDice...")
    }

    /// A no-devices-found message.
    pub fn no_devices() -> Self {
        Self::new("Keine GoDice gefunden.")
    }

    /// A devices-found message.
    pub fn devices_found(count: usize) -> Self {
        Self::new(format!("{count} GoDice gefunden, verbinde..."))
    }

    /// A dice-connected message.
    pub fn dice_connected(slot: u8, name: &str) -> Self {
        Self::new(format!("Würfel {slot} verbunden: {name}"))
    }

    /// An all-dice-connected message.
    pub fn all_dice_connected() -> Self {
        Self::new("Alle 5 Würfel verbunden. Spiel bereit!")
    }

    /// A connection-failed message.
    pub fn connection_failed(name: &str, error: &str) -> Self {
        Self::new(format!("Verbindung fehlgeschlagen für {name}: {error}"))
    }

    /// A scan-failed message.
    pub fn scan_failed(error: &str) -> Self {
        Self::new(format!("Scan fehlgeschlagen: {error}"))
    }

    /// A roll-started message.
    pub fn roll_started() -> Self {
        Self::new("Würfeln...")
    }

    /// A roll-complete message.
    pub fn roll_complete() -> Self {
        Self::new("Wurf abgeschlossen. Kategorie wählen.")
    }

    /// A roll-timed-out message.
    pub fn roll_timed_out() -> Self {
        Self::new("Zeitüberschreitung beim Würfeln.")
    }

    /// A dice-disconnected message.
    pub fn dice_disconnected(slot: u8) -> Self {
        Self::new(format!("Würfel {slot} getrennt."))
    }

    /// A score-entered message.
    pub fn score_entered(player: &str, category: &str, score: u32) -> Self {
        Self::new(format!("{player}: {category} = {score} Punkte"))
    }

    /// A game-over message.
    pub fn game_over() -> Self {
        Self::new("Spiel beendet!")
    }

    /// A waiting-for-roll message.
    pub fn waiting_for_roll(player: &str) -> Self {
        Self::new(format!("{player} ist am Zug. Würfeln zum Starten."))
    }

    /// A hold-toggled message.
    pub fn hold_toggled(slot: u8, held: bool) -> Self {
        if held {
            Self::new(format!("Würfel {slot} gehalten"))
        } else {
            Self::new(format!("Würfel {slot} freigegeben"))
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
        assert!(msg.as_str().contains("gehalten"));
    }

    #[test]
    fn hold_toggled_released() {
        let msg = UiMessage::hold_toggled(1, false);
        assert!(msg.as_str().contains("freigegeben"));
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
