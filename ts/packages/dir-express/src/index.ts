import type { RequestHandler } from "express";

export interface Options {
  endpoint: string;
}

export function agntcyDirMiddleware(opts: Options): RequestHandler {
  if (!opts.endpoint) throw new Error("endpoint required");
  return (_req, res) => {
    res.status(501).json({ error: "agntcy-dir wire integration pending" });
  };
}
