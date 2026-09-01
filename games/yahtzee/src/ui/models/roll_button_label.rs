use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// The text displayed on the roll button.
///
/// Encodes the button label as an enum to avoid stringly-typed APIs
/// and allow German localization in one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RollButtonLabel {
    /// Initial state — start the first roll.
    Wuerfeln,
    /// Second roll — roll again.
    NochmalWuerfeln,
    /// Third and final roll — last chance.
    LetzterWurf,
    /// No rolls remaining — must enter a score.
    KeineWuerfeUbrig,
    /// Waiting for dice to become stable.
    WurfLauft,
    /// Game is over.
    SpielBeendet,
}

impl RollButtonLabel {
    /// Get the label text for the button.
    pub fn text(self) -> &'static str {
        match self {
            Self::Wuerfeln => "Würfeln",
            Self::NochmalWuerfeln => "Nochmal würfeln",
            Self::LetzterWurf => "Letzter Wurf",
            Self::KeineWuerfeUbrig => "Keine Würfe übrig",
            Self::WurfLauft => "Wurf läuft...",
            Self::SpielBeendet => "Spiel beendet",
        }
    }

    /// Whether the button should be sensitive (clickable).
    pub fn is_sensitive(self) -> bool {
        match self {
            Self::Wuerfeln | Self::NochmalWuerfeln | Self::LetzterWurf => true,
            Self::KeineWuerfeUbrig | Self::WurfLauft | Self::SpielBeendet => false,
        }
    }

    /// Derive the label from the current roll count (1-based).
    pub fn from_roll_count(roll: u8) -> Self {
        match roll {
            0 => Self::Wuerfeln,
            1 => Self::NochmalWuerfeln,
            2 => Self::LetzterWurf,
            _ => Self::KeineWuerfeUbrig,
        }
    }
}

impl Display for RollButtonLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_wuerfeln() {
        assert_eq!(RollButtonLabel::Wuerfeln.text(), "Würfeln");
    }

    #[test]
    fn text_nochmal() {
        assert_eq!(RollButtonLabel::NochmalWuerfeln.text(), "Nochmal würfeln");
    }

    #[test]
    fn text_letzter() {
        assert_eq!(RollButtonLabel::LetzterWurf.text(), "Letzter Wurf");
    }

    #[test]
    fn sensitive_for_active_rolls() {
        assert!(RollButtonLabel::Wuerfeln.is_sensitive());
        assert!(RollButtonLabel::NochmalWuerfeln.is_sensitive());
        assert!(RollButtonLabel::LetzterWurf.is_sensitive());
    }

    #[test]
    fn insensitive_for_inactive_states() {
        assert!(!RollButtonLabel::KeineWuerfeUbrig.is_sensitive());
        assert!(!RollButtonLabel::WurfLauft.is_sensitive());
        assert!(!RollButtonLabel::SpielBeendet.is_sensitive());
    }

    #[test]
    fn from_roll_count_zero() {
        assert_eq!(RollButtonLabel::from_roll_count(0), RollButtonLabel::Wuerfeln);
    }

    #[test]
    fn from_roll_count_one() {
        assert_eq!(RollButtonLabel::from_roll_count(1), RollButtonLabel::NochmalWuerfeln);
    }

    #[test]
    fn from_roll_count_two() {
        assert_eq!(RollButtonLabel::from_roll_count(2), RollButtonLabel::LetzterWurf);
    }

    #[test]
    fn from_roll_count_three() {
        assert_eq!(RollButtonLabel::from_roll_count(3), RollButtonLabel::KeineWuerfeUbrig);
    }

    #[test]
    fn display() {
        assert_eq!(RollButtonLabel::Wuerfeln.to_string(), "Würfeln");
    }
}
