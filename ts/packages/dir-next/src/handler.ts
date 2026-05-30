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
      return new Response(
        JSON.stringify({ error: "agntcy-dir wire integration pending" }),
        {
          status: 501,
          headers: { "content-type": "application/json" },
        },
      );
    },
  };
}
