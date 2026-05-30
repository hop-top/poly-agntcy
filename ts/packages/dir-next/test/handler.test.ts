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
});
