<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Spiffe;

/**
 * v0 skeleton of the SPIFFE Workload API client over a Unix domain
 * socket.
 *
 * The Workload API is a gRPC service published by the local SPIRE
 * agent on a UDS endpoint identified by the SPIFFE_ENDPOINT_SOCKET
 * env var (default /tmp/spire-agent/public/api.sock). This skeleton
 * captures the surface needed by SpiffeCredentials: fetching an
 * X509-SVID and the corresponding trust bundle.
 *
 * Full gRPC wire transport over UDS lands in a follow-up commit;
 * this v0 returns stub data so the type surface is locked.
 */
final class WorkloadApiClient
{
    public const DEFAULT_SOCKET = '/tmp/spire-agent/public/api.sock';

    public function __construct(
        private readonly string $socketPath = self::DEFAULT_SOCKET,
        private readonly int $timeoutSeconds = 5,
    ) {
    }

    public function socketPath(): string
    {
        return $this->socketPath;
    }

    public function timeoutSeconds(): int
    {
        return $this->timeoutSeconds;
    }

    /**
     * Fetches the workload's X509-SVID.
     *
     * v0: throws because no SPIRE agent is wired. The signature is the
     * contract the full implementation will honour.
     */
    public function fetchX509Svid(): X509Svid
    {
        throw new \RuntimeException('SPIFFE Workload API not implemented in v0 stub');
    }

    /**
     * Fetches X509 trust bundles for federated trust domains.
     *
     * @return list<X509Bundle>
     */
    public function fetchX509Bundles(): array
    {
        throw new \RuntimeException('SPIFFE Workload API not implemented in v0 stub');
    }

    /**
     * Long-lived stream watching for SVID rotation.
     *
     * @param callable(X509Svid): void $onUpdate
     */
    public function watchX509Svid(callable $onUpdate): never
    {
        unset($onUpdate);
        throw new \RuntimeException('SPIFFE Workload API not implemented in v0 stub');
    }
}
