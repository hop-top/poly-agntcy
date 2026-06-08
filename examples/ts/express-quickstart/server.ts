import express from "express";
import { agntcyDirMiddleware } from "@hop-top/agntcy-dir-express";

const app = express();
app.use(express.json());

app.post(
  "/agntcy",
  agntcyDirMiddleware({
    endpoint: process.env.AGNTCY_ENDPOINT ?? "http://localhost:8888",
  }),
);

const port = Number(process.env.PORT ?? 3000);
app.listen(port, () => {
  console.log(`express-quickstart listening on :${port}`);
});
