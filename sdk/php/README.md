# poly-agntcy PHP SDK

Composer workspace hosting the PHP SDK packages for the AGNTCY Agent
Directory Service (DIR). Path repositories link the packages so they
resolve from source during development. Framework adapters live under
`adapters/` once promoted from `php/packages/`.

## Packages

- `packages/dir` — core SDK + CLI (`PolyAgntcy\Dir\`). Credentials
  (Insecure / TLS / SPIFFE handle), `Client`, and the `agntcy` CLI.
- `packages/dir-spiffe` — SPIFFE-backed `SpiffeCredentials`
  (Workload API client, X.509 SVID + trust bundle).

## Install

```sh
composer install --no-interaction
```

Symlinks the SDK packages into `vendor/poly-agntcy/`.

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
