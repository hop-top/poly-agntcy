<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Spiffe;

use PolyAgntcy\Dir\Credentials;

/**
 * Credentials backed by a SPIRE agent on a local Unix domain socket.
 *
 * v0 stub: returns null tlsOptions so callers fall back to plaintext
 * during bootstrap. Full impl fetches an X509-SVID via
 * WorkloadApiClient and materialises stream-context ssl options
 * (local_cert, local_pk, cafile) on each call.
 */
final class SpiffeCredentials implements Credentials
{
    public function __construct(
        private readonly string $socketPath = WorkloadApiClient::DEFAULT_SOCKET,
        private readonly ?WorkloadApiClient $client = null,
    ) {
    }

    public function socketPath(): string
    {
        return $this->socketPath;
    }

    public function tlsOptions(): ?array
    {
        return null;
    }

    private function client(): WorkloadApiClient
    {
        return $this->client ?? new WorkloadApiClient($this->socketPath);
    }
}
