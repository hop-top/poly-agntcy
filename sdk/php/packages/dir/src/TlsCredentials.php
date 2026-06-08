<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir;

final class TlsCredentials implements Credentials
{
    /**
     * @param array<string,mixed> $options
     */
    public function __construct(private readonly array $options)
    {
    }

    public function tlsOptions(): ?array
    {
        return $this->options;
    }
}
