<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir;

use PolyAgntcy\Dir\Generated\Agent;
use PolyAgntcy\Dir\Generated\DescribeRequest;
use PolyAgntcy\Dir\Generated\DirectoryServiceClient;
use PolyAgntcy\Dir\Generated\DiscoverRequest;
use PolyAgntcy\Dir\Generated\PublishRequest;
use PolyAgntcy\Dir\Generated\RegisterRequest;
use PolyAgntcy\Dir\Generated\VerifyRequest;
use Psr\Http\Client\ClientInterface;

final class Client
{
    private ?DirectoryServiceClient $stub = null;

    public function __construct(
        private readonly string $endpoint,
        private readonly Credentials $credentials,
        private readonly ?ClientInterface $http = null,
    ) {
        if ($endpoint === '') {
            throw new \InvalidArgumentException('endpoint required');
        }
    }

    public function endpoint(): string
    {
        return $this->endpoint;
    }

    public function credentials(): Credentials
    {
        return $this->credentials;
    }

    public function httpClient(): ?ClientInterface
    {
        return $this->http;
    }

    public function register(Agent $agent): string
    {
        $req = (new RegisterRequest())->setAgent($agent);
        [$res, $_] = $this->stub()->Register($req);
        return $res->id;
    }

    /**
     * @return list<Agent>
     */
    public function discover(string $capability): array
    {
        $req = (new DiscoverRequest())->setCapability($capability);
        [$res, $_] = $this->stub()->Discover($req);
        return $res->agents;
    }

    public function describe(string $id): ?Agent
    {
        $req = (new DescribeRequest())->setId($id);
        [$res, $_] = $this->stub()->Describe($req);
        return $res->agent;
    }

    public function publish(Agent $agent): string
    {
        $req = (new PublishRequest())->setAgent($agent);
        [$res, $_] = $this->stub()->Publish($req);
        return $res->version;
    }

    public function verify(string $id): bool
    {
        $req = (new VerifyRequest())->setId($id);
        [$res, $_] = $this->stub()->Verify($req);
        return $res->valid;
    }

    private function stub(): DirectoryServiceClient
    {
        if ($this->stub === null) {
            $this->stub = new DirectoryServiceClient($this->endpoint, $this->stubOptions());
        }
        return $this->stub;
    }

    /**
     * @return array<string,mixed>
     */
    private function stubOptions(): array
    {
        $tls = $this->credentials->tlsOptions();
        $opts = ['credentials' => $tls === null ? null : ['ssl' => $tls]];
        return $opts;
    }
}
