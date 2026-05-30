# poly-agntcy Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stand up `hop-top/poly-agntcy` — 14 publishable packages across Go, Rust, PHP, TypeScript, and Python, all implementing the AGNTCY Directory Service (DIR) spec, with release-please + dotgithub publish pipeline wired end-to-end.

**Architecture:** Five language roots (`go/`, `rs/`, `php/`, `ts/`, `py/`) under a poly-kit-scaffolded polyglot repo. Go/Rust/PHP ship first-party DIR clients generated from buf BSR; TS/Python ship framework-adapter packages wrapping official `agntcy-dir` SDKs. Per-package release-please tracks emit `<component>/v<version>` tags consumed by `hop-top/.github/.github/workflows/publish-on-tag.yml@v0`.

**Tech Stack:** mise (toolchain SOT), buf (codegen from `buf.build/agntcy/dir` BSR), Go 1.23 + connect-go + cobra, Rust 1.83 + tonic + clap, PHP 8.2 + composer + Symfony Console, Node 22 + pnpm + commander, Python 3.12 + uv + typer, release-please, hop-top/.github reusable workflows.

**Reference spec:** [`docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`](../specs/2026-05-30-poly-agntcy-design.md)

**Reference SKILL:** [`hop-top/.github SKILL.md`](https://github.com/hop-top/.github/blob/main/SKILL.md)

---

## Phase guide

| Phase | Theme | Blocks |
|---|---|---|
| 1 | Repo scaffold + tooling (mise, devcontainer, .gitignore, license) | All later phases |
| 2 | Protobuf pipeline (buf.yaml, buf.gen.yaml, mise gen task) | Phases 3, 4, 5 |
| 3 | Go SDK (core + spiffe + CLI) | Phase 8 |
| 4 | Rust SDK (core + spiffe + CLI) | Phase 8 |
| 5 | PHP SDK (core + laravel + symfony + spiffe + CLI) | Phase 8 |
| 6 | TS adapters (next + hono + express + CLI) | Phase 8 |
| 7 | Python adapters (fastapi + flask + django + CLI) | Phase 8 |
| 8 | Cross-lang examples + integration tests | Phase 9 |
| 9 | Release pipeline (release-please, publish.yml, preflight, mirrors) | — |

Phases 3-7 can run in parallel after Phase 2 completes. Phase 8 needs all of 3-7. Phase 9 can start once Phase 1 finishes (it's metadata + workflows, no code deps).

---

# Phase 1 — Repo scaffold + tooling

Establishes the polyglot repo structure with poly-kit's scaffolder, then customizes for our 14-package layout.

## Task 1.1 — Run poly-kit scaffold

**Files:**
- Create: `mise.toml`, `.env.example`, `.devcontainer/devcontainer.json`, `.devcontainer/docker-compose.yml`, `.devcontainer/otel-config.yaml`, `.gitignore`, `.gitattributes`, `LICENSE`, `README.md` (skeleton), `Makefile`, plus per-lang dirs (`go/`, `rs/`, `php/`, `ts/`, `py/`)

- [ ] **Step 1: Verify poly-kit scaffold.sh exists at expected path**

```bash
ls /Users/jadb/.w/ideacrafterslabs/poly-kit/hops/main/templates/scaffold.sh
```

Expected: file listed.

- [ ] **Step 2: Run scaffold against the repo with all 5 langs**

Working dir: `/Users/jadb/.w/ideacrafterslabs/poly-agntcy`

```bash
bash /Users/jadb/.w/ideacrafterslabs/poly-kit/hops/main/templates/scaffold.sh app \
  --langs go,rs,php,ts,py \
  --name poly-agntcy \
  --org hop-top \
  --target /Users/jadb/.w/ideacrafterslabs/poly-agntcy
```

Expected output: emits `mise.toml`, `.devcontainer/`, `.env.example`, per-lang dirs, `.github/workflows/ci.yml` skeleton.

- [ ] **Step 3: Verify emitted files**

```bash
ls /Users/jadb/.w/ideacrafterslabs/poly-agntcy/
test -f mise.toml && test -d .devcontainer && test -d go && test -d rs && test -d php && test -d ts && test -d py && echo OK
```

Expected: `OK`.

- [ ] **Step 4: Commit scaffold output**

```bash
git add -A
git commit -m "chore: scaffold poly-agntcy repo via poly-kit"
```

Expected: one commit, clean tree.

## Task 1.2 — Pin mise versions per spec §14

**Files:**
- Modify: `mise.toml`

- [ ] **Step 1: Replace `mise.toml` `[tools]` block with pinned versions**

Replace existing `[tools]` block content with:

```toml
[tools]
go = "1.23"
rust = "1.83"
node = "22"
pnpm = "9"
python = "3.12"
uv = "latest"
php = "8.2"
composer = "latest"
buf = "1.47"
```

- [ ] **Step 2: Verify mise can resolve all tools**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy && mise install
```

Expected: each tool installs cleanly (may take several minutes on first run).

- [ ] **Step 3: Commit**

```bash
git add mise.toml
git commit -m "chore: pin mise tool versions"
```

## Task 1.3 — Apache-2.0 LICENSE

**Files:**
- Create: `LICENSE` (overwrite scaffold default if it's MIT)

- [ ] **Step 1: Verify LICENSE content**

```bash
head -1 /Users/jadb/.w/ideacrafterslabs/poly-agntcy/LICENSE
```

Expected: `Apache License`. If not, replace with the full Apache-2.0 text (https://www.apache.org/licenses/LICENSE-2.0.txt) substituting `Copyright 2026 Jad Bitar` on the rights line.

- [ ] **Step 2: Commit if changed**

```bash
git add LICENSE
git diff --cached --quiet || git commit -m "chore: switch LICENSE to Apache-2.0"
```

## Task 1.4 — README skeleton

**Files:**
- Create: `README.md`

- [ ] **Step 1: Write README**

Content:

````markdown
# poly-agntcy

Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR).

| Language | Status | Install |
|---|---|---|
| Go | First-party | `go get hop.top/agntcy` |
| Rust | First-party | `cargo add poly-agntcy-dir` |
| PHP | First-party | `composer require poly-agntcy/dir` |
| TypeScript | Adapters | `npm i @poly-agntcy/dir-next` (wraps official `agntcy-dir`) |
| Python | Adapters | `pip install poly-agntcy-dir-fastapi` (wraps official `agntcy-dir`) |

For TS and Python core DIR access, use AGNTCY's official SDKs:
- TS: [`agntcy/dir-sdk-javascript`](https://github.com/agntcy/dir-sdk-javascript)
- Py: [`agntcy/dir-sdk-python`](https://github.com/agntcy/dir-sdk-python)

## Design

See [`docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`](docs/superpowers/specs/2026-05-30-poly-agntcy-design.md).

## License

Apache-2.0.
````

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add README skeleton"
```

## Task 1.5 — Top-level mise tasks

**Files:**
- Modify: `mise.toml`

- [ ] **Step 1: Append `[tasks.*]` sections**

Append to `mise.toml`:

```toml
[tasks.install]
description = "Install all language dependencies"
run = [
  "mise install",
  "(cd rs && cargo fetch)",
  "(cd php && composer install --no-interaction)",
  "(cd ts && pnpm install)",
  "(cd py && uv sync)",
]

[tasks.gen]
description = "Regenerate protobuf stubs (go, rs, php)"
run = "buf generate proto"

[tasks.lint]
description = "Lint all languages"
depends = ["lint:go", "lint:rs", "lint:php", "lint:ts", "lint:py"]

["tasks.lint:go"]
dir = "go"
run = "golangci-lint run ./..."

["tasks.lint:rs"]
dir = "rs"
run = "cargo clippy --workspace --all-targets -- -D warnings"

["tasks.lint:php"]
dir = "php"
run = "vendor/bin/phpstan analyse"

["tasks.lint:ts"]
dir = "ts"
run = "pnpm -r lint"

["tasks.lint:py"]
dir = "py"
run = "uv run mypy --strict ."

[tasks.test]
description = "Test all languages"
depends = ["test:go", "test:rs", "test:php", "test:ts", "test:py"]

["tasks.test:go"]
dir = "go"
run = "go test ./..."

["tasks.test:rs"]
dir = "rs"
run = "cargo test --workspace"

["tasks.test:php"]
dir = "php"
run = "vendor/bin/phpunit"

["tasks.test:ts"]
dir = "ts"
run = "pnpm -r test"

["tasks.test:py"]
dir = "py"
run = "uv run pytest"

[tasks.cross]
description = "Cross-language integration test"
dir = "examples/cross"
run = "./run.sh"
```

- [ ] **Step 2: Verify mise lists tasks**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy && mise tasks
```

Expected: `install`, `gen`, `lint`, `test`, `cross` plus per-lang sub-tasks listed.

- [ ] **Step 3: Commit**

```bash
git add mise.toml
git commit -m "chore: add top-level mise tasks (install/gen/lint/test/cross)"
```

## Task 1.6 — CI workflow (`.github/workflows/ci.yml`)

**Files:**
- Create or modify: `.github/workflows/ci.yml` (scaffold may have emitted a skeleton)

- [ ] **Step 1: Write/overwrite ci.yml**

```yaml
name: ci
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  lint-test:
    strategy:
      fail-fast: false
      matrix:
        lang: [go, rs, php, ts, py]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: jdx/mise-action@v2
      - name: regen protobuf stubs
        if: matrix.lang == 'go' || matrix.lang == 'rs' || matrix.lang == 'php'
        run: mise run gen
      - name: lint
        run: mise run lint:${{ matrix.lang }}
      - name: test
        run: mise run test:${{ matrix.lang }}

  cross-lang:
    needs: lint-test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: jdx/mise-action@v2
      - name: bring up local DIR
        run: docker compose -f examples/cross/docker-compose.yml up -d
      - name: regen
        run: mise run gen
      - name: cross-lang integration
        run: mise run cross
      - name: tear down
        if: always()
        run: docker compose -f examples/cross/docker-compose.yml down
```

- [ ] **Step 2: Verify YAML parses**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy && yamllint .github/workflows/ci.yml 2>/dev/null || python -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add matrix lint+test workflow with cross-lang job"
```

---

# Phase 2 — Protobuf pipeline

Wire buf BSR consumption + per-language codegen.

## Task 2.1 — `proto/buf.yaml`

**Files:**
- Create: `proto/buf.yaml`
- Create: `proto/README.md`

- [ ] **Step 1: Write buf.yaml**

```yaml
version: v2
deps:
  - buf.build/agntcy/dir
```

- [ ] **Step 2: Write proto/README.md**

````markdown
# proto

Single source of truth for protobuf bindings.

We don't check in `.proto` files. Instead, `buf.yaml` declares a BSR module dependency on [`buf.build/agntcy/dir`](https://buf.build/agntcy/dir), pinned by `buf.lock`.

## Regenerate stubs

```sh
mise run gen
```

Generates Go, Rust, and PHP stubs into the respective language roots (gitignored). TS and Python don't need generation — they consume the official `agntcy-dir` SDK directly.

## Bump spec version

1. Edit `buf.yaml` dep version pin (or run `buf dep update` to refresh `buf.lock`)
2. `mise run gen`
3. Fix per-language breakage
4. Commit with `BREAKING CHANGE:` trailer if wire-incompatible
````

- [ ] **Step 3: Resolve BSR dep**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/proto && buf dep update
```

Expected: `buf.lock` written with digest pin.

- [ ] **Step 4: Commit**

```bash
git add proto/
git commit -m "feat: add proto/ with buf BSR ref to agntcy/dir"
```

## Task 2.2 — `proto/buf.gen.yaml`

**Files:**
- Create: `proto/buf.gen.yaml`

- [ ] **Step 1: Write buf.gen.yaml**

```yaml
version: v2
inputs:
  - module: buf.build/agntcy/dir
plugins:
  # Go
  - remote: buf.build/protocolbuffers/go
    out: ../go/internal/proto
    opt: paths=source_relative
  - remote: buf.build/connectrpc/go
    out: ../go/internal/proto
    opt: paths=source_relative
  # Rust
  - remote: buf.build/community/neoeinstein-prost
    out: ../rs/poly-agntcy-dir/src/proto
  - remote: buf.build/community/neoeinstein-tonic
    out: ../rs/poly-agntcy-dir/src/proto
  # PHP
  - remote: buf.build/protocolbuffers/php
    out: ../php/packages/dir/src/Generated
  - remote: buf.build/grpc/php
    out: ../php/packages/dir/src/Generated
```

- [ ] **Step 2: Smoke-test generation**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy && mise run gen
```

Expected: stubs land in `go/internal/proto/`, `rs/poly-agntcy-dir/src/proto/`, `php/packages/dir/src/Generated/`.

- [ ] **Step 3: Verify gitignored**

Check `go/.gitignore`, `rs/.gitignore`, `php/.gitignore` contain:
- `internal/proto/` (go)
- `src/proto/` (rs)
- `src/Generated/` (php)

If missing, add them. The lang-specific `.gitignore` files should be created/extended now, not in later phases.

- [ ] **Step 4: Commit**

```bash
git add proto/buf.gen.yaml go/.gitignore rs/.gitignore php/.gitignore
git commit -m "feat: configure buf codegen for go/rs/php"
```

---

# Phase 3 — Go SDK

Two Go modules: `hop.top/agntcy` (core + CLI) and `hop.top/agntcy/spiffe` (extension).

## Task 3.1 — Initialize Go module

**Files:**
- Create: `go/go.mod`, `go/.gitignore`

- [ ] **Step 1: Write `go/go.mod`**

```
module hop.top/agntcy

go 1.23
```

- [ ] **Step 2: Ensure `.gitignore` exists**

`go/.gitignore`:

```
internal/proto/
*.test
*.out
```

- [ ] **Step 3: Commit**

```bash
git add go/go.mod go/.gitignore
git commit -m "chore(go): init module hop.top/agntcy"
```

## Task 3.2 — Credentials interface

**Files:**
- Create: `go/dir/credentials.go`, `go/dir/credentials_test.go`

- [ ] **Step 1: Write failing test**

`go/dir/credentials_test.go`:

```go
package dir_test

import (
	"crypto/tls"
	"testing"

	"hop.top/agntcy/dir"
)

func TestInsecureCredentials(t *testing.T) {
	c := dir.InsecureCredentials{}
	if c.TLS() != nil {
		t.Fatalf("InsecureCredentials.TLS() must return nil")
	}
}

func TestTlsCredentials(t *testing.T) {
	c := dir.TlsCredentials{Config: &tls.Config{ServerName: "example"}}
	got := c.TLS()
	if got == nil || got.ServerName != "example" {
		t.Fatalf("TlsCredentials.TLS() must return underlying config")
	}
}
```

- [ ] **Step 2: Run test, expect failure**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/go && go test ./dir/...
```

Expected: compile error (Credentials/InsecureCredentials/TlsCredentials not defined).

- [ ] **Step 3: Implement**

`go/dir/credentials.go`:

```go
package dir

import "crypto/tls"

// Credentials abstracts the TLS posture used by the DIR client.
//
// Two stock impls (InsecureCredentials, TlsCredentials) ship in this
// package. SPIFFE-backed credentials live in hop.top/agntcy/spiffe.
type Credentials interface {
	TLS() *tls.Config
}

// InsecureCredentials disables TLS. Dev/test only.
type InsecureCredentials struct{}

func (InsecureCredentials) TLS() *tls.Config { return nil }

// TlsCredentials carries a stdlib *tls.Config.
type TlsCredentials struct {
	Config *tls.Config
}

func (c TlsCredentials) TLS() *tls.Config { return c.Config }
```

- [ ] **Step 4: Run test, expect pass**

```bash
go test ./dir/...
```

Expected: `PASS`.

- [ ] **Step 5: Commit**

```bash
git add go/dir/credentials.go go/dir/credentials_test.go
git commit -m "feat(go): add Credentials interface (Insecure + Tls impls)"
```

## Task 3.3 — DIR client skeleton

**Files:**
- Create: `go/dir/client.go`, `go/dir/client_test.go`

- [ ] **Step 1: Run codegen so stubs exist**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy && mise run gen
```

Expected: `go/internal/proto/` populated.

- [ ] **Step 2: Add connect-go + grpc deps**

```bash
cd go
go get connectrpc.com/connect@latest
go get google.golang.org/grpc@latest
go mod tidy
```

- [ ] **Step 3: Write failing test**

`go/dir/client_test.go`:

```go
package dir_test

import (
	"testing"

	"hop.top/agntcy/dir"
)

func TestNewClient_RequiresEndpoint(t *testing.T) {
	_, err := dir.NewClient(dir.Options{})
	if err == nil {
		t.Fatalf("expected error when Endpoint is empty")
	}
}

func TestNewClient_OK(t *testing.T) {
	c, err := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if c == nil {
		t.Fatalf("client must not be nil")
	}
}
```

- [ ] **Step 4: Run, expect failure**

```bash
go test ./dir/...
```

Expected: compile error.

- [ ] **Step 5: Implement**

`go/dir/client.go`:

```go
package dir

import (
	"errors"
	"net/http"

	"connectrpc.com/connect"
)

// Options configures a Client.
type Options struct {
	Endpoint    string
	Credentials Credentials
	HTTPClient  *http.Client
}

// Client is a DIR API client.
type Client struct {
	opts Options
	http *http.Client
}

// NewClient constructs a DIR client.
func NewClient(opts Options) (*Client, error) {
	if opts.Endpoint == "" {
		return nil, errors.New("dir: Endpoint required")
	}
	if opts.Credentials == nil {
		opts.Credentials = InsecureCredentials{}
	}
	httpc := opts.HTTPClient
	if httpc == nil {
		httpc = &http.Client{}
	}
	if tlsCfg := opts.Credentials.TLS(); tlsCfg != nil {
		if t, ok := httpc.Transport.(*http.Transport); ok {
			t.TLSClientConfig = tlsCfg
		} else {
			httpc.Transport = &http.Transport{TLSClientConfig: tlsCfg}
		}
	}
	_ = connect.Code(0) // keep import live until rpc methods land
	return &Client{opts: opts, http: httpc}, nil
}
```

- [ ] **Step 6: Run, expect pass**

```bash
go test ./dir/...
```

Expected: `PASS`.

- [ ] **Step 7: Commit**

```bash
git add go/dir/client.go go/dir/client_test.go go/go.mod go/go.sum
git commit -m "feat(go): add DIR client skeleton with Options"
```

## Task 3.4 — DIR Register / Discover / Describe / Publish / Verify methods

**Files:**
- Modify: `go/dir/client.go`
- Create: `go/dir/methods_test.go`

- [ ] **Step 1: Inspect generated stubs to find the connect-go service handle**

```bash
ls /Users/jadb/.w/ideacrafterslabs/poly-agntcy/go/internal/proto/
grep -r "ServiceClient" go/internal/proto/ | head -5
```

Identify the generated connect client (e.g. `dirv1connect.DirectoryServiceClient` or similar; exact name depends on AGNTCY's `.proto`). Capture it.

- [ ] **Step 2: Write failing test using a roundtripper that returns a canned response**

`go/dir/methods_test.go`:

```go
package dir_test

import (
	"bytes"
	"context"
	"io"
	"net/http"
	"testing"

	"hop.top/agntcy/dir"
)

type fakeRT struct {
	body []byte
}

func (f fakeRT) RoundTrip(r *http.Request) (*http.Response, error) {
	return &http.Response{
		StatusCode: 200,
		Body:       io.NopCloser(bytes.NewReader(f.body)),
		Header:     http.Header{"Content-Type": []string{"application/proto"}},
	}, nil
}

func TestClient_Discover_RoundtripsThroughHTTPClient(t *testing.T) {
	c, err := dir.NewClient(dir.Options{
		Endpoint:    "https://dir.example",
		Credentials: dir.InsecureCredentials{},
		HTTPClient:  &http.Client{Transport: fakeRT{body: []byte{}}},
	})
	if err != nil {
		t.Fatalf("NewClient: %v", err)
	}
	// Smoke: just check that Discover does not panic with empty input.
	// Detailed wire tests live in cross-lang integration (Phase 8).
	_, _ = c.Discover(context.Background(), dir.DiscoverParams{})
	// Pass if no panic; nil-error not asserted (fake body is empty).
}
```

- [ ] **Step 3: Run, expect failure**

```bash
go test ./dir/...
```

Expected: compile error (`Discover` / `DiscoverParams` undefined).

- [ ] **Step 4: Implement methods**

Append to `go/dir/client.go` (substitute exact generated package name & method signatures discovered in Step 1):

```go
import (
	// ... existing imports ...
	dirv1connect "hop.top/agntcy/internal/proto/agntcy/dir/v1/dirv1connect"
	dirv1 "hop.top/agntcy/internal/proto/agntcy/dir/v1"
)

// DiscoverParams is the input to Discover.
type DiscoverParams struct {
	Capability string
}

// DiscoverResult is the output of Discover.
type DiscoverResult struct {
	Agents []*dirv1.AgentDescriptor
}

// Discover queries the DIR for agents matching capability.
func (c *Client) Discover(ctx context.Context, p DiscoverParams) (*DiscoverResult, error) {
	svc := dirv1connect.NewDirectoryServiceClient(c.http, c.opts.Endpoint)
	req := connect.NewRequest(&dirv1.DiscoverRequest{Capability: p.Capability})
	resp, err := svc.Discover(ctx, req)
	if err != nil {
		return nil, err
	}
	return &DiscoverResult{Agents: resp.Msg.GetAgents()}, nil
}

// (Repeat the same pattern for Register, Describe, Publish, Verify —
// each maps to one RPC on the generated DirectoryServiceClient.
// Exact field names per AGNTCY's dir.proto.)
```

If the generated service name differs from `DirectoryServiceClient`, adjust accordingly. If the spec has different RPC names (e.g. `RegisterAgent` instead of `Register`), use the actual generated names but expose Go-idiomatic wrapper names.

- [ ] **Step 5: Run, expect pass**

```bash
go test ./dir/...
```

Expected: `PASS`.

- [ ] **Step 6: Commit**

```bash
git add go/dir/
git commit -m "feat(go): add Register/Discover/Describe/Publish/Verify methods"
```

## Task 3.5 — Go CLI (`cmd/agntcy/`)

**Files:**
- Create: `go/cmd/agntcy/main.go`, `go/cmd/agntcy/root.go`, `go/cmd/agntcy/discover.go`, `go/cmd/agntcy/register.go`, `go/cmd/agntcy/describe.go`, `go/cmd/agntcy/publish.go`, `go/cmd/agntcy/verify.go`, `go/cmd/agntcy/main_test.go`

- [ ] **Step 1: Add cobra dep**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/go && go get github.com/spf13/cobra@latest && go mod tidy
```

- [ ] **Step 2: Write failing smoke test**

`go/cmd/agntcy/main_test.go`:

```go
package main

import (
	"bytes"
	"testing"
)

func TestRootHelp(t *testing.T) {
	cmd := newRootCmd()
	cmd.SetArgs([]string{"--help"})
	var buf bytes.Buffer
	cmd.SetOut(&buf)
	if err := cmd.Execute(); err != nil {
		t.Fatalf("execute: %v", err)
	}
	if !bytes.Contains(buf.Bytes(), []byte("register")) {
		t.Fatalf("help must mention register: %s", buf.String())
	}
}
```

- [ ] **Step 3: Run, expect failure**

```bash
go test ./cmd/...
```

Expected: compile error.

- [ ] **Step 4: Implement root + subcommands**

`go/cmd/agntcy/main.go`:

```go
package main

import (
	"fmt"
	"os"
)

func main() {
	if err := newRootCmd().Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
```

`go/cmd/agntcy/root.go`:

```go
package main

import "github.com/spf13/cobra"

func newRootCmd() *cobra.Command {
	root := &cobra.Command{
		Use:   "agntcy",
		Short: "AGNTCY Directory Service CLI",
	}
	root.PersistentFlags().String("endpoint", "", "DIR endpoint URL")
	root.AddCommand(
		newRegisterCmd(),
		newDiscoverCmd(),
		newDescribeCmd(),
		newPublishCmd(),
		newVerifyCmd(),
	)
	return root
}
```

`go/cmd/agntcy/discover.go` (template — repeat similarly for register/describe/publish/verify):

```go
package main

import (
	"context"
	"fmt"

	"github.com/spf13/cobra"
	"hop.top/agntcy/dir"
)

func newDiscoverCmd() *cobra.Command {
	var capability string
	c := &cobra.Command{
		Use:   "discover",
		Short: "Discover agents by capability",
		RunE: func(cmd *cobra.Command, args []string) error {
			endpoint, _ := cmd.Flags().GetString("endpoint")
			client, err := dir.NewClient(dir.Options{
				Endpoint:    endpoint,
				Credentials: dir.InsecureCredentials{},
			})
			if err != nil {
				return err
			}
			res, err := client.Discover(context.Background(), dir.DiscoverParams{Capability: capability})
			if err != nil {
				return err
			}
			for _, a := range res.Agents {
				fmt.Println(a.GetName())
			}
			return nil
		},
	}
	c.Flags().StringVar(&capability, "capability", "", "Capability to search for")
	return c
}
```

Create `register.go`, `describe.go`, `publish.go`, `verify.go` following the same pattern. Each is ~30 lines, calls the corresponding `Client.<Method>`, prints output.

- [ ] **Step 5: Run, expect pass**

```bash
go test ./cmd/...
```

Expected: `PASS`.

- [ ] **Step 6: Verify CLI builds**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/go && go build -o /tmp/agntcy ./cmd/agntcy && /tmp/agntcy --help
```

Expected: help text listing `register`, `discover`, `describe`, `publish`, `verify`.

- [ ] **Step 7: Commit**

```bash
git add go/cmd/ go/go.mod go/go.sum
git commit -m "feat(go): add agntcy CLI (register/discover/describe/publish/verify)"
```

## Task 3.6 — SPIFFE submodule (`go/spiffe/`)

**Files:**
- Create: `go/spiffe/go.mod`, `go/spiffe/credentials.go`, `go/spiffe/credentials_test.go`

- [ ] **Step 1: Init module**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/go/spiffe
go mod init hop.top/agntcy/spiffe
```

`go/spiffe/go.mod` should now declare `module hop.top/agntcy/spiffe`.

- [ ] **Step 2: Add deps**

```bash
go get hop.top/agntcy@v0.0.0-00010101000000-000000000000
go get github.com/spiffe/go-spiffe/v2@latest
```

Then in `go/spiffe/go.mod`, add a `replace` directive pointing at the parent path:

```
replace hop.top/agntcy => ../
```

Run `go mod tidy`.

- [ ] **Step 3: Write failing test**

`go/spiffe/credentials_test.go`:

```go
package spiffe_test

import (
	"testing"

	"hop.top/agntcy/dir"
	"hop.top/agntcy/spiffe"
)

func TestCredentials_ImplementsInterface(t *testing.T) {
	var _ dir.Credentials = (*spiffe.Credentials)(nil)
}
```

- [ ] **Step 4: Run, expect failure**

```bash
cd go/spiffe && go test ./...
```

Expected: compile error.

- [ ] **Step 5: Implement**

`go/spiffe/credentials.go`:

```go
// Package spiffe provides SPIFFE-backed Credentials for the DIR client.
package spiffe

import (
	"crypto/tls"

	"github.com/spiffe/go-spiffe/v2/workloadapi"
)

// Credentials carries a SPIFFE X509Source and produces a TLS config that
// trusts SPIFFE peers in the same trust domain.
type Credentials struct {
	Source *workloadapi.X509Source
}

// TLS returns a *tls.Config that uses the SPIFFE source for client certs.
func (c *Credentials) TLS() *tls.Config {
	if c == nil || c.Source == nil {
		return nil
	}
	return &tls.Config{
		GetClientCertificate: func(*tls.CertificateRequestInfo) (*tls.Certificate, error) {
			svid, err := c.Source.GetX509SVID()
			if err != nil {
				return nil, err
			}
			return &tls.Certificate{
				Certificate: rawChain(svid.Certificates),
				PrivateKey:  svid.PrivateKey,
			}, nil
		},
		InsecureSkipVerify: false,
	}
}

func rawChain(certs []interface{ Raw() []byte }) [][]byte {
	out := make([][]byte, len(certs))
	for i, c := range certs {
		out[i] = c.Raw()
	}
	return out
}
```

(Adjust if the `go-spiffe` v2 API differs from this sketch — verify against pkg.go.dev before committing.)

- [ ] **Step 6: Run, expect pass**

```bash
go test ./...
```

Expected: `PASS`.

- [ ] **Step 7: Commit**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy
git add go/spiffe/
git commit -m "feat(go-spiffe): add SPIFFE-backed Credentials"
```

---

# Phase 4 — Rust SDK

Workspace with three crates: `poly-agntcy-dir` (lib), `poly-agntcy-dir-spiffe` (lib), `poly-agntcy` (bin).

## Task 4.1 — Initialize workspace

**Files:**
- Create: `rs/Cargo.toml`, `rs/.gitignore`

- [ ] **Step 1: Write workspace manifest**

`rs/Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = [
  "poly-agntcy-dir",
  "poly-agntcy-dir-spiffe",
  "poly-agntcy",
]

[workspace.package]
version = "0.1.0-alpha.0"
edition = "2021"
license = "Apache-2.0"
repository = "https://github.com/hop-top/poly-agntcy"
```

- [ ] **Step 2: Write `.gitignore`**

`rs/.gitignore`:

```
target/
poly-agntcy-dir/src/proto/
```

- [ ] **Step 3: Commit**

```bash
git add rs/Cargo.toml rs/.gitignore
git commit -m "chore(rs): init workspace"
```

## Task 4.2 — `poly-agntcy-dir` crate (core)

**Files:**
- Create: `rs/poly-agntcy-dir/Cargo.toml`, `rs/poly-agntcy-dir/src/lib.rs`, `rs/poly-agntcy-dir/src/credentials.rs`, `rs/poly-agntcy-dir/src/client.rs`, `rs/poly-agntcy-dir/tests/smoke.rs`

- [ ] **Step 1: Cargo manifest**

`rs/poly-agntcy-dir/Cargo.toml`:

```toml
[package]
name = "poly-agntcy-dir"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "AGNTCY Directory Service SDK for Rust"

[dependencies]
tonic = "0.12"
prost = "0.13"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
rustls = "0.23"
thiserror = "2"

[build-dependencies]
# Stubs come from `mise run gen`; no build.rs codegen.
```

- [ ] **Step 2: Generate stubs**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy && mise run gen
```

Expected: `rs/poly-agntcy-dir/src/proto/` populated.

- [ ] **Step 3: Write failing test**

`rs/poly-agntcy-dir/tests/smoke.rs`:

```rust
use poly_agntcy_dir::{Credentials, InsecureCredentials, TlsCredentials};

#[test]
fn insecure_credentials_returns_no_tls() {
    let c = InsecureCredentials;
    assert!(c.tls().is_none());
}

#[test]
fn tls_credentials_returns_config() {
    let cfg = rustls::ClientConfig::builder().with_no_client_auth();
    let _ = TlsCredentials { config: cfg };
}
```

(Adjust rustls builder per actual 0.23 API.)

- [ ] **Step 4: Run, expect failure**

```bash
cd rs && cargo test -p poly-agntcy-dir
```

Expected: compile error.

- [ ] **Step 5: Implement lib**

`rs/poly-agntcy-dir/src/lib.rs`:

```rust
//! AGNTCY Directory Service SDK.

pub mod credentials;
pub mod client;
pub mod proto;

pub use credentials::{Credentials, InsecureCredentials, TlsCredentials};
pub use client::{Client, Options};
```

`rs/poly-agntcy-dir/src/credentials.rs`:

```rust
use std::sync::Arc;

/// Abstracts the TLS posture used by [`crate::client::Client`].
pub trait Credentials: Send + Sync {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>>;
}

/// No TLS. Dev/test only.
pub struct InsecureCredentials;

impl Credentials for InsecureCredentials {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>> {
        None
    }
}

/// Carries a rustls ClientConfig.
pub struct TlsCredentials {
    pub config: rustls::ClientConfig,
}

impl Credentials for TlsCredentials {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>> {
        Some(Arc::new(self.config.clone()))
    }
}
```

`rs/poly-agntcy-dir/src/client.rs`:

```rust
use std::sync::Arc;
use crate::Credentials;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Endpoint required")]
    EndpointRequired,
    #[error("transport: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("status: {0}")]
    Status(#[from] tonic::Status),
}

pub struct Options {
    pub endpoint: String,
    pub credentials: Arc<dyn Credentials>,
}

pub struct Client {
    opts: Options,
}

impl Client {
    pub fn new(opts: Options) -> Result<Self, Error> {
        if opts.endpoint.is_empty() {
            return Err(Error::EndpointRequired);
        }
        Ok(Self { opts })
    }

    // discover/register/describe/publish/verify methods land in a follow-up
    // task once the generated tonic client signature is known.
}
```

`rs/poly-agntcy-dir/src/proto/mod.rs`:

```rust
//! Generated protobuf bindings. Gitignored; produced by `mise run gen`.
//!
//! Exact module structure depends on AGNTCY's dir.proto namespace.
include!("agntcy.dir.v1.rs");
```

(Adjust `include!` path based on what `protoc-gen-prost` emits — may be `mod.rs` or per-namespace files.)

- [ ] **Step 6: Run, expect pass**

```bash
cargo test -p poly-agntcy-dir
```

Expected: `PASS` (or at least compiles + smoke tests pass).

- [ ] **Step 7: Commit**

```bash
git add rs/poly-agntcy-dir/
git commit -m "feat(rs): add poly-agntcy-dir crate with Credentials + Client skeleton"
```

## Task 4.3 — Rust DIR methods

**Files:**
- Modify: `rs/poly-agntcy-dir/src/client.rs`
- Create: `rs/poly-agntcy-dir/tests/methods.rs`

- [ ] **Step 1: Inspect generated tonic client**

```bash
grep -r "Client" rs/poly-agntcy-dir/src/proto/ | head -5
```

Identify the generated tonic client type (e.g. `directory_service_client::DirectoryServiceClient<Channel>`). Capture it.

- [ ] **Step 2: Implement methods**

Append to `rs/poly-agntcy-dir/src/client.rs` (substitute generated names):

```rust
use crate::proto::directory_service_client::DirectoryServiceClient;
use crate::proto::{DiscoverRequest, AgentDescriptor};
use tonic::transport::{Channel, ClientTlsConfig};

pub struct DiscoverParams {
    pub capability: String,
}

pub struct DiscoverResult {
    pub agents: Vec<AgentDescriptor>,
}

impl Client {
    async fn channel(&self) -> Result<Channel, Error> {
        let mut endpoint = Channel::from_shared(self.opts.endpoint.clone())
            .map_err(|_| Error::EndpointRequired)?;
        if let Some(_tls) = self.opts.credentials.tls() {
            let tls = ClientTlsConfig::new(); // wire rustls cfg per tonic-0.12 API
            endpoint = endpoint.tls_config(tls)?;
        }
        Ok(endpoint.connect().await?)
    }

    pub async fn discover(&self, p: DiscoverParams) -> Result<DiscoverResult, Error> {
        let ch = self.channel().await?;
        let mut c = DirectoryServiceClient::new(ch);
        let resp = c.discover(DiscoverRequest { capability: p.capability }).await?;
        Ok(DiscoverResult { agents: resp.into_inner().agents })
    }

    // Repeat for register, describe, publish, verify.
}
```

- [ ] **Step 3: Write test**

`rs/poly-agntcy-dir/tests/methods.rs`:

```rust
use poly_agntcy_dir::{Client, Options, InsecureCredentials};
use std::sync::Arc;

#[test]
fn new_client_requires_endpoint() {
    let r = Client::new(Options {
        endpoint: String::new(),
        credentials: Arc::new(InsecureCredentials),
    });
    assert!(r.is_err());
}
```

- [ ] **Step 4: Run + commit**

```bash
cargo test -p poly-agntcy-dir
git add rs/poly-agntcy-dir/
git commit -m "feat(rs): add DIR client methods (discover/register/describe/publish/verify)"
```

## Task 4.4 — `poly-agntcy-dir-spiffe` crate

**Files:**
- Create: `rs/poly-agntcy-dir-spiffe/Cargo.toml`, `rs/poly-agntcy-dir-spiffe/src/lib.rs`

- [ ] **Step 1: Cargo manifest**

```toml
[package]
name = "poly-agntcy-dir-spiffe"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "SPIFFE-backed credentials for poly-agntcy-dir"

[dependencies]
poly-agntcy-dir = { path = "../poly-agntcy-dir", version = "0.1.0-alpha.0" }
spiffe = "0.6"
tokio = { version = "1", features = ["rt-multi-thread"] }
rustls = "0.23"
```

(Check `spiffe` crate version on crates.io — it's evolving; pin to the latest stable minor.)

- [ ] **Step 2: Implement**

`rs/poly-agntcy-dir-spiffe/src/lib.rs`:

```rust
//! SPIFFE-backed Credentials for poly-agntcy-dir.

use std::sync::Arc;
use poly_agntcy_dir::Credentials;
use spiffe::workload_api::client::WorkloadApiClient;

pub struct SpiffeCredentials {
    inner: Arc<WorkloadApiClient>,
}

impl SpiffeCredentials {
    pub async fn from_default_socket() -> Result<Self, spiffe::workload_api::client::ClientError> {
        let inner = WorkloadApiClient::default().await?;
        Ok(Self { inner: Arc::new(inner) })
    }
}

impl Credentials for SpiffeCredentials {
    fn tls(&self) -> Option<Arc<rustls::ClientConfig>> {
        // Production impl: fetch X509 SVIDs, build rustls config with
        // SPIFFE peer verifier. See spiffe-rustls integration.
        // For day-one ship: return None and have user pre-build rustls cfg.
        None
    }
}
```

(This is a deliberate v0 stub — full SPIFFE-rustls integration is a follow-on. The crate exists to claim the package name + provide the interface; full impl tracked as a separate task post-bootstrap.)

- [ ] **Step 3: Build + commit**

```bash
cd rs && cargo build -p poly-agntcy-dir-spiffe
git add rs/poly-agntcy-dir-spiffe/
git commit -m "feat(rs-spiffe): add SPIFFE credentials crate (v0 stub)"
```

## Task 4.5 — Rust CLI (`poly-agntcy` bin crate)

**Files:**
- Create: `rs/poly-agntcy/Cargo.toml`, `rs/poly-agntcy/src/main.rs`, `rs/poly-agntcy/src/cli.rs`

- [ ] **Step 1: Cargo manifest**

```toml
[package]
name = "poly-agntcy"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "AGNTCY Directory Service CLI (Rust)"

[[bin]]
name = "agntcy"
path = "src/main.rs"

[dependencies]
poly-agntcy-dir = { path = "../poly-agntcy-dir", version = "0.1.0-alpha.0" }
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
anyhow = "1"
```

- [ ] **Step 2: Write CLI**

`rs/poly-agntcy/src/main.rs`:

```rust
use clap::Parser;

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    cli.run().await
}
```

`rs/poly-agntcy/src/cli.rs`:

```rust
use clap::{Parser, Subcommand};
use poly_agntcy_dir::{Client, Options, InsecureCredentials, DiscoverParams};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "agntcy")]
pub struct Cli {
    #[arg(long)]
    endpoint: String,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Register { #[arg(long)] descriptor: String },
    Discover { #[arg(long)] capability: String },
    Describe { #[arg(long)] name: String },
    Publish { #[arg(long)] descriptor: String },
    Verify { #[arg(long)] envelope: String },
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        let client = Client::new(Options {
            endpoint: self.endpoint,
            credentials: Arc::new(InsecureCredentials),
        })?;
        match self.cmd {
            Cmd::Discover { capability } => {
                let r = client.discover(DiscoverParams { capability }).await?;
                for a in r.agents {
                    println!("{}", a.name);
                }
            }
            // Repeat for register/describe/publish/verify when methods land.
            _ => anyhow::bail!("not yet implemented"),
        }
        Ok(())
    }
}
```

- [ ] **Step 3: Build + commit**

```bash
cd rs && cargo build -p poly-agntcy
git add rs/poly-agntcy/
git commit -m "feat(rs): add agntcy CLI bin crate"
```

---

# Phase 5 — PHP SDK

Five Composer packages under `php/packages/`: `dir`, `dir-laravel`, `dir-symfony`, `dir-spiffe`, `cli` (CLI bin ships inside `dir` per spec §6 amendment).

## Task 5.1 — Root composer.json + path repos

**Files:**
- Create: `php/composer.json`, `php/.gitignore`

- [ ] **Step 1: Root composer.json**

`php/composer.json`:

```json
{
  "name": "poly-agntcy/php-workspace",
  "type": "project",
  "license": "Apache-2.0",
  "require": {
    "php": "^8.2"
  },
  "repositories": [
    { "type": "path", "url": "packages/dir" },
    { "type": "path", "url": "packages/dir-laravel" },
    { "type": "path", "url": "packages/dir-symfony" },
    { "type": "path", "url": "packages/dir-spiffe" }
  ],
  "require-dev": {
    "phpunit/phpunit": "^11",
    "phpstan/phpstan": "^2"
  },
  "config": {
    "sort-packages": true
  }
}
```

- [ ] **Step 2: `.gitignore`**

```
vendor/
composer.lock
packages/*/vendor/
packages/dir/src/Generated/
```

- [ ] **Step 3: Commit**

```bash
git add php/composer.json php/.gitignore
git commit -m "chore(php): init workspace composer.json with path repos"
```

## Task 5.2 — `packages/dir/` core

**Files:**
- Create: `php/packages/dir/composer.json`, `php/packages/dir/src/Credentials.php`, `php/packages/dir/src/InsecureCredentials.php`, `php/packages/dir/src/TlsCredentials.php`, `php/packages/dir/src/Client.php`, `php/packages/dir/tests/CredentialsTest.php`, `php/packages/dir/bin/agntcy`

- [ ] **Step 1: composer.json**

```json
{
  "name": "poly-agntcy/dir",
  "type": "library",
  "description": "AGNTCY Directory Service SDK + CLI for PHP",
  "license": "Apache-2.0",
  "require": {
    "php": "^8.2",
    "psr/http-client": "^1.0",
    "psr/http-message": "^1.0|^2.0",
    "psr/log": "^3.0",
    "symfony/console": "^6.4|^7.0"
  },
  "autoload": {
    "psr-4": {
      "PolyAgntcy\\Dir\\": "src/",
      "PolyAgntcy\\Dir\\Generated\\": "src/Generated/"
    }
  },
  "autoload-dev": {
    "psr-4": {
      "PolyAgntcy\\Dir\\Tests\\": "tests/"
    }
  },
  "bin": ["bin/agntcy"]
}
```

- [ ] **Step 2: Credentials interface**

`php/packages/dir/src/Credentials.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir;

interface Credentials
{
    /**
     * Returns an array of TLS stream-context options, or null for plaintext.
     *
     * Keys match PHP's stream context "ssl" options.
     */
    public function tlsOptions(): ?array;
}
```

`php/packages/dir/src/InsecureCredentials.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir;

final class InsecureCredentials implements Credentials
{
    public function tlsOptions(): ?array
    {
        return null;
    }
}
```

`php/packages/dir/src/TlsCredentials.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir;

final class TlsCredentials implements Credentials
{
    /**
     * @param array<string,mixed> $options
     */
    public function __construct(private readonly array $options) {}

    public function tlsOptions(): ?array
    {
        return $this->options;
    }
}
```

- [ ] **Step 3: Test**

`php/packages/dir/tests/CredentialsTest.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Tests;

use PHPUnit\Framework\TestCase;
use PolyAgntcy\Dir\InsecureCredentials;
use PolyAgntcy\Dir\TlsCredentials;

final class CredentialsTest extends TestCase
{
    public function testInsecureReturnsNull(): void
    {
        $this->assertNull((new InsecureCredentials())->tlsOptions());
    }

    public function testTlsReturnsOptions(): void
    {
        $opts = ['verify_peer' => true];
        $this->assertSame($opts, (new TlsCredentials($opts))->tlsOptions());
    }
}
```

- [ ] **Step 4: Run tests**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/php && composer install --no-interaction && vendor/bin/phpunit packages/dir/tests
```

Expected: 2 passing tests.

- [ ] **Step 5: Client skeleton**

`php/packages/dir/src/Client.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir;

use Psr\Http\Client\ClientInterface;

final class Client
{
    public function __construct(
        private readonly string $endpoint,
        private readonly Credentials $credentials,
        private readonly ?ClientInterface $http = null,
    ) {
        if ($endpoint === '') {
            throw new \InvalidArgumentException('endpoint required');
        }
    }

    public function discover(string $capability): array
    {
        // Wire to Connect-over-HTTP/1.1 against generated stubs in src/Generated/.
        // Full impl in Task 5.3.
        return [];
    }

    // register/describe/publish/verify follow Task 5.3.
}
```

- [ ] **Step 6: CLI bin**

`php/packages/dir/bin/agntcy`:

```php
#!/usr/bin/env php
<?php
declare(strict_types=1);

require_once dirname(__DIR__).'/vendor/autoload.php';

use Symfony\Component\Console\Application;
use PolyAgntcy\Dir\Cli\DiscoverCommand;

$app = new Application('agntcy', '0.1.0-alpha.0');
$app->add(new DiscoverCommand());
// register/describe/publish/verify commands added similarly.
$app->run();
```

`php/packages/dir/src/Cli/DiscoverCommand.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Cli;

use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\OutputInterface;
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;

#[AsCommand(name: 'discover')]
final class DiscoverCommand extends Command
{
    protected function configure(): void
    {
        $this->addOption('endpoint', null, InputOption::VALUE_REQUIRED, 'DIR endpoint');
        $this->addOption('capability', null, InputOption::VALUE_REQUIRED, 'Capability');
    }

    protected function execute(InputInterface $in, OutputInterface $out): int
    {
        $client = new Client(
            (string) $in->getOption('endpoint'),
            new InsecureCredentials(),
        );
        foreach ($client->discover((string) $in->getOption('capability')) as $a) {
            $out->writeln($a->name);
        }
        return Command::SUCCESS;
    }
}
```

- [ ] **Step 7: Smoke + commit**

```bash
cd php && composer install --no-interaction
chmod +x packages/dir/bin/agntcy
git add php/packages/dir/
git commit -m "feat(php): add poly-agntcy/dir package (Credentials + Client + CLI)"
```

## Task 5.3 — PHP DIR wire methods

Defer until `mise run gen` produces stable PHP stubs and we know the exact namespace. Steps:

- [ ] **Step 1: Run gen, inspect stub namespace**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy && mise run gen
ls php/packages/dir/src/Generated/
```

- [ ] **Step 2: Wire `Client::discover/register/describe/publish/verify`** using either:
  - `grpc/grpc` PHP extension (preferred when available), OR
  - Pure-PHP Connect protocol over PSR-18 (fallback) — Buf provides `connect-php` library

Pick one based on what scaffolds cleanly. Document the choice in `php/packages/dir/README.md`.

- [ ] **Step 3: Add wire-level tests** behind a `@group integration` marker (skipped without `DIR_ENDPOINT` env)

- [ ] **Step 4: Commit**

```bash
git add php/packages/dir/
git commit -m "feat(php): wire DIR client methods to generated stubs"
```

## Task 5.4 — `packages/dir-laravel/`

**Files:**
- Create: `php/packages/dir-laravel/composer.json`, `php/packages/dir-laravel/src/AgntcyServiceProvider.php`, `php/packages/dir-laravel/config/agntcy.php`, `php/packages/dir-laravel/src/Facades/Agntcy.php`

- [ ] **Step 1: composer.json**

```json
{
  "name": "poly-agntcy/dir-laravel",
  "type": "library",
  "description": "Laravel integration for poly-agntcy/dir",
  "license": "Apache-2.0",
  "require": {
    "php": "^8.2",
    "poly-agntcy/dir": "*",
    "illuminate/support": "^11.0|^12.0",
    "illuminate/contracts": "^11.0|^12.0"
  },
  "autoload": {
    "psr-4": { "PolyAgntcy\\Dir\\Laravel\\": "src/" }
  },
  "extra": {
    "laravel": {
      "providers": [
        "PolyAgntcy\\Dir\\Laravel\\AgntcyServiceProvider"
      ],
      "aliases": {
        "Agntcy": "PolyAgntcy\\Dir\\Laravel\\Facades\\Agntcy"
      }
    }
  }
}
```

- [ ] **Step 2: ServiceProvider**

`php/packages/dir-laravel/src/AgntcyServiceProvider.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Laravel;

use Illuminate\Support\ServiceProvider;
use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;

final class AgntcyServiceProvider extends ServiceProvider
{
    public function register(): void
    {
        $this->mergeConfigFrom(__DIR__.'/../config/agntcy.php', 'agntcy');

        $this->app->singleton(Client::class, function ($app) {
            $cfg = $app['config']['agntcy'];
            return new Client(
                $cfg['endpoint'],
                new InsecureCredentials(),
            );
        });
    }

    public function boot(): void
    {
        $this->publishes([
            __DIR__.'/../config/agntcy.php' => config_path('agntcy.php'),
        ], 'agntcy-config');
    }
}
```

`php/packages/dir-laravel/config/agntcy.php`:

```php
<?php
declare(strict_types=1);

return [
    'endpoint' => env('AGNTCY_DIR_ENDPOINT', 'https://directory.example'),
    'timeout' => (int) env('AGNTCY_TIMEOUT', 5),
];
```

`php/packages/dir-laravel/src/Facades/Agntcy.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Laravel\Facades;

use Illuminate\Support\Facades\Facade;
use PolyAgntcy\Dir\Client;

final class Agntcy extends Facade
{
    protected static function getFacadeAccessor(): string
    {
        return Client::class;
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add php/packages/dir-laravel/
git commit -m "feat(php-laravel): add Laravel ServiceProvider + facade"
```

## Task 5.5 — `packages/dir-symfony/`

**Files:**
- Create: `php/packages/dir-symfony/composer.json`, `php/packages/dir-symfony/src/AgntcyBundle.php`, `php/packages/dir-symfony/src/DependencyInjection/AgntcyExtension.php`, `php/packages/dir-symfony/src/DependencyInjection/Configuration.php`

- [ ] **Step 1: composer.json**

```json
{
  "name": "poly-agntcy/dir-symfony",
  "type": "symfony-bundle",
  "description": "Symfony bundle for poly-agntcy/dir",
  "license": "Apache-2.0",
  "require": {
    "php": "^8.2",
    "poly-agntcy/dir": "*",
    "symfony/framework-bundle": "^6.4|^7.0",
    "symfony/config": "^6.4|^7.0",
    "symfony/dependency-injection": "^6.4|^7.0"
  },
  "autoload": {
    "psr-4": { "PolyAgntcy\\Dir\\Symfony\\": "src/" }
  }
}
```

- [ ] **Step 2: Bundle + DI**

`php/packages/dir-symfony/src/AgntcyBundle.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Symfony;

use Symfony\Component\HttpKernel\Bundle\Bundle;

final class AgntcyBundle extends Bundle {}
```

`php/packages/dir-symfony/src/DependencyInjection/Configuration.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Symfony\DependencyInjection;

use Symfony\Component\Config\Definition\Builder\TreeBuilder;
use Symfony\Component\Config\Definition\ConfigurationInterface;

final class Configuration implements ConfigurationInterface
{
    public function getConfigTreeBuilder(): TreeBuilder
    {
        $tb = new TreeBuilder('agntcy');
        $tb->getRootNode()
            ->children()
                ->scalarNode('endpoint')->defaultValue('https://directory.example')->end()
                ->integerNode('timeout')->defaultValue(5)->end()
            ->end();
        return $tb;
    }
}
```

`php/packages/dir-symfony/src/DependencyInjection/AgntcyExtension.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Symfony\DependencyInjection;

use PolyAgntcy\Dir\Client;
use PolyAgntcy\Dir\InsecureCredentials;
use Symfony\Component\DependencyInjection\ContainerBuilder;
use Symfony\Component\DependencyInjection\Definition;
use Symfony\Component\HttpKernel\DependencyInjection\Extension;

final class AgntcyExtension extends Extension
{
    public function load(array $configs, ContainerBuilder $container): void
    {
        $cfg = $this->processConfiguration(new Configuration(), $configs);
        $container->setDefinition(Client::class, (new Definition(Client::class))
            ->setArgument(0, $cfg['endpoint'])
            ->setArgument(1, new Definition(InsecureCredentials::class))
            ->setPublic(true));
    }
}
```

- [ ] **Step 3: Commit**

```bash
git add php/packages/dir-symfony/
git commit -m "feat(php-symfony): add Symfony bundle with DI extension"
```

## Task 5.6 — `packages/dir-spiffe/`

**Files:**
- Create: `php/packages/dir-spiffe/composer.json`, `php/packages/dir-spiffe/src/SpiffeWorkloadClient.php`, `php/packages/dir-spiffe/src/SpiffeCredentials.php`

- [ ] **Step 1: composer.json**

```json
{
  "name": "poly-agntcy/dir-spiffe",
  "type": "library",
  "description": "SPIFFE-backed credentials for poly-agntcy/dir",
  "license": "Apache-2.0",
  "require": {
    "php": "^8.2",
    "poly-agntcy/dir": "*"
  },
  "autoload": {
    "psr-4": { "PolyAgntcy\\Dir\\Spiffe\\": "src/" }
  }
}
```

- [ ] **Step 2: Minimal SPIFFE Workload API client (v0 stub)**

`php/packages/dir-spiffe/src/SpiffeCredentials.php`:

```php
<?php
declare(strict_types=1);

namespace PolyAgntcy\Dir\Spiffe;

use PolyAgntcy\Dir\Credentials;

final class SpiffeCredentials implements Credentials
{
    public function __construct(private readonly string $socketPath = '/tmp/spire-agent/public/api.sock') {}

    public function tlsOptions(): ?array
    {
        // v0 stub: return null; full SPIFFE Workload API integration follows.
        // Tracked as separate task post-bootstrap.
        return null;
    }
}
```

(Spec calls for "~300 LOC SPIFFE workload-API client". This v0 stub claims the package name + interface; full impl lands later. Document in package README.)

- [ ] **Step 3: Commit**

```bash
git add php/packages/dir-spiffe/
git commit -m "feat(php-spiffe): add SPIFFE credentials package (v0 stub)"
```

## Task 5.7 — phpstan + phpunit config

**Files:**
- Create: `php/phpstan.neon`, `php/phpunit.xml.dist`

- [ ] **Step 1: phpstan.neon**

```yaml
parameters:
  level: max
  paths:
    - packages/dir/src
    - packages/dir-laravel/src
    - packages/dir-symfony/src
    - packages/dir-spiffe/src
  excludePaths:
    - packages/dir/src/Generated
```

- [ ] **Step 2: phpunit.xml.dist**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<phpunit xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:noNamespaceSchemaLocation="vendor/phpunit/phpunit/phpunit.xsd"
         bootstrap="vendor/autoload.php"
         colors="true">
    <testsuites>
        <testsuite name="dir">
            <directory>packages/dir/tests</directory>
            <directory>packages/dir-laravel/tests</directory>
            <directory>packages/dir-symfony/tests</directory>
            <directory>packages/dir-spiffe/tests</directory>
        </testsuite>
    </testsuites>
</phpunit>
```

- [ ] **Step 3: Run + commit**

```bash
cd php && composer install --no-interaction
vendor/bin/phpstan analyse
vendor/bin/phpunit
git add php/phpstan.neon php/phpunit.xml.dist
git commit -m "chore(php): add phpstan + phpunit config"
```

---

# Phase 6 — TypeScript adapters

Three adapter packages wrapping the official `agntcy-dir` npm package: `dir-next`, `dir-hono`, `dir-express`. CLI ships inside `dir-next`'s `bin/` field.

## Task 6.1 — pnpm workspace

**Files:**
- Create: `ts/package.json`, `ts/pnpm-workspace.yaml`, `ts/tsconfig.base.json`, `ts/.gitignore`

- [ ] **Step 1: Root package.json**

```json
{
  "name": "poly-agntcy-ts",
  "private": true,
  "type": "module",
  "scripts": {
    "build": "pnpm -r build",
    "test": "pnpm -r test",
    "lint": "pnpm -r lint"
  },
  "devDependencies": {
    "typescript": "^5.6",
    "vitest": "^2.1",
    "@types/node": "^22"
  },
  "packageManager": "pnpm@9.12.0"
}
```

- [ ] **Step 2: workspace yaml**

```yaml
packages:
  - "packages/*"
```

- [ ] **Step 3: tsconfig.base.json**

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "declaration": true,
    "outDir": "dist",
    "rootDir": "src"
  }
}
```

- [ ] **Step 4: .gitignore**

```
node_modules/
dist/
*.tsbuildinfo
```

- [ ] **Step 5: Install + commit**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/ts && pnpm install
git add ts/package.json ts/pnpm-workspace.yaml ts/tsconfig.base.json ts/.gitignore
git commit -m "chore(ts): init pnpm workspace"
```

## Task 6.2 — `packages/dir-next/`

**Files:**
- Create: `ts/packages/dir-next/package.json`, `ts/packages/dir-next/tsconfig.json`, `ts/packages/dir-next/src/index.ts`, `ts/packages/dir-next/src/handler.ts`, `ts/packages/dir-next/src/cli.ts`, `ts/packages/dir-next/test/handler.test.ts`

- [ ] **Step 1: package.json**

```json
{
  "name": "@poly-agntcy/dir-next",
  "version": "0.1.0-alpha.0",
  "description": "Next.js adapter for AGNTCY Directory Service",
  "license": "Apache-2.0",
  "type": "module",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js"
    }
  },
  "bin": {
    "agntcy": "./dist/cli.js"
  },
  "scripts": {
    "build": "tsc -p tsconfig.json",
    "test": "vitest run",
    "lint": "tsc --noEmit"
  },
  "peerDependencies": {
    "agntcy-dir": "^1.3",
    "next": "^14.0.0 || ^15.0.0"
  },
  "dependencies": {
    "commander": "^12"
  }
}
```

- [ ] **Step 2: tsconfig**

```json
{
  "extends": "../../tsconfig.base.json",
  "compilerOptions": {
    "outDir": "dist",
    "rootDir": "src"
  },
  "include": ["src/**/*"]
}
```

- [ ] **Step 3: Failing test**

`ts/packages/dir-next/test/handler.test.ts`:

```ts
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
```

- [ ] **Step 4: Implement**

`ts/packages/dir-next/src/handler.ts`:

```ts
export interface Options {
  endpoint: string;
}

export interface RouteHandlers {
  POST: (req: Request) => Promise<Response>;
}

export function createDirectoryRouteHandler(opts: Options): RouteHandlers {
  if (!opts.endpoint) {
    throw new Error("endpoint required");
  }
  return {
    POST: async (_req: Request): Promise<Response> => {
      // Wire-level integration with `agntcy-dir` lands in follow-up.
      return new Response(JSON.stringify({ ok: true }), {
        headers: { "Content-Type": "application/json" },
      });
    },
  };
}
```

`ts/packages/dir-next/src/index.ts`:

```ts
export { createDirectoryRouteHandler } from "./handler.js";
export type { Options, RouteHandlers } from "./handler.js";
```

- [ ] **Step 5: CLI**

`ts/packages/dir-next/src/cli.ts`:

```ts
#!/usr/bin/env node
import { Command } from "commander";

const program = new Command("agntcy")
  .version("0.1.0-alpha.0")
  .option("--endpoint <url>", "DIR endpoint")
  .description("AGNTCY Directory Service CLI");

program
  .command("discover")
  .requiredOption("--capability <cap>", "Capability to search for")
  .action(async (opts) => {
    // Delegate to official agntcy-dir; minimal stub for now.
    const endpoint = program.opts().endpoint;
    if (!endpoint) {
      console.error("--endpoint required");
      process.exit(1);
    }
    console.log(`discover ${opts.capability} @ ${endpoint}`);
  });

// register/describe/publish/verify added similarly.

program.parse();
```

- [ ] **Step 6: Build + test**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/ts/packages/dir-next
pnpm build
pnpm test
```

- [ ] **Step 7: Commit**

```bash
git add ts/packages/dir-next/
git commit -m "feat(ts-next): add @poly-agntcy/dir-next with Route Handler + CLI"
```

## Task 6.3 — `packages/dir-hono/`

**Files:**
- Create: `ts/packages/dir-hono/package.json`, `ts/packages/dir-hono/tsconfig.json`, `ts/packages/dir-hono/src/index.ts`, `ts/packages/dir-hono/test/plugin.test.ts`

- [ ] **Step 1: package.json**

```json
{
  "name": "@poly-agntcy/dir-hono",
  "version": "0.1.0-alpha.0",
  "description": "Hono plugin for AGNTCY Directory Service",
  "license": "Apache-2.0",
  "type": "module",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js"
    }
  },
  "scripts": {
    "build": "tsc -p tsconfig.json",
    "test": "vitest run",
    "lint": "tsc --noEmit"
  },
  "peerDependencies": {
    "agntcy-dir": "^1.3",
    "hono": "^4"
  }
}
```

- [ ] **Step 2: tsconfig** (same shape as Task 6.2 Step 2)

- [ ] **Step 3: Failing test**

`ts/packages/dir-hono/test/plugin.test.ts`:

```ts
import { describe, it, expect } from "vitest";
import { Hono } from "hono";
import { agntcyDirPlugin } from "../src";

describe("agntcyDirPlugin", () => {
  it("registers a POST route on /agntcy", async () => {
    const app = new Hono().use("/agntcy", agntcyDirPlugin({ endpoint: "https://dir.example" }));
    const res = await app.request("/agntcy", { method: "POST" });
    expect(res.status).toBe(200);
  });
});
```

- [ ] **Step 4: Implement**

`ts/packages/dir-hono/src/index.ts`:

```ts
import type { MiddlewareHandler } from "hono";

export interface Options {
  endpoint: string;
}

export function agntcyDirPlugin(opts: Options): MiddlewareHandler {
  if (!opts.endpoint) throw new Error("endpoint required");
  return async (c) => {
    return c.json({ ok: true });
  };
}
```

- [ ] **Step 5: Build/test/commit**

```bash
cd ts/packages/dir-hono && pnpm build && pnpm test
git add ts/packages/dir-hono/
git commit -m "feat(ts-hono): add @poly-agntcy/dir-hono plugin"
```

## Task 6.4 — `packages/dir-express/`

Same pattern as Task 6.3. Pkg name `@poly-agntcy/dir-express`, exports an Express middleware factory `agntcyDirMiddleware(opts)`. Peer dep `express ^4 || ^5`.

- [ ] **Step 1-5: Repeat Task 6.3 pattern**, substituting `dir-express`, `express`, and writing the middleware as:

```ts
import type { RequestHandler } from "express";

export interface Options {
  endpoint: string;
}

export function agntcyDirMiddleware(opts: Options): RequestHandler {
  if (!opts.endpoint) throw new Error("endpoint required");
  return (req, res) => {
    res.json({ ok: true });
  };
}
```

Test uses `supertest` (devDep) to hit the middleware. Build + commit.

```bash
git add ts/packages/dir-express/
git commit -m "feat(ts-express): add @poly-agntcy/dir-express middleware"
```

---

# Phase 7 — Python adapters

Three adapter packages wrapping the official `agntcy-dir` PyPI package: `dir-fastapi`, `dir-flask`, `dir-django`. CLI ships inside `dir-fastapi` per `pyproject.toml` `[project.scripts]`.

## Task 7.1 — uv workspace

**Files:**
- Create: `py/pyproject.toml`, `py/.gitignore`

- [ ] **Step 1: pyproject.toml**

```toml
[project]
name = "poly-agntcy-py-workspace"
version = "0.0.0"
description = "Workspace root (not published)"
requires-python = ">=3.10"

[tool.uv.workspace]
members = ["packages/*"]

[tool.uv.sources]
poly-agntcy-dir-fastapi = { workspace = true }
poly-agntcy-dir-flask = { workspace = true }
poly-agntcy-dir-django = { workspace = true }

[dependency-groups]
dev = [
  "mypy>=1.13",
  "pytest>=8",
  "pytest-asyncio>=0.24",
]
```

- [ ] **Step 2: .gitignore**

```
.venv/
__pycache__/
*.egg-info/
dist/
.pytest_cache/
.mypy_cache/
```

- [ ] **Step 3: Sync + commit**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/py && uv sync
git add py/pyproject.toml py/.gitignore
git commit -m "chore(py): init uv workspace"
```

## Task 7.2 — `packages/dir-fastapi/`

**Files:**
- Create: `py/packages/dir-fastapi/pyproject.toml`, `py/packages/dir-fastapi/src/poly_agntcy_dir_fastapi/__init__.py`, `py/packages/dir-fastapi/src/poly_agntcy_dir_fastapi/router.py`, `py/packages/dir-fastapi/src/poly_agntcy_dir_fastapi/cli.py`, `py/packages/dir-fastapi/tests/test_router.py`

- [ ] **Step 1: pyproject.toml**

```toml
[project]
name = "poly-agntcy-dir-fastapi"
version = "0.1.0a0"
description = "FastAPI adapter for AGNTCY Directory Service"
license = "Apache-2.0"
requires-python = ">=3.10"
dependencies = [
  "agntcy-dir>=1.3",
  "fastapi>=0.115",
  "typer>=0.15",
]

[project.scripts]
agntcy = "poly_agntcy_dir_fastapi.cli:app"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatch.build.targets.wheel]
packages = ["src/poly_agntcy_dir_fastapi"]
```

- [ ] **Step 2: Failing test**

`py/packages/dir-fastapi/tests/test_router.py`:

```python
from fastapi import FastAPI
from fastapi.testclient import TestClient
from poly_agntcy_dir_fastapi import create_directory_router


def test_router_mounts():
    app = FastAPI()
    app.include_router(create_directory_router(endpoint="https://dir.example"))
    client = TestClient(app)
    res = client.post("/agntcy")
    assert res.status_code == 200


def test_endpoint_required():
    import pytest
    with pytest.raises(ValueError):
        create_directory_router(endpoint="")
```

- [ ] **Step 3: Implement**

`py/packages/dir-fastapi/src/poly_agntcy_dir_fastapi/__init__.py`:

```python
from .router import create_directory_router

__all__ = ["create_directory_router"]
```

`py/packages/dir-fastapi/src/poly_agntcy_dir_fastapi/router.py`:

```python
from fastapi import APIRouter


def create_directory_router(*, endpoint: str) -> APIRouter:
    if not endpoint:
        raise ValueError("endpoint required")
    router = APIRouter()

    @router.post("/agntcy")
    async def _agntcy_handler() -> dict:
        return {"ok": True}

    return router
```

`py/packages/dir-fastapi/src/poly_agntcy_dir_fastapi/cli.py`:

```python
import typer

app = typer.Typer(name="agntcy", help="AGNTCY Directory Service CLI")


@app.command()
def discover(endpoint: str = typer.Option(..., "--endpoint"),
             capability: str = typer.Option(..., "--capability")) -> None:
    typer.echo(f"discover {capability} @ {endpoint}")


# register/describe/publish/verify added similarly.


if __name__ == "__main__":
    app()
```

- [ ] **Step 4: Test + commit**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy/py && uv sync && uv run pytest packages/dir-fastapi
git add py/packages/dir-fastapi/
git commit -m "feat(py-fastapi): add poly-agntcy-dir-fastapi with APIRouter + CLI"
```

## Task 7.3 — `packages/dir-flask/`

Same pattern as Task 7.2. Pkg `poly-agntcy-dir-flask`, exposes `create_directory_blueprint(*, endpoint)`. Test uses Flask's test client.

- [ ] **Step 1-4: Repeat Task 7.2 pattern**, substituting `dir-flask`, `flask`, and writing the blueprint factory:

```python
from flask import Blueprint, jsonify


def create_directory_blueprint(*, endpoint: str) -> Blueprint:
    if not endpoint:
        raise ValueError("endpoint required")
    bp = Blueprint("agntcy", __name__)

    @bp.route("/agntcy", methods=["POST"])
    def _handler():
        return jsonify({"ok": True})

    return bp
```

```bash
git add py/packages/dir-flask/
git commit -m "feat(py-flask): add poly-agntcy-dir-flask with Blueprint factory"
```

## Task 7.4 — `packages/dir-django/`

Same pattern. Pkg `poly-agntcy-dir-django`, ships a Django app with `urls.py` exposing `/agntcy/` POST endpoint.

- [ ] **Step 1: pyproject.toml** with `dependencies = ["agntcy-dir>=1.3", "django>=4.2"]`

- [ ] **Step 2: Django app structure**

`py/packages/dir-django/src/poly_agntcy_dir_django/apps.py`:

```python
from django.apps import AppConfig


class AgntcyDirConfig(AppConfig):
    name = "poly_agntcy_dir_django"
    label = "agntcy_dir"
```

`py/packages/dir-django/src/poly_agntcy_dir_django/urls.py`:

```python
from django.urls import path
from .views import agntcy_handler

urlpatterns = [path("agntcy/", agntcy_handler, name="agntcy")]
```

`py/packages/dir-django/src/poly_agntcy_dir_django/views.py`:

```python
from django.http import JsonResponse
from django.views.decorators.http import require_POST


@require_POST
def agntcy_handler(request):
    return JsonResponse({"ok": True})
```

- [ ] **Step 3: Test + commit**

```bash
git add py/packages/dir-django/
git commit -m "feat(py-django): add poly-agntcy-dir-django app"
```

---

# Phase 8 — Cross-lang examples + integration tests

## Task 8.1 — Local DIR docker-compose

**Files:**
- Create: `examples/cross/docker-compose.yml`, `examples/cross/README.md`

- [ ] **Step 1: docker-compose**

`examples/cross/docker-compose.yml`:

```yaml
services:
  dir:
    image: ghcr.io/agntcy/dir:latest
    ports:
      - "8888:8888"
    healthcheck:
      test: ["CMD", "wget", "-q", "-O-", "http://localhost:8888/healthz"]
      interval: 5s
      timeout: 3s
      retries: 10
```

(Verify image name from `agntcy/dir` repo's README before committing; substitute the actual published image.)

- [ ] **Step 2: README**

````markdown
# Cross-language integration test

Stands up a local DIR instance and runs Go, Python, and PHP agents against it:

1. Go agent registers `inventory-agent`
2. Python agent discovers it by capability
3. PHP agent sends a signed message to it

## Run

```sh
docker compose up -d
./run.sh
docker compose down
```
````

- [ ] **Step 3: Commit**

```bash
git add examples/cross/
git commit -m "feat(cross): add docker-compose for local DIR"
```

## Task 8.2 — Per-lang quickstart examples

Create one minimal example per language in `examples/<lang>/`. Each example: register an identity, publish a descriptor, discover a peer, send a signed message.

Minimum required examples (one per package shipped):

- `examples/go/register-and-discover/main.go`
- `examples/rs/register-and-discover/src/main.rs`
- `examples/php/laravel-quickstart/` (Laravel skeleton)
- `examples/php/symfony-quickstart/` (Symfony skeleton)
- `examples/php/vanilla-quickstart/` (plain PHP)
- `examples/ts/next-route/app/api/agntcy/route.ts`
- `examples/ts/hono-edge/index.ts`
- `examples/ts/express-quickstart/server.ts`
- `examples/py/fastapi-quickstart/main.py`
- `examples/py/flask-quickstart/app.py`
- `examples/py/django-quickstart/`

- [ ] **Step 1-N: Each example is small (~20-50 lines), committed as one task per example**

For each:
```bash
git add examples/<path>
git commit -m "docs(examples): add <lang>/<framework> quickstart"
```

## Task 8.3 — Cross-lang run.sh

**Files:**
- Create: `examples/cross/run.sh`, `examples/cross/go-agent.go`, `examples/cross/py-agent.py`, `examples/cross/php-agent.php`

- [ ] **Step 1: run.sh**

```bash
#!/usr/bin/env bash
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
cd "$HERE"

# Wait for DIR to come up.
for i in {1..30}; do
  curl -sf http://localhost:8888/healthz && break
  sleep 1
done

# 1. Go registers
(cd "$HERE/.." && go run examples/cross/go-agent.go --endpoint http://localhost:8888 register)

# 2. Python discovers
uv run examples/cross/py-agent.py --endpoint http://localhost:8888 discover --capability inventory

# 3. PHP sends signed message
php examples/cross/php-agent.php --endpoint http://localhost:8888 send --target inventory-agent

echo "Cross-lang integration test passed."
```

- [ ] **Step 2: Make executable + commit**

```bash
chmod +x examples/cross/run.sh
git add examples/cross/
git commit -m "feat(cross): add cross-language integration runner"
```

---

# Phase 9 — Release pipeline

Wires release-please + dotgithub publish workflows + creates the 5 mirror repos.

## Task 9.1 — `.github/release-please-config.json`

**Files:**
- Create: `.github/release-please-config.json`, `.github/.release-please-manifest.json`

- [ ] **Step 1: Manifest**

`.github/.release-please-manifest.json`:

```json
{
  "go": "0.1.0-alpha.0",
  "go/spiffe": "0.1.0-alpha.0",
  "rs/poly-agntcy-dir": "0.1.0-alpha.0",
  "rs/poly-agntcy-dir-spiffe": "0.1.0-alpha.0",
  "php/packages/dir": "0.1.0-alpha.0",
  "php/packages/dir-laravel": "0.1.0-alpha.0",
  "php/packages/dir-symfony": "0.1.0-alpha.0",
  "php/packages/dir-spiffe": "0.1.0-alpha.0",
  "ts/packages/dir-next": "0.1.0-alpha.0",
  "ts/packages/dir-hono": "0.1.0-alpha.0",
  "ts/packages/dir-express": "0.1.0-alpha.0",
  "py/packages/dir-fastapi": "0.1.0a0",
  "py/packages/dir-flask": "0.1.0a0",
  "py/packages/dir-django": "0.1.0a0"
}
```

- [ ] **Step 2: Config**

`.github/release-please-config.json`:

```json
{
  "$schema": "https://raw.githubusercontent.com/googleapis/release-please/main/schemas/config.json",
  "release-type": "go",
  "versioning": "prerelease",
  "prerelease-type": "alpha.0",
  "packages": {
    "go": {
      "release-type": "go",
      "component": "go",
      "package-name": "hop.top/agntcy"
    },
    "go/spiffe": {
      "release-type": "go",
      "component": "go-spiffe",
      "package-name": "hop.top/agntcy/spiffe"
    },
    "rs/poly-agntcy-dir": {
      "release-type": "rust",
      "component": "rs",
      "package-name": "poly-agntcy-dir"
    },
    "rs/poly-agntcy-dir-spiffe": {
      "release-type": "rust",
      "component": "rs-spiffe",
      "package-name": "poly-agntcy-dir-spiffe"
    },
    "php/packages/dir": {
      "release-type": "php",
      "component": "php",
      "package-name": "poly-agntcy/dir"
    },
    "php/packages/dir-laravel": {
      "release-type": "php",
      "component": "php-laravel",
      "package-name": "poly-agntcy/dir-laravel"
    },
    "php/packages/dir-symfony": {
      "release-type": "php",
      "component": "php-symfony",
      "package-name": "poly-agntcy/dir-symfony"
    },
    "php/packages/dir-spiffe": {
      "release-type": "php",
      "component": "php-spiffe",
      "package-name": "poly-agntcy/dir-spiffe"
    },
    "ts/packages/dir-next": {
      "release-type": "node",
      "component": "ts-next",
      "package-name": "@poly-agntcy/dir-next"
    },
    "ts/packages/dir-hono": {
      "release-type": "node",
      "component": "ts-hono",
      "package-name": "@poly-agntcy/dir-hono"
    },
    "ts/packages/dir-express": {
      "release-type": "node",
      "component": "ts-express",
      "package-name": "@poly-agntcy/dir-express"
    },
    "py/packages/dir-fastapi": {
      "release-type": "python",
      "component": "py-fastapi",
      "package-name": "poly-agntcy-dir-fastapi"
    },
    "py/packages/dir-flask": {
      "release-type": "python",
      "component": "py-flask",
      "package-name": "poly-agntcy-dir-flask"
    },
    "py/packages/dir-django": {
      "release-type": "python",
      "component": "py-django",
      "package-name": "poly-agntcy-dir-django"
    }
  }
}
```

- [ ] **Step 3: Commit**

```bash
git add .github/release-please-config.json .github/.release-please-manifest.json
git commit -m "ci: add release-please config for 14 packages"
```

## Task 9.2 — `.github/workflows/release-please.yml`

**Files:**
- Create: `.github/workflows/release-please.yml`

- [ ] **Step 1: Write workflow**

```yaml
name: release-please
on:
  push:
    branches: [main]
permissions:
  contents: write
  pull-requests: write
jobs:
  release-please:
    runs-on: ubuntu-latest
    steps:
      - uses: googleapis/release-please-action@v4
        with:
          token: ${{ secrets.GH_RELEASE_PLEASE_PAT || secrets.GITHUB_TOKEN }}
          config-file: .github/release-please-config.json
          manifest-file: .github/.release-please-manifest.json
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/release-please.yml
git commit -m "ci: add release-please workflow"
```

## Task 9.3 — `.github/workflows/publish.yml`

**Files:**
- Create: `.github/workflows/publish.yml`

- [ ] **Step 1: Write workflow per [hop-top/.github SKILL.md quick-start](https://github.com/hop-top/.github/blob/main/SKILL.md#quick-start-tldr)**

```yaml
name: publish
on:
  push:
    tags: ['*/v*']

jobs:
  publish:
    permissions:
      contents: read
      id-token: write
    uses: hop-top/.github/.github/workflows/publish-on-tag.yml@v0
    secrets:
      NPM_REGISTRY_TOKEN:   ${{ secrets.NPM_REGISTRY_TOKEN }}
      CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
      PACKAGIST_USERNAME:   ${{ secrets.PACKAGIST_USERNAME }}
      PACKAGIST_TOKEN:      ${{ secrets.PACKAGIST_TOKEN }}
      GH_MIRROR_PAT:        ${{ secrets.GH_MIRROR_PAT }}
    with:
      ecosystems: |
        go:          { dir: go,                          ecosystem: go,  mirror: hop-top/agntcy }
        go-spiffe:   { dir: go/spiffe,                   ecosystem: go,  mirror: hop-top/agntcy }
        rs:          { dir: rs/poly-agntcy-dir,          ecosystem: rs,  package: poly-agntcy-dir,        mirror: hop-top/agntcy-rs }
        rs-spiffe:   { dir: rs/poly-agntcy-dir-spiffe,   ecosystem: rs,  package: poly-agntcy-dir-spiffe, mirror: hop-top/agntcy-rs }
        php:         { dir: php/packages/dir,            ecosystem: php, package: "poly-agntcy/dir",         mirror: hop-top/agntcy-php }
        php-laravel: { dir: php/packages/dir-laravel,    ecosystem: php, package: "poly-agntcy/dir-laravel", mirror: hop-top/agntcy-php }
        php-symfony: { dir: php/packages/dir-symfony,    ecosystem: php, package: "poly-agntcy/dir-symfony", mirror: hop-top/agntcy-php }
        php-spiffe:  { dir: php/packages/dir-spiffe,     ecosystem: php, package: "poly-agntcy/dir-spiffe",  mirror: hop-top/agntcy-php }
        ts-next:     { dir: ts/packages/dir-next,        ecosystem: ts,  package: "@poly-agntcy/dir-next",   mirror: hop-top/agntcy-ts }
        ts-hono:     { dir: ts/packages/dir-hono,        ecosystem: ts,  package: "@poly-agntcy/dir-hono",   mirror: hop-top/agntcy-ts }
        ts-express:  { dir: ts/packages/dir-express,     ecosystem: ts,  package: "@poly-agntcy/dir-express",mirror: hop-top/agntcy-ts }
        py-fastapi:  { dir: py/packages/dir-fastapi,     ecosystem: py,  package: poly-agntcy-dir-fastapi,   mirror: hop-top/agntcy-py }
        py-flask:    { dir: py/packages/dir-flask,       ecosystem: py,  package: poly-agntcy-dir-flask,     mirror: hop-top/agntcy-py }
        py-django:   { dir: py/packages/dir-django,      ecosystem: py,  package: poly-agntcy-dir-django,    mirror: hop-top/agntcy-py }
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/publish.yml
git commit -m "ci: add publish-on-tag workflow"
```

## Task 9.4 — Preflight workflow

**Files:**
- Create: `.github/workflows/release-please-preflight.yml`

- [ ] **Step 1: Write workflow**

```yaml
name: release-please-preflight
on:
  pull_request:
    branches: [main]
jobs:
  preflight:
    uses: hop-top/.github/.github/workflows/release-please-preflight.yml@v0
    with:
      config-file: .github/release-please-config.json
      manifest-file: .github/.release-please-manifest.json
      publish-file: .github/workflows/publish.yml
```

(Check `hop-top/.github`'s actual preflight inputs — adjust if signature differs.)

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/release-please-preflight.yml
git commit -m "ci: add release-please preflight"
```

## Task 9.5 — Create GitHub repo + mirror repos

**Files:** none (this is a `gh repo create` step)

- [ ] **Step 1: Create primary repo**

```bash
gh repo create hop-top/poly-agntcy --private --description "Polyglot AGNTCY DIR SDK suite" --homepage https://github.com/hop-top/poly-agntcy
```

- [ ] **Step 2: Push current main**

```bash
cd /Users/jadb/.w/ideacrafterslabs/poly-agntcy
git remote add origin git@github.com:hop-top/poly-agntcy.git
git push -u origin main
```

- [ ] **Step 3: Create 5 mirror repos (empty, public — visibility matches what each registry expects)**

```bash
gh repo create hop-top/agntcy        --public --description "Go SDK mirror (read-only)"
gh repo create hop-top/agntcy-rs     --public --description "Rust SDK mirror (read-only)"
gh repo create hop-top/agntcy-php    --public --description "PHP SDK mirror (read-only)"
gh repo create hop-top/agntcy-ts     --public --description "TypeScript SDK mirror (read-only)"
gh repo create hop-top/agntcy-py     --public --description "Python SDK mirror (read-only)"
```

- [ ] **Step 4: Confirm source repo visibility is private**

```bash
gh repo view hop-top/poly-agntcy --json visibility
```

Expected: `{"visibility":"PRIVATE"}`. If not, `gh repo edit hop-top/poly-agntcy --visibility private`.

## Task 9.6 — Configure repo secrets

This is a manual + GitHub web step. Per [hop-top/.github SKILL.md secrets reference](https://github.com/hop-top/.github/blob/main/SKILL.md):

- [ ] **Step 1: Add per-registry tokens to repo secrets**

Via `gh secret set`:

```bash
gh secret set NPM_REGISTRY_TOKEN --repo hop-top/poly-agntcy < <(echo "$NPM_TOKEN")
gh secret set CARGO_REGISTRY_TOKEN --repo hop-top/poly-agntcy < <(echo "$CARGO_TOKEN")
gh secret set PACKAGIST_USERNAME --repo hop-top/poly-agntcy < <(echo "$PACKAGIST_USER")
gh secret set PACKAGIST_TOKEN --repo hop-top/poly-agntcy < <(echo "$PACKAGIST_TOKEN_VAL")
gh secret set GH_MIRROR_PAT --repo hop-top/poly-agntcy < <(echo "$MIRROR_PAT")
```

PyPI uses OIDC trusted publishing — no token. Configure trusted publishers on PyPI web for each `poly-agntcy-dir-*` package (see [hop-top/.github docs/browser-playbooks.md](https://github.com/hop-top/.github/blob/main/docs/browser-playbooks.md)).

Packagist needs each `poly-agntcy/dir-*` package registered + the GitHub webhook configured.

- [ ] **Step 2: Document secrets in `docs/release-runbook.md`** (not committed credentials, just where they come from and how to rotate)

## Task 9.7 — Vanity-import setup for `hop.top/agntcy`

**Files:** none in this repo; this is a hop.top infra step.

- [ ] **Step 1: Configure `hop.top/agntcy/?go-get=1` to serve**

```html
<meta name="go-import" content="hop.top/agntcy git https://github.com/hop-top/poly-agntcy">
<meta name="go-source" content="hop.top/agntcy https://github.com/hop-top/poly-agntcy https://github.com/hop-top/poly-agntcy/tree/main/go{/dir} https://github.com/hop-top/poly-agntcy/blob/main/go{/dir}/{file}#L{line}">
```

Note: vanity points at `poly-agntcy/go/` (not the mirror) so private repo doesn't break Go module resolution — adjust per how hop.top's existing vanity infra handles per-org subpath routing.

Alternative once mirror exists: point vanity at `hop-top/agntcy` (public Go mirror) so module resolution works for external consumers without source-repo access.

- [ ] **Step 2: Verify**

```bash
curl -sH "Accept: text/html" "https://hop.top/agntcy?go-get=1" | grep go-import
```

Expected: `<meta name="go-import" ...>` tag visible.

## Task 9.8 — Set track + tasks complete

- [ ] **Step 1: Mark all bootstrap-dir tasks done in tlc**

This happens incrementally during execution (each task commit includes `tlc task complete T-NNNN --note "<sha>"`).

- [ ] **Step 2: Move track to completed**

```bash
tlc -C /Users/jadb/.w/ideacrafterslabs/poly-agntcy track update bootstrap-dir --status completed
```

---

# Reference: file structure summary

After this plan executes, the repo has this shape (matches spec §5):

```
poly-agntcy/
├── README.md, LICENSE, CONTRIBUTING.md, SECURITY.md, CODE_OF_CONDUCT.md
├── mise.toml, .env.example, .gitignore, .gitattributes, Makefile
├── .devcontainer/{devcontainer.json,docker-compose.yml,otel-config.yaml}
├── .github/
│   ├── workflows/{ci.yml,release-please.yml,publish.yml,release-please-preflight.yml}
│   ├── release-please-config.json
│   └── .release-please-manifest.json
├── proto/{buf.yaml,buf.gen.yaml,buf.lock,README.md}
├── go/
│   ├── go.mod, .gitignore
│   ├── dir/{credentials.go,client.go,*_test.go}
│   ├── cmd/agntcy/{main.go,root.go,register.go,discover.go,describe.go,publish.go,verify.go,main_test.go}
│   ├── internal/proto/ (gitignored)
│   └── spiffe/{go.mod,credentials.go,credentials_test.go}
├── rs/
│   ├── Cargo.toml, .gitignore
│   ├── poly-agntcy-dir/{Cargo.toml,src/{lib.rs,credentials.rs,client.rs,proto/},tests/}
│   ├── poly-agntcy-dir-spiffe/{Cargo.toml,src/lib.rs}
│   └── poly-agntcy/{Cargo.toml,src/{main.rs,cli.rs}}
├── php/
│   ├── composer.json, .gitignore, phpstan.neon, phpunit.xml.dist
│   └── packages/
│       ├── dir/{composer.json,bin/agntcy,src/{Credentials.php,InsecureCredentials.php,TlsCredentials.php,Client.php,Cli/,Generated/},tests/}
│       ├── dir-laravel/{composer.json,src/{AgntcyServiceProvider.php,Facades/Agntcy.php},config/agntcy.php}
│       ├── dir-symfony/{composer.json,src/{AgntcyBundle.php,DependencyInjection/}}
│       └── dir-spiffe/{composer.json,src/SpiffeCredentials.php}
├── ts/
│   ├── package.json, pnpm-workspace.yaml, tsconfig.base.json, .gitignore
│   └── packages/
│       ├── dir-next/{package.json,tsconfig.json,src/{index.ts,handler.ts,cli.ts},test/}
│       ├── dir-hono/{package.json,tsconfig.json,src/index.ts,test/}
│       └── dir-express/{package.json,tsconfig.json,src/index.ts,test/}
├── py/
│   ├── pyproject.toml, .gitignore
│   └── packages/
│       ├── dir-fastapi/{pyproject.toml,src/poly_agntcy_dir_fastapi/{__init__.py,router.py,cli.py},tests/}
│       ├── dir-flask/{pyproject.toml,src/poly_agntcy_dir_flask/{__init__.py,blueprint.py},tests/}
│       └── dir-django/{pyproject.toml,src/poly_agntcy_dir_django/{apps.py,urls.py,views.py},tests/}
├── docs/
│   ├── adr/0001-0006.md (TBD in execution)
│   ├── architecture.md, spec-version.md, release-runbook.md
│   └── superpowers/{specs/,plans/}/
└── examples/
    ├── go/, rs/, php/, ts/, py/ (per-lang quickstarts)
    └── cross/{docker-compose.yml,run.sh,go-agent.go,py-agent.py,php-agent.php,README.md}
```

# Reference: ADRs to write during Phase 9 (Task 9.9 — not yet broken out)

Per spec §13, six ADRs land before first release:

- `docs/adr/0001-scope-dir-only.md`
- `docs/adr/0002-protobuf-bsr.md`
- `docs/adr/0003-spiffe-optional.md`
- `docs/adr/0004-release-per-package.md`
- `docs/adr/0005-layout-and-adapters.md`
- `docs/adr/0006-naming-and-mirrors.md`

Each ADR follows the standard template: Context, Decision, Consequences. Content sourced from the §4 Decisions log in the spec.

- [ ] **Step 1: Stub each ADR with title + Status: accepted, citing the spec sections**
- [ ] **Step 2: Commit `docs(adr): seed ADR-0001..0006`**

(This is a single commit-batch task — the ADRs are short and reference the spec for full context.)

---

# Execution notes

- **Phase parallelism:** Phases 3, 4, 5, 6, 7 can be dispatched to separate subagents after Phase 2 completes. They share no code. Each phase produces commits on `main` (per CLAUDE.md "MUST work from main"). If conflicts emerge, serialize.
- **TDD discipline:** Every "implement" step has a preceding "write failing test" step. Don't skip.
- **Bite size:** Most steps are 2-5 minutes. The exceptions are Steps that say "repeat for register/describe/publish/verify" — these are intentional pattern-application steps where the canonical example is shown for one method and the engineer applies the same shape to the rest. If a step takes >15min, split it.
- **Commit hygiene:** Every step that produces code commits. Squash on PR merge if the per-step commits feel noisy, but during execution keep them granular for rollback.
- **Stub disclosures:** Tasks 4.4 (Rust SPIFFE), 5.6 (PHP SPIFFE), and the discover/register/... method wiring in 3.4/4.3/5.3 are intentionally v0 stubs. Full wire impl tracked as follow-on track post-bootstrap (`bootstrap-dir-wire` or similar).
