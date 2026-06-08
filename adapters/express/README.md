# @poly-agntcy/dir-express

Express middleware for the AGNTCY Directory Service.

Status: alpha. Wire integration with `agntcy-dir` is pending. The current
build ships function/type contracts only; runtime calls respond with HTTP
501 until the upstream client is wired in.

## Install

```sh
pnpm add @poly-agntcy/dir-express agntcy-dir express
```

`agntcy-dir` peer floor is `>=0.3` — the highest version currently published
on npm. The floor will move once the upstream client publishes a stable
`1.x`.

### Install workaround: buf BSR transitive auth

`agntcy-dir` transitively pulls protobuf descriptors from the Buf Schema
Registry. If `pnpm install` fails with a BSR auth error, opt out of
peer auto-install and skip lifecycle scripts:

```sh
pnpm install --config.auto-install-peers=false --ignore-scripts
```

## Usage

```ts
import express from "express";
import { agntcyDirMiddleware } from "@poly-agntcy/dir-express";

const app = express();
app.post(
  "/agntcy",
  agntcyDirMiddleware({ endpoint: process.env.AGNTCY_DIR_ENDPOINT! }),
);
```

## License

Apache-2.0
