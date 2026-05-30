# poly-agntcy PHP workspace

Composer workspace hosting the PHP SDK suite for the AGNTCY Agent
Directory Service (DIR). Path repositories link the four packages so
all of them resolve from source during development.

## Packages

- `packages/dir` — core SDK + CLI (`PolyAgntcy\Dir\`). Credentials
  (Insecure / TLS / SPIFFE handle), `Client`, and the `agntcy` CLI.
- `packages/dir-laravel` — Laravel `ServiceProvider` + `Agntcy` facade
  for `poly-agntcy/dir`.
- `packages/dir-symfony` — Symfony bundle exposing `Client` via DI.
- `packages/dir-spiffe` — SPIFFE-backed `SpiffeCredentials`
  (Workload API client, X.509 SVID + trust bundle).

## Install

```sh
composer install --no-interaction
```

Symlinks the four packages into `vendor/poly-agntcy/`.

## Test

```sh
vendor/bin/phpunit --testdox
```

## Static analysis

```sh
vendor/bin/phpstan analyse --memory-limit=1G
```

Configuration lives in `phpstan.neon` (level `max`, excludes
`packages/dir/src/Generated`).

## License

Apache-2.0.
