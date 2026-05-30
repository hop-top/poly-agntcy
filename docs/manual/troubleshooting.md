# Troubleshooting poly-agntcy

## Common Issues

### Command not found

Ensure poly-agntcy is installed and on your PATH:

```sh
which poly-agntcy
```

If missing, reinstall (see [install](install.md)).

### Permission denied

Check file permissions. On Unix systems:

```sh
chmod +x $(which poly-agntcy)
```

### Config not loading

Verify config file location and syntax:

```sh
poly-agntcy --verbose <command>
```

This prints which config files were loaded.

### Unexpected output format

Set the output format explicitly:

```sh
poly-agntcy --output json <command>
```

## Debug Mode

Run with verbose output for detailed diagnostics:

```sh
poly-agntcy --verbose <command>
```

## Getting Help

1. Check this troubleshooting guide
2. Search existing issues on GitHub
3. Open a new issue with:
   - poly-agntcy version (`poly-agntcy --version`)
   - OS and architecture
   - Steps to reproduce
   - Expected vs actual behavior
