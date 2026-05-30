//! SPIFFE-backed Credentials for poly-agntcy-dir.
//!
//! v0 stub. Holds the socket path that would feed a Workload API
//! client and returns None for `tls()`. Full SVID fetch + rustls
//! verifier wiring lands in a follow-up once upstream rustls
//! integration stabilises in the `spiffe` crate.

use std::sync::Arc;

use poly_agntcy_dir::Credentials;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("workload api socket required")]
    SocketRequired,
}

pub struct SpiffeCredentials {
    socket: String,
}

impl SpiffeCredentials {
    pub fn new(socket: impl Into<String>) -> Result<Self, Error> {
        let socket = socket.into();
        if socket.is_empty() {
            return Err(Error::SocketRequired);
        }
        Ok(Self { socket })
    }

    pub fn from_default_socket() -> Result<Self, Error> {
        let socket = std::env::var("SPIFFE_ENDPOINT_SOCKET").map_err(|_| Error::SocketRequired)?;
        Self::new(socket)
    }

    pub fn socket(&self) -> &str {
        &self.socket
    }
}

impl Credentials for SpiffeCredentials {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>> {
        None
    }
}
