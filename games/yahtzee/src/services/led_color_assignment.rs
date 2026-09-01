use crate::models::player::Player;
use crate::models::player_color::PlayerColor;
use crate::models::player_index::PlayerIndex;
use crate::services::led_effect::LedEffect;

/// Manages the assignment of LED colors to players.
///
/// This service is responsible for translating player colors into
/// LED effects that can be applied to the physical dice. It provides
/// methods for the active player indicator and turn transition effects.
pub struct LedColorAssignment {
    /// The color assignments for each player, indexed by player index.
    assignments: Vec<PlayerColor>,
}

impl LedColorAssignment {
    /// Create a new color assignment from a list of players.
    ///
    /// Each player's color is extracted and stored for quick lookup.
    pub fn new(players: &[Player]) -> Self {
        let assignments = players.iter().map(|p| p.color()).collect();
        Self { assignments }
    }

    /// Get the color assigned to a player by index.
    ///
    /// Returns `None` if the index is out of range.
    pub fn color_for(&self, index: PlayerIndex) -> Option<PlayerColor> {
        self.assignments.get(index.get()).copied()
    }

    /// Create the LED effect for the active player indicator.
    ///
    /// All dice glow in the active player's color.
    pub fn active_player_effect(&self, index: PlayerIndex) -> Option<LedEffect> {
        self.color_for(index).map(LedEffect::active_player)
    }

    /// Create the LED effect for a turn transition.
    ///
    /// In multi-player mode, this pulses the dice in the new player's
    /// color to signal the handoff.
    pub fn turn_transition_effect(&self, index: PlayerIndex) -> Option<LedEffect> {
        let color = self.color_for(index)?;
        Some(LedEffect::Celebrate { pulse_count: 2, color })
    }

    /// Create the LED effect to turn all dice off.
    pub fn all_off_effect() -> LedEffect {
        LedEffect::all_off()
    }

    /// Returns the number of assigned players.
    pub fn player_count(&self) -> usize {
        self.assignments.len()
    }

    /// Returns true if all players have distinct colors.
    pub fn has_distinct_colors(&self) -> bool {
        for i in 0..self.assignments.len() {
            for j in (i + 1)..self.assignments.len() {
                if self.assignments[i] == self.assignments[j] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::player_name::PlayerName;
    use crate::models::player_type::PlayerType;

    fn make_players() -> Vec<Player> {
        vec![
            Player::new(PlayerName::new("Alice").unwrap(), PlayerColor::RED, PlayerType::Human),
            Player::new(PlayerName::new("Bob").unwrap(), PlayerColor::BLUE, PlayerType::Human),
            Player::new(PlayerName::new("Carol").unwrap(), PlayerColor::GREEN, PlayerType::Human),
        ]
    }

    #[test]
    fn new_assigns_colors() {
        let players = make_players();
        let assignment = LedColorAssignment::new(&players);
        assert_eq!(assignment.player_count(), 3);
    }

    #[test]
    fn color_for_valid_index() {
        let players = make_players();
        let assignment = LedColorAssignment::new(&players);
        assert_eq!(assignment.color_for(PlayerIndex::new(0)), Some(PlayerColor::RED));
        assert_eq!(assignment.color_for(PlayerIndex::new(1)), Some(PlayerColor::BLUE));
        assert_eq!(assignment.color_for(PlayerIndex::new(2)), Some(PlayerColor::GREEN));
    }

    #[test]
    fn color_for_out_of_range() {
        let players = make_players();
        let assignment = LedColorAssignment::new(&players);
        assert_eq!(assignment.color_for(PlayerIndex::new(3)), None);
    }

    #[test]
    fn active_player_effect() {
        let players = make_players();
        let assignment = LedColorAssignment::new(&players);
        let effect = assignment.active_player_effect(PlayerIndex::new(0));
        assert!(effect.is_some());
        assert_eq!(effect.unwrap(), LedEffect::active_player(PlayerColor::RED));
    }

    #[test]
    fn active_player_effect_out_of_range() {
        let players = make_players();
        let assignment = LedColorAssignment::new(&players);
        assert!(assignment.active_player_effect(PlayerIndex::new(10)).is_none());
    }

    #[test]
    fn turn_transition_effect() {
        let players = make_players();
        let assignment = LedColorAssignment::new(&players);
        let effect = assignment.turn_transition_effect(PlayerIndex::new(1));
        assert!(effect.is_some());
        assert_eq!(
            effect.unwrap(),
            LedEffect::Celebrate {
                pulse_count: 2,
                color: PlayerColor::BLUE
            }
        );
    }

    #[test]
    fn all_off_effect() {
        assert_eq!(LedColorAssignment::all_off_effect(), LedEffect::all_off());
    }

    #[test]
    fn has_distinct_colors_true() {
        let players = make_players();
        let assignment = LedColorAssignment::new(&players);
        assert!(assignment.has_distinct_colors());
    }

    #[test]
    fn has_distinct_colors_false() {
        let players = vec![
            Player::new(PlayerName::new("Alice").unwrap(), PlayerColor::RED, PlayerType::Human),
            Player::new(PlayerName::new("Bob").unwrap(), PlayerColor::RED, PlayerType::Human),
        ];
        let assignment = LedColorAssignment::new(&players);
        assert!(!assignment.has_distinct_colors());
    }
}
