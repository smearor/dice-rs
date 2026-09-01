//! yahtzee — Kniffel (Yahtzee) game engine built on dice-rs.
//!
//! Provides the core game logic for Kniffel: scorecard management,
//! scoring rules, validation, and game state. Designed as a showcase
//! project for the `dice-rs` library with physical GoDice integration.

pub mod error;
pub mod models;
pub mod rules;
pub mod services;
pub mod ui;

pub use error::Result;
pub use error::YahtzeeError;
pub use models::category::ScoreCategory;
pub use models::category_section::CategorySection;
pub use models::dice_set::DiceSet;
pub use models::dice_slot::DiceSlot;
pub use models::game_state::GameState;
pub use models::hold_mask::HoldMask;
pub use models::player::Player;
pub use models::player_color::PlayerColor;
pub use models::player_index::PlayerIndex;
pub use models::player_name::PlayerName;
pub use models::player_type::PlayerType;
pub use models::roll_count::RollCount;
pub use models::round_number::RoundNumber;
pub use models::score::Score;
pub use models::score_entry::ScoreEntry;
pub use models::scorecard::Scorecard;
pub use models::turn_phase::TurnPhase;
pub use services::DiceService;
pub use services::EventBridge;
pub use services::GameEvent;
pub use services::LedEffect;
pub use services::LedService;
pub use services::RollDetector;
pub use services::RollState;
pub use services::SlotMapping;
