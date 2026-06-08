<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Generated;

/**
 * Placeholder class declarations matching the codegen output of
 * buf.build/protocolbuffers/php + buf.build/grpc/php for the AGNTCY
 * Directory Service. Real classes are emitted into this directory by
 * `mise run gen` and the directory itself is gitignored. This shim
 * exists so phpstan analysis of the hand-written wire layer in
 * src/ resolves typed references without requiring codegen on every
 * build.
 *
 * Replace by deleting this file once codegen output is committed or
 * available in the analysis path.
 */

if (!class_exists(\HopTop\Agntcy\Dir\Generated\Agent::class, false)) {
    /**
     * @phpstan-type AgentArray array{name?: string, capabilities?: list<string>}
     */
    class Agent
    {
        public string $name = '';

        /** @var list<string> */
        public array $capabilities = [];
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\RegisterRequest::class, false)) {
    class RegisterRequest
    {
        public function setAgent(Agent $agent): self
        {
            return $this;
        }
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\RegisterResponse::class, false)) {
    class RegisterResponse
    {
        public string $id = '';
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\DiscoverRequest::class, false)) {
    class DiscoverRequest
    {
        public function setCapability(string $capability): self
        {
            return $this;
        }
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\DiscoverResponse::class, false)) {
    class DiscoverResponse
    {
        /** @var list<Agent> */
        public array $agents = [];
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\DescribeRequest::class, false)) {
    class DescribeRequest
    {
        public function setId(string $id): self
        {
            return $this;
        }
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\DescribeResponse::class, false)) {
    class DescribeResponse
    {
        public ?Agent $agent = null;
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\PublishRequest::class, false)) {
    class PublishRequest
    {
        public function setAgent(Agent $agent): self
        {
            return $this;
        }
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\PublishResponse::class, false)) {
    class PublishResponse
    {
        public string $version = '';
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\VerifyRequest::class, false)) {
    class VerifyRequest
    {
        public function setId(string $id): self
        {
            return $this;
        }
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\VerifyResponse::class, false)) {
    class VerifyResponse
    {
        public bool $valid = false;
    }
}

if (!class_exists(\HopTop\Agntcy\Dir\Generated\DirectoryServiceClient::class, false)) {
    /**
     * Mirrors buf grpc/php codegen: a synchronous unary stub. Real impl
     * extends \Grpc\BaseStub and returns \Grpc\UnaryCall. The shim returns
     * a tuple-shape so hand-written wire code stays compilable without
     * the grpc/grpc PHP extension being present in static analysis.
     */
    class DirectoryServiceClient
    {
        public function __construct(string $hostname, array $opts = [])
        {
            unset($hostname, $opts);
        }

        /**
         * @return array{0: RegisterResponse, 1: object}
         */
        public function Register(RegisterRequest $request): array
        {
            unset($request);
            return [new RegisterResponse(), new \stdClass()];
        }

        /**
         * @return array{0: DiscoverResponse, 1: object}
         */
        public function Discover(DiscoverRequest $request): array
        {
            unset($request);
            return [new DiscoverResponse(), new \stdClass()];
        }

        /**
         * @return array{0: DescribeResponse, 1: object}
         */
        public function Describe(DescribeRequest $request): array
        {
            unset($request);
            return [new DescribeResponse(), new \stdClass()];
        }

        /**
         * @return array{0: PublishResponse, 1: object}
         */
        public function Publish(PublishRequest $request): array
        {
            unset($request);
            return [new PublishResponse(), new \stdClass()];
        }

        /**
         * @return array{0: VerifyResponse, 1: object}
         */
        public function Verify(VerifyRequest $request): array
        {
            unset($request);
            return [new VerifyResponse(), new \stdClass()];
        }
    }
}
