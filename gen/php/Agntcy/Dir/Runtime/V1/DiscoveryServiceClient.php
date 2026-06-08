<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Runtime\V1;

/**
 * DiscoveryService defines the gRPC service for discovering workloads.
 * The service itself does not actually perform discovery, but rather queries the
 * underlying store for existing workloads.
 * It supports filtering workloads based on provided criteria.
 */
class DiscoveryServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Get a workload by its identifier.
     * @param \Agntcy\Dir\Runtime\V1\GetWorkloadRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Runtime\V1\Workload>
     */
    public function GetWorkload(\Agntcy\Dir\Runtime\V1\GetWorkloadRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.runtime.v1.DiscoveryService/GetWorkload',
        $argument,
        ['\Agntcy\Dir\Runtime\V1\Workload', 'decode'],
        $metadata, $options);
    }

    /**
     * List all workloads based on filters.
     * @param \Agntcy\Dir\Runtime\V1\ListWorkloadsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function ListWorkloads(\Agntcy\Dir\Runtime\V1\ListWorkloadsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.runtime.v1.DiscoveryService/ListWorkloads',
        $argument,
        ['\Agntcy\Dir\Runtime\V1\Workload', 'decode'],
        $metadata, $options);
    }

}
