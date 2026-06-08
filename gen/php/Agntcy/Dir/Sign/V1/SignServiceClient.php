<?php
// GENERATED CODE -- DO NOT EDIT!

// Original file comments:
// Copyright AGNTCY Contributors (https://github.com/agntcy)
// SPDX-License-Identifier: Apache-2.0
//
namespace Agntcy\Dir\Sign\V1;

/**
 * SignService provides methods to sign and verify records.
 *
 * NOTE: This is a client-side service and is not available on the server.
 */
class SignServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Sign record using keyless OIDC based provider or
     * using PEM-encoded private key with an optional passphrase.
     * @param \Agntcy\Dir\Sign\V1\SignRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Sign\V1\SignResponse>
     */
    public function Sign(\Agntcy\Dir\Sign\V1\SignRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.sign.v1.SignService/Sign',
        $argument,
        ['\Agntcy\Dir\Sign\V1\SignResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Verify signed record using keyless OIDC based provider or
     * using PEM-encoded public key.
     * @param \Agntcy\Dir\Sign\V1\VerifyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall<\Agntcy\Dir\Sign\V1\VerifyResponse>
     */
    public function Verify(\Agntcy\Dir\Sign\V1\VerifyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/agntcy.dir.sign.v1.SignService/Verify',
        $argument,
        ['\Agntcy\Dir\Sign\V1\VerifyResponse', 'decode'],
        $metadata, $options);
    }

}
