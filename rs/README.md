# poly-agntcy

Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)

## Install

```sh
cargo install --git https://github.com/agntcy/poly-agntcy
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
mise install                # installs Rust (and other pinned tools)
mise run install            # fetches crates (cargo fetch)
cp .env.example .env        # adapter defaults (telemetry, storage, queue, log, config)
```

Optional — start telemetry services (otel-collector + jaeger):

```sh
docker compose -f .devcontainer/docker-compose.yml up -d
open http://localhost:16686 # Jaeger UI for traces
```

## Tool versions

Rust, linters, and release tooling are pinned in `mise.toml`
inside a kit-managed block. Refresh procedure
(`kit init --check` / `kit init --update`) is documented in
poly-kit's
[templates/RUNBOOK-UPGRADE.md](https://github.com/hop-top/poly-kit/blob/main/templates/RUNBOOK-UPGRADE.md).

## Development

Prerequisites: `mise` (installs the pinned Rust + cargo toolchain).

```sh
make setup       # cargo fetch
make check       # fmt-check + clippy + test
make test        # cargo test
make lint        # cargo fmt --check + clippy
```

## License

See [LICENSE](LICENSE).

---
Maintained by Jad Bitar.
