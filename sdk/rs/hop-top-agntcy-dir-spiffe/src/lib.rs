//! SPIFFE-backed Credentials for hop-top-agntcy-dir.
//!
//! v0 stub. Constructors currently return [`Error::NotYetWired`] to
//! prevent silent plaintext fallback: the `tls()` impl would return
//! `None`, which `Client::channel()` treats as "no TLS". Full SVID
//! fetch + rustls verifier wiring lands in a follow-up once upstream
//! rustls integration stabilises in the `spiffe` crate.

use std::sync::Arc;

use hop_top_agntcy_dir::Credentials;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("workload api socket required")]
    SocketRequired,
    #[error("spiffe credentials not yet wired; tracking issue pending")]
    NotYetWired,
}

#[deprecated(
    note = "v0 stub; SPIFFE TLS wiring pending; constructors return NotYetWired; do not use in production"
)]
pub struct SpiffeCredentials {
    socket: String,
}

#[allow(deprecated)]
impl SpiffeCredentials {
    pub fn new(socket: impl Into<String>) -> Result<Self, Error> {
        let socket = socket.into();
        if socket.is_empty() {
            return Err(Error::SocketRequired);
        }
        let _ = socket;
        Err(Error::NotYetWired)
    }

    pub fn from_default_socket() -> Result<Self, Error> {
        let socket = std::env::var("SPIFFE_ENDPOINT_SOCKET").map_err(|_| Error::SocketRequired)?;
        Self::new(socket)
    }

    pub fn socket(&self) -> &str {
        &self.socket
    }
}

#[allow(deprecated)]
impl Credentials for SpiffeCredentials {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>> {
        None
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_socket() {
        let r = SpiffeCredentials::new("");
        assert!(matches!(r, Err(Error::SocketRequired)));
    }

    #[test]
    fn new_returns_not_yet_wired() {
        let r = SpiffeCredentials::new("unix:///tmp/spire-agent/public/api.sock");
        assert!(matches!(r, Err(Error::NotYetWired)));
    }
}
