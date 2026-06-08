<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Events\V1;

/**
 * EventService provides real-time event streaming for all system operations.
 * Events are delivered from subscription time forward with no history or replay.
 * This service enables external applications to react to system changes in real-time.
 */
class EventServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Listen establishes a streaming connection to receive events.
     * Events are only delivered while the stream is active.
     * On disconnect, missed events are not recoverable.
     * @param \Agntcy\Dir\Events\V1\ListenRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function Listen(\Agntcy\Dir\Events\V1\ListenRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/agntcy.dir.events.v1.EventService/Listen',
        $argument,
        ['\Agntcy\Dir\Events\V1\ListenResponse', 'decode'],
        $metadata, $options);
    }

}
