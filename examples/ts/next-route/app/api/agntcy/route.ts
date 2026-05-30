import { createDirectoryRouteHandler } from "@poly-agntcy/dir-next";

const handlers = createDirectoryRouteHandler({
  endpoint: process.env.AGNTCY_ENDPOINT ?? "http://localhost:8888",
});

export const POST = handlers.POST;
