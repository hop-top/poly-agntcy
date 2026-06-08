<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Naming\V1;

/**
 * NamingService provides methods for name resolution and verification.
 * Note: Verification is performed automatically by the backend scheduler
 * for signed records with verifiable names (http://, https:// prefixes).
 */
class NamingServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * GetVerificationInfo retrieves the verification info for a record.
     * @param \Agntcy\Dir\Naming\V1\GetVerificationInfoRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Naming\V1\GetVerificationInfoResponse>
     */
    public function GetVerificationInfo(\Agntcy\Dir\Naming\V1\GetVerificationInfoRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.naming.v1.NamingService/GetVerificationInfo',
        $argument,
        ['\Agntcy\Dir\Naming\V1\GetVerificationInfoResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Resolve resolves a record reference (name with optional version) to CIDs.
     * Supports Docker-style references:
     *   - "name" -> returns all versions (newest first)
     *   - "name:version" -> returns the specific version
     *   - "name@cid" -> hash-verified lookup (latest version)
     *   - "name:version@cid" -> hash-verified lookup (specific version)
     * Returns an error if no matching record is found.
     * @param \Agntcy\Dir\Naming\V1\ResolveRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Naming\V1\ResolveResponse>
     */
    public function Resolve(\Agntcy\Dir\Naming\V1\ResolveRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.naming.v1.NamingService/Resolve',
        $argument,
        ['\Agntcy\Dir\Naming\V1\ResolveResponse', 'decode'],
        $metadata, $options);
    }

}
