# ADR-0006: Naming — `poly-agntcy` source, `hop.top/agntcy` Go vanity, per-language mirror

- Status: Partially superseded by ADR-0008 (package names) and ADR-0010 (mirror topology)
- Date: 2026-05-30
- Deciders: jadb

## Context

Three names must be chosen and reconciled:

1. The source repo name on GitHub (`hop-top/<???>`).
2. The Go module path users import.
3. The mirror repo names per language for downstream-friendly clones.

Constraints:

- The source repo holds five language trees, so its name should be
  language-agnostic.
- Go module paths are public API: choosing `github.com/hop-top/poly-
  agntcy/go` locks consumers to the GitHub URL and the `/go` subpath
  forever. A vanity host decouples the import path from the repo
  layout.
- Mirror names follow `hop-top/.github` SKILL.md convention: the Go
  mirror takes the bare name slot; other langs get a `-<lang>` suffix.

## Decision

- **Source repo**: `hop-top/poly-agntcy` — language-agnostic, signals
  the polyglot intent.
- **Go module path**: `hop.top/agntcy` — vanity host decouples import
  path from GitHub URL and from the in-repo `go/` subpath.
- **Mirrors**: one per language with per-component tags pushed via the
  shared publish workflow:
  - `hop-top/agntcy` (Go — bare-name slot)
  - `hop-top/agntcy-rs`
  - `hop-top/agntcy-php`
  - `hop-top/agntcy-ts`
  - `hop-top/agntcy-py`

The `hop.top` vanity host serves a `<meta>` redirect for Go module
resolution. The contract:

```html
<meta name="go-import" content="hop.top/agntcy git https://github.com/hop-top/agntcy">
```

Pointing the import path at the `hop-top/agntcy` mirror (not the
source repo) means `go get hop.top/agntcy` resolves cleanly without
the `/go` subpath that monorepo Go modules normally require.

## Consequences

Positive:

- `go get hop.top/agntcy` is a clean, short, brand-consistent import.
- Source repo can be renamed or restructured without breaking `go get`.
- Mirror naming aligns with the existing hop-top convention; the Go
  mirror taking the bare slot signals primacy.
- Each language ecosystem gets a focused mirror — Rust users browse
  `hop-top/agntcy-rs` and see only Rust.

Negative:

- Vanity host adds one moving piece (DNS + meta-serving page) that must
  stay up for `go get` to work.
- The Go mirror push must land before the first tag is cut, or
  `go get hop.top/agntcy@v0.1.0` 404s.

Neutral:

- Non-Go ecosystems use their registry name directly
  (`poly-agntcy-dir` on crates.io, `poly-agntcy/dir` on Packagist,
  `@poly-agntcy/dir-next` on npm, `poly-agntcy-dir-fastapi` on PyPI).
  No vanity layer needed for those.

## Alternatives considered

- `github.com/hop-top/poly-agntcy/go` as the Go module path —
  rejected: locks consumers to the GitHub URL and the `/go` subpath
  forever; rename/restructure breaks every dependent.
- Single mirror with all five languages — rejected: defeats the point
  of mirrors (downstream-friendly, focused clones).
- Bare `hop.top` (no `/agntcy` segment) — rejected: collides with
  future sibling AGNTCY-family modules (`hop.top/agntcy-acp`, etc.).

## Related

- Design spec: `docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`
  sections 7, decisions §4 rows 8–10
- Repo naming convention: hop-top/.github SKILL.md
