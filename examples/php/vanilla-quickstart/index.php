<?php

declare(strict_types=1);

require __DIR__.'/vendor/autoload.php';

use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\Generated\Agent;
use PolyAgntcy\Dir\InsecureCredentials;

$endpoint   = getenv('AGNTCY_ENDPOINT') ?: 'http://localhost:8888';
$capability = getenv('AGNTCY_CAPABILITY') ?: 'inventory';

$client = new Client($endpoint, new InsecureCredentials());

$agent = (new Agent())->setName('inventory-agent')->setCapabilities([$capability]);

$id = $client->register($agent);
echo "registered id={$id}\n";

foreach ($client->discover($capability) as $a) {
    echo "discovered {$a->name}\n";
}

$version = $client->publish($agent);
echo "published version={$version}\n";
