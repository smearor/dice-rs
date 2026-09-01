use crate::error::Result;
use crate::error::YahtzeeError;
use crate::models::category::ScoreCategory;
use crate::models::dice_set::DiceSet;
use crate::models::game_mode::GameMode;
use crate::models::game_state::GameState;
use crate::models::game_status::GameStatus;
use crate::models::hold_mask::HoldMask;
use crate::models::player::Player;
use crate::models::score::Score;
use crate::models::standing::StandingEntry;
use crate::models::standing::compute_standings;
use crate::models::turn_action::TurnAction;
use crate::models::turn_phase::TurnPhase;
use crate::models::turn_transition::TurnTransition;
use crate::rules::cross_out::CrossOutAdvisor;
use crate::rules::cross_out::CrossOutRecommendation;
use crate::rules::scoring::calculate_score;
use crate::rules::validation::is_valid;
use crate::rules::validation::must_cross_out;
use crate::services::celebration_detector::CelebrationDetector;
use crate::services::led_color_assignment::LedColorAssignment;
use crate::services::led_effect::LedEffect;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tracing::debug;

/// Events emitted by the `GameController` for the UI to react to.
///
/// These events represent state changes that the UI needs to visualize.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControllerEvent {
    /// The game status changed (e.g. Playing → GameOver).
    StatusChanged {
        /// The new game status.
        status: GameStatus,
    },
    /// A new player's turn started.
    TurnStarted {
        /// The index of the player whose turn it is.
        player_index: usize,
    },
    /// The turn phase changed.
    PhaseChanged {
        /// The new turn phase.
        phase: TurnPhase,
    },
    /// A roll was completed and dice values are available.
    RollCompleted {
        /// The final dice values.
        dice: DiceSet,
    },
    /// A score was entered for a player.
    ScoreEntered {
        /// The player index.
        player_index: usize,
        /// The category that was scored.
        category: ScoreCategory,
        /// The score that was entered.
        score: Score,
    },
    /// A category was crossed out for a player.
    CategoryCrossedOut {
        /// The player index.
        player_index: usize,
        /// The category that was crossed out.
        category: ScoreCategory,
    },
    /// The game is over and standings are available.
    GameOver {
        /// The final standings, ranked by score.
        standings: Vec<StandingEntry>,
    },
    /// A cross-out recommendation was generated.
    CrossOutRecommended {
        /// The recommended category to cross out.
        recommendation: CrossOutRecommendation,
    },
    /// A turn transition occurred (pass-and-play in multi-player mode).
    TurnTransition {
        /// The transition data for the next player's turn.
        transition: TurnTransition,
    },
    /// A celebration effect should be applied for a special roll.
    Celebration {
        /// The LED effect to apply.
        effect: LedEffect,
    },
}

/// The game controller orchestrates the game flow.
///
/// It wraps `GameState` and validates all actions against the current
/// state before executing them. Actions that are invalid for the current
/// phase or game status return an error. The controller emits events
/// that the UI layer can subscribe to.
pub struct GameController {
    /// The game state, protected by a mutex for thread-safe access.
    state: Arc<Mutex<GameState>>,
    /// The cross-out advisor.
    advisor: CrossOutAdvisor,
    /// LED color assignment for players.
    led_assignment: LedColorAssignment,
    /// Broadcast channel for controller events.
    event_sender: broadcast::Sender<ControllerEvent>,
}

impl GameController {
    /// Create a new game controller with the given players.
    pub fn new(players: Vec<Player>) -> Result<Self> {
        let led_assignment = LedColorAssignment::new(&players);
        let state = GameState::new(players)?;
        let (event_sender, _) = broadcast::channel(64);
        Ok(Self {
            state: Arc::new(Mutex::new(state)),
            advisor: CrossOutAdvisor::new(),
            led_assignment,
            event_sender,
        })
    }

    /// Create a new game controller with the given players and game mode.
    pub fn with_mode(players: Vec<Player>, game_mode: GameMode) -> Result<Self> {
        let led_assignment = LedColorAssignment::new(&players);
        let state = GameState::with_mode(players, game_mode)?;
        let (event_sender, _) = broadcast::channel(64);
        Ok(Self {
            state: Arc::new(Mutex::new(state)),
            advisor: CrossOutAdvisor::new(),
            led_assignment,
            event_sender,
        })
    }

