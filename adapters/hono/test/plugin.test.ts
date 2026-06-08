import { describe, it, expect } from "vitest";
import { Hono } from "hono";
import { agntcyDirPlugin } from "../src";

describe("agntcyDirPlugin", () => {
  it("responds 501 with pending-integration error body", async () => {
    const app = new Hono().use(
      "/agntcy",
      agntcyDirPlugin({ endpoint: "https://dir.example" }),
    );
    const res = await app.request("/agntcy", { method: "POST" });
    expect(res.status).toBe(501);
    expect(await res.json()).toEqual({
      error: "agntcy-dir wire integration pending",
    });
  });
});
