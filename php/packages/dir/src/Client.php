<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir;

use Psr\Http\Client\ClientInterface;

final class Client
{
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

    /**
     * @return list<object>
     */
    public function discover(string $capability): array
    {
        unset($capability);
        return [];
    }
}
