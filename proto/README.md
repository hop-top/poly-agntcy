# proto

Single source of truth for protobuf bindings.

We don't check in `.proto` files. Instead, `buf.yaml` declares a BSR module dependency on [`buf.build/agntcy/dir`](https://buf.build/agntcy/dir), pinned by `buf.lock`.

## Regenerate stubs

```sh
mise run gen
```

Generates Go, Rust, and PHP stubs into the respective language roots (gitignored). TS and Python don't need generation — they consume the official `agntcy-dir` SDK directly.

## Bump spec version

1. Edit `buf.yaml` dep version pin (or run `buf dep update` to refresh `buf.lock`)
2. `mise run gen`
3. Fix per-language breakage
4. Commit with `BREAKING CHANGE:` trailer if wire-incompatible
