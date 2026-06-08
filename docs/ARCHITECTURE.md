# Architecture at a glance

This page is the 60-second tour. For depth, read the [design spec](superpowers/specs/2026-05-30-poly-agntcy-design.md).

## Use this when

- You're new to the codebase and want to know how the pieces fit.
- You're deciding which package to import.
- You're reviewing a change that crosses language boundaries.

## Result

After reading this page, you'll know what's in each language root, how the protobuf pipeline feeds them, and where to look next.

## The shape

Role-based layout (per ADR-0007): SDKs under `sdk/<lang>/`, framework
integrations under `adapters/<framework>/`, generated code under
`gen/<lang>/`, the single CLI at `cmd/agntcy/`. Fourteen publishable
packages total.

```
poly-agntcy/
├── cmd/agntcy/                       # Go CLI (only binary in the repo)
├── contracts/                        # buf pin + lint config (BSR is source of truth)
├── gen/                              # generated code, committed
│   ├── go/                           # hop.top/agntcy/gen/go module
│   ├── rs/
│   └── php/
├── sdk/                              # protocol clients (libraries only)
│   ├── go/                           # hop.top/agntcy module
│   │   └── spiffe/                   # optional SPIFFE module
│   ├── rs/                           # Cargo workspace
│   │   ├── hop-top-agntcy-dir/       # core crate
│   │   └── hop-top-agntcy-dir-spiffe/
│   ├── php/packages/                 # Composer path workspace
│   │   ├── dir/                      # hop-top/agntcy-dir
│   │   └── dir-spiffe/               # hop-top/agntcy-dir-spiffe
│   ├── ts/                           # pnpm workspace root (TS is adapter-only)
│   └── py/                           # uv workspace root (Py is adapter-only)
├── adapters/                         # framework integrations
│   ├── laravel/  symfony/            # PHP
│   ├── next/  hono/  express/        # TypeScript
│   └── fastapi/  flask/  django/     # Python
├── go.work                           # multi-module workspace at repo root
├── examples/                         # one per adapter + cross-lang integration
└── docs/                             # spec, ADRs, runbooks
```

## How traffic flows

```
Your app ── poly-agntcy adapter ── Credentials ── tonic/connect/grpc-php/fetch ── DIR service
                                       │
                                       ├── InsecureCredentials (dev only)
                                       ├── TlsCredentials      (production default)
                                       └── SpiffeCredentials   (optional, separate package)
```

The adapter package provides a `Client` (or framework integration: APIRouter, ServiceProvider, etc.). The `Client` takes `Credentials` and calls one of five DIR methods: `Register`, `Discover`, `Describe`, `Publish`, `Verify`.

## Where things come from

- **Protobuf types** come from [`buf.build/agntcy/dir`](https://buf.build/agntcy/dir) via the BSR — pinned in [`contracts/buf.lock`](../contracts/buf.lock). We don't check in `.proto` files. `mise run gen` regenerates committed stubs under `gen/<lang>/`. TS and Python wrap the official `agntcy-dir` SDK directly.
- **SPIFFE support** lives in separate packages so the core has no mandatory SPIFFE dependency.
- **CLI tools** ride along inside an existing released package per language (see [ADR-0001](adr/0001-scope-dir-only-phase-one.md) and the design spec §6) to keep the publishable-package count at 14.

## Adapter vs first-party

- **First-party** (Go, Rust, PHP) — we own the wire protocol implementation against the proto stubs.
- **Adapter** (TypeScript, Python) — we wrap the official AGNTCY SDKs and add typed framework integrations.

This split is documented in [ADR-0001](adr/0001-scope-dir-only-phase-one.md) (scope) and [ADR-0007](adr/0007-role-based-layout-sdk-adapter-split.md) (layout).

## Next steps

- Pick your language: [install](manual/install.md)
- Run a real call: [getting started](manual/getting-started.md)
- Read the decisions log: [ADRs](adr/)
- Read the full spec: [design spec](superpowers/specs/2026-05-30-poly-agntcy-design.md)
