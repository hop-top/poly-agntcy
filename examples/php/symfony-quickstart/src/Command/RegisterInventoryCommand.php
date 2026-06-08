<?php

declare(strict_types=1);

namespace App\Command;

use HopTop\Agntcy\Dir\Client;
use HopTop\Agntcy\Dir\Generated\Agent;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\OutputInterface;

#[AsCommand(name: 'agntcy:register', description: 'Register an inventory agent with the DIR')]
final class RegisterInventoryCommand extends Command
{
    public function __construct(private readonly Client $client)
    {
        parent::__construct();
    }

    protected function configure(): void
    {
        $this->addOption('capability', null, InputOption::VALUE_REQUIRED, 'capability label', 'inventory');
    }

    protected function execute(InputInterface $input, OutputInterface $output): int
    {
        $cap = (string) $input->getOption('capability');
        $agent = (new Agent())->setName('inventory-agent')->setCapabilities([$cap]);
        $id = $this->client->register($agent);
        $output->writeln("registered id={$id}");
        return Command::SUCCESS;
    }
}
