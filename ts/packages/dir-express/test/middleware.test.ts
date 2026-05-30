import { describe, it, expect } from "vitest";
import express from "express";
import request from "supertest";
import { agntcyDirMiddleware } from "../src";

describe("agntcyDirMiddleware", () => {
  it("responds 501 with pending-integration error body", async () => {
    const app = express();
    app.post(
      "/agntcy",
      agntcyDirMiddleware({ endpoint: "https://dir.example" }),
    );
    const res = await request(app).post("/agntcy");
    expect(res.status).toBe(501);
    expect(res.body).toEqual({
      error: "agntcy-dir wire integration pending",
    });
  });

  it("throws if endpoint missing", () => {
    expect(() => agntcyDirMiddleware({ endpoint: "" })).toThrow();
  });
});
