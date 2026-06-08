<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Tests;

use PHPUnit\Framework\TestCase;
use PolyAgntcy\Dir\InsecureCredentials;
use PolyAgntcy\Dir\TlsCredentials;

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
