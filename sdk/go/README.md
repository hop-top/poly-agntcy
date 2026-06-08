# agntcy (Go)

Go SDK + CLI for the AGNTCY Agent Directory Service (DIR).

## Install

SDK:

```sh
go get hop.top/agntcy
```

CLI:

```sh
go install hop.top/agntcy/cmd/agntcy@latest
```

## SDK usage

```go
import "hop.top/agntcy/dir"

client, err := dir.NewClient(dir.Options{
    Endpoint:    "https://dir.example.org",
    Credentials: dir.InsecureCredentials{}, // dev/test only
})
if err != nil {
    return err
}

res, err := client.Register(ctx, dir.RegisterParams{
    Agent: &dir.AgentDescriptor{Name: "demo", Endpoint: "https://demo.example.org"},
})
```

Available methods: `Register`, `Discover`, `Describe`, `Publish`, `Verify`.

## Credentials

- `dir.InsecureCredentials{}` — no TLS, dev only.
- `dir.TlsCredentials{Config: tlsCfg}` — stdlib `*tls.Config`.
- `hop.top/agntcy/spiffe` submodule — SPIFFE-backed mTLS. See
  [`spiffe/`](./spiffe).

## CLI

```sh
agntcy register --endpoint https://dir.example.org --name demo --agent-endpoint https://demo.example.org
agntcy discover --endpoint https://dir.example.org --capability translate
agntcy describe --endpoint https://dir.example.org --id <id>
agntcy publish  --endpoint https://dir.example.org --id <id> --payload ./payload.bin
agntcy verify   --endpoint https://dir.example.org --id <id> --signature ./sig.bin
```

## Development

```sh
make test   # go test ./...
make vet    # go vet ./...
make build  # builds bin/agntcy
```