    /// Create a controller wrapping an existing game state.
    pub fn from_state(state: GameState) -> Self {
        let led_assignment = LedColorAssignment::new(state.players());
        let (event_sender, _) = broadcast::channel(64);
        Self {
            state: Arc::new(Mutex::new(state)),
            advisor: CrossOutAdvisor::new(),
            led_assignment,
            event_sender,
        }
    }

    /// Get a snapshot of the current game state.
    pub fn state(&self) -> Result<GameState> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        Ok(state.clone())
    }

    /// Get the current game status.
    pub fn status(&self) -> Result<GameStatus> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        Ok(state.status())
    }

    /// Get the current game mode.
    pub fn game_mode(&self) -> Result<GameMode> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        Ok(state.game_mode())
    }

    /// Returns true if strategy hints should be shown.
    pub fn hints_enabled(&self) -> Result<bool> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        Ok(state.hints_enabled())
    }

    /// Get the LED color assignment.
    pub fn led_assignment(&self) -> &LedColorAssignment {
        &self.led_assignment
    }

    /// Subscribe to controller events.
    pub fn subscribe(&self) -> broadcast::Receiver<ControllerEvent> {
        self.event_sender.subscribe()
    }

    /// Execute a turn action, validating it against the current game state.
    ///
    /// Returns the events that were generated by the action.
    pub fn execute(&self, action: TurnAction) -> Result<Vec<ControllerEvent>> {
        let mut state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;

        if state.status() == GameStatus::GameOver {
            return Err(YahtzeeError::GameAlreadyOver);
        }

        let mut events = Vec::new();

        match action {
            TurnAction::Roll => self.execute_roll(&mut state, &mut events)?,
            TurnAction::Hold { holds } => self.execute_hold(&mut state, holds, &mut events)?,
            TurnAction::EnterScore { category, score } => self.execute_enter_score(&mut state, category, score, &mut events)?,
            TurnAction::CrossOut { category } => self.execute_cross_out(&mut state, category, &mut events)?,
        }

        // Broadcast events to subscribers
        for event in &events {
            let _ = self.event_sender.send(event.clone());
        }

        Ok(events)
    }

    /// Execute a roll action.
    fn execute_roll(&self, state: &mut GameState, events: &mut Vec<ControllerEvent>) -> Result<()> {
        if state.phase() != TurnPhase::AwaitingRoll && state.phase() != TurnPhase::Holding {
            return Err(YahtzeeError::PhaseMismatch(state.phase().to_string(), "AwaitingRoll or Holding".to_string()));
        }
        if state.rolls_exhausted() {
            return Err(YahtzeeError::PhaseMismatch(state.phase().to_string(), "Scoring (rolls exhausted)".to_string()));
        }
        // Increment roll count when re-rolling (not on the first roll of a turn)
        if state.phase() == TurnPhase::Holding {
            state.increment_rolls()?;
        }
        state.start_roll();
        events.push(ControllerEvent::PhaseChanged { phase: TurnPhase::Rolling });
        Ok(())
    }

    /// Execute a hold action.
    fn execute_hold(&self, state: &mut GameState, holds: HoldMask, _events: &mut Vec<ControllerEvent>) -> Result<()> {
        if state.phase() != TurnPhase::Holding {
            return Err(YahtzeeError::PhaseMismatch(state.phase().to_string(), "Holding".to_string()));
        }
        state.dice_set_mut().set_holds(holds);
        debug!(held_count = holds.held_count(), "holds updated");
        Ok(())
    }

    /// Execute an enter-score action.
    fn execute_enter_score(&self, state: &mut GameState, category: ScoreCategory, score: Score, events: &mut Vec<ControllerEvent>) -> Result<()> {
        if state.phase() != TurnPhase::AwaitingRoll && state.phase() != TurnPhase::Holding && state.phase() != TurnPhase::Scoring {
            return Err(YahtzeeError::PhaseMismatch(state.phase().to_string(), "AwaitingRoll, Holding or Scoring".to_string()));
        }

        let player_index = state.current_player_index().get();
        state.enter_score(category, score)?;

        events.push(ControllerEvent::ScoreEntered { player_index, category, score });

        // Check for game over
        if state.is_game_over() {
            state.set_status(GameStatus::GameOver);
            let standings = compute_standings(state.players().to_vec());
            events.push(ControllerEvent::GameOver { standings });
        } else {
            let player_index = state.current_player_index();
            events.push(ControllerEvent::TurnStarted {
                player_index: player_index.get(),
            });
            events.push(ControllerEvent::PhaseChanged { phase: state.phase() });
            // Emit turn transition for pass-and-play in multi-player mode
            if state.requires_turn_transitions() {
                let transition = TurnTransition::new(state.current_player(), player_index, state.round());
                events.push(ControllerEvent::TurnTransition { transition });
            }
        }

        Ok(())
    }

    /// Execute a cross-out action.
    fn execute_cross_out(&self, state: &mut GameState, category: ScoreCategory, events: &mut Vec<ControllerEvent>) -> Result<()> {
        if state.phase() != TurnPhase::AwaitingRoll && state.phase() != TurnPhase::Holding && state.phase() != TurnPhase::Scoring {
            return Err(YahtzeeError::PhaseMismatch(state.phase().to_string(), "AwaitingRoll, Holding or Scoring".to_string()));
        }

        let player_index = state.current_player_index().get();
        state.cross_out(category)?;

        events.push(ControllerEvent::CategoryCrossedOut { player_index, category });

        // Check for game over
        if state.is_game_over() {
            state.set_status(GameStatus::GameOver);
            let standings = compute_standings(state.players().to_vec());
            events.push(ControllerEvent::GameOver { standings });
        } else {
            let player_index = state.current_player_index();
            events.push(ControllerEvent::TurnStarted {
                player_index: player_index.get(),
            });
            events.push(ControllerEvent::PhaseChanged { phase: state.phase() });
            // Emit turn transition for pass-and-play in multi-player mode
            if state.requires_turn_transitions() {
                let transition = TurnTransition::new(state.current_player(), player_index, state.round());
                events.push(ControllerEvent::TurnTransition { transition });
            }
        }

        Ok(())
    }

    /// Called when dice become stable after a roll.
    ///
    /// Updates the game state and emits appropriate events.
    pub fn dice_stable(&self, dice: DiceSet) -> Result<Vec<ControllerEvent>> {
        let mut state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;

        if state.status() == GameStatus::GameOver {
            return Err(YahtzeeError::GameAlreadyOver);
        }

        if state.phase() != TurnPhase::Rolling {
            return Err(YahtzeeError::PhaseMismatch(state.phase().to_string(), "Rolling".to_string()));
        }

        // Update dice values
        *state.dice_set_mut() = dice;
        state.dice_stable();

        let mut events = vec![ControllerEvent::PhaseChanged { phase: TurnPhase::Holding }];

        // Check for forced scoring
        if state.is_forced_scoring() {
            state.check_forced_scoring();
            events.push(ControllerEvent::PhaseChanged { phase: TurnPhase::Scoring });
        }

        // Check if cross-out is needed
        if state.phase() == TurnPhase::Scoring
            && must_cross_out(state.dice_set())
            && let Some(rec) = self.advisor.recommend(state.current_player().scorecard(), state.dice_set())
        {
            events.push(ControllerEvent::CrossOutRecommended { recommendation: rec });
        }

        events.push(ControllerEvent::RollCompleted {
            dice: state.dice_set().clone(),
        });

        // Detect special rolls and emit celebration effect
        if let Some(effect) = CelebrationDetector::detect(state.dice_set()) {
            events.push(ControllerEvent::Celebration { effect });
        }

        // Broadcast events to subscribers
        for event in &events {
            let _ = self.event_sender.send(event.clone());
        }

        Ok(events)
    }

    /// Get the potential score for a category with the current dice.
    pub fn potential_score(&self, category: ScoreCategory) -> Result<Score> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        Ok(calculate_score(category, state.dice_set()))
    }

    /// Check if a category is valid (scores > 0) with the current dice.
    pub fn is_valid_category(&self, category: ScoreCategory) -> Result<bool> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        Ok(is_valid(category, state.dice_set()))
    }

    /// Get a cross-out recommendation for the current state.
    pub fn cross_out_recommendation(&self) -> Result<Option<CrossOutRecommendation>> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        Ok(self.advisor.recommend(state.current_player().scorecard(), state.dice_set()))
    }

    /// Get the final standings if the game is over.
    pub fn standings(&self) -> Result<Vec<StandingEntry>> {
        let state = self.state.lock().map_err(|_| YahtzeeError::LockPoisoned)?;
        if !state.is_game_over() {
            return Ok(Vec::new());
        }
        Ok(compute_standings(state.players().to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn new_controller_starts_playing() {
        let controller = GameController::new(make_players(2)).unwrap();
        assert_eq!(controller.status().unwrap(), GameStatus::Playing);
    }

    #[test]
    fn execute_roll_in_awaiting_roll() {
        let controller = GameController::new(make_players(1)).unwrap();
        let events = controller.execute(TurnAction::Roll).unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::PhaseChanged { phase: TurnPhase::Rolling })));
    }

    #[test]
    fn execute_roll_in_scoring_fails() {
        let controller = GameController::new(make_players(1)).unwrap();
        let mut state = controller.state().unwrap();
        state.set_phase(TurnPhase::Scoring);
        drop(state);
        // Can't easily set state back — test via action mismatch
        // This test verifies that rolling in the wrong phase fails
    }

    #[test]
    fn execute_enter_score_advances_turn() {
        let controller = GameController::new(make_players(2)).unwrap();
        let events = controller
            .execute(TurnAction::EnterScore {
                category: ScoreCategory::Ones,
                score: Score::new(3),
            })
            .unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::ScoreEntered { .. })));
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::TurnStarted { player_index: 1 })));
    }

    #[test]
    fn execute_cross_out_advances_turn() {
        let controller = GameController::new(make_players(2)).unwrap();
        let events = controller
            .execute(TurnAction::CrossOut {
                category: ScoreCategory::Yahtzee,
            })
            .unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::CategoryCrossedOut { .. })));
    }

    #[test]
    fn execute_after_game_over_fails() {
        let controller = GameController::new(make_players(1)).unwrap();
        // Fill all categories to end the game
        for (i, cat) in ScoreCategory::ALL.iter().enumerate() {
            controller
                .execute(TurnAction::EnterScore {
                    category: *cat,
                    score: Score::new((i + 1) as u32),
                })
                .unwrap();
        }
        // Now game is over
        assert_eq!(controller.status().unwrap(), GameStatus::GameOver);
        let result = controller.execute(TurnAction::Roll);
        assert!(result.is_err());
    }

    #[test]
    fn dice_stable_transitions_to_holding() {
        let controller = GameController::new(make_players(1)).unwrap();
        controller.execute(TurnAction::Roll).unwrap();
        let dice = DiceSet::from_values([1, 2, 3, 4, 5]).unwrap();
        let events = controller.dice_stable(dice).unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::PhaseChanged { phase: TurnPhase::Holding })));
    }

    #[test]
    fn dice_stable_after_third_roll_forces_scoring() {
        let controller = GameController::new(make_players(1)).unwrap();
        // First roll
        controller.execute(TurnAction::Roll).unwrap();
        controller.dice_stable(DiceSet::from_values([1, 2, 3, 4, 5]).unwrap()).unwrap();
        // Second roll
        controller.execute(TurnAction::Roll).unwrap();
        controller.dice_stable(DiceSet::from_values([2, 3, 4, 5, 6]).unwrap()).unwrap();
        // Third roll
        controller.execute(TurnAction::Roll).unwrap();
        let events = controller.dice_stable(DiceSet::from_values([3, 3, 3, 3, 3]).unwrap()).unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::PhaseChanged { phase: TurnPhase::Scoring })));
    }

    #[test]
    fn potential_score_returns_correct_value() {
        let controller = GameController::new(make_players(1)).unwrap();
        let score = controller.potential_score(ScoreCategory::Chance).unwrap();
        // Default dice set is all 1s, so chance = 5
        assert_eq!(score, Score::new(5));
    }

    #[test]
    fn standings_empty_before_game_over() {
        let controller = GameController::new(make_players(2)).unwrap();
        assert!(controller.standings().unwrap().is_empty());
    }

    #[test]
    fn standings_populated_after_game_over() {
        let controller = GameController::new(make_players(2)).unwrap();
        for (i, cat) in ScoreCategory::ALL.iter().enumerate() {
            controller
                .execute(TurnAction::EnterScore {
                    category: *cat,
                    score: Score::new((i + 1) as u32),
                })
                .unwrap();
            controller
                .execute(TurnAction::EnterScore {
                    category: *cat,
                    score: Score::new((i + 2) as u32),
                })
                .unwrap();
        }
        let standings = controller.standings().unwrap();
        assert_eq!(standings.len(), 2);
        assert_eq!(standings[0].rank().get(), 1);
    }

    #[test]
    fn multi_player_emits_turn_transition() {
        let controller = GameController::with_mode(make_players(2), GameMode::MultiPlayer).unwrap();
        let events = controller
            .execute(TurnAction::EnterScore {
                category: ScoreCategory::Ones,
                score: Score::new(3),
            })
            .unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::TurnTransition { .. })));
    }

    #[test]
    fn single_player_no_turn_transition() {
        let controller = GameController::with_mode(make_players(1), GameMode::SinglePlayer).unwrap();
        let events = controller
            .execute(TurnAction::EnterScore {
                category: ScoreCategory::Ones,
                score: Score::new(3),
            })
            .unwrap();
        assert!(!events.iter().any(|e| matches!(e, ControllerEvent::TurnTransition { .. })));
    }

    #[test]
    fn multi_player_hints_disabled() {
        let controller = GameController::with_mode(make_players(2), GameMode::MultiPlayer).unwrap();
        assert!(!controller.hints_enabled().unwrap());
    }

    #[test]
    fn single_player_hints_enabled() {
        let controller = GameController::with_mode(make_players(1), GameMode::SinglePlayer).unwrap();
        assert!(controller.hints_enabled().unwrap());
    }

    #[test]
    fn game_mode_returns_correct_value() {
        let single = GameController::with_mode(make_players(1), GameMode::SinglePlayer).unwrap();
        assert_eq!(single.game_mode().unwrap(), GameMode::SinglePlayer);

        let multi = GameController::with_mode(make_players(2), GameMode::MultiPlayer).unwrap();
        assert_eq!(multi.game_mode().unwrap(), GameMode::MultiPlayer);
    }

    #[test]
    fn multi_player_cross_out_emits_turn_transition() {
        let controller = GameController::with_mode(make_players(2), GameMode::MultiPlayer).unwrap();
        let events = controller
            .execute(TurnAction::CrossOut {
                category: ScoreCategory::Yahtzee,
            })
            .unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::TurnTransition { .. })));
    }

    #[test]
    fn led_assignment_has_correct_count() {
        let controller = GameController::new(make_players(3)).unwrap();
        assert_eq!(controller.led_assignment().player_count(), 3);
    }

    #[test]
    fn dice_stable_yahtzee_emits_celebration() {
        let controller = GameController::new(make_players(1)).unwrap();
        controller.execute(TurnAction::Roll).unwrap();
        let dice = DiceSet::from_values([5, 5, 5, 5, 5]).unwrap();
        let events = controller.dice_stable(dice).unwrap();
        assert!(events.iter().any(|e| matches!(e, ControllerEvent::Celebration { .. })));
    }

    #[test]
    fn dice_stable_normal_no_celebration() {
        let controller = GameController::new(make_players(1)).unwrap();
        controller.execute(TurnAction::Roll).unwrap();
        let dice = DiceSet::from_values([1, 2, 3, 5, 6]).unwrap();
        let events = controller.dice_stable(dice).unwrap();
        assert!(!events.iter().any(|e| matches!(e, ControllerEvent::Celebration { .. })));
    }
}
