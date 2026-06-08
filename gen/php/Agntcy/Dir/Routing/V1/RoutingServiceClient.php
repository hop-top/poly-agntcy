<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Routing\V1;

/**
 * Defines an interface for announcement and discovery
 * of records across interconnected network.
 *
 * Middleware should be used to control who can perform these RPCs.
 * Policies for the middleware can be handled via separate service.
 */
class RoutingServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Announce to the network that this peer is providing a given record.
     * This enables other peers to discover this record and retrieve it
     * from this peer. Listeners can use this event to perform custom operations,
     * for example by cloning the record.
     *
     * Items need to be periodically republished (eg. 24h) to the network
     * to avoid stale data. Republication should be done in the background.
     * @param \Agntcy\Dir\Routing\V1\PublishRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Google\Protobuf\GPBEmpty>
     */
    public function Publish(\Agntcy\Dir\Routing\V1\PublishRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.routing.v1.RoutingService/Publish',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Stop serving this record to the network. If other peers try
     * to retrieve this record, the peer will refuse the request.
     * @param \Agntcy\Dir\Routing\V1\UnpublishRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Google\Protobuf\GPBEmpty>
     */
    public function Unpublish(\Agntcy\Dir\Routing\V1\UnpublishRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.routing.v1.RoutingService/Unpublish',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Search records based on the request across the network.
     * This will search the network for the record with the given parameters.
     *
     * It is possible that the records are stale or that they do not exist.
     * Some records may be provided by multiple peers.
     *
     * Results from the search can be used as an input
     * to Pull operation to retrieve the records.
     * @param \Agntcy\Dir\Routing\V1\SearchRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function Search(\Agntcy\Dir\Routing\V1\SearchRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.routing.v1.RoutingService/Search',
        $argument,
        ['\Agntcy\Dir\Routing\V1\SearchResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * List all records that this peer is currently providing
     * that match the given parameters.
     * This operation does not interact with the network.
     * @param \Agntcy\Dir\Routing\V1\ListRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function List(\Agntcy\Dir\Routing\V1\ListRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.routing.v1.RoutingService/List',
        $argument,
        ['\Agntcy\Dir\Routing\V1\ListResponse', 'decode'],
        $metadata, $options);
    }

}
