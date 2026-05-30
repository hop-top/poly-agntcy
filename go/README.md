# poly-agntcy

Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)

## Install

```sh
go install hop.top/agntcy/poly-agntcy@latest
```

Or download a binary from
[Releases](https://hop.top/agntcy/poly-agntcy/releases).

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

## Configuration

Config file: `~/.config/poly-agntcy/config.yaml`

Environment variables prefixed with `POLY_AGNTCY_` are
also recognized.

## Getting started

Toolchain is pinned in `mise.toml`. Install everything in one go:

```sh
mise install                # installs Go (and other pinned tools)
mise run install            # downloads ecosystem deps (go mod download)
cp .env.example .env        # adapter defaults (telemetry, storage, queue, log, config)
```

Optional — start telemetry services (otel-collector + jaeger):

```sh
docker compose -f .devcontainer/docker-compose.yml up -d
open http://localhost:16686 # Jaeger UI for traces
```

## Tool versions

Go toolchain, linters, and release tooling are pinned in
`mise.toml` at the project root, inside a `# >>> kit-managed >>>`
block tracked against poly-kit's central manifest. User-added tools
above the marker are preserved verbatim.

Refresh procedure (`kit init --check` / `kit init --update`) is
documented in poly-kit's
[templates/RUNBOOK-UPGRADE.md](https://github.com/hop-top/poly-kit/blob/main/templates/RUNBOOK-UPGRADE.md).

## Development

Prerequisites: `mise` (installs the pinned Go toolchain), GNU Make.

```sh
make setup    # download deps
make check    # lint + test + links
make build    # build binary
```

### Put `poly-agntcy` on your PATH

```sh
make symlink
```

`make symlink` builds the binary, walks `$PATH`, picks the first
writable user-bin dir (in priority order: `$XDG_BIN_HOME`,
`~/.local/bin`, `~/bin` on Unix; `%USERPROFILE%\bin`,
`%USERPROFILE%\.local\bin`, `%LOCALAPPDATA%\Programs` on Windows),
and links `bin/poly-agntcy` into it. On native Windows it writes a
`.cmd` shim instead of a symlink to avoid the Admin / Developer
Mode requirement.

Subsequent `make build` runs auto-deploy because the link points
at the live `./bin/poly-agntcy`.

```sh
make symlink                       # idempotent: no-op when already linked
make symlink SYMLINK_DIR=/opt/bin  # override candidate-dir search
make symlink FORCE=1               # replace a link pointing elsewhere
```

If no candidate dir is on `$PATH`, the command prints a hint
asking you to add `$HOME/.local/bin` to `$PATH` or pass
`SYMLINK_DIR=`.

## License

See [LICENSE](LICENSE).

---
Maintained by Jad Bitar.
