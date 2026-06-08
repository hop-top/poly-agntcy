<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Symfony\DependencyInjection;

use Symfony\Component\Config\Definition\Builder\TreeBuilder;
use Symfony\Component\Config\Definition\ConfigurationInterface;

final class Configuration implements ConfigurationInterface
{
    public function getConfigTreeBuilder(): TreeBuilder
    {
        $tb = new TreeBuilder('agntcy');
        $root = $tb->getRootNode();
        $root
            ->children()
                ->scalarNode('endpoint')->defaultValue('https://directory.example')->end()
                ->integerNode('timeout')->defaultValue(5)->end()
            ->end();

        return $tb;
    }
}
