<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Laravel;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Support\ServiceProvider;
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;

final class AgntcyServiceProvider extends ServiceProvider
{
    public function register(): void
    {
        $this->mergeConfigFrom(__DIR__.'/../config/agntcy.php', 'agntcy');

        $this->app->singleton(Client::class, static function (Application $app): Client {
            /** @var array{endpoint: string, timeout: int} $cfg */
            $cfg = $app['config']->get('agntcy');
            return new Client($cfg['endpoint'], new InsecureCredentials());
        });
    }

    public function boot(): void
    {
        $this->publishes([
            __DIR__.'/../config/agntcy.php' => $this->app->configPath('agntcy.php'),
        ], 'agntcy-config');
    }
}
