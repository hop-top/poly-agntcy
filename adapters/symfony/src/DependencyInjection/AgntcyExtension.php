<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Symfony\DependencyInjection;

use HopTop\Agntcy\Dir\Client;
use HopTop\Agntcy\Dir\InsecureCredentials;
use Symfony\Component\DependencyInjection\ContainerBuilder;
use Symfony\Component\DependencyInjection\Definition;
use Symfony\Component\HttpKernel\DependencyInjection\Extension;

final class AgntcyExtension extends Extension
{
    /**
     * @param array<int,array<string,mixed>> $configs
     */
    public function load(array $configs, ContainerBuilder $container): void
    {
        /** @var array{endpoint: string, timeout: int} $cfg */
        $cfg = $this->processConfiguration(new Configuration(), $configs);

        $container->setDefinition(
            Client::class,
            (new Definition(Client::class))
                ->setArgument(0, $cfg['endpoint'])
                ->setArgument(1, new Definition(InsecureCredentials::class))
                ->setPublic(true),
        );
    }
}
