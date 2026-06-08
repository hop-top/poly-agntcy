# Python SDK

Python does not ship a first-party SDK in this repo. See [ADR-0001](../../docs/adr/0001-scope-dir-only-phase-one.md) and [ADR-0007](../../docs/adr/0007-role-based-layout-sdk-adapter-split.md).

Framework integrations live under `adapters/py/` (FastAPI, Flask, Django) once
the adapter migration completes.

## Current state

This directory is a placeholder. The Python `pyproject.toml` uv workspace and
its `packages/*` adapter members still live under `py/` at the repo root —
moving the workspace root here before its members migrate would break the
`[tool.uv.workspace]` member globs.

## Next step

The workspace root (`py/pyproject.toml`, `py/.gitignore`) moves into this
directory in the adapter-extract phase, alongside the relocation of
`py/packages/dir-fastapi`, `py/packages/dir-flask`, and `py/packages/dir-django`
to `adapters/py/`.
