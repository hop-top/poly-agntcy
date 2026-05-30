# poly-agntcy

Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)

## Install

```sh
composer global require agntcy/poly-agntcy
```

## Usage

```sh
poly-agntcy --help
poly-agntcy --version
```

### Output formats

Every command supports the kit output flag suite:

```sh
poly-agntcy list --format json
poly-agntcy list --format yaml
poly-agntcy list --cols name,status
poly-agntcy list -o results.json   # ext infers format
poly-agntcy list --format-help     # show all formats + options
```

### Commands

```sh
poly-agntcy hello                  # Hello, World!
poly-agntcy hello Alice            # Hello, Alice!
poly-agntcy list                   # exercise the output flag suite
```

## Getting started

Toolchain is pinned in `mise.toml`. Install everything in one go:

```sh
mise install                # installs PHP/Composer (and other pinned tools)
mise run install            # installs ecosystem deps (composer install)
cp .env.example .env        # adapter defaults (telemetry, storage, queue, log, config)
```

Optional — start telemetry services (otel-collector + jaeger):

```sh
docker compose -f .devcontainer/docker-compose.yml up -d
open http://localhost:16686 # Jaeger UI for traces
```

## Tool versions

PHP, Composer, and release tooling are pinned in `mise.toml`
inside a kit-managed block. Refresh procedure
(`kit init --check` / `kit init --update`) is documented in
poly-kit's
[templates/RUNBOOK-UPGRADE.md](https://github.com/hop-top/poly-kit/blob/main/templates/RUNBOOK-UPGRADE.md).

## Development

Prerequisites: `mise` (installs the pinned PHP + Composer toolchain).

```sh
make setup       # install deps
make check       # lint + test + link check
make test        # phpunit
make lint        # phpstan
```

## License

See [LICENSE](LICENSE).

---
Maintained by Jad Bitar.
