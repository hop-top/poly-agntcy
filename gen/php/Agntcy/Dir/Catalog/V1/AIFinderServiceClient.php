<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Catalog\V1;

/**
 * AIFinderService is the federated AI Finder discovery surface over the AI
 * Catalog model. It is exposed over gRPC and, via grpc-gateway, as a REST API.
 */
class AIFinderServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * ListAgents returns catalog entries with deterministic, cacheable browsing
     * semantics (database filtering, no relevance ranking).
     * @param \Agntcy\Dir\Catalog\V1\ListAgentsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Catalog\V1\ListAgentsResponse>
     */
    public function ListAgents(\Agntcy\Dir\Catalog\V1\ListAgentsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.catalog.v1.AIFinderService/ListAgents',
        $argument,
        ['\Agntcy\Dir\Catalog\V1\ListAgentsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetWellKnownCatalog returns the AI Catalog well-known document (host
     * descriptor, published entries, and dynamic collections) at the RFC 8615 URI.
     * @param \Agntcy\Dir\Catalog\V1\GetWellKnownCatalogRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Catalog\V1\GetWellKnownCatalogResponse>
     */
    public function GetWellKnownCatalog(\Agntcy\Dir\Catalog\V1\GetWellKnownCatalogRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.catalog.v1.AIFinderService/GetWellKnownCatalog',
        $argument,
        ['\Agntcy\Dir\Catalog\V1\GetWellKnownCatalogResponse', 'decode'],
        $metadata, $options);
    }

}
