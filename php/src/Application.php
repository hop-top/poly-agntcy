<?php

declare(strict_types=1);

namespace Agntcy\PolyAgntcy;

use HopTop\Kit\Output\Flags;
use Symfony\Component\Console\Application as SymfonyApplication;
use Agntcy\PolyAgntcy\Commands\HelloCommand;
use Agntcy\PolyAgntcy\Commands\ListCommand;

/**
 * poly-agntcy CLI entry point.
 *
 * Wires the kit Output flag suite (--format, --format-opt, --format-help,
 * --cols, --template, -o/--output) on every command, then registers the
 * built-in commands. Add your own commands by extending KitCommand and
 * calling {@see SymfonyApplication::add()} below.
 */
final class Application extends SymfonyApplication
{
    public function __construct()
    {
        parent::__construct('poly-agntcy', '0.0.0');

        // Wire the cross-language output flag suite. Every Command added
        // after this call picks up --format/--cols/etc. automatically.
        Flags::register($this);

        $this->add(new HelloCommand());
        $this->add(new ListCommand());
    }
}
