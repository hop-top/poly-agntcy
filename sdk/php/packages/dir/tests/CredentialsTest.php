<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Tests;

use PHPUnit\Framework\TestCase;
use HopTop\Agntcy\Dir\InsecureCredentials;
use HopTop\Agntcy\Dir\TlsCredentials;

final class CredentialsTest extends TestCase
{
    public function testInsecureReturnsNull(): void
    {
        $this->assertNull((new InsecureCredentials())->tlsOptions());
    }

    public function testTlsReturnsOptions(): void
    {
        $opts = ['verify_peer' => true];
        $this->assertSame($opts, (new TlsCredentials($opts))->tlsOptions());
    }
}
