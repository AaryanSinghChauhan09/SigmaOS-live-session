use sigmaos_live_session::{LiveSession, LiveSessionManager, SessionState};

#[test]
fn test_live_session_integration_aaa() {
    let mut manager = LiveSessionManager::new();
    let _ = manager.create_session("ephemeral-1", "Sigma Live ISO").unwrap();

    let session = manager.get_session_mut("ephemeral-1").unwrap();
    session.add_app("sigmaos-desktop-environment");
    session.add_app("sigmaos-file-manager");

    assert_eq!(session.apps.len(), 2);
    assert_eq!(session.state, SessionState::Active);
}
