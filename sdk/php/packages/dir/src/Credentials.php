<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir;

interface Credentials
{
    /**
     * Returns an array of TLS stream-context options, or null for plaintext.
     *
     * Keys match PHP's stream context "ssl" options.
     *
     * @return array<string,mixed>|null
     */
    public function tlsOptions(): ?array;
}
