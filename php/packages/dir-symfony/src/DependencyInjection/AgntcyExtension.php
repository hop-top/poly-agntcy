<?php

declare(strict_types=1);

namespace PolyAgntcy\Dir\Symfony\DependencyInjection;

use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;
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
