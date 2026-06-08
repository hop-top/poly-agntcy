use std::sync::Arc;

use hop_top_agntcy_dir::{Client, DiscoverParams, InsecureCredentials, Options};

#[test]
fn new_client_requires_endpoint() {
    let r = Client::new(Options {
        endpoint: String::new(),
        credentials: Arc::new(InsecureCredentials),
    });
    assert!(r.is_err());
}

#[tokio::test]
async fn discover_fails_without_server() {
    let c = Client::new(Options {
        endpoint: "http://127.0.0.1:1".into(),
        credentials: Arc::new(InsecureCredentials),
    })
    .unwrap();
    let r = c
        .discover(DiscoverParams {
            capability: "translate".into(),
        })
        .await;
    assert!(r.is_err());
}
