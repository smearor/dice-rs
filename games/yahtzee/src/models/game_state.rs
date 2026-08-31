use crate::error::Result;
use crate::error::YahtzeeError;
use crate::models::dice_set::DiceSet;
use crate::models::player::Player;
use crate::models::player_index::PlayerIndex;
use crate::models::roll_count::RollCount;
use crate::models::round_number::RoundNumber;
use crate::models::score::Score;
use crate::models::turn_phase::TurnPhase;
use serde::Deserialize;
use serde::Serialize;

/// Overall game state managing players, turns, and rounds.
///
/// The game state tracks the current player, round, turn phase, roll
/// count, and dice set. It enforces the game flow: players take turns
/// rolling up to 3 times, then must enter a score. After 13 rounds,
/// the game is over.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// All players in the game.
    players: Vec<Player>,
    /// The index of the current player.
    current_player: PlayerIndex,
    /// The current round number (1-13).
    round: RoundNumber,
    /// The current phase of the turn.
    phase: TurnPhase,
    /// How many rolls the current player has used this turn.
    rolls_used: RollCount,
    /// The current dice set.
    dice_set: DiceSet,
}

impl GameState {
    /// Create a new game with the given players.
    ///
    /// Returns an error if the player list is empty.
    pub fn new(players: Vec<Player>) -> Result<Self> {
        if players.is_empty() {
            return Err(YahtzeeError::NotEnoughPlayers(0));
        }
        Ok(Self {
            players,
            current_player: PlayerIndex::new(0),
            round: RoundNumber::FIRST,
            phase: TurnPhase::AwaitingRoll,
            rolls_used: RollCount::FIRST,
            dice_set: DiceSet::new(),
        })
    }

