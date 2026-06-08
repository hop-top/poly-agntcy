#!/usr/bin/env bash
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
cd "$HERE"

ENDPOINT=${ENDPOINT:-http://localhost:8888}

for _ in {1..30}; do
  if curl -sf "$ENDPOINT/healthz" >/dev/null; then
    break
  fi
  sleep 1
done

(cd "$HERE/../.." && go run ./e2e/conformance/go-agent.go --endpoint "$ENDPOINT" register)

uv run "$HERE/py-agent.py" --endpoint "$ENDPOINT" discover --capability inventory

php "$HERE/php-agent.php" --endpoint "$ENDPOINT" send --target inventory-agent

echo "Cross-lang integration test passed."
