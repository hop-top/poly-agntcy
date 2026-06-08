# contracts

Pin + lint layer for the protocol contracts this repo implements. BSR
remains the source of truth for `.proto` files; this directory does
not host `.proto` content locally.

`buf.yaml` declares a BSR module dependency on
[`buf.build/agntcy/dir`](https://buf.build/agntcy/dir), pinned by
`buf.lock`. `buf.gen.yaml` controls the per-language codegen output
paths.

Future home for non-proto schemas (JSON Schema / OpenAPI) when those
land.

## Regenerate stubs

```sh
mise run gen
```

Generates Go, Rust, and PHP stubs into the respective language roots
(gitignored). TS and Python don't need generation — they consume the
official `agntcy-dir` SDK directly.

## Bump spec version

1. Edit `buf.yaml` dep version pin (or run `buf dep update` to refresh
   `buf.lock`)
2. `mise run gen`
3. Fix per-language breakage
4. Commit with `BREAKING CHANGE:` trailer if wire-incompatible
