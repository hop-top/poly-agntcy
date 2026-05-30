import { describe, it, expect } from "vitest";
import { Hono } from "hono";
import { agntcyDirPlugin } from "../src";

describe("agntcyDirPlugin", () => {
  it("registers a POST route on /agntcy", async () => {
    const app = new Hono().use(
      "/agntcy",
      agntcyDirPlugin({ endpoint: "https://dir.example" }),
    );
    const res = await app.request("/agntcy", { method: "POST" });
    expect(res.status).toBe(200);
  });
});
