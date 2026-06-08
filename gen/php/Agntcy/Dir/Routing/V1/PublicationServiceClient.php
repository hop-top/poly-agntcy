<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Routing\V1;

/**
 * PublicationService manages publication requests for announcing records to the DHT.
 * 
 * Publications are stored in the database and processed by a worker that runs every hour.
 * The publication workflow:
 * 1. Publications are created via routing's Publish RPC by specifying either a query, a list of CIDs, or all records
 * 2. Publication requests are added to the database
 * 3. PublicationWorker queries the data using the publication request from the database to get the list of CIDs to be published
 * 4. PublicationWorker announces the records with these CIDs to the DHT
 */
class PublicationServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * CreatePublication creates a new publication request that will be processed by the PublicationWorker.
     * The publication request can specify either a query, a list of specific CIDs, or all records to be announced to the DHT.
     * @param \Agntcy\Dir\Routing\V1\PublishRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Routing\V1\CreatePublicationResponse>
     */
    public function CreatePublication(\Agntcy\Dir\Routing\V1\PublishRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.routing.v1.PublicationService/CreatePublication',
        $argument,
        ['\Agntcy\Dir\Routing\V1\CreatePublicationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * ListPublications returns a stream of all publication requests in the system.
     * This allows monitoring of pending, processing, and completed publication requests.
     * @param \Agntcy\Dir\Routing\V1\ListPublicationsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function ListPublications(\Agntcy\Dir\Routing\V1\ListPublicationsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.routing.v1.PublicationService/ListPublications',
        $argument,
        ['\Agntcy\Dir\Routing\V1\ListPublicationsItem', 'decode'],
        $metadata, $options);
    }

    /**
     * GetPublication retrieves details of a specific publication request by its identifier.
     * This includes the current status and any associated metadata.
     * @param \Agntcy\Dir\Routing\V1\GetPublicationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Routing\V1\GetPublicationResponse>
     */
    public function GetPublication(\Agntcy\Dir\Routing\V1\GetPublicationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.routing.v1.PublicationService/GetPublication',
        $argument,
        ['\Agntcy\Dir\Routing\V1\GetPublicationResponse', 'decode'],
        $metadata, $options);
    }

}
