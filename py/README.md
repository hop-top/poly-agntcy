# poly-agntcy Python workspace

uv workspace hosting three framework adapters for the AGNTCY Directory Service.

## Packages

- `packages/dir-fastapi` — FastAPI `APIRouter` factory + `agntcy` Typer CLI.
- `packages/dir-flask` — Flask `Blueprint` factory.
- `packages/dir-django` — Django app providing URLs and views.

All three depend on upstream `agntcy-dir` (pre-1.0; pinned `>=0.3,<2`).

## Requirements

- Python 3.12+
- [`uv`](https://docs.astral.sh/uv/) for env + dependency management.

## Common commands

```sh
uv sync                       # install workspace + dev deps
uv run pytest                 # run all package tests
uv run ruff check packages    # lint
uv run ruff format packages   # format
uv run mypy packages          # typecheck (strict)
```

Run a single package's tests:

```sh
uv run pytest packages/dir-fastapi
```

## Layout

```
py/
  pyproject.toml          # workspace root + mypy/ruff config
  packages/
    dir-fastapi/
    dir-flask/
    dir-django/
```
