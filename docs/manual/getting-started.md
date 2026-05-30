# Getting Started with poly-agntcy

## Quick Start

1. Install poly-agntcy (see [install](install.md))

2. Initialize a new project:
   ```sh
   poly-agntcy init
   ```

3. Run your first command:
   ```sh
   poly-agntcy help
   ```

## Basic Usage

```sh
# Show help
poly-agntcy --help

# Show version
poly-agntcy --version

# Run with verbose output
poly-agntcy --verbose <command>
```

## Configuration

poly-agntcy looks for configuration in:

1. `./poly-agntcy.yaml` (project-local)
2. `~/.config/poly-agntcy/config.yaml` (user)
3. Environment variables prefixed with
   `poly-agntcy_` (uppercase)

See [configuration](configuration.md) for details.

## Next Steps

- [Configuration Reference](configuration.md)
- [Commands Reference](commands.md)
- [Troubleshooting](troubleshooting.md)
