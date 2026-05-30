import { describe, it, expect } from "vitest";
import { createDirectoryRouteHandler } from "../src/handler";

describe("createDirectoryRouteHandler", () => {
  it("returns an object with POST handler", () => {
    const h = createDirectoryRouteHandler({ endpoint: "https://dir.example" });
    expect(typeof h.POST).toBe("function");
  });

  it("throws if endpoint missing", () => {
    expect(() => createDirectoryRouteHandler({ endpoint: "" })).toThrow();
  });

  it("POST responds 501 with pending-integration error body", async () => {
    const h = createDirectoryRouteHandler({ endpoint: "https://dir.example" });
    const res = await h.POST(new Request("https://example/agntcy", { method: "POST" }));
    expect(res.status).toBe(501);
    expect(await res.json()).toEqual({
      error: "agntcy-dir wire integration pending",
    });
  });
});
