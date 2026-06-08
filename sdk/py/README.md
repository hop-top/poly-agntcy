# hop-top/agntcy Python workspace

uv workspace hosting three framework adapters for the AGNTCY Directory
Service. Per ADR-0001, Python ships adapter packages on top of the
upstream `agntcy-dir` SDK rather than a first-party Python SDK.

## Adapters

- `../../adapters/fastapi` — `agntcy-dir-fastapi`: FastAPI `APIRouter`
  factory + `agntcy` Typer CLI.
- `../../adapters/flask` — `agntcy-dir-flask`: Flask `Blueprint`
  factory.
- `../../adapters/django` — `agntcy-dir-django`: Django app providing
  URLs and views.

All three depend on upstream `agntcy-dir` (pre-1.0; pinned `>=0.3,<2`).
Published distribution names use the vendor prefix
(`hop-top-agntcy-dir-fastapi` etc., per ADR-0008); importable Python
modules drop the vendor prefix for cleaner imports
(`from agntcy_dir_fastapi import …`).

## Requirements

- Python 3.12+
- [`uv`](https://docs.astral.sh/uv/) for env + dependency management.

## Common commands

Run from `sdk/py/`:

```sh
uv sync --all-packages                          # install workspace + adapter runtime deps
uv run pytest --import-mode=importlib --rootdir=. ../../adapters
uv run ruff check ../../adapters                # lint
uv run ruff format ../../adapters               # format
uv run mypy ../../adapters/fastapi/src \
          ../../adapters/flask/src \
          ../../adapters/django/src             # typecheck (strict)
```

Run a single adapter's tests:

```sh
uv run pytest --import-mode=importlib --rootdir=. ../../adapters/fastapi
```

## Layout

```
sdk/py/
  pyproject.toml               # workspace root + mypy/ruff/pytest config
  README.md                    # this file
adapters/
  fastapi/                     # hop-top-agntcy-dir-fastapi
  flask/                       # hop-top-agntcy-dir-flask
  django/                      # hop-top-agntcy-dir-django
```

Workspace members declared in `pyproject.toml` `[tool.uv.workspace]`
point at `../../adapters/<framework>/` via relative paths.
