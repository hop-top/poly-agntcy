import type { RequestHandler } from "express";

export interface Options {
  endpoint: string;
}

export function agntcyDirMiddleware(opts: Options): RequestHandler {
  if (!opts.endpoint) throw new Error("endpoint required");
  return (_req, res) => {
    res.json({ ok: true });
  };
}
