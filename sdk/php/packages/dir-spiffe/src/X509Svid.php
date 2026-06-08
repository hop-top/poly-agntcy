<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Spiffe;

/**
 * X509-SVID: leaf certificate, private key, intermediate chain.
 */
final class X509Svid
{
    /**
     * @param list<string> $intermediates
     */
    public function __construct(
        public readonly SpiffeId $id,
        public readonly string $certificatePem,
        public readonly string $privateKeyPem,
        public readonly array $intermediates = [],
        public readonly ?int $expiresAt = null,
    ) {
    }
}
