# Upgrading poly-agntcy

## Check Current Version

```sh
poly-agntcy --version
```

## Upgrade Methods

### Go

```sh
go install hop.top/agntcy/poly-agntcy@latest
```

### npm

```sh
npm update -g poly-agntcy
```

### pip

```sh
pip install --upgrade poly-agntcy
```

### Homebrew

```sh
brew upgrade poly-agntcy
```

## Breaking Changes

Check the CHANGELOG for breaking changes between
versions before upgrading.

## Rollback

If an upgrade causes issues, install the previous
version explicitly:

```sh
# Go
go install hop.top/agntcy/poly-agntcy@v<PREVIOUS_VERSION>

# npm
npm install -g poly-agntcy@<PREVIOUS_VERSION>

# pip
pip install poly-agntcy==<PREVIOUS_VERSION>
```
