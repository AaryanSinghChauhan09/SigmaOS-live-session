//! SigmaOS Live Session implementation
//! Follows: Fail-Fast, Separation of Concerns, Deterministic State

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct LiveSession {
    pub id: String,
    pub name: String,
    pub state: SessionState,
    pub apps: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl LiveSession {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            state: SessionState::Active,
            apps: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_app(&mut self, app: impl Into<String>) {
        self.apps.push(app.into());
    }

    pub fn suspend(&mut self) -> Result<(), &'static str> {
        if self.state == SessionState::Terminated {
            return Err("Cannot suspend a terminated session");
        }
        self.state = SessionState::Suspended;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), &'static str> {
        if self.state == SessionState::Terminated {
            return Err("Cannot resume a terminated session");
        }
        self.state = SessionState::Active;
        Ok(())
    }

    pub fn terminate(&mut self) {
        self.state = SessionState::Terminated;
    }

    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active
    }
}

pub struct LiveSessionManager {
    sessions: HashMap<String, LiveSession>,
}

impl LiveSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, id: &str, name: &str) -> Result<&LiveSession, &'static str> {
        if self.sessions.contains_key(id) {
            return Err("Session ID already exists");
        }
        let session = LiveSession::new(id, name);
        self.sessions.insert(id.to_string(), session);
        Ok(self.sessions.get(id).unwrap())
    }

    pub fn get_session(&self, id: &str) -> Option<&LiveSession> {
        self.sessions.get(id)
    }

    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut LiveSession> {
        self.sessions.get_mut(id)
    }

    pub fn list_sessions(&self) -> Vec<&LiveSession> {
        self.sessions.values().collect()
    }

    pub fn remove_session(&mut self, id: &str) -> Option<LiveSession> {
        self.sessions.remove(id)
    }
}

impl Default for LiveSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_session_lifecycle_aaa() {
        // Arrange
        let mut session = LiveSession::new("sess-1", "Default Desktop");
        assert!(session.is_active());

        // Act & Assert Suspend
        assert!(session.suspend().is_ok());
        assert_eq!(session.state, SessionState::Suspended);
        assert!(!session.is_active());

        // Act & Assert Resume
        assert!(session.resume().is_ok());
        assert_eq!(session.state, SessionState::Active);

        // Terminate
        session.terminate();
        assert_eq!(session.state, SessionState::Terminated);
        assert!(session.suspend().is_err());
        assert!(session.resume().is_err());
    }

    #[test]
    fn test_live_session_manager_aaa() {
        let mut mgr = LiveSessionManager::new();
        assert!(mgr.list_sessions().is_empty());

        let res = mgr.create_session("s1", "Test Session");
        assert!(res.is_ok());

        assert!(mgr.create_session("s1", "Duplicate").is_err());
        assert_eq!(mgr.list_sessions().len(), 1);

        let s = mgr.get_session("s1").expect("session found");
        assert_eq!(s.name, "Test Session");

        assert!(mgr.remove_session("s1").is_some());
        assert!(mgr.list_sessions().is_empty());
    }
}
