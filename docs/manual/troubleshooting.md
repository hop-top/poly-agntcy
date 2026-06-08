# Troubleshoot poly-agntcy

Common symptoms across all five language packages.

## Use this when

- A package install or build fails.
- A DIR call returns an unexpected error.
- CI is red for a reason that isn't your code.

## Connection issues

| Symptom | Likely cause | Fix |
|---|---|---|
| `connection refused` to `localhost:8888` | DIR service not running | `cd examples/cross && docker compose up -d` |
| `x509: certificate signed by unknown authority` | `TlsCredentials` against a dev cert | Use `InsecureCredentials` in dev or supply a custom CA pool |
| `transport: authentication handshake failed` | TLS endpoint but credentials are `InsecureCredentials` | Switch to `TlsCredentials` or `SpiffeCredentials` |
| All requests succeed with HTTP 501 | Adapter stub body | TypeScript adapters return 501 until upstream wire integration lands; see [release notes](../../CHANGELOG.md) |

## Build and install issues

### Go: `cannot find package "hop.top/agntcy"`

The vanity import requires either:

1. A configured `hop.top` redirect (see [vanity-import](../vanity-import.md)), or
2. A `replace` directive pointing at the GitHub source until the redirect is live.

For local development, add to your `go.mod`:

```
replace hop.top/agntcy => github.com/hop-top/poly-agntcy/go vX.Y.Z
```

### Rust: `feature 'cli' is required`

You tried to build the `agntcy` binary without the `cli` feature.

```sh
cargo build --bin agntcy --features cli
```

The feature gates `clap` and `anyhow` so they don't leak into library consumers.

### PHP: `class HopTop\Agntcy\Dir\Client not found` after composer install

The PSR-4 autoloader isn't regenerated. Run:

```sh
composer dump-autoload
```

If that doesn't fix it, the `Stub.php` autoload (in `php/packages/dir/composer.json` `autoload.files`) may have been removed. Restore it from `git`.

### TypeScript: `Cannot find module 'agntcy-dir'`

The upstream `agntcy-dir` npm package pulls a transitive dep from the Buf BSR npm registry. Either configure BSR auth or install with peer resolution disabled:

```sh
pnpm install --config.auto-install-peers=false --ignore-scripts
```

The adapter packages declare `agntcy-dir` as a peer, not a direct dep, so they install regardless.

### Python: `ModuleNotFoundError: No module named 'agntcy_dir'`

The upstream `agntcy-dir` package on PyPI is pre-1.0. Pin to the installable floor in your `pyproject.toml`:

```toml
dependencies = ["agntcy-dir>=0.3,<2"]
```

## CI issues

### `mise install` fails on PHP

The `asdf:php@8.2` plugin can't build PHP from source on the runner (missing `re2c` or similar prereq). The CI workflow for PHP (`ci-php.yml`) uses `shivammathur/setup-php@v2` instead. If you see `mise install` failing on PHP, you're on a workflow that hasn't been updated.

### release-please fails: `value at path package.version is not tagged`

A package source file declares a `version` that doesn't match the manifest in `.github/.release-please-manifest.json`. For Rust crates specifically, `version.workspace = true` doesn't work with release-please — each crate must declare its own literal `version`.

## Getting more diagnostics

For Go/Rust/Python/PHP, set log level via environment:

```sh
RUST_LOG=debug cargo run        # rust
AGNTCY_LOG=debug ./bin/agntcy   # go
DEBUG=1 php example.php         # php
PYTHONUNBUFFERED=1 LOG_LEVEL=DEBUG uv run …  # python
```

## Still stuck

1. Read the [architecture](../ARCHITECTURE.md) to confirm you have the right package.
2. Check the [release runbook](../release-runbook.md) for known publish/CI issues.
3. Search [open issues](https://github.com/hop-top/poly-agntcy/issues) on GitHub.
4. Open a new issue with: package + version, OS, exact error, steps to reproduce.
