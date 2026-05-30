<?php

declare(strict_types=1);

namespace Agntcy\PolyAgntcy\Commands;

use HopTop\Kit\Cli\KitCommand;
use HopTop\Kit\Output\Formatter\ColumnSpec;
use Symfony\Component\Console\Attribute\AsCommand;

/**
 * Demo list command — exercises the kit output flag suite.
 *
 * Try:
 *
 *   poly-agntcy list
 *   poly-agntcy list --format json
 *   poly-agntcy list --format yaml
 *   poly-agntcy list --cols name,status
 *   poly-agntcy list -o /tmp/out.json   # ext infers json
 *   poly-agntcy list --format-help
 */
#[AsCommand(name: 'list', description: 'Demo list command — exercises the output flag suite.')]
final class ListCommand extends KitCommand
{
    protected function handle(): int
    {
        $items = [
            ['name' => 'alpha', 'count' => 1, 'status' => 'ok'],
            ['name' => 'beta', 'count' => 2, 'status' => 'warn'],
        ];

        $this->render($items, columns: [
            ColumnSpec::of(header: 'name', key: 'name', priority: 9),
            ColumnSpec::of(header: 'count', key: 'count', priority: 7),
            ColumnSpec::of(header: 'status', key: 'status', priority: 5),
        ]);

        return self::SUCCESS;
    }
}
