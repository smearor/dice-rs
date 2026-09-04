use crate::i18n::Localized;
use crate::impl_display_localized;

/// The text displayed on the roll button.
///
/// Encodes the button label as an enum to avoid stringly-typed APIs
/// and allow localization in one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RollButtonLabel {
    /// Initial state — start the first roll.
    Roll,
    /// Second roll — roll again.
    Reroll,
    /// Third and final roll — last chance.
    LastRoll,
    /// No rolls remaining — must enter a score.
    NoRollsLeft,
    /// Waiting for dice to become stable.
    Rolling,
    /// Game is over.
    GameOver,
}

impl RollButtonLabel {
    /// Whether the button should be sensitive (clickable).
    pub fn is_sensitive(self) -> bool {
        match self {
            Self::Roll | Self::Reroll | Self::LastRoll => true,
            Self::NoRollsLeft | Self::Rolling | Self::GameOver => false,
        }
    }

    /// Derive the label from the current roll count (1-based).
    pub fn from_roll_count(roll: u8) -> Self {
        match roll {
            0 => Self::Roll,
            1 => Self::Reroll,
            2 => Self::LastRoll,
            _ => Self::NoRollsLeft,
        }
    }
}

impl Localized for RollButtonLabel {
    fn fluent_key(&self) -> &'static str {
        match self {
            Self::Roll => "roll-button-roll",
            Self::Reroll => "roll-button-reroll",
            Self::LastRoll => "roll-button-last",
            Self::NoRollsLeft => "roll-button-none",
            Self::Rolling => "roll-button-rolling",
            Self::GameOver => "roll-button-game-over",
        }
    }
}

impl_display_localized!(RollButtonLabel);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_roll() {
        assert!(!RollButtonLabel::Roll.localized().is_empty());
    }

    #[test]
    fn text_reroll() {
        assert!(!RollButtonLabel::Reroll.localized().is_empty());
    }

    #[test]
    fn text_last_roll() {
        assert!(!RollButtonLabel::LastRoll.localized().is_empty());
    }

    #[test]
    fn sensitive_for_active_rolls() {
        assert!(RollButtonLabel::Roll.is_sensitive());
        assert!(RollButtonLabel::Reroll.is_sensitive());
        assert!(RollButtonLabel::LastRoll.is_sensitive());
    }

    #[test]
    fn insensitive_for_inactive_states() {
        assert!(!RollButtonLabel::NoRollsLeft.is_sensitive());
        assert!(!RollButtonLabel::Rolling.is_sensitive());
        assert!(!RollButtonLabel::GameOver.is_sensitive());
    }

    #[test]
    fn from_roll_count_zero() {
        assert_eq!(RollButtonLabel::from_roll_count(0), RollButtonLabel::Roll);
    }

    #[test]
    fn from_roll_count_one() {
        assert_eq!(RollButtonLabel::from_roll_count(1), RollButtonLabel::Reroll);
    }

    #[test]
    fn from_roll_count_two() {
        assert_eq!(RollButtonLabel::from_roll_count(2), RollButtonLabel::LastRoll);
    }

    #[test]
    fn from_roll_count_three() {
        assert_eq!(RollButtonLabel::from_roll_count(3), RollButtonLabel::NoRollsLeft);
    }

    #[test]
    fn display() {
        assert!(!RollButtonLabel::Roll.to_string().is_empty());
    }
}
