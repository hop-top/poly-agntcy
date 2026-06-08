<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Store\V1;

/**
 * Defines an interface for content-addressable storage
 * service for objects.
 *
 * Max object size: 4MB (to fully fit in a single request)
 * Max metadata size: 100KB
 *
 * Store service can be implemented by various storage backends,
 * such as local file system, OCI registry, etc.
 *
 * Middleware should be used to control who can perform these RPCs.
 * Policies for the middleware can be handled via separate service.
 *
 * Each operation is performed sequentially, meaning that
 * for the N-th request, N-th response will be returned.
 * If an error occurs, the stream will be cancelled.
 */
class StoreServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Push performs write operation for given records.
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\BidiStreamingCall
     */
    public function Push($metadata = [], $options = []) {
        return $this->_bidiRequest('/agntcy.dir.store.v1.StoreService/Push',
        ['\Agntcy\Dir\Core\V1\RecordRef','decode'],
        $metadata, $options);
    }

    /**
     * Pull performs read operation for given records.
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\BidiStreamingCall
     */
    public function Pull($metadata = [], $options = []) {
        return $this->_bidiRequest('/agntcy.dir.store.v1.StoreService/Pull',
        ['\Agntcy\Dir\Core\V1\Record','decode'],
        $metadata, $options);
    }

    /**
     * Lookup resolves basic metadata for the records.
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\BidiStreamingCall
     */
    public function Lookup($metadata = [], $options = []) {
        return $this->_bidiRequest('/agntcy.dir.store.v1.StoreService/Lookup',
        ['\Agntcy\Dir\Core\V1\RecordMeta','decode'],
        $metadata, $options);
    }

    /**
     * Remove performs delete operation for the records.
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ClientStreamingCall
     */
    public function Delete($metadata = [], $options = []) {
        return $this->_clientStreamRequest('/agntcy.dir.store.v1.StoreService/Delete',
        ['\Google\Protobuf\GPBEmpty','decode'],
        $metadata, $options);
    }

    /**
     * PushReferrer performs write operation for record referrers.
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\BidiStreamingCall
     */
    public function PushReferrer($metadata = [], $options = []) {
        return $this->_bidiRequest('/agntcy.dir.store.v1.StoreService/PushReferrer',
        ['\Agntcy\Dir\Store\V1\PushReferrerResponse','decode'],
        $metadata, $options);
    }

    /**
     * PullReferrer performs read operation for record referrers.
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\BidiStreamingCall
     */
    public function PullReferrer($metadata = [], $options = []) {
        return $this->_bidiRequest('/agntcy.dir.store.v1.StoreService/PullReferrer',
        ['\Agntcy\Dir\Store\V1\PullReferrerResponse','decode'],
        $metadata, $options);
    }

    /**
     * DeleteReferrer performs delete operation for record referrers.
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\BidiStreamingCall
     */
    public function DeleteReferrer($metadata = [], $options = []) {
        return $this->_bidiRequest('/agntcy.dir.store.v1.StoreService/DeleteReferrer',
        ['\Agntcy\Dir\Store\V1\DeleteReferrerResponse','decode'],
        $metadata, $options);
    }

}
