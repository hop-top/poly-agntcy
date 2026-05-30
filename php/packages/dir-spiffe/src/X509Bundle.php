<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Spiffe;

/**
 * X509 trust bundle for a given trust domain.
 */
final class X509Bundle
{
    /**
     * @param list<string> $caCertsPem
     */
    public function __construct(
        public readonly string $trustDomain,
        public readonly array $caCertsPem,
    ) {
    }
}
