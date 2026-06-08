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

## Quickstart

One example per shipped package lives under [`examples/`](examples/) — register an identity, publish a descriptor, discover a peer. A cross-language conformance smoke harness sits at [`e2e/conformance/`](e2e/conformance/) (see its README for current limitations).

```sh
make setup        # install all toolchains via mise
make test         # run every language's test suite
make lint         # run every language's linter
make conformance  # bring up local DIR + run cross-lang smoke
```

## Docs

Start at the [docs index](docs/) — organized by what you're trying to do (install a package, run your first call, understand the design, operate a release).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Security reports: [SECURITY.md](SECURITY.md).

## License

Apache-2.0.
