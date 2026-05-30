# go / register-and-discover

Minimal example: open a DIR `Client`, register an agent descriptor,
discover it back by capability, then publish a signed payload.

## Run

```sh
go run ./examples/go/register-and-discover \
  --endpoint http://localhost:8888 \
  --capability inventory
```
