# ADR-0009: Mirror publish topology — defer multi-component-per-mirror fix

- Status: Proposed
- Date: 2026-06-08
- Deciders: jadb
- Related: ADR-0006 (mirror naming), ADR-0007 (role-based layout, phase 8)

## Context

T-0058 set out to verify the publish workflow's mirror push works
end-to-end after the role-based layout migration. Investigating the
shared workflows surfaced a pre-existing architectural mismatch
between poly-agntcy's `publish.yml` and what
`hop-top/.github/.github/workflows/publish-on-tag.yml@v0` (and its
delegate `mirror-subtree.yml`) actually support.

### How the shared workflow works

`publish-on-tag.yml@v0`:

1. Triggered by a tag matching `<component>/v<version>`.
2. Parses the tag, looks up the matching entry in the caller's
   `ecosystems:` map.
3. Runs the ecosystem-specific publish workflow (npm/cargo/PyPI/
   Packagist/Go-module-noop).
4. Calls `mirror-subtree.yml@main` once with the component's
   `dir:` as `source-prefix` and `mirror:` as `target-repo`.

`mirror-subtree.yml`:

1. `git subtree split --prefix="$PREFIX" HEAD` — produces a SHA whose
   tree is `<repo>/<PREFIX>/**` flattened to root.
2. `git push mirror "${SHA}:refs/heads/main" --force` — overwrites the
   mirror's `main` branch with the split tree.
3. Force-tags the new SHA on the mirror.

The push is `--force` because the mirror's `main` is by construction
a function of the latest tagged source state. There is no merge, no
union of trees from prior tags.

### poly-agntcy's `publish.yml` after T-0055

Fourteen components, mapped to five mirrors:

| Mirror | Components |
|---|---|
| `hop-top/agntcy` (Go) | `go` (`sdk/go`), `go-spiffe` (`sdk/go/spiffe`) |
| `hop-top/agntcy-rs` | `rs` (`sdk/rs/hop-top-agntcy-dir`), `rs-spiffe` (`sdk/rs/hop-top-agntcy-dir-spiffe`) |
| `hop-top/agntcy-php` | `php` (`sdk/php/packages/dir`), `php-laravel` (`adapters/laravel`), `php-symfony` (`adapters/symfony`), `php-spiffe` (`sdk/php/packages/dir-spiffe`) |
| `hop-top/agntcy-ts` | `ts-next` (`adapters/next`), `ts-hono` (`adapters/hono`), `ts-express` (`adapters/express`) |
| `hop-top/agntcy-py` | `py-fastapi` (`adapters/fastapi`), `py-flask` (`adapters/flask`), `py-django` (`adapters/django`) |

When `go/v0.1.0` is pushed, the mirror push subtree-splits `sdk/go/`
and force-pushes that tree to `hop-top/agntcy`. The mirror now
contains the SDK tree.

When `go-spiffe/v0.1.0` is pushed later, the mirror push subtree-splits
`sdk/go/spiffe/` and force-pushes *that* tree to `hop-top/agntcy`.
The mirror's `main` is now just the SPIFFE extension; the SDK is gone.

`go get hop.top/agntcy/dir` (which resolves to
`hop-top/agntcy@<commit>/dir`) breaks. Same failure mode for every
multi-component mirror.

This was not introduced by ADR-0007. The same mismatch existed in the
flat layout (`go/` and `go/spiffe/` both targeted `hop-top/agntcy`),
but no release has been cut against poly-agntcy yet (manifest pins
all packages at `0.0.0`) so the breakage hasn't manifested.

### How the working sibling repos do it

`hop-top/poly-kit` (the closest sibling that ships): every published
package maps to exactly one mirror, and every mirror receives content
from exactly one component.

- `kit` → `hop-top/kit` (whole repo via `dir: .`)
- `kit-ts` → `hop-top/kit-ts` (`sdk/ts` only)
- `kit-py` → `hop-top/kit-py` (`sdk/py` only)
- etc.

There is no multi-component-to-one-mirror pattern in any working
hop-top repo we can crib from.

## Decision

**Defer the fix.** Investigation only in this ADR; no design choice
yet. T-0058's "verify vanity" sub-goal cannot be discharged until the
topology is fixed, because the topology is what determines whether
the mirror's `main` resembles the import path consumers expect.

The fix needs a separate ADR-0010 picking one of the two options
below, plus an execution PR implementing it.

## Options for the future ADR-0010

### Option 1 — one mirror per package

Match `poly-kit`'s working pattern. Create dedicated mirrors for the
non-core packages on each language side:

