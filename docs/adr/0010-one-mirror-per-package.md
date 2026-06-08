# ADR-0010: One mirror per package — `agntcy-<lang>-<rest>` naming

- Status: Accepted
- Date: 2026-06-08
- Deciders: jadb
- Supersedes: ADR-0009 (which deferred this decision)
- Touches: ADR-0006 (mirror naming)

## Context

ADR-0009 surfaced the architectural mismatch between poly-agntcy's
`publish.yml` (multiple components per mirror) and the shared
`hop-top/.github/.github/workflows/mirror-subtree.yml@main` (one
`git push --force` per tag, last-tag-wins). Three options were
enumerated; this ADR picks one and finalizes the naming.

## Decision

**Option 1: one mirror per published package.** Match the working
pattern in `hop-top/poly-kit`.

Naming pattern: **`hop-top/agntcy-<lang>-<rest>`** — language
appears first as the primary axis, framework or qualifier follows.

| Component | Mirror | Status |
|---|---|---|
| `go` | `hop-top/agntcy` | Existing — root Go mirror, vanity-bound |
| `go-spiffe` | `hop-top/agntcy-go-spiffe` | New |
| `rs` | `hop-top/agntcy-rs` | Existing |
| `rs-spiffe` | `hop-top/agntcy-rs-spiffe` | New |
| `php` | `hop-top/agntcy-php` | Existing |
| `php-spiffe` | `hop-top/agntcy-php-spiffe` | New |
| `php-laravel` | `hop-top/agntcy-php-laravel` | New |
| `php-symfony` | `hop-top/agntcy-php-symfony` | New |
| `ts-next` | `hop-top/agntcy-ts-next` | New |
| `ts-hono` | `hop-top/agntcy-ts-hono` | New |
| `ts-express` | `hop-top/agntcy-ts-express` | New |
| `py-fastapi` | `hop-top/agntcy-py-fastapi` | New |
| `py-flask` | `hop-top/agntcy-py-flask` | New |
| `py-django` | `hop-top/agntcy-py-django` | New |

Nine new mirrors. The shared `mirror-subtree.yml` calls
`gh repo create` if the target doesn't exist, so mirrors auto-create
on first tag push to that component (provided `GH_MIRROR_PAT` has
`repo` + admin permissions on the `hop-top` org).

### Existing aggregated mirrors

`hop-top/agntcy-ts` and `hop-top/agntcy-py` were named optimistically
under ADR-0006 ("per-language mirror") but were never the right
shape for the shared workflow's one-subtree-per-tag model. With
this ADR they become orphans.

Decision: archive both. They contain nothing of value (no release
ever cut, no consumer ever pinned them, no Go vanity bound to them).
Archiving preserves the URL for any external link that happens to
exist while making the read-only state explicit.

This is a one-time manual `gh repo archive` operation outside the
publish pipeline.

## Consequences

Positive:

- Each tag → one mirror → no overwrite races. Matches the shared
  workflow's design.
- Each ecosystem registry (crates.io / npm / Packagist / PyPI)
  already publishes one package per release; the mirror layer now
  mirrors that 1:1.
- `mirror-subtree.yml`'s `gh repo create` handles new-mirror
  provisioning automatically — no separate setup script.

Negative:

- Mirror count grows from 5 to 12 active (9 new + 3 retained:
  `agntcy`, `agntcy-rs`, `agntcy-php`). `agntcy-ts` and `agntcy-py`
  archived.
