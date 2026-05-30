<?php

declare(strict_types=1);

namespace App\Console\Commands;

use Illuminate\Console\Command;
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\Generated\Agent;

final class RegisterInventoryAgent extends Command
{
    protected $signature = 'agntcy:register {--capability=inventory}';
    protected $description = 'Register an agent descriptor against the configured DIR';

    public function handle(Client $client): int
    {
        $capability = (string) $this->option('capability');
        $agent = (new Agent())->setName('inventory-agent')->setCapabilities([$capability]);
        $id = $client->register($agent);
        $this->info("registered id={$id}");
        return self::SUCCESS;
    }
}
