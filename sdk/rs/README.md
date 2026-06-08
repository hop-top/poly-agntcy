# poly-agntcy (Rust)

Rust SDK for the AGNTCY Agent Directory Service (DIR).

## Workspace

- `poly-agntcy-dir` — core SDK crate. Async gRPC `Client` for DIR
  with `register`, `discover`, `describe`, `publish`, `verify`. Ships
  the `agntcy` CLI bin behind the `cli` feature.
- `poly-agntcy-dir-spiffe` — optional SPIFFE-backed `Credentials`
  extension. v0 stub; full SVID + rustls verifier wiring pending.

## Install

```sh
cargo add poly-agntcy-dir
```

## Library usage

```rust
use std::sync::Arc;
use poly_agntcy_dir::{Client, DiscoverParams, InsecureCredentials, Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Options {
        endpoint: "http://localhost:50051".into(),
        credentials: Arc::new(InsecureCredentials),
    })?;
    let r = client
        .discover(DiscoverParams { capability: "translate".into() })
        .await?;
    for a in r.agents {
        println!("{}", a.name);
    }
    Ok(())
}
```

Swap `InsecureCredentials` for `TlsCredentials { config }` to enable
TLS with a pre-built `rustls::ClientConfig`.

## CLI

The `agntcy` bin is gated behind the `cli` feature:

```sh
cargo install poly-agntcy-dir --features cli
agntcy --endpoint http://localhost:50051 discover --capability translate
```

## SPIFFE credentials

See [`poly-agntcy-dir-spiffe`](./poly-agntcy-dir-spiffe) for the
SPIFFE Workload API extension. The v0 type is gated behind a
`NotYetWired` error to prevent silent plaintext fallback until the
SVID + rustls integration ships.

## License

Apache-2.0. See [LICENSE](../LICENSE).
