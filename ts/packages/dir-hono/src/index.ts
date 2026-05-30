import type { MiddlewareHandler } from "hono";

export interface Options {
  endpoint: string;
}

export function agntcyDirPlugin(opts: Options): MiddlewareHandler {
  if (!opts.endpoint) throw new Error("endpoint required");
  return async (c) => {
    return c.json({ error: "agntcy-dir wire integration pending" }, 501);
  };
}
