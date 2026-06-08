# Install a poly-agntcy package

Pick your language and install the matching package. There is no single binary — each language ships its own SDK and (where applicable) CLI.

## Use this when

- You're starting a new project that talks to an AGNTCY DIR service.
- You want to add DIR support to an existing app written in Go, Rust, PHP, TypeScript, or Python.

## Before you begin

You need:

- A package manager for your language (`go`, `cargo`, `composer`, `npm`/`pnpm`, or `pip`/`uv`).
- A DIR endpoint to talk to (local `docker compose up` in [`e2e/conformance/`](../../e2e/conformance/) gets you one).

## Quick install

| Language | Command |
|---|---|
| Go | `go get hop.top/agntcy` |
| Rust | `cargo add hop-top-agntcy-dir` |
| PHP (core) | `composer require hop-top/agntcy-dir` |
| PHP (Laravel) | `composer require hop-top/agntcy-dir-laravel` |
| PHP (Symfony) | `composer require hop-top/agntcy-dir-symfony` |
| TypeScript (Next.js) | `pnpm add @hop-top/agntcy-dir-next` |
| TypeScript (Hono) | `pnpm add @hop-top/agntcy-dir-hono` |
| TypeScript (Express) | `pnpm add @hop-top/agntcy-dir-express` |
| Python (FastAPI) | `uv add hop-top-agntcy-dir-fastapi` |
| Python (Flask) | `uv add hop-top-agntcy-dir-flask` |
| Python (Django) | `uv add hop-top-agntcy-dir-django` |

The CLI ships inside the relevant package per language:

| Language | CLI binary | Lives in |
|---|---|---|
| Go | `agntcy` | `hop.top/agntcy` (`go install hop.top/agntcy/cmd/agntcy@latest`) |
| Rust | `agntcy` | `hop-top-agntcy-dir` (`cargo install hop-top-agntcy-dir --features cli`) |
| PHP | `agntcy` | `hop-top/agntcy-dir` (Composer `bin/`) |
| TypeScript | `agntcy` | `@hop-top/agntcy-dir-next` (`npx @hop-top/agntcy-dir-next`) |
| Python | `agntcy` | `hop-top-agntcy-dir-fastapi` (`uv tool install hop-top-agntcy-dir-fastapi`) |

## Add SPIFFE support (optional)

If you want SPIFFE-backed credentials instead of plain TLS, install the extension package:

| Language | Command |
|---|---|
| Go | `go get hop.top/agntcy-go-spiffe` |
| Rust | `cargo add hop-top-agntcy-dir-spiffe` |
| PHP | `composer require hop-top/agntcy-dir-spiffe` |

TS and Python adapters get SPIFFE through the official upstream SDK; no separate install needed.

## Verify

For Go, Rust, PHP, or Python CLIs:

```sh
agntcy --help
```

You should see the five DIR subcommands: `register`, `discover`, `describe`, `publish`, `verify`.

For a library install, write a one-line program that imports the package — `go build`, `cargo check`, `composer install`, `pnpm tsc --noEmit`, or `python -c "import agntcy_dir_fastapi"` all succeeding is enough.

## Next steps

- Run your first call: [getting started](getting-started.md)
- Browse working examples: [`examples/`](../../examples/)
- Hit a problem: [troubleshooting](troubleshooting.md)
