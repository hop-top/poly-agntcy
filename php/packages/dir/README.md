# poly-agntcy/dir

PHP SDK for the AGNTCY Agent Directory Service (DIR).

## Install

```sh
composer require poly-agntcy/dir
```

## Wire protocol

DIR calls go over gRPC via the `grpc/grpc` PHP extension. Generated
stubs land in `src/Generated/` from `mise run gen` (buf codegen via
`buf.build/protocolbuffers/php` + `buf.build/grpc/php`).

A placeholder shim (`src/Generated/Stub.php`) provides interface
declarations matching the would-be generated `DirectoryServiceClient`
so static analysis passes without codegen. Real codegen output
overrides the shim at runtime (and the shim's `class_exists` guards
prevent collision).

## Credentials

- `InsecureCredentials` — plaintext.
- `TlsCredentials` — TLS via PHP stream-context `ssl` options.
- `SpiffeCredentials` — ships in `poly-agntcy/dir-spiffe`.

## Quickstart

```php
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;

$dir = new Client('directory.example:443', new InsecureCredentials());
foreach ($dir->discover('chat') as $agent) {
    echo $agent->name."\n";
}
```

## CLI

```sh
vendor/bin/agntcy discover --endpoint directory.example:443 --capability chat
```

## Integration tests

Wire-level tests live under `tests/Integration/` and are gated by the
`DIR_ENDPOINT` env var; they're skipped by default.
