import { Hono } from "hono";
import { agntcyDirPlugin } from "@hop-top/agntcy-dir-hono";

type Env = { Bindings: { AGNTCY_ENDPOINT?: string } };

const app = new Hono<Env>();

app.post(
  "/agntcy",
  agntcyDirPlugin({ endpoint: process.env.AGNTCY_ENDPOINT ?? "http://localhost:8888" }),
);

app.get("/", (c) => c.text("hop-top/agntcy hono-edge example"));

export default app;
