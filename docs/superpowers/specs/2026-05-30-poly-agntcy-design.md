# poly-agntcy — design

Date: 2026-05-30
Status: approved (brainstorming)
Owner: jadb

## 1. Summary

`hop-top/poly-agntcy` — a polyglot Agent Directory Service (DIR) SDK
suite covering five languages around the AGNTCY DIR spec
([`agntcy/dir-spec`](https://github.com/agntcy/dir-spec)).

- Go, Rust, PHP — first-party SDKs (filling gaps in the AGNTCY org).
- TypeScript, Python — adapter packages on top of official
  [`agntcy/dir-sdk-javascript`](https://github.com/agntcy/dir-sdk-javascript)
  and [`agntcy/dir-sdk-python`](https://github.com/agntcy/dir-sdk-python).
- Per-language CLI (`agntcy register|discover|describe|publish|verify`).
- Optional SPIFFE extension packages per language.

Phase 1 of an AGNTCY polyglot family. Orchestration, runtime, ACP,
Identity, SLIM, OASF are explicitly out of scope for this repo and
deferred to sibling repos (`poly-agntcy-acp`, `poly-agntcy-runtime`,
etc.) — decision deferred.

## 2. Why

Three gaps + two opportunities:

- Gaps: no official Go/Rust/PHP SDK for DIR.
- Opportunity 1: official `dir-sdk-javascript` is plain JS-typed; we
  add typed framework adapters (Next, Hono, Express).
- Opportunity 2: official `dir-sdk-python` has no framework
  integration; we add FastAPI, Flask, Django adapters.

All five languages reach DIR parity around a single spec version,
pinned via buf BSR. Spec bumps propagate via one config change +
regen.

## 3. Non-goals (phase 1)

- No orchestration engine.
- No agent runtime.
- No ACP / Identity / SLIM / OASF spec coverage.
- No persistence layer beyond what DIR specifies.
- No UI.
- No reimplementation of `dir-sdk-javascript` or `dir-sdk-python` —
  TS and Python ship adapter packages only.

## 4. Decisions log

| # | Decision | Outcome |
|---|----------|---------|
| 1 | Approach | A — polyglot DIR SDK suite |
| 2 | Lang scope | A3 — first-party Go/Rust/PHP + adapter-only TS/Py |
| 3 | Layout | L3 — flat per-lang root (`go/`, `rs/`, `php/`, `ts/`, `py/`) |
| 4 | Protobuf | P2 — local `buf.gen.yaml`, BSR ref pin, codegen on demand |
| 5 | SPIFFE | S2 — optional extension package per lang |
| 6 | Release | R1 — per-package release-please tracks |
| 7 | Scaffold | (c) — poly-kit `scaffold.sh app --langs go,rs,php,ts,py`, keep CLI per lang |
| 8 | Go module | Reading 1 — vanity `hop.top/agntcy`, repo stays `poly-agntcy` |
| 9 | Mirror layout | (ii) — one mirror per language, per-component tags |
| 10 | Vanity host | hop.top serves `<meta>` redirect |

## 5. Repo layout

```
poly-agntcy/                            # hop-top/poly-agntcy
├── README.md                           # routing: gaps vs adapters
├── LICENSE                             # Apache-2.0 (matches agntcy/* repos)
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── mise.toml                           # SOT for tool versions
├── .env.example
├── .devcontainer/                      # compose-mode + otel + jaeger
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                      # matrix lint+test per lang
│   │   ├── release-please.yml          # standing release PRs
│   │   ├── release-please-preflight.yml
│   │   └── publish.yml                 # calls hop-top/.github@v0
│   ├── release-please-config.json      # 14-package manifest
│   └── .release-please-manifest.json
├── proto/
│   ├── buf.yaml                        # BSR dep on buf.build/agntcy/dir
│   ├── buf.gen.yaml                    # per-lang gen targets
│   └── README.md
├── go/
│   ├── go.mod                          # hop.top/agntcy
│   ├── dir/                            # DIR client (connect-go)
│   ├── cmd/agntcy/                     # Cobra CLI
│   ├── internal/transport/
│   ├── spiffe/                         # separate module (own go.mod)
│   └── examples/
├── rs/
│   ├── Cargo.toml                      # workspace
│   ├── poly-agntcy-dir/                # core (tonic)
│   ├── poly-agntcy-dir-spiffe/         # SPIFFE extension
│   ├── poly-agntcy/                    # CLI bin (clap)
│   └── examples/
├── php/
│   ├── composer.json                   # path repo for sub-pkgs
│   ├── packages/
│   │   ├── dir/                        # poly-agntcy/dir
│   │   ├── dir-laravel/
│   │   ├── dir-symfony/
│   │   ├── dir-spiffe/
│   │   └── cli/                        # poly-agntcy/cli (Symfony Console)
│   └── examples/
├── ts/
│   ├── package.json                    # pnpm workspace root, private
│   ├── pnpm-workspace.yaml
│   ├── packages/
│   │   ├── dir-next/                   # @poly-agntcy/dir-next
│   │   ├── dir-hono/
│   │   ├── dir-express/
│   │   └── cli/                        # @poly-agntcy/cli (commander)
│   └── examples/
├── py/
│   ├── pyproject.toml                  # uv workspace, private
│   ├── packages/
│   │   ├── dir-fastapi/
│   │   ├── dir-flask/
│   │   ├── dir-django/
│   │   └── cli/                        # poly-agntcy-cli (Typer)
│   └── examples/
├── docs/
│   ├── adr/                            # ADR-0001..0006
│   ├── architecture.md
│   ├── spec-version.md                 # BSR ref ↔ DIR spec ↔ official SDK
│   └── superpowers/specs/              # this doc + future
└── examples/cross/                     # cross-lang end-to-end test
```

## 6. Publishable packages (14)

| `path` | `component` (tag prefix) | Registry name | Ecosystem |
|---|---|---|---|
| `go` | `go` | `hop.top/agntcy` | go |
| `go/spiffe` | `go-spiffe` | `hop.top/agntcy/spiffe` | go |
| `rs/poly-agntcy-dir` | `rs` | `poly-agntcy-dir` | rs |
| `rs/poly-agntcy-dir-spiffe` | `rs-spiffe` | `poly-agntcy-dir-spiffe` | rs |
| `php/packages/dir` | `php` | `poly-agntcy/dir` | php |
| `php/packages/dir-laravel` | `php-laravel` | `poly-agntcy/dir-laravel` | php |
| `php/packages/dir-symfony` | `php-symfony` | `poly-agntcy/dir-symfony` | php |
| `php/packages/dir-spiffe` | `php-spiffe` | `poly-agntcy/dir-spiffe` | php |
| `ts/packages/dir-next` | `ts-next` | `@poly-agntcy/dir-next` | ts |
| `ts/packages/dir-hono` | `ts-hono` | `@poly-agntcy/dir-hono` | ts |
| `ts/packages/dir-express` | `ts-express` | `@poly-agntcy/dir-express` | ts |
| `py/packages/dir-fastapi` | `py-fastapi` | `poly-agntcy-dir-fastapi` | py |
| `py/packages/dir-flask` | `py-flask` | `poly-agntcy-dir-flask` | py |
| `py/packages/dir-django` | `py-django` | `poly-agntcy-dir-django` | py |

CLI tools are not separately released. They ship as bin targets /
console scripts inside an existing released package:

- Go: `go/cmd/agntcy/` is a bin in the `hop.top/agntcy` module
  (released via `go` component).
- Rust: `rs/poly-agntcy-dir/` declares a `[[bin]]` target `agntcy`
  (released via `rs` component, no separate CLI crate).
- PHP: `php/packages/cli/` is published as the `poly-agntcy/dir`
  package's `bin/` entry; the `cli/` dir is part of the `dir`
  package's path tree, not a separate Composer package (released
  via `php` component).
- TS: `ts/packages/cli/` is published as `@poly-agntcy/dir-next`'s
  `bin/` field (released via `ts-next` component).
- Py: `py/packages/cli/` is exposed via `[project.scripts]` in
  `poly-agntcy-dir-fastapi`'s `pyproject.toml` (released via
  `py-fastapi` component).

This keeps the tag count at 14.

All 14 components are single-segment per
[hop-top/.github SKILL.md tag-shape glob trap](https://github.com/hop-top/.github/blob/main/SKILL.md#tag-shape-glob-trap).

## 7. Mirrors (per-language)

Five mirror repos, per-component tags. Each push lands the subtree
of its language root.

| Source path | Mirror | Components pushed |
|---|---|---|
| `go/` | `hop-top/agntcy` | `go`, `go-spiffe` |
| `rs/` | `hop-top/agntcy-rs` | `rs`, `rs-spiffe` |
| `php/` | `hop-top/agntcy-php` | `php`, `php-laravel`, `php-symfony`, `php-spiffe` |
| `ts/` | `hop-top/agntcy-ts` | `ts-next`, `ts-hono`, `ts-express` |
| `py/` | `hop-top/agntcy-py` | `py-fastapi`, `py-flask`, `py-django` |

Mirror naming aligns with
[hop-top/.github SKILL.md repo naming convention](https://github.com/hop-top/.github/blob/main/SKILL.md#repo-naming-convention)
(Go takes bare-name slot; others get `-<lang>` suffix).

## 8. Protobuf strategy

Single SOT: `proto/buf.yaml` BSR module ref to `buf.build/agntcy/dir`.

```yaml
# proto/buf.yaml
version: v2
deps:
  - buf.build/agntcy/dir
```

`proto/buf.gen.yaml` declares per-language generation targets:

- Go: `protoc-gen-go`, `protoc-gen-connect-go` → `go/internal/proto/`
- Rust: `protoc-gen-prost`, `protoc-gen-tonic` → `rs/poly-agntcy-dir/src/proto/`
- PHP: `protoc-gen-php`, `protoc-gen-php-grpc` → `php/packages/dir/src/Generated/`
- TS/Py: no generation. They import the official SDK
  (`agntcy-dir` on npm, `agntcy-dir` on PyPI), which already wraps
  the BSR stubs.

All generated stubs gitignored. `mise run gen` regenerates.
Contributors install `buf` via mise; no system buf required.

Spec bump procedure: edit BSR ref in `proto/buf.yaml`, run
`mise run gen`, fix any breakage per-lang, open one PR with
`BREAKING CHANGE:` trailer if spec broke wire compat. release-please
opens 14 standing PRs (or fewer if not every lang changed).

## 9. SPIFFE strategy

Core packages depend only on standard TLS:

- Go: `crypto/tls`
- Rust: `rustls` (via `tonic`)
- PHP: OpenSSL ext

SPIFFE delivered as separate optional packages:

- `hop.top/agntcy/spiffe` (separate Go module, separate go.mod)
- `poly-agntcy-dir-spiffe` Rust crate (depends on `poly-agntcy-dir`)
- `poly-agntcy/dir-spiffe` Composer package (writes a minimal SPIFFE
  workload-API client, ~300 LOC, gRPC over UDS — no official PHP
  SPIFFE lib exists)

TS/Py adapters inherit SPIFFE behavior from the official SDKs
(both already pull SPIFFE as a required dep).

Each lang ships a `Credentials` interface in its core package with
two stock impls (`InsecureCredentials`, `TlsCredentials`). SPIFFE
extension provides a third (`SpiffeCredentials`).

Examples ship with `TlsCredentials` paths by default; a separate
`examples/<lang>/spiffe/` shows the SPIFFE upgrade.

## 10. Release pipeline

`release-please` (consumer-side) + `hop-top/.github/publish.yml@v0`
(publish + mirror).

### release-please

Single `.github/release-please-config.json` with 14 package entries.
Each package: `release-type` per ecosystem (`go`, `rust`, `php`,
`node`, `python`), `component` = single-segment tag prefix from
table §6.

Path-based, so a commit touching only `php/packages/dir-laravel/`
opens a release PR only for `php-laravel`.

### publish.yml

Matches the
[hop-top/.github quick-start template](https://github.com/hop-top/.github/blob/main/SKILL.md#quick-start-tldr).
`on: push: tags: ['*/v*']`. `uses: hop-top/.github/.github/workflows/publish-on-tag.yml@v0`.

Ecosystems block maps all 14 components. Example slice:

```yaml
ecosystems: |
  go:          { dir: go,                          ecosystem: go,  mirror: hop-top/agntcy }
  go-spiffe:   { dir: go/spiffe,                   ecosystem: go,  mirror: hop-top/agntcy }
  rs:          { dir: rs/poly-agntcy-dir,          ecosystem: rs,  package: poly-agntcy-dir,        mirror: hop-top/agntcy-rs }
  rs-spiffe:   { dir: rs/poly-agntcy-dir-spiffe,   ecosystem: rs,  package: poly-agntcy-dir-spiffe, mirror: hop-top/agntcy-rs }
  php:         { dir: php/packages/dir,            ecosystem: php, package: "poly-agntcy/dir",      mirror: hop-top/agntcy-php }
  # … (full table in implementation plan)
```

### Preflight

`.github/workflows/release-please-preflight.yml` calls
`hop-top/.github/.github/workflows/release-please-preflight.yml@v0`
to validate the three-way name alignment
(`release-please-config.component` ↔ `publish.yml:ecosystems` key
↔ mirror basename) at PR time, not at tag-push time.

### Secrets

Per [hop-top/.github SKILL.md secrets reference](https://github.com/hop-top/.github/blob/main/SKILL.md):

- `NPM_REGISTRY_TOKEN`
- `CARGO_REGISTRY_TOKEN`
- `PACKAGIST_USERNAME` + `PACKAGIST_TOKEN`
- `GH_MIRROR_PAT`
- PyPI uses OIDC trusted publishing (no token)
- Go has no publish secret (proxy.golang.org auto-pulls)

## 11. CI

Single `.github/workflows/ci.yml`. Matrix per-lang. Steps:

1. checkout
2. `jdx/mise-action@v2`
3. `mise run gen` (regen stubs for go/rs/php)
4. parallel jobs per lang: lint, test
5. cross-lang integration job: `examples/cross/` against
   docker-composed local DIR instance

Cross-lang job runs on PR + nightly. Lang-specific jobs run
path-filtered (Go changes don't trigger Rust tests).

## 12. Examples

```
examples/
  go/register-and-discover/
  rs/register-and-discover/
  php/laravel-quickstart/
  php/symfony-quickstart/
  php/vanilla-quickstart/
  ts/next-route/
  ts/hono-edge/
  ts/express-quickstart/
  py/fastapi-quickstart/
  py/flask-quickstart/
  py/django-quickstart/
  cross/                                # Go ↔ Py ↔ PHP via local DIR
```

Each lang example: register an identity, publish a descriptor,
discover a peer by capability, send a signed message.

Cross example uses docker-compose to bring up a local DIR instance
and runs the three agents against it. Same example used in CI.

## 13. ADRs

| # | Title |
|---|-------|
| ADR-0001 | Scope: DIR only, phase 1 of polyglot AGNTCY family |
| ADR-0002 | Protobuf: BSR pin, codegen via mise, no checked-in stubs |
| ADR-0003 | SPIFFE: optional extension package per language |
| ADR-0004 | Release: per-package release-please tracks |
| ADR-0005 | Layout: L3 flat-per-lang, TS/Py adapter-only |
| ADR-0006 | Naming: `poly-agntcy` source, `hop.top/agntcy` Go vanity, per-language mirror |

## 14. Tooling

`mise.toml` is the SOT for tool versions:

```toml
[tools]
go = "1.23"
rust = "1.83"
node = "22"
pnpm = "9"
python = "3.12"
uv = "latest"
php = "8.2"
composer = "latest"
buf = "1.47"
```

Top-level tasks:

- `mise run install` — install all lang deps
- `mise run gen` — regen protobuf stubs (go, rs, php)
- `mise run lint` — lint all langs (parallel)
- `mise run test` — test all langs (parallel)
- `mise run cross` — cross-lang integration test

## 15. Quality bar

- Apache-2.0 (matches all `agntcy/*` repos)
- All langs strict-typed (Go: `golangci-lint`, Rust: `cargo clippy
  -D warnings`, PHP: PHPStan max + strict types, TS: `strict: true`,
  Py: `mypy --strict`)
- Test coverage target: 80% per package (informational, not gated)
- Examples are executable + CI-tested
- Public API stability: minor-pin recommended pre-1.0

## 16. Open questions

None at design time. Implementation plan will surface specifics
(exact dep versions, exact buf BSR ref to pin, exact `mise.toml`
versions).

## 17. Future phases (out of scope here, but mapped)

Sibling repos (or sibling top-level dirs — decision deferred):

- `poly-agntcy-acp` — Agent Connect Protocol SDK suite
- `poly-agntcy-identity` — Identity SDK suite
- `poly-agntcy-slim` — SLIM transport SDK suite
- `poly-agntcy-oasf` — OASF schema toolkit
- `poly-agntcy-runtime` — agent runtime / orchestration

Each follows the same A3+L3+P2+S2+R1 shape as this repo.
