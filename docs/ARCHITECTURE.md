# Architecture at a glance

This page is the 60-second tour. For depth, read the [design spec](superpowers/specs/2026-05-30-poly-agntcy-design.md).

## Use this when

- You're new to the codebase and want to know how the pieces fit.
- You're deciding which package to import.
- You're reviewing a change that crosses language boundaries.

## Result

After reading this page, you'll know what's in each language root, how the protobuf pipeline feeds them, and where to look next.

## The shape

Five language roots under one repo. Each ships one or more publishable packages — fourteen in total.

```
poly-agntcy/
├── proto/                    # buf BSR ref to agntcy/dir (single source of truth)
├── go/                       # first-party SDK + agntcy CLI
│   └── spiffe/               # optional SPIFFE credentials (separate module)
├── rs/                       # Cargo workspace
│   ├── hop-top-agntcy-dir/      # core crate + agntcy bin
│   └── hop-top-agntcy-dir-spiffe/
├── php/packages/             # Composer path workspace
│   ├── dir/                  # core
│   ├── dir-laravel/          # Laravel ServiceProvider
│   ├── dir-symfony/          # Symfony Bundle
│   └── dir-spiffe/
├── ts/packages/              # pnpm workspace
│   ├── dir-next/             # Next.js Route Handler + agntcy CLI
│   ├── dir-hono/             # Hono plugin
│   └── dir-express/          # Express middleware
├── py/packages/              # uv workspace
│   ├── dir-fastapi/          # FastAPI APIRouter + Typer CLI
│   ├── dir-flask/            # Flask Blueprint
│   └── dir-django/           # Django app
├── examples/                 # one per package + cross-lang integration
└── docs/                     # spec, ADRs, runbooks
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

- **Protobuf types** come from [`buf.build/agntcy/dir`](https://buf.build/agntcy/dir) via the BSR. We don't check in `.proto` files. `mise run gen` regenerates Go/Rust/PHP stubs on demand. TS and Python wrap the official `agntcy-dir` SDK directly.
- **SPIFFE support** lives in separate packages so the core has no mandatory SPIFFE dependency.
- **CLI tools** ride along inside an existing released package per language (see [ADR-0001](adr/0001-scope-dir-only-phase-one.md) and the design spec §6) to keep the publishable-package count at 14.

## Adapter vs first-party

- **First-party** (Go, Rust, PHP) — we own the wire protocol implementation against the proto stubs.
- **Adapter** (TypeScript, Python) — we wrap the official AGNTCY SDKs and add typed framework integrations.

This split is documented in [ADR-0005](adr/0005-layout-l3-flat-per-language.md).

## Next steps

- Pick your language: [install](manual/install.md)
- Run a real call: [getting started](manual/getting-started.md)
- Read the decisions log: [ADRs](adr/)
- Read the full spec: [design spec](superpowers/specs/2026-05-30-poly-agntcy-design.md)
