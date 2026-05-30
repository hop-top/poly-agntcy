# ts / next-route

Next.js App Router route handler that exposes the DIR adapter as
`POST /api/agntcy`.

## Run

```sh
pnpm install
pnpm dev
curl -X POST http://localhost:3000/api/agntcy -d '{"capability":"inventory"}'
```
