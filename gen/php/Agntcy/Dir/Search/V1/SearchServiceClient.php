<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Search\V1;

/**
 */
class SearchServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Search for record CIDs that match the given parameters.
     * Returns only CIDs for efficient lookups and piping to other commands.
     * This operation does not interact with the network.
     * @param \Agntcy\Dir\Search\V1\SearchCIDsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function SearchCIDs(\Agntcy\Dir\Search\V1\SearchCIDsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.search.v1.SearchService/SearchCIDs',
        $argument,
        ['\Agntcy\Dir\Search\V1\SearchCIDsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Search for full records that match the given parameters.
     * Returns complete record data including all metadata, skills, domains, etc.
     * This operation does not interact with the network.
     * @param \Agntcy\Dir\Search\V1\SearchRecordsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function SearchRecords(\Agntcy\Dir\Search\V1\SearchRecordsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.search.v1.SearchService/SearchRecords',
        $argument,
        ['\Agntcy\Dir\Search\V1\SearchRecordsResponse', 'decode'],
        $metadata, $options);
    }

}
