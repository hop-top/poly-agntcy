<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Laravel\Facades;

use Illuminate\Support\Facades\Facade;
use PolyAgntcy\Dir\Client;

final class Agntcy extends Facade
{
    protected static function getFacadeAccessor(): string
    {
        return Client::class;
    }
}
