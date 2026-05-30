# Cross-language integration test

Stands up a local DIR instance and runs Go, Python, and PHP agents
against it:

1. Go agent registers `inventory-agent`
2. Python agent discovers it by capability
3. PHP agent sends a signed message to it

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
