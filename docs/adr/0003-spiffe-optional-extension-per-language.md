# ADR-0003: SPIFFE — optional extension package per language

- Status: Accepted
- Date: 2026-05-30
- Deciders: jadb

## Context

SPIFFE/SPIRE is the deployment story most cited by AGNTCY users running
agents in zero-trust meshes — but baking the SPIFFE workload-API client
into the core DIR SDK forces every consumer to pull a heavy dep tree
(Go: `go-spiffe/v2`; Rust: `spiffe`; PHP: no maintained client at all).

Most early consumers will run mTLS with a static cert bundle, not SPIFFE.
Forcing the SPIFFE dep penalises the common case.

## Decision

Core packages depend only on stdlib TLS (`crypto/tls`, `rustls`, OpenSSL
ext). SPIFFE ships as a separate optional package per language:

- `hop.top/agntcy/spiffe` (separate Go module, own `go.mod`)
- `hop-top-agntcy-dir-spiffe` (Rust crate)
- `hop-top/agntcy-dir-spiffe` (Composer package — includes a ~300 LOC
  minimal workload-API client over UDS gRPC; no maintained PHP SPIFFE lib
  exists)

Each lang exposes a `Credentials` interface in core with two stock impls
(`InsecureCredentials`, `TlsCredentials`). The SPIFFE extension provides
a third (`SpiffeCredentials`).

TS/Py adapters inherit SPIFFE behaviour from the official SDKs, which
already pull SPIFFE as a required dep.

## Consequences

Positive:

- Common-case `TlsCredentials` users avoid the SPIFFE dep tree.
- SPIFFE consumers opt in by importing one extra package; no
  monkey-patching, no feature flags.
- Separate Go module means independent semver for SPIFFE breakage.

Negative:

- PHP package ships a hand-rolled SPIFFE workload-API client (~300 LOC).
  Maintenance burden until an upstream PHP SPIFFE lib appears.
- Documentation has to teach the `Credentials` interface explicitly so
  users find the extension path.

Neutral:

- TS/Py asymmetry: SPIFFE is inherited, not optional, because upstream
  baked it in.

## Alternatives considered

- SPIFFE in core — rejected: dep weight penalises non-SPIFFE users; PHP
  has no client to depend on.
- Feature-flag at build time — rejected: Go/Rust/PHP build tooling for
  conditional deps is awkward; extension package is the idiomatic shape.
- Pluggable creds provider with no built-in SPIFFE — rejected: pushes
  the SPIFFE client maintenance onto every consumer.

## Related

- Design spec: `docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`
  section 9
