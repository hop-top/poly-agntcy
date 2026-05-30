# poly-agntcy

Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR).

| Language | Status | Install |
|---|---|---|
| Go | First-party | `go get hop.top/agntcy` |
| Rust | First-party | `cargo add poly-agntcy-dir` |
| PHP | First-party | `composer require poly-agntcy/dir` |
| TypeScript | Adapters | `npm i @poly-agntcy/dir-next` (wraps official `agntcy-dir`) |
| Python | Adapters | `pip install poly-agntcy-dir-fastapi` (wraps official `agntcy-dir`) |

For TS and Python core DIR access, use AGNTCY's official SDKs:
- TS: [`agntcy/dir-sdk-javascript`](https://github.com/agntcy/dir-sdk-javascript)
- Py: [`agntcy/dir-sdk-python`](https://github.com/agntcy/dir-sdk-python)

## Design

See [`docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`](docs/superpowers/specs/2026-05-30-poly-agntcy-design.md).

## License

Apache-2.0.
