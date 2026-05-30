<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Tests\Integration;

use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\TestCase;
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;

#[Group('integration')]
final class WireTest extends TestCase
{
    protected function setUp(): void
    {
        $endpoint = getenv('DIR_ENDPOINT');
        if ($endpoint === false || $endpoint === '') {
            $this->markTestSkipped('DIR_ENDPOINT not set');
        }
    }

    public function testDiscoverRoundtrip(): void
    {
        $endpoint = (string) getenv('DIR_ENDPOINT');
        $client = new Client($endpoint, new InsecureCredentials());
        $results = $client->discover('chat');
        $this->assertIsArray($results);
    }
}
