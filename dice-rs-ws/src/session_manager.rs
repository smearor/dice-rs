use std::collections::HashMap;

use dice_rs::service::dice::Dice;

use crate::session::Session;
use crate::session::SessionId;

/// Manages all active sessions across WebSocket clients.
pub struct SessionManager {
    sessions: HashMap<SessionId, Session>,
}

impl SessionManager {
    /// Create a new session manager.
    pub fn new() -> Self {
        Self { sessions: HashMap::new() }
    }

    /// Create a new session for a connected dice.
    pub fn create(&mut self, dice: Dice, address: String) -> SessionId {
        let id = format!("s{}", self.sessions.len() + 1);
        let session = Session::new(id.clone(), dice, address);
        self.sessions.insert(id.clone(), session);
        id
    }

    /// Get a session by ID.
    pub fn get(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }

    /// Get a mutable session by ID.
    #[allow(dead_code)]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    /// Remove and return a session by ID.
    pub fn remove(&mut self, id: &str) -> Option<Session> {
        self.sessions.remove(id)
    }

    /// Get all active session IDs.
    #[allow(dead_code)]
    pub fn session_ids(&self) -> Vec<SessionId> {
        self.sessions.keys().cloned().collect()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
