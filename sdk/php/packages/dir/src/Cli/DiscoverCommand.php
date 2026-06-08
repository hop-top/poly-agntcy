<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Cli;

use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\OutputInterface;

#[AsCommand(name: 'discover')]
final class DiscoverCommand extends Command
{
    protected function configure(): void
    {
        $this->addOption('endpoint', null, InputOption::VALUE_REQUIRED, 'DIR endpoint');
        $this->addOption('capability', null, InputOption::VALUE_REQUIRED, 'Capability');
    }

    protected function execute(InputInterface $input, OutputInterface $output): int
    {
        $endpoint = (string) $input->getOption('endpoint');
        $capability = (string) $input->getOption('capability');

        $client = new Client($endpoint, new InsecureCredentials());
        foreach ($client->discover($capability) as $agent) {
            $name = is_object($agent) && property_exists($agent, 'name') ? (string) $agent->name : '';
            $output->writeln($name);
        }

        return Command::SUCCESS;
    }
}
