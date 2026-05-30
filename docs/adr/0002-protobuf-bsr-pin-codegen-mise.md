# ADR-0002: Protobuf — BSR pin, codegen via mise, no checked-in stubs

- Status: Accepted
- Date: 2026-05-30
- Deciders: jadb

## Context

DIR is a gRPC spec. Five languages need wire-compatible stubs against the
same spec version. Three failure modes to avoid:

1. Stubs drift between languages (Go on `v0.4`, Rust on `v0.5`).
2. Stubs checked into git rot and become the source of truth instead of
   the spec.
3. Contributors need a system `buf`/`protoc` install before they can run
   the build.

The AGNTCY spec already publishes a Buf Schema Registry module at
`buf.build/agntcy/dir`. That is the canonical wire definition.

## Decision

Single SOT: `proto/buf.yaml` declares a BSR dep on `buf.build/agntcy/dir`
pinned to a specific module ref. `proto/buf.gen.yaml` declares per-
language generation targets. All generated stubs are gitignored.
Contributors run `mise run gen` to regenerate; `buf` is provisioned via
`mise.toml`, so no system install is required.

TS and Py do not generate from BSR — they import the official
`agntcy-dir` packages on npm and PyPI, which already wrap the stubs.

## Consequences

Positive:

- One config change + regen propagates a spec bump across Go, Rust, PHP.
- Zero generated code in diffs — review focuses on hand-written wrappers.
- Reproducible builds: `mise.toml` pins `buf` version; BSR ref pins spec.

Negative:

- BSR digest pin can fail on bootstrap if `buf.lock` is missing or BSR
  auth is misconfigured. CI lands `buf.lock` on first green run; local
  bootstrap may need a one-shot `buf dep update` before `mise run gen`
  succeeds.
- `mise run gen` is a required step before lint/test in CI matrix.

Neutral:

- TS/Py path differs from Go/Rust/PHP — they consume official SDK
  binaries rather than regenerating from BSR. Documented in
  `docs/spec-version.md`.

## Alternatives considered

- Check in generated stubs — rejected: drift between langs, review noise,
  spec-bump merge conflicts.
- Vendor the `.proto` files into `proto/` — rejected: duplicates the
  AGNTCY upstream; BSR ref is the canonical pin.
- `protoc` directly without `buf` — rejected: BSR dep resolution + lint
  rules are buf-native; `protoc` loses both.

## Related

- Design spec: `docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`
  section 8
- Tooling: `mise.toml` provides `buf 1.47`
