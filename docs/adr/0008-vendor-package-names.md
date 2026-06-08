# ADR-0008: Vendor-rooted package names — `hop-top/agntcy-*`

- Status: Accepted
- Date: 2026-06-08
- Deciders: jadb
- Partially supersedes: ADR-0006 (package name choices only; source repo,
  Go vanity, and mirror names from ADR-0006 are unchanged)

## Context

ADR-0006 set names assuming `poly-` would be the published package
prefix (`poly-agntcy-dir`, `@poly-agntcy/dir-hono`, `PolyAgntcy\Dir\…`).
The `poly-` prefix was repurposed from its original intent: it signals
that this *source repository* is polyglot (multi-language), not that
*published packages* are polyglot. Each published package targets one
language; the polyglot characteristic lives at the source-repo level
and is already conveyed by `poly-agntcy` as a repo name.

Consequences of the original choice:

- `poly-agntcy-dir` on crates.io reads as "polyglot version of
  agntcy-dir" — but it's just the Rust crate.
- `@poly-agntcy/dir-hono` on npm hides that `hop-top` is the publisher.
  Discoverability for the org name suffers.
- PHP namespace `PolyAgntcy\Dir\…` doesn't reflect the actual GitHub
  org (`HopTop`).

Mirror naming from ADR-0006 (`hop-top/agntcy`, `hop-top/agntcy-rs`,
etc.) already follows the vendor + product-line pattern. Published
package names should match.

## Decision

Published package names use the vendor + product-line pattern:

| Ecosystem | Pattern | Examples |
|-----------|---------|----------|
| crates.io | `hop-top-agntcy-<thing>` | `hop-top-agntcy-dir`, `hop-top-agntcy-dir-spiffe` |
| npm | `@hop-top/agntcy-<thing>` | `@hop-top/agntcy-dir-next`, `@hop-top/agntcy-dir-hono`, `@hop-top/agntcy-dir-express` |
| Packagist | `hop-top/agntcy-<thing>` | `hop-top/agntcy-dir`, `hop-top/agntcy-dir-laravel`, `hop-top/agntcy-dir-symfony`, `hop-top/agntcy-dir-spiffe` |
| PyPI | `hop-top-agntcy-<thing>` | `hop-top-agntcy-dir-fastapi`, `hop-top-agntcy-dir-flask`, `hop-top-agntcy-dir-django` |
| Go vanity | `hop.top/agntcy[/<thing>]` (unchanged) | `hop.top/agntcy/dir`, `hop.top/agntcy/spiffe` |
| PHP PSR-4 | `HopTop\Agntcy\Dir\…` | `HopTop\Agntcy\Dir\Client`, `HopTop\Agntcy\Dir\Laravel\…` |
| Python module | `agntcy_dir_<framework>` | `agntcy_dir_fastapi`, `agntcy_dir_flask`, `agntcy_dir_django` |

Source repo name (`hop-top/poly-agntcy`), Go vanity host
(`hop.top/agntcy`), and per-language mirror repo names from ADR-0006
are unchanged. Only published-package names and PHP/Python language
identifiers change.

Python note: the PyPI distribution name (`hop-top-agntcy-dir-fastapi`)
and the importable Python module (`agntcy_dir_fastapi`) intentionally
diverge. `pip install hop-top-agntcy-dir-fastapi` followed by
`import agntcy_dir_fastapi` is a common pattern for vendor-rooted
distribution + clean import path. Documented in adapter READMEs.

## Consequences

Positive:

- Vendor (`hop-top`) appears in every published package name. Org
  discoverability matches existing GitHub topology.
- Product line (`agntcy`) appears in every published package name.
  Cross-ecosystem search consistency.
- PHP namespace matches GitHub org (`HopTop\…`) rather than
  source-repo-prefix (`PolyAgntcy\…`).
- No package name carries the `poly-` repo-type signal that consumers
  don't need.

Negative:

- One coordinated breaking-change release across all 14 packages.
  Every consumer's import paths or package references change.
- ADR-0006 must be read alongside this ADR; the part about source repo
  name and mirror naming still applies, but the package-name examples
  in ADR-0006 are obsolete.
- Python distribution-vs-module divergence (`hop-top-agntcy-dir-X` vs
  `agntcy_dir_X`) is a contributor onboarding cost; adapter READMEs
  must document it explicitly.

Neutral:

- Source code in this repo is renamed in one PR alongside the layout
  migration (per T-0055). No two-step deprecation.

## Alternatives considered

- **Drop the `agntcy` middle segment** — `@hop-top/dir-hono`,
  `hop-top-dir-fastapi`. Rejected: loses the product-line signal,
  collides with potential future `hop-top/<other-product>` packages.
- **Keep `poly-` prefix** (status quo from ADR-0006). Rejected: see
  Context — `poly-` is a repo-type marker, not a package-namespace
  marker.
- **Use `@agntcy` npm scope and `agntcy-*` crate/PyPI names** (no
  vendor at all). Rejected: collides with the AGNTCY upstream org's
  own published packages; would create attribution confusion.
- **Phased rename across releases instead of one PR.** Rejected:
  packages haven't shipped 1.0 yet (all at `0.0.0`); pre-1.0 is exactly
  when this kind of break has zero external cost.

## Related

- ADR-0006: Naming — source repo, Go vanity, mirrors. Package-name
  examples there are obsolete; source/vanity/mirror parts remain.
- Migration tracked under T-0055 (adapters promotion); the rename was
  folded into that task rather than scheduled separately because the
  blast radius is identical (every package metadata touched anyway).
