# Release runbook

Source of truth for what gets released, how, and what credentials drive it.

## Packages (14)

See [`docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`](superpowers/specs/2026-05-30-poly-agntcy-design.md) §6 for the canonical table. Components map 1:1 to release-please entries in [`.github/release-please-config.json`](../.github/release-please-config.json).

## Flow

1. Commits land on `main` with Conventional Commits.
2. `release-please.yml` opens or updates standing release PRs per package, one per touched component.
3. Merging a release PR cuts tag `<component>/v<version>` (e.g. `go/v0.1.0`).
4. Tag push fires `publish.yml`, which calls `hop-top/.github/.github/workflows/publish-on-tag.yml@v0` and routes to the ecosystem publisher.
5. The reusable workflow also pushes the language subtree to its mirror (`hop-top/agntcy-*`).

`release-please-preflight.yml` validates the three-way alignment (release-please component ↔ publish ecosystems key ↔ mirror basename) on every PR.

## Credentials

All registry credentials live as **org-level secrets on `hop-top`** with `ALL` repo visibility. New repos created under `hop-top` inherit them automatically — no per-repo `gh secret set` needed.

| Secret | Ecosystem | Source | Rotation |
|---|---|---|---|
| `NPM_REGISTRY_TOKEN` | npm | npmjs.com → access token (automation) | Quarterly |
| `CARGO_REGISTRY_TOKEN` | crates.io | crates.io → API tokens | Annual |
| `PACKAGIST_USERNAME` | Packagist | packagist.org account name | On account change |
| `PACKAGIST_TOKEN` | Packagist | packagist.org → API token | Quarterly |
| `GH_MIRROR_PAT` | mirror push | github.com PAT with `contents:write` on mirrors | Annual |
| `PYPI_REGISTRY_TOKEN` | PyPI (fallback) | pypi.org → API token | OIDC preferred, see below |
| `GH_RELEASE_PLEASE_PAT` | release-please | github.com PAT for cross-repo bot writes | Annual |
| `RELEASE_BOT_APP_ID` | release-please | GitHub App id | On app rotation |
| `RELEASE_BOT_PRIVATE_KEY` | release-please | GitHub App private key | On app rotation |

`Go` ships via `proxy.golang.org` and needs no secret. See [Vanity import](vanity-import.md) for module resolution.

## PyPI OIDC trusted publishing (per package)

Three Python packages publish via OIDC instead of `PYPI_REGISTRY_TOKEN`:

- `hop-top-agntcy-dir-fastapi`
- `hop-top-agntcy-dir-flask`
- `hop-top-agntcy-dir-django`

Each one needs a trusted publisher configured on PyPI (one-time, web UI):

1. Log into pypi.org as the package owner.
2. Project → Settings → Publishing → Add a new trusted publisher → GitHub.
3. Set: owner `hop-top`, repository `poly-agntcy`, workflow filename `publish.yml`, environment (leave blank).
4. Repeat per package.

Until the trusted publishers are registered, the Python publish jobs fall back to `PYPI_REGISTRY_TOKEN` (org secret, present).

## Packagist webhook (per package)

Four PHP packages need their Packagist project linked to GitHub for auto-update:

- `hop-top/agntcy-dir`
- `hop-top/agntcy-dir-laravel`
- `hop-top/agntcy-dir-symfony`
- `hop-top/agntcy-dir-spiffe`

For each:

1. packagist.org → Submit → paste `https://github.com/hop-top/agntcy-php` (mirror).
2. Settings → API → "Show your API token" — already captured as `PACKAGIST_TOKEN`.
3. GitHub mirror repo → Settings → Webhooks → Add webhook → URL `https://packagist.org/api/github?username=<PACKAGIST_USERNAME>` → Content type `application/x-www-form-urlencoded` → token = `PACKAGIST_TOKEN`.

The mirror push (`publish-on-tag.yml@v0`) handles the GitHub-side push; the webhook is what tells Packagist to pull. One-time setup per package.

## Mirrors

| Source path | Mirror |
|---|---|
| `go/` | `hop-top/agntcy` |
| `rs/` | `hop-top/agntcy-rs` |
| `php/` | `hop-top/agntcy-php` |
| `ts/` | `hop-top/agntcy-ts` |
| `py/` | `hop-top/agntcy-py` |

All five mirrors exist as empty public repos. `publish-on-tag.yml@v0` populates them on first tag push.

## Sanity checks before first release

- [ ] `gh secret list --org hop-top` returns the 6 publish secrets above.
- [ ] `.github/release-please-config.json` and `.github/.release-please-manifest.json` exist and parse (`jq empty`).
- [ ] All 14 components in the config have a corresponding row in `.github/workflows/publish.yml` ecosystems block.
- [ ] PyPI trusted publishers registered for the 3 Py packages, or `PYPI_REGISTRY_TOKEN` is current.
- [ ] Packagist projects registered + webhooks active for the 4 PHP packages.

## First release

Trigger by merging `main` with at least one `feat:` commit per component you want to release. `release-please` will open standing PRs at `0.1.0-alpha.0` (per `bump-minor-pre-major: true`).
