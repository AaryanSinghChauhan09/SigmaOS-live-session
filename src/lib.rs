//! SigmaOS Live Session
//! Live session support for stateless, ephemeral environments.

pub mod live_session;

pub use live_session::{LiveSession, LiveSessionManager, SessionState};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
