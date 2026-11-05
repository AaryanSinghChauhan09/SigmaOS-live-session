#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceType { EphemeralRam, EncryptedLuks, PlainCow }

#[derive(Debug)]
pub struct LiveSessionConfig {
    pub persistence: PersistenceType,
}

impl LiveSessionConfig {
    pub fn new(p: PersistenceType) -> Self { Self { persistence: p } }
    pub fn is_encrypted(&self) -> bool { self.persistence == PersistenceType::EncryptedLuks }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_live_session_config() {
        let cfg = LiveSessionConfig::new(PersistenceType::EncryptedLuks);
        assert!(cfg.is_encrypted());
    }
}
