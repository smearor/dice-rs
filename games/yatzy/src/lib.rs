//! yatzy — Kniffel (Yatzy) game engine built on dice-rs.
//!
//! Provides the core game logic for Kniffel: scorecard management,
//! scoring rules, validation, and game state. Designed as a showcase
//! project for the `dice-rs` library with physical GoDice integration.

pub mod error;
pub mod i18n;
pub mod models;
pub mod rules;
pub mod services;
pub mod strategy;
pub mod ui;

pub use error::Result;
pub use error::YatzyError;
pub use models::category::ScoreCategory;
pub use models::category_section::CategorySection;
pub use models::dice_set::DiceSet;
pub use models::dice_slot::DiceSlot;
pub use models::game_mode::GameMode;
pub use models::game_settings::GameSettings;
pub use models::game_settings::PlayerSettingsEntry;
pub use models::game_state::GameState;
pub use models::game_status::GameStatus;
pub use models::highscore::HighscoreList;
pub use models::highscore_entry::HighscoreEntry;
pub use models::hold_mask::HoldMask;
pub use models::player::Player;
pub use models::player_color::PlayerColor;
pub use models::player_config::PlayerConfig;
pub use models::player_config::PlayerSetup;
pub use models::player_count::PlayerCount;
pub use models::player_index::PlayerIndex;
pub use models::player_name::PlayerName;
pub use models::player_type::PlayerType;
pub use models::reconnection_request::ReconnectionRequest;
pub use models::roll_count::RollCount;
pub use models::roll_result::RollResult;
pub use models::round_number::RoundNumber;
pub use models::score::Score;
pub use models::score_entry::ScoreEntry;
pub use models::scorecard::Scorecard;
pub use models::standing::StandingEntry;
pub use models::standing::WinnerRank;
pub use models::standing::compute_standings;
pub use models::turn_action::TurnAction;
pub use models::turn_phase::TurnPhase;
pub use models::turn_transition::TurnTransition;
pub use rules::CrossOutAdvisor;
pub use rules::CrossOutRecommendation;
pub use rules::calculate_score;
pub use rules::is_valid;
pub use services::CelebrationDetector;
pub use services::ControllerEvent;
pub use services::DiceService;
pub use services::EventBridge;
pub use services::GameController;
pub use services::GameEvent;
pub use services::HighscoreStore;
pub use services::LedColorAssignment;
pub use services::LedEffect;
pub use services::LedService;
pub use services::ReconnectionManager;
pub use services::RollDetector;
pub use services::RollState;
pub use services::SettingsStore;
pub use services::SlotMapping;
pub use strategy::AiDecision;
pub use strategy::CategoryChoice;
pub use strategy::ComputerAi;
pub use strategy::ExpectedValue;
pub use strategy::HoldDecision;
pub use strategy::Probability;
