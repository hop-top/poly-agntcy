# poly-agntcy

Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)

## Install

```sh
pip install poly-agntcy
```

Or with [uv](https://docs.astral.sh/uv/):

```sh
uv tool install poly-agntcy
```

## Usage

```sh
poly-agntcy --help
poly-agntcy --version
```

### Output formats

```sh
poly-agntcy --format json
poly-agntcy --format yaml
```

### Commands

```sh
poly-agntcy hello
poly-agntcy hello --name You
```

## Getting started

Toolchain is pinned in `mise.toml`. Install everything in one go:

```sh
mise install                # installs Python + uv (and other pinned tools)
mise run install            # syncs ecosystem deps (uv sync)
cp .env.example .env        # adapter defaults (telemetry, storage, queue, log, config)
```

Optional — start telemetry services (otel-collector + jaeger):

```sh
docker compose -f .devcontainer/docker-compose.yml up -d
open http://localhost:16686 # Jaeger UI for traces
```

## Tool versions

Python, uv, ruff, and release tooling are pinned in `mise.toml`
inside a kit-managed block. Refresh procedure
(`kit init --check` / `kit init --update`) is documented in
poly-kit's
[templates/RUNBOOK-UPGRADE.md](https://github.com/hop-top/poly-kit/blob/main/templates/RUNBOOK-UPGRADE.md).

## Development

Prerequisites: `mise` (installs the pinned Python toolchain + uv),
[Task](https://taskfile.dev).

```sh
task setup    # sync deps
task check    # lint + typecheck + test
task format   # auto-format
```

## License

See [LICENSE](LICENSE).

---
Maintained by Jad Bitar.
