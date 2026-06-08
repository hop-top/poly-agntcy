<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Laravel;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Support\ServiceProvider;
use HopTop\Agntcy\Dir\Client;
use HopTop\Agntcy\Dir\InsecureCredentials;

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