- The Go SPIFFE module path changes from `hop.top/agntcy/spiffe`
  to `hop.top/agntcy-go-spiffe`. Reason: the `hop.top` vanity
  worker only resolves single-segment names (`hop.top/<pkg>` →
  `github.com/hop-top/<pkg>` by convention; multi-segment paths
  like `hop.top/agntcy/spiffe` fall through to the main site
  proxy and don't get a `go-import` meta). With per-package
  mirrors, the spiffe content no longer lives under the
  `hop-top/agntcy` mirror, so the multi-segment path can't
  resolve via the convention. Renaming to a single-segment module
  path aligns with the convention: `hop.top/agntcy-go-spiffe?go-get=1`
  → `github.com/hop-top/agntcy-go-spiffe` automatically.
  Pre-1.0; the only in-repo consumer was the test in
  `sdk/go/spiffe/credentials_test.go`.
- Per-package mirrors are visually noisier in the `hop-top` org
  listing. Counterargument: noise reflects reality (14 packages
  exist; pretending they're 5 mirrors was the original mistake).

Neutral:

- ADR-0006's "per-language mirror" framing is partially superseded.
  The source repo name (`hop-top/poly-agntcy`), Go vanity host
  (`hop.top/agntcy`), and the existing 3 mirrors that *are*
  one-package-per-mirror (`agntcy`, `agntcy-rs`, `agntcy-php`) stay
  per ADR-0006.

## Implementation

In this PR:

1. Update `.github/workflows/publish.yml` `ecosystems` map: each of
   the 14 components gets its own `mirror:` entry.
2. Rename Go SPIFFE module: `sdk/go/spiffe/go.mod` from
   `hop.top/agntcy/spiffe` to `hop.top/agntcy-go-spiffe`. Update
   the one in-repo consumer
   (`sdk/go/spiffe/credentials_test.go`) and release-please-config
   `package-name`. Single-segment vanity resolves via convention.
3. Update `docs/vanity-import.md` published-modules table to the
   new `hop.top/agntcy-go-spiffe` mapping.
4. Mark ADR-0009's Status as `Superseded by ADR-0010`.
5. Update ADR index in `docs/adr/README.md`.

Out of band (manual ops, not in this PR):

- `gh repo archive hop-top/agntcy-ts hop-top/agntcy-py` — retire
  the aggregated mirrors.

Vanity infra needs **no manual change**: the `hop.top` Cloudflare
worker falls back to convention (`hop.top/<pkg>` →
`github.com/hop-top/<pkg>`) when no formula override exists.
`hop.top/agntcy-go-spiffe` will resolve automatically the moment
the mirror exists. Same for all other new single-segment mirror
names — `hop.top/agntcy-rs-spiffe` etc. aren't used as Go module
paths (only Go uses vanity), but the convention works for any
single-segment name.

Verification post-merge (next release cycle):

- Push a real release-please tag (e.g., `go/v0.1.0-alpha.0`) and
  confirm `mirror-subtree.yml` creates and populates
  `hop-top/agntcy` cleanly.
- Push `go-spiffe/v0.1.0-alpha.0` and confirm
  `hop-top/agntcy-go-spiffe` auto-creates and `hop-top/agntcy` is
  unaffected.

## Alternatives reconsidered

Both rejected here for the reasons documented in ADR-0009:

- **Option 2: restructure source into single-prefix bundles** —
  works only for Go; awkward for the other four ecosystems.
- **Option 3: hybrid (Option 2 for Go, Option 1 for the rest)** —
  considered but adds asymmetry without payoff. Go's existing
  `sdk/go/` already structurally contains both `dir/` and `spiffe/`,
  so it *could* go via Option 2 — but separating `go-spiffe/v*` as
  its own release with its own changelog (current release-please
  setup) is more useful than collapsing the two. Keep them as
  separate components with separate mirrors.

## Related

- ADR-0006: Mirror naming convention. Per-language framing
  partially superseded; per-package additions follow the same
  vendor + product-line prefix.
- ADR-0009: Investigation that surfaced this. Superseded by this
  ADR.
- `hop-top/.github/.github/workflows/mirror-subtree.yml@main` — the
  workflow that does `gh repo create` on first push.
- `hop-top/poly-kit` `publish.yml` — reference implementation of
  one-package-per-mirror pattern.
