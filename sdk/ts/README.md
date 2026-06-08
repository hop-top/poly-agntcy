# hop-top agntcy TypeScript adapters

pnpm workspace shipping framework adapters for the AGNTCY Directory
Service.

## Packages

| Package | Role |
|---------|------|
| [`@hop-top/agntcy-dir-next`](../../adapters/next) | Next.js Route Handler + `agntcy` CLI |
| [`@hop-top/agntcy-dir-hono`](../../adapters/hono) | Hono plugin |
| [`@hop-top/agntcy-dir-express`](../../adapters/express) | Express middleware |

All three are alpha. Function/type contracts are stable; runtime stubs
respond with HTTP 501 until upstream `agntcy-dir` wire integration lands.

## Install

```sh
pnpm install --config.auto-install-peers=false --ignore-scripts
```

`--config.auto-install-peers=false --ignore-scripts` works around a
transitive Buf Schema Registry auth error pulled in by `agntcy-dir`. See
each package README for the same note in adopter context.

## Build / test

```sh
pnpm -r build                                    # compile each package
pnpm -r exec tsc --noEmit -p tsconfig.json       # typecheck
pnpm -r test                                     # vitest
```

## License

Apache-2.0
