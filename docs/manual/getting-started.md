# Run your first DIR call

Make a DIR call from your language of choice. This is the minimum working path — production hardening comes later.

## Use this when

- You've installed a poly-agntcy package and want to confirm it works against a real DIR endpoint.
- You're evaluating which adapter shape fits your app.

## Result

After this guide, you'll have called `discover` against a local DIR instance from at least one language and seen a real response.

## Before you begin

You need:

- A DIR endpoint. The fastest way is the local docker-compose harness in [`examples/cross/`](../../examples/cross/).
- The poly-agntcy package for your language. See [install](install.md).

Start a local DIR instance from the repo root:

```sh
cd examples/cross
docker compose up -d
```

This brings up DIR on `http://localhost:8888`. Verify it's healthy:

```sh
curl -fsS http://localhost:8888/healthz
```

## Quick path

The fastest way to see a working call is the per-language example:

```sh
# Go
cd examples/go/register-and-discover && go run .

# Rust
cd examples/rs/register-and-discover && cargo run

# PHP (vanilla)
cd examples/php/vanilla-quickstart && composer install && php index.php

# TypeScript (Next.js Route Handler)
cd examples/ts/next-route && pnpm install && pnpm dev

# Python (FastAPI)
cd examples/py/fastapi-quickstart && uv run uvicorn main:app
```

Each example registers an identity, publishes a descriptor, and discovers a peer.

## Use the library directly

### Go

```go
package main

import (
    "context"
    "fmt"

    "hop.top/agntcy/dir"
)

func main() {
    c, err := dir.NewClient(dir.Options{
        Endpoint:    "http://localhost:8888",
        Credentials: dir.InsecureCredentials{},
    })
    if err != nil {
        panic(err)
    }
    res, err := c.Discover(context.Background(), dir.DiscoverParams{Capability: "inventory"})
    if err != nil {
        panic(err)
    }
    fmt.Printf("%+v\n", res.Agents)
}
```

### Rust

```rust
use poly_agntcy_dir::{Client, Options, InsecureCredentials, DiscoverParams};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(Options {
        endpoint: "http://localhost:8888".to_string(),
        credentials: Arc::new(InsecureCredentials),
    });
    let res = client.discover(DiscoverParams { capability: "inventory".into() }).await?;
    println!("{:?}", res.agents);
    Ok(())
}
```

### PHP

```php
<?php
declare(strict_types=1);

use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;

$client = new Client(
    endpoint: 'http://localhost:8888',
    credentials: new InsecureCredentials(),
);

$res = $client->discover(capability: 'inventory');
var_dump($res->agents);
```

### TypeScript (Next.js)

```ts
// app/api/agntcy/route.ts
import { createDirectoryHandler } from "@poly-agntcy/dir-next";

export const POST = createDirectoryHandler({
  endpoint: "http://localhost:8888",
});
```

### Python (FastAPI)

```py
from fastapi import FastAPI
from poly_agntcy_dir_fastapi import create_directory_router

app = FastAPI()
app.include_router(create_directory_router(endpoint="http://localhost:8888"))
```

## Verify

Each example or library call should return a non-empty response (even if the empty case is an empty array — the call should not error). If you get a connection refused, the DIR endpoint isn't up.

## How it works

Every adapter exposes a `Client` (or framework integration: `APIRouter`, `ServiceProvider`, `Plugin`, etc.) configured with an endpoint and a `Credentials` value. The client implements five DIR methods: `Register`, `Discover`, `Describe`, `Publish`, `Verify`.

Production usage swaps `InsecureCredentials` for `TlsCredentials` (system trust roots) or `SpiffeCredentials` (workload-API identity). See the [SPIFFE ADR](../adr/0003-spiffe-optional-extension-per-language.md).

## Next steps

- See more examples: [`examples/`](../../examples/)
- Read the architecture: [architecture](../ARCHITECTURE.md)
- Hit a problem: [troubleshooting](troubleshooting.md)
