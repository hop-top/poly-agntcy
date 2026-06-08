<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Store\V1;

/**
 * SyncService provides functionality for synchronizing objects between Directory nodes.
 *
 * This service enables one-way synchronization from a remote Directory node to the local node,
 * allowing distributed Directory instances to share and replicate objects. The service supports
 * both on-demand synchronization and tracking of sync operations through their lifecycle.
 */
class SyncServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * CreateSync initiates a new synchronization operation from a remote Directory node.
     *
     * The operation is non-blocking and returns immediately with a sync ID that can be used
     * to track progress and manage the sync operation.
     * @param \Agntcy\Dir\Store\V1\CreateSyncRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Store\V1\CreateSyncResponse>
     */
    public function CreateSync(\Agntcy\Dir\Store\V1\CreateSyncRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.store.v1.SyncService/CreateSync',
        $argument,
        ['\Agntcy\Dir\Store\V1\CreateSyncResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * ListSyncs returns a stream of all sync operations known to the system.
     *
     * This includes active, completed, and failed synchronizations.
     * @param \Agntcy\Dir\Store\V1\ListSyncsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function ListSyncs(\Agntcy\Dir\Store\V1\ListSyncsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.store.v1.SyncService/ListSyncs',
        $argument,
        ['\Agntcy\Dir\Store\V1\ListSyncsItem', 'decode'],
        $metadata, $options);
    }

    /**
     * GetSync retrieves detailed status information for a specific synchronization.
     * @param \Agntcy\Dir\Store\V1\GetSyncRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Store\V1\GetSyncResponse>
     */
    public function GetSync(\Agntcy\Dir\Store\V1\GetSyncRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.store.v1.SyncService/GetSync',
        $argument,
        ['\Agntcy\Dir\Store\V1\GetSyncResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * DeleteSync removes a synchronization operation from the system.
     * @param \Agntcy\Dir\Store\V1\DeleteSyncRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Store\V1\DeleteSyncResponse>
     */
    public function DeleteSync(\Agntcy\Dir\Store\V1\DeleteSyncRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.store.v1.SyncService/DeleteSync',
        $argument,
        ['\Agntcy\Dir\Store\V1\DeleteSyncResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * RequestRegistryCredentials requests registry credentials between two Directory nodes.
     *
     * This RPC allows a requesting node to authenticate with this node and obtain
     * temporary registry credentials for secure OCI synchronization.
     * @param \Agntcy\Dir\Store\V1\RequestRegistryCredentialsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Store\V1\RequestRegistryCredentialsResponse>
     */
    public function RequestRegistryCredentials(\Agntcy\Dir\Store\V1\RequestRegistryCredentialsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.store.v1.SyncService/RequestRegistryCredentials',
        $argument,
        ['\Agntcy\Dir\Store\V1\RequestRegistryCredentialsResponse', 'decode'],
        $metadata, $options);
    }

}
