use dice_rs::service::dice::Dice;
use dice_rs::service::dice::DiceEvent;
use tokio::sync::broadcast;

/// A unique session identifier.
pub type SessionId = String;

/// Represents a single client's connection to a dice.
pub struct Session {
    /// The session ID.
    #[allow(dead_code)]
    pub id: SessionId,
    /// The connected dice handle.
    pub dice: Dice,
    /// The device address.
    #[allow(dead_code)]
    pub address: String,
    /// Active event subscription receiver.
    #[allow(dead_code)]
    pub event_receiver: broadcast::Receiver<DiceEvent>,
}

impl Session {
    /// Create a new session.
    pub fn new(id: SessionId, dice: Dice, address: String) -> Self {
        let event_receiver = dice.subscribe();
        Self {
            id,
            dice,
            address,
            event_receiver,
        }
    }
}
