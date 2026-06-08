<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Laravel\Facades;

use Illuminate\Support\Facades\Facade;
use HopTop\Agntcy\Dir\Client;

final class Agntcy extends Facade
{
    protected static function getFacadeAccessor(): string
    {
        return Client::class;
    }
}