- Keep `hop-top/agntcy` for `sdk/go` only.
- Create `hop-top/agntcy-spiffe-go` for `sdk/go/spiffe`.
- Keep `hop-top/agntcy-rs` for `sdk/rs/hop-top-agntcy-dir` only.
- Create `hop-top/agntcy-spiffe-rs` for the SPIFFE Rust crate.
- For PHP/TS/Py: each adapter gets its own mirror
  (`hop-top/agntcy-dir-php` for SDK, `hop-top/agntcy-laravel-php`,
  `hop-top/agntcy-symfony-php`, etc.) — or matching whatever Packagist /
  npm consumers actually expect.

**Vanity contract impact**: `go get hop.top/agntcy/spiffe` no longer
resolves to a subdirectory of `hop-top/agntcy` — it resolves to a
separate mirror. ADR-0006's vanity meta tag needs an additional
`<meta>` entry for `hop.top/agntcy/spiffe`. (Or the spiffe Go module
becomes `hop.top/agntcy-spiffe` to match the mirror name; that's a
breaking change for any consumer.)

**Trade-offs**:
- Pro: matches the working pattern in poly-kit, no shared-workflow
  changes needed.
- Pro: each mirror's release tags are unambiguous.
- Con: more mirror repos to manage (poly-agntcy goes from 5 mirrors
  to ~9).
- Con: vanity Go-import contract gets more verbose (one meta per
  module).

### Option 2 — restructure source so one prefix produces the whole mirror

Bundle all of a language's published packages into a single source
prefix that, when subtree-split, produces a tree containing every
package as a subdirectory.

- Go is already close: `sdk/go/` contains `dir/`, `spiffe/`, and
  `go.mod`. The Go vanity `hop.top/agntcy/dir` resolves to
  `hop-top/agntcy@<commit>/dir`. **A single `sdk/go/` subtree push
  to `hop-top/agntcy` already produces the right tree.** Drop the
  `go-spiffe` component entirely; it ships as part of the `go`
  component release.
- Rust would need to combine both crates into one mirror tree with
  a top-level `Cargo.toml` workspace pointing at both. Each crate
  publishes to crates.io under its own name; the mirror is just for
  source-of-truth visibility.
- PHP / TS / Python: combine SDK + adapters under one prefix per
  language and one mirror per language. Mirror push uses
  `dir: sdk/<lang>` and includes adapters via workspace path refs
  that resolve at install time.

**Trade-offs**:
- Pro: keeps mirror count at 5.
- Pro: vanity contract unchanged.
- Con: source-tree layout shifts again (mostly cosmetic — workspaces
  point at adapter paths via relative refs already).
- Con: tag-to-release mapping diverges from package-to-release. One
  tag fires multiple package publishes; release notes get noisy.
- Con: works elegantly for Go (where vanity == one mirror == one
  module already), awkwardly for Rust/PHP/TS/Py where each language
  has multiple independent packages.

### Honest take

**Option 1** matches the working pattern, costs more mirror repos,
and breaks the implicit "one mirror per language" idea from ADR-0006.
But it's the path that works without touching the shared workflow.

**Option 2** preserves ADR-0006's mirror naming, but only works
cleanly for Go and requires re-explaining how the publish flow
relates to package releases (one tag → multiple ecosystem
publishes).

A hybrid is also possible: Option 2 for Go (it's already structured
right), Option 1 for everything else.

## Consequences

For this investigation ADR:

- T-0058 in tlc remains in-progress; cannot be completed without
  the ADR-0010 decision.
- The current `publish.yml` is **functionally broken for any tag
  push that follows another tag targeting the same mirror**. Cutting
  a release before ADR-0010 lands will reproduce the breakage.
- ADR-0007 phase 8 ("mirror push remap + vanity verification")
  re-scopes from "verify it works" to "design topology that can
  work."

## Related

- ADR-0006: Naming + vanity + mirrors. Mirror naming convention
  (`hop-top/agntcy-<lang>`) is in tension with Option 1 (which
  introduces per-package mirrors).
- ADR-0007: Role-based layout. Phase 8 needs the ADR-0010 decision
  before it can be marked done.
- `hop-top/.github/.github/workflows/publish-on-tag.yml@v0` —
  unchanged behavior; the constraint sits there.
- `hop-top/.github/.github/workflows/mirror-subtree.yml@main` —
  the `git push mirror --force` is where the overwrite happens.
- `hop-top/poly-kit` `publish.yml` — reference implementation of
  one-mirror-per-package pattern.

## Decision (for this ADR)

None. Investigation complete; recommend ADR-0010 to choose between
Option 1 / Option 2 / hybrid.
