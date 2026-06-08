# Releasing

## How releases work

1. Conventional Commits on `main` trigger [release-please](https://github.com/googleapis/release-please).
2. release-please opens or updates a standing release PR with per-package version bumps + changelog entries — one entry per [component](.github/release-please-config.json) touched.
3. Merging the release PR cuts tags `<component>/v<version>` (e.g. `go/v0.1.0`, `php-laravel/v0.1.0`).
4. Each tag push fires `publish.yml`, which delegates to `hop-top/.github/.github/workflows/publish-on-tag.yml@v0`.
5. That workflow publishes to the relevant registry and pushes the language subtree to its mirror.

## Components

Fourteen release tracks, one per publishable package. See [the design spec §6](docs/superpowers/specs/2026-05-30-poly-agntcy-design.md#6-publishable-packages-14).

| Component | Path | Ecosystem | Registry name |
|---|---|---|---|
| `go` | `go/` | go | `hop.top/agntcy` |
| `go-spiffe` | `go/spiffe/` | go | `hop.top/agntcy/spiffe` |
| `rs` | `rs/poly-agntcy-dir/` | rust | `poly-agntcy-dir` |
| `rs-spiffe` | `rs/poly-agntcy-dir-spiffe/` | rust | `poly-agntcy-dir-spiffe` |
| `php` | `php/packages/dir/` | php | `poly-agntcy/dir` |
| `php-laravel` | `php/packages/dir-laravel/` | php | `poly-agntcy/dir-laravel` |
| `php-symfony` | `php/packages/dir-symfony/` | php | `poly-agntcy/dir-symfony` |
| `php-spiffe` | `php/packages/dir-spiffe/` | php | `poly-agntcy/dir-spiffe` |
| `ts-next` | `ts/packages/dir-next/` | npm | `@poly-agntcy/dir-next` |
| `ts-hono` | `ts/packages/dir-hono/` | npm | `@poly-agntcy/dir-hono` |
| `ts-express` | `ts/packages/dir-express/` | npm | `@poly-agntcy/dir-express` |
| `py-fastapi` | `py/packages/dir-fastapi/` | PyPI | `poly-agntcy-dir-fastapi` |
| `py-flask` | `py/packages/dir-flask/` | PyPI | `poly-agntcy-dir-flask` |
| `py-django` | `py/packages/dir-django/` | PyPI | `poly-agntcy-dir-django` |

Each component releases independently. A commit touching only `php/packages/dir-laravel/` opens a release PR only for `php-laravel`.

## Version policy

Pre-1.0 (`0.x.y`):

- `feat:` bumps minor (`0.1.0` → `0.2.0`)
- `fix:` bumps patch (`0.1.0` → `0.1.1`)
- `BREAKING CHANGE:` still bumps minor pre-1.0

Configured via [`bump-minor-pre-major: true`](.github/release-please-config.json).

Post-1.0: standard SemVer.

## Credentials

All registry credentials are configured as **organization-level secrets on `hop-top`** with `ALL` repo visibility. New repos under the org inherit them automatically.

See [docs/release-runbook.md](docs/release-runbook.md) for the secret inventory, rotation cadence, and per-package PyPI OIDC + Packagist webhook setup.

## Spec version bumps

When the AGNTCY DIR spec moves forward:

1. Edit the BSR ref in [`proto/buf.yaml`](proto/buf.yaml).
2. Run `mise run gen` to regenerate Go/Rust/PHP stubs.
3. Fix per-language breakage.
4. Open one PR with `BREAKING CHANGE:` trailer if the spec broke wire compat. release-please opens up to 14 standing PRs in response (or fewer if not every language was affected).
