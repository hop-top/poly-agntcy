<?php

declare(strict_types=1);

use Illuminate\Support\Facades\Route;
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\Generated\Agent;

Route::get('/agntcy/discover/{capability}', function (string $capability, Client $client) {
    $agents = $client->discover($capability);
    return response()->json([
        'capability' => $capability,
        'agents'     => array_map(fn (Agent $a) => $a->name, $agents),
    ]);
});
