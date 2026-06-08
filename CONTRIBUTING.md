# Contributing to poly-agntcy

Thanks for your interest in contributing.

## Getting started

1. Fork the repository
2. Clone your fork locally
3. Create a feature branch: `git checkout -b feat/my-change`
4. Make your changes
5. Run tests: `make test`
6. Commit using [Conventional Commits](https://conventionalcommits.org)
7. Push and open a Pull Request

## Development setup

```sh
make setup
```

Equivalent to `mise run install_all` — installs all language toolchains and dependencies (Go, Rust, PHP, Node, Python, buf). Requires [mise](https://mise.jdx.dev) to be installed.

## Repo shape

Five language roots under one repo:

- `go/` — first-party Go SDK (`hop.top/agntcy`) + `agntcy` CLI
- `rs/` — first-party Rust workspace (`poly-agntcy-dir`, `poly-agntcy-dir-spiffe`)
- `php/` — first-party PHP packages (`poly-agntcy/dir{,-laravel,-symfony,-spiffe}`)
- `ts/` — TypeScript adapters (`@poly-agntcy/dir-{next,hono,express}`) wrapping the official `agntcy-dir` npm package
- `py/` — Python adapters (`poly-agntcy-dir-{fastapi,flask,django}`) wrapping the official `agntcy-dir` PyPI package

Per-language work happens inside its dir. Cross-cutting changes (protobuf, workflows, ADRs) live at the repo root.

## Code style

- Follow existing conventions in each language root.
- Run linters before submitting: `make lint`.
- Keep changes focused; one concern per PR.
- Read the [design spec](docs/superpowers/specs/2026-05-30-poly-agntcy-design.md) for architecture decisions.

## Commit messages

Use [Conventional Commits](https://conventionalcommits.org):

```
feat(scope): add new feature
fix(scope): correct a bug
docs: update readme
test: add missing tests
```

The `scope` matches a [release-please component](.github/release-please-config.json): `go`, `go-spiffe`, `rs`, `rs-spiffe`, `php`, `php-laravel`, `php-symfony`, `php-spiffe`, `ts-next`, `ts-hono`, `ts-express`, `py-fastapi`, `py-flask`, `py-django`. Scopeless commits trigger no release.

Breaking changes use the `!` suffix or a `BREAKING CHANGE:` footer per the spec.

## Pull requests

- Reference related issues in the PR description
- Keep PRs small and reviewable
- Ensure CI passes before requesting review
- Update documentation if behavior changes

## Issues

- Search existing issues before opening a new one
- Use issue templates when available
- Provide reproduction steps for bugs

## Code of conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
