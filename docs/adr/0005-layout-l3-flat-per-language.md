# ADR-0005: Layout — L3 flat-per-lang, TS/Py adapter-only

- Status: Accepted
- Date: 2026-05-30
- Deciders: jadb

## Context

A five-language monorepo can be shaped multiple ways:

- **L1**: deep nested taxonomy (`languages/go/sdks/dir/...`).
- **L2**: shared `packages/` root with language prefixes.
- **L3**: flat per-language root (`go/`, `rs/`, `php/`, `ts/`, `py/`).
- **L4**: separate sibling repos, no monorepo.

Each language ecosystem expects its own build conventions at the root of
its tree (Go: `go.mod`; Rust: `Cargo.toml` workspace; PHP: `composer.json`
path repo; TS: `pnpm-workspace.yaml`; Python: `pyproject.toml` uv
workspace). Deep nesting fights those conventions.

Mirror push targets (per ADR-0006 / spec §7) need to subtree a single
top-level directory cleanly into a per-language mirror repo.

## Decision

Adopt layout L3: one flat directory per language at the repo root
(`go/`, `rs/`, `php/`, `ts/`, `py/`), each owning its own build root and
workspace config. TS and Python ship adapter packages only on top of the
official `agntcy-dir` SDKs; Go, Rust, PHP ship full first-party SDKs.

Each language root contains a `packages/` (or equivalent) directory
holding the publishable units listed in spec §6.

## Consequences

Positive:

- Each language root is idiomatic for its ecosystem — tools like
  `cargo`, `pnpm`, `uv`, `composer` work without flags or workarounds.
- `git subtree push` from `<lang>/` to the mirror repo is a one-liner.
- Path-based release-please attribution is unambiguous.
- Contributors only `cd` into their language root and ignore the rest.

Negative:

- Cross-language code sharing requires explicit examples in
  `examples/cross/` — no implicit shared utilities directory.
- Top-level diff churn when multiple langs change in one PR.

Neutral:

- Adapter-only TS/Py keeps those trees lean (no transport, no codegen).

## Alternatives considered

- L1 deep nested taxonomy — rejected: fights every language's
  conventions; mirrors get awkward subtree roots.
- L2 shared `packages/` — rejected: forces every lang into one
  workspace shape; loses native ergonomics.
- L4 sibling repos — rejected: loses single-PR spec-bump propagation;
  loses cross-lang integration test (spec §11–12).
- L3 with TS/Py also first-party — rejected: reimplements maintained
  upstream SDKs (see ADR-0001).

## Related

- Design spec: `docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`
  sections 5, 6
