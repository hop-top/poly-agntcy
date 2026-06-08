# Install a poly-agntcy package

Pick your language and install the matching package. There is no single binary — each language ships its own SDK and (where applicable) CLI.

## Use this when

- You're starting a new project that talks to an AGNTCY DIR service.
- You want to add DIR support to an existing app written in Go, Rust, PHP, TypeScript, or Python.

## Before you begin

You need:

- A package manager for your language (`go`, `cargo`, `composer`, `npm`/`pnpm`, or `pip`/`uv`).
- A DIR endpoint to talk to (local `docker compose up` in [`examples/cross/`](../../examples/cross/) gets you one).

## Quick install

| Language | Command |
|---|---|
| Go | `go get hop.top/agntcy` |
| Rust | `cargo add poly-agntcy-dir` |
| PHP (core) | `composer require poly-agntcy/dir` |
| PHP (Laravel) | `composer require poly-agntcy/dir-laravel` |
| PHP (Symfony) | `composer require poly-agntcy/dir-symfony` |
| TypeScript (Next.js) | `pnpm add @poly-agntcy/dir-next` |
| TypeScript (Hono) | `pnpm add @poly-agntcy/dir-hono` |
| TypeScript (Express) | `pnpm add @poly-agntcy/dir-express` |
| Python (FastAPI) | `uv add poly-agntcy-dir-fastapi` |
| Python (Flask) | `uv add poly-agntcy-dir-flask` |
| Python (Django) | `uv add poly-agntcy-dir-django` |

The CLI ships inside the relevant package per language:

| Language | CLI binary | Lives in |
|---|---|---|
| Go | `agntcy` | `hop.top/agntcy` (`go install hop.top/agntcy/cmd/agntcy@latest`) |
| Rust | `agntcy` | `poly-agntcy-dir` (`cargo install poly-agntcy-dir --features cli`) |
| PHP | `agntcy` | `poly-agntcy/dir` (Composer `bin/`) |
| TypeScript | `agntcy` | `@poly-agntcy/dir-next` (`npx @poly-agntcy/dir-next`) |
| Python | `agntcy` | `poly-agntcy-dir-fastapi` (`uv tool install poly-agntcy-dir-fastapi`) |

## Add SPIFFE support (optional)

If you want SPIFFE-backed credentials instead of plain TLS, install the extension package:

| Language | Command |
|---|---|
| Go | `go get hop.top/agntcy/spiffe` |
| Rust | `cargo add poly-agntcy-dir-spiffe` |
| PHP | `composer require poly-agntcy/dir-spiffe` |

TS and Python adapters get SPIFFE through the official upstream SDK; no separate install needed.

## Verify

For Go, Rust, PHP, or Python CLIs:

```sh
agntcy --help
```

You should see the five DIR subcommands: `register`, `discover`, `describe`, `publish`, `verify`.

For a library install, write a one-line program that imports the package — `go build`, `cargo check`, `composer install`, `pnpm tsc --noEmit`, or `python -c "import poly_agntcy_dir_fastapi"` all succeeding is enough.

## Next steps

- Run your first call: [getting started](getting-started.md)
- Browse working examples: [`examples/`](../../examples/)
- Hit a problem: [troubleshooting](troubleshooting.md)
