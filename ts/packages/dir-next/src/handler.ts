export interface Options {
  endpoint: string;
}

export interface RouteHandlers {
  POST: (req: Request) => Promise<Response>;
}

export function createDirectoryRouteHandler(opts: Options): RouteHandlers {
  if (!opts.endpoint) {
    throw new Error("endpoint required");
  }
  return {
    POST: async (_req: Request): Promise<Response> => {
      return new Response(JSON.stringify({ ok: true }), {
        headers: { "Content-Type": "application/json" },
      });
    },
  };
}
