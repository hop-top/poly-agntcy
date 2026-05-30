use std::sync::Arc;

/// TLS posture for [`crate::client::Client`].
pub trait Credentials: Send + Sync {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>>;
}

/// No TLS. Dev/test only.
pub struct InsecureCredentials;

impl Credentials for InsecureCredentials {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>> {
        None
    }
}

/// Wraps a pre-built rustls ClientConfig.
pub struct TlsCredentials {
    pub config: Arc<rustls::ClientConfig>,
}

impl Credentials for TlsCredentials {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>> {
        Some(self.config.clone())
    }
}
