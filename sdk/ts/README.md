# sdk/ts

TypeScript does not ship a first-party SDK. The Node/TS ecosystem consumes the
upstream `agntcy-dir` SDK directly; the `dir-next`, `dir-hono`, and
`dir-express` packages under this repo are framework **adapters**, not SDKs,
and live under `adapters/<framework>/` after the adapter migration completes.

See `docs/adr/0007-role-based-layout-sdk-adapter-split.md` for the layout
rationale, and `docs/adr/0001-typescript-no-first-party-sdk.md` for the
upstream-consumption decision.

## Status

Placeholder. The TS workspace root (`package.json`, `pnpm-workspace.yaml`,
`pnpm-lock.yaml`, `tsconfig.base.json`, `.npmrc`, `.gitignore`, `README.md`)
currently still lives at `ts/` because the adapter packages have not yet been
moved out of `ts/packages/`. Moving the workspace root before the adapters
would leave `pnpm-workspace.yaml`'s `packages: - 'packages/*'` glob pointing
at an empty directory and break `pnpm install`.

Once the adapter migration lands and `pnpm-workspace.yaml` is rewritten to
point at the new adapter locations, the workspace root files move here and
this README is replaced with real package documentation.