    /// Get all players.
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    /// Get a mutable reference to all players.
    pub fn players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    /// Get the current player.
    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player.get()]
    }

    /// Get a mutable reference to the current player.
    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player.get()]
    }

    /// Get the current player index.
    pub fn current_player_index(&self) -> PlayerIndex {
        self.current_player
    }

    /// Get the current round number.
    pub fn round(&self) -> RoundNumber {
        self.round
    }

    /// Get the current turn phase.
    pub fn phase(&self) -> TurnPhase {
        self.phase
    }

    /// Set the turn phase.
    pub fn set_phase(&mut self, phase: TurnPhase) {
        self.phase = phase;
    }

    /// Get the number of rolls used in the current turn.
    pub fn rolls_used(&self) -> RollCount {
        self.rolls_used
    }

    /// Get the current dice set.
    pub fn dice_set(&self) -> &DiceSet {
        &self.dice_set
    }

    /// Get a mutable reference to the dice set.
    pub fn dice_set_mut(&mut self) -> &mut DiceSet {
        &mut self.dice_set
    }

    /// Increment the roll count. Returns an error if max rolls reached.
    pub fn increment_rolls(&mut self) -> Result<()> {
        self.rolls_used = self.rolls_used.increment()?;
        Ok(())
    }

    /// Returns true if the current player has used all 3 rolls.
    pub fn rolls_exhausted(&self) -> bool {
        self.rolls_used.is_exhausted()
    }

    /// Start a new roll: reset dice holds and set phase to Rolling.
    pub fn start_roll(&mut self) {
        self.dice_set.clear_holds();
        self.phase = TurnPhase::Rolling;
    }

    /// Mark dice as stable and transition to Holding phase.
    pub fn dice_stable(&mut self) {
        self.phase = TurnPhase::Holding;
    }

    /// Transition to the scoring phase.
    pub fn enter_scoring(&mut self) {
        self.phase = TurnPhase::Scoring;
    }

    /// Enter a score for the current player in the given category,
    /// then advance to the next turn.
    pub fn enter_score(&mut self, category: crate::models::category::ScoreCategory, score: Score) -> Result<()> {
        self.current_player_mut().scorecard_mut().enter(category, score)?;
        self.end_turn();
        Ok(())
    }

    /// Cross out a category for the current player, then advance to the next turn.
    pub fn cross_out(&mut self, category: crate::models::category::ScoreCategory) -> Result<()> {
        self.current_player_mut().scorecard_mut().cross_out(category)?;
        self.end_turn();
        Ok(())
    }

    /// End the current turn and advance to the next player or round.
    fn end_turn(&mut self) {
        self.phase = TurnPhase::TurnEnd;
        let player_count = self.players.len();
        let next_player = self.current_player.next_wrapping(player_count);

        // If we wrapped back to player 0, advance the round.
        if next_player.get() == 0
            && let Ok(next_round) = self.round.increment()
        {
            self.round = next_round;
        }
        // If round.increment() fails, we're on the last round — game over.

        self.current_player = next_player;
        self.rolls_used = RollCount::FIRST;
        self.dice_set.reset();
        self.phase = TurnPhase::AwaitingRoll;
    }

    /// Advance to the next player's turn manually.
    pub fn next_turn(&mut self) {
        self.end_turn();
    }

    /// Check if the game is over (all 13 rounds completed by all players).
    pub fn is_game_over(&self) -> bool {
        self.players.iter().all(|p| p.scorecard().is_complete())
    }

    /// Get the winner(s) with the highest grand total score.
    /// Returns an empty slice if the game is not over.
    pub fn winners(&self) -> Vec<&Player> {
        if !self.is_game_over() {
            return Vec::new();
        }
        let max_score = self.players.iter().map(|p| p.grand_total()).max().unwrap_or(Score::ZERO);
        self.players.iter().filter(|p| p.grand_total() == max_score).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::category::ScoreCategory;
    use crate::models::player_color::PlayerColor;
    use crate::models::player_name::PlayerName;
    use crate::models::player_type::PlayerType;

    fn make_players(count: usize) -> Vec<Player> {
        (0..count)
            .map(|i| {
                Player::new(
                    PlayerName::new(format!("Player {i}")).unwrap(),
                    PlayerColor::DEFAULTS[i % PlayerColor::DEFAULTS.len()],
                    PlayerType::Human,
                )
            })
            .collect()
    }

    #[test]
    fn new_game_starts_at_round_1_player_0() {
        let state = GameState::new(make_players(2)).unwrap();
        assert_eq!(state.round(), RoundNumber::FIRST);
        assert_eq!(state.current_player_index().get(), 0);
        assert_eq!(state.phase(), TurnPhase::AwaitingRoll);
        assert_eq!(state.rolls_used(), RollCount::FIRST);
    }

    #[test]
    fn new_game_empty_players_fails() {
        assert!(GameState::new(Vec::new()).is_err());
    }

    #[test]
    fn increment_rolls() {
        let mut state = GameState::new(make_players(1)).unwrap();
        state.increment_rolls().unwrap();
        assert_eq!(state.rolls_used().get(), 2);
    }

    #[test]
    fn increment_rolls_at_max_fails() {
        let mut state = GameState::new(make_players(1)).unwrap();
        state.increment_rolls().unwrap();
        state.increment_rolls().unwrap();
        assert!(state.increment_rolls().is_err());
    }

    #[test]
    fn enter_score_advances_turn() {
        let mut state = GameState::new(make_players(2)).unwrap();
        state.enter_score(ScoreCategory::Ones, Score::new(3)).unwrap();
        assert_eq!(state.current_player_index().get(), 1);
        assert_eq!(state.rolls_used(), RollCount::FIRST);
        assert_eq!(state.phase(), TurnPhase::AwaitingRoll);
    }

    #[test]
    fn enter_score_wraps_to_player_0_and_advances_round() {
        let mut state = GameState::new(make_players(2)).unwrap();
        // Player 0
        state.enter_score(ScoreCategory::Ones, Score::new(3)).unwrap();
        // Player 1
        state.enter_score(ScoreCategory::Ones, Score::new(2)).unwrap();
        // Should be back to player 0, round 2
        assert_eq!(state.current_player_index().get(), 0);
        assert_eq!(state.round().get(), 2);
    }

    #[test]
    fn cross_out_advances_turn() {
        let mut state = GameState::new(make_players(2)).unwrap();
        state.cross_out(ScoreCategory::Yahtzee).unwrap();
        assert_eq!(state.current_player_index().get(), 1);
    }

    #[test]
    fn is_game_over_false_at_start() {
        let state = GameState::new(make_players(2)).unwrap();
        assert!(!state.is_game_over());
    }

    #[test]
    fn is_game_over_true_when_all_complete() {
        let mut state = GameState::new(make_players(1)).unwrap();
        // Fill all 13 categories
        for (i, cat) in ScoreCategory::ALL.iter().enumerate() {
            state.enter_score(*cat, Score::new((i + 1) as u32)).unwrap();
        }
        assert!(state.is_game_over());
    }

    #[test]
    fn winners_returns_highest_score() {
        let mut state = GameState::new(make_players(2)).unwrap();
        // Fill all 13 categories for both players over 13 rounds.
        // Each round: player 0 enters category[i], player 1 enters category[i].
        for (i, cat) in ScoreCategory::ALL.iter().enumerate() {
            state.enter_score(*cat, Score::new((i + 1) as u32)).unwrap();
            state.enter_score(*cat, Score::new((i + 2) as u32)).unwrap();
        }
        // Player 1 has higher scores — single winner
        assert_eq!(state.winners().len(), 1);
        assert_eq!(state.winners()[0].name().as_str(), "Player 1");
    }

    #[test]
    fn winners_empty_before_game_over() {
        let state = GameState::new(make_players(2)).unwrap();
        assert!(state.winners().is_empty());
    }

    #[test]
    fn start_roll_clears_holds_and_sets_phase() {
        let mut state = GameState::new(make_players(1)).unwrap();
        state.dice_set_mut().set_holds(crate::models::hold_mask::HoldMask::all());
        state.start_roll();
        assert_eq!(state.phase(), TurnPhase::Rolling);
        assert_eq!(state.dice_set().holds().held_count(), 0);
    }

    #[test]
    fn phase_transitions() {
        let mut state = GameState::new(make_players(1)).unwrap();
        state.start_roll();
        assert_eq!(state.phase(), TurnPhase::Rolling);
        state.dice_stable();
        assert_eq!(state.phase(), TurnPhase::Holding);
        state.enter_scoring();
        assert_eq!(state.phase(), TurnPhase::Scoring);
    }
}
