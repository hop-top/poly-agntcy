# Cross-language conformance

Home for the cross-SDK conformance suite per ADR-0007 phase 7. Current
state is a smoke harness, not a complete conformance suite; see
"Current limitations" below.

## What this dir is for

The role-based layout (ADR-0007) reserves `e2e/conformance/` for one
suite that exercises every SDK against the same DIR protocol fixture.
The intent: ship-blocking proof that the Go, Rust, PHP, TS, and Python
SDKs interoperate at the wire level.

## What this dir actually contains today

- `docker-compose.yml` — runs `ghcr.io/agntcy/dir-apiserver:latest` on
  `:8888` for gRPC/HTTP.
- `run.sh` — waits for `/healthz`, then runs three agents serially.
- `go-agent.go` — uses `hop.top/agntcy/dir` to register
  `inventory-agent`. **Real wire calls.**
- `py-agent.py` — argparse-only stub. `_discover` returns a hardcoded
  `["inventory-agent"]`; no DIR API call. **Not a real test.**
- `php-agent.php` — prints a fake signature. No DIR API call.
  **Not a real test.**

## Current limitations

1. Only Go actually talks to DIR. Python and PHP "agents" are stubs.
2. The CI step that runs this suite (`mise run conformance` in
   `.github/workflows/ci-conformance.yml`) carries
   `continue-on-error: true` — failures don't block merge. So the
   "ship-blocking gate" property the ADR envisions doesn't hold yet.
3. Rust and TypeScript SDKs aren't exercised at all.
4. No fixture protocol — each agent invents its own scenario.

## What's needed to make this a real conformance suite

- Replace the Python and PHP stubs with calls through the real
  framework adapters (`hop-top-agntcy-dir-fastapi`,
  `hop-top/agntcy-dir-laravel`, etc.).
- Add Rust and TypeScript agents.
- Define a single fixture sequence (register → discover → describe →
  publish → verify) that every agent runs.
- Assert observable state from the DIR API (not just "process exited
  zero").
- Drop `continue-on-error` so failures actually fail CI.

Each of these is a follow-up task; this dir rename is just the
structural prerequisite.

## Run

```sh
docker compose up -d
./run.sh
docker compose down
```

## Image

`ghcr.io/agntcy/dir-apiserver:latest` — the AGNTCY DIR API server,
listening on `0.0.0.0:8888` for gRPC/HTTP. See
[`agntcy/dir`](https://github.com/agntcy/dir) for source and
release tags.
