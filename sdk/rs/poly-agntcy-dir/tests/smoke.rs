use std::sync::Arc;

use poly_agntcy_dir::{Client, Credentials, InsecureCredentials, Options};

#[test]
fn insecure_credentials_returns_no_tls() {
    let c = InsecureCredentials;
    assert!(c.tls().is_none());
}

#[test]
fn new_client_requires_endpoint() {
    let r = Client::new(Options {
        endpoint: String::new(),
        credentials: Arc::new(InsecureCredentials),
    });
    assert!(r.is_err());
}

#[test]
fn new_client_accepts_endpoint() {
    let r = Client::new(Options {
        endpoint: "http://127.0.0.1:50051".into(),
        credentials: Arc::new(InsecureCredentials),
    });
    assert!(r.is_ok());
}
