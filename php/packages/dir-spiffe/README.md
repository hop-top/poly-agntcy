# poly-agntcy/dir-spiffe

SPIFFE-backed credentials for `poly-agntcy/dir`.

## Status

v0 stub. The public surface (`SpiffeId`, `X509Svid`, `X509Bundle`,
`WorkloadApiClient`, `SpiffeCredentials`) is in place. Wire-level
transport over the SPIRE agent UDS is not yet implemented —
`WorkloadApiClient` methods throw `RuntimeException` and
`SpiffeCredentials::tlsOptions()` returns null, so callers fall back
to plaintext during bootstrap.

Full implementation tracked in a separate post-bootstrap task.

## Install

```sh
composer require poly-agntcy/dir-spiffe
```

## Usage (post-impl)

```php
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\Spiffe\SpiffeCredentials;

$dir = new Client('directory.example:443', new SpiffeCredentials());
```
