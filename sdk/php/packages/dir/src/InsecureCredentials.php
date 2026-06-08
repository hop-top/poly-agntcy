<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir;

final class InsecureCredentials implements Credentials
{
    public function tlsOptions(): ?array
    {
        return null;
    }
}
