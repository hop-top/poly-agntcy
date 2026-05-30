import { describe, it, expect } from "vitest";
import express from "express";
import request from "supertest";
import { agntcyDirMiddleware } from "../src";

describe("agntcyDirMiddleware", () => {
  it("returns ok json from mounted route", async () => {
    const app = express();
    app.post(
      "/agntcy",
      agntcyDirMiddleware({ endpoint: "https://dir.example" }),
    );
    const res = await request(app).post("/agntcy");
    expect(res.status).toBe(200);
    expect(res.body).toEqual({ ok: true });
  });

  it("throws if endpoint missing", () => {
    expect(() => agntcyDirMiddleware({ endpoint: "" })).toThrow();
  });
});
