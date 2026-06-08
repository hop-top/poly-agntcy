# ADR-0007: Layout — role-based (sdk/adapter/cmd) over flat-per-language

- Status: Accepted
- Date: 2026-06-08
- Deciders: jadb
- Supersedes: ADR-0005

## Context

ADR-0005 adopted L3 flat-per-language (`go/`, `rs/`, `php/`, `ts/`, `py/`)
with each language root owning its build conventions and mirror push
target. Six months of accreted code reveals structural mismatches that the
flat layout hides rather than exposes:

1. **SDK vs. adapter conflation.** TS/Python/PHP `packages/` directories
   are named like SDKs (`dir-hono`, `dir-express`, `dir-fastapi`,
   `dir-django`, `dir-flask`, `dir-next`) but are framework adapters on
   top of upstream `agntcy-dir` SDKs (per ADR-0001). The directory
   layout does not distinguish "protocol client" from "framework
   integration." Contributors and consumers must read each package
   README to learn its role.

2. **Generated code scattered across language trees.** `buf.gen.yaml`
   emits to `go/internal/proto/`, `rs/poly-agntcy-dir/src/proto/`, and
   `php/packages/dir/src/Generated/`. No single diff signals "the
   protocol contract changed." Reviewers checking the impact of a BSR
   pin bump must inspect five locations.

3. **No structural home for cross-language conformance.**
   `examples/cross/` is the conformance suite in embryo — multi-language
   agents driven by `docker-compose` — but its position under
   `examples/` signals "illustrative," not "gating."

4. **Per-language dirs encourage convention drift.** The boundary is
   "language," not "role." Go grew a CLI (`cmd/agntcy/`) and SPIFFE
   extension (`spiffe/`) at the language root; Rust grew two flat crates;
   TS/Python grew framework-adapter packages. Each lang invents its own
   subdivision because the root taxonomy provides none.

5. **Contracts directory is a misnomer.** `proto/` contains only
   `buf.yaml` and `buf.gen.yaml` referencing `buf.build/agntcy/dir`.
   No `.proto` files live locally. The directory's name implies
   ownership the repo does not have.

ADR-0005's positives remain real — native ecosystem ergonomics, clean
subtree push, path-based release-please attribution. A successor layout
must preserve them, not trade them away.

## Decision

Adopt a role-based layout that separates protocol clients (SDKs),
framework integrations (adapters), generated code, and binaries into
top-level dirs, with language as a secondary axis:

```
poly-agntcy/
├── cmd/agntcy/              # Go CLI (only binary in the repo)
├── contracts/               # buf pin + lint config + non-proto schemas
├── gen/                     # generated code, committed
│   ├── go/  ts/  py/  rs/  php/
├── sdk/                     # protocol clients
│   ├── go/                  # + go/spiffe/ extension
│   ├── rs/                  # + rs/spiffe/ extension
│   ├── php/
│   ├── ts/                  # thin wrapper over upstream agntcy-dir
│   └── py/                  # thin wrapper over upstream agntcy-dir
├── adapters/                # framework integrations
│   ├── hono/  express/  next/  fastapi/  django/  flask/
├── e2e/conformance/         # promoted from examples/cross/
├── examples/{go,ts,py,rs,php}/
├── docs/
├── scripts/
└── go.work · Makefile · mise.toml · etc.
```

Language ecosystem build roots move with their packages:
- `sdk/go/`, `cmd/agntcy/` participate in the root `go.work`.
- `sdk/rs/Cargo.toml` becomes a Rust workspace covering `sdk/rs/*` crates.
- `sdk/ts/`, `adapters/{hono,express,next}/` join a root
  `pnpm-workspace.yaml`.
- `sdk/py/`, `adapters/{fastapi,django,flask}/` join a root
  `uv`-managed workspace.
- `sdk/php/`, `adapters/<php-framework>/` share a root `composer.json`
  path-repo config.

Mirrors continue to be per-language but now subtree multiple top-level
paths, not one. The mirror push workflow (per ADR-0006) gains an
explicit path-set per language rather than a single directory.

## Consequences

Positive:

- SDK vs. adapter is structural, not documented-in-README. New
  contributors see the architecture in `ls`.
- One `gen/` diff per contract change. CI gets a single "contract
  drifted" signal. Reviewers see protocol impact in one place.
- Conformance becomes first-class via `e2e/conformance/`. CI can gate
  releases on it without renaming `examples/`.
- New adapters land in `adapters/<framework>/` without inventing a home
  inside a language tree. Framework, not language, is the unit of
  ownership for adapters.
- `contracts/` honestly names what the dir does (pin + lint + non-proto
  schemas) without implying local proto ownership.
- Future languages join `sdk/<lang>/` with a known shape, not a blank
  slate.

Negative:

- Per-package release-please configs (14 entries per ADR-0004) and the
  publish workflow's `ecosystems:` block (`dir:` paths for each package)
  all need path updates in one coordinated PR. High-risk single change;
  mitigate by staging in a fork against a throwaway tag first.
- Mirror push gains a path-remap responsibility. Today each ecosystem
  entry maps one `dir:` to one mirror. Post-restructure, the Go mirror
  needs `sdk/go/` (SDK) + `cmd/agntcy/` (binary, if `go install` is
  supported) assembled into one mirror tree. Verify
  `hop-top/.github/.github/workflows/publish-on-tag.yml@v0` supports
  multiple source paths per mirror, or add that support before
  migration phase 8.
- One coordinated breaking-change release across all 14 packages. Every
  in-repo import path moves; consumers see new release-please component
  paths in CHANGELOG.
- Native ecosystem ergonomics weaken slightly: each language no longer
  has one tree from `cd <lang>/`. Contributors working on the Python
  stack `cd` into `sdk/py/` and `adapters/fastapi/` rather than one
  `py/` root. Workspace tooling (uv, pnpm) handles this, but it is a
  real ergonomic shift.

Neutral:

- The CLI count argument (one binary in Go, libraries elsewhere) is
  unchanged. The new layout reflects existing reality rather than
  changing it.
- BSR remains the source of truth for protos. `contracts/` is the pin
  layer, same trust model as today.
- **Go vanity import (`hop.top/agntcy/...`) is preserved.** The vanity
  meta tag still points at `hop-top/agntcy`. Mirror push remaps
  `sdk/go/` → mirror root, so the mirror's root layout (`dir/`,
  `spiffe/`, `go.mod`) is unchanged. `go get hop.top/agntcy/dir`
  continues to resolve. This is a one-line change to the publish
  workflow's `dir:` entries, not a change to the vanity contract from
  ADR-0006.
- **Homebrew distribution (when added) is unaffected.** No
  `.goreleaser.yaml` or Homebrew formula exists today. When introduced,
  the formula references GitHub release assets by tag and asset name,
  not by source paths. Only the build workflow's `working-directory`
  (or goreleaser `builds[].main`) shifts from `go/cmd/agntcy` to
  `cmd/agntcy`. Tag shape (`cmd-agntcy/v*` or rolled into the `go`
  package release per ADR-0004 spec §6) is independent of layout.

## Migration

Phased to bound blast radius. Each phase is independently mergeable
behind the existing release pipeline.

1. **ADR + freeze.** This ADR lands. Per-lang CLIs (Go only) and SDK
   APIs frozen for new features during cutover. Tag `pre-restructure`.
2. **`contracts/` rename.** Move `proto/` → `contracts/`. Update
   `buf.gen.yaml` paths but keep current output destinations. No code
   moves yet.
3. **`gen/` centralization.** Update `buf.gen.yaml` to emit to
   `gen/<lang>/`. Each language's existing generated-code path becomes a
   thin re-export module pointing at `gen/<lang>/`. Commit generated
   outputs.
4. **SDK extraction per language.** Lang by lang, move the protocol
   client code into `sdk/<lang>/`. Adapters stay in place during this
   phase; they import from the new SDK path.
5. **Adapters promotion.** Move `<lang>/packages/dir-<framework>/` →
   `adapters/<framework>/`. Update workspace configs. Update
   release-please paths.
6. **Go CLI promotion.** Move `go/cmd/agntcy/` → `cmd/agntcy/`. Update
   `go.work`.
7. **Examples + conformance.** Promote `examples/cross/` →
   `e2e/conformance/`. Wire into CI as a release gate.
8. **Mirrors + vanity.** Update mirror push workflow for path-sets.
   Verify `hop.top/agntcy` vanity resolution still works (or update the
   meta tag).
9. **Cleanup.** Remove deprecated re-export shims. Delete empty
   per-language root dirs. Update all docs.

Each phase ships behind a release. The repo is functional between
phases; consumers see two layouts coexist during the transition window.

## Alternatives considered

- **Status quo (ADR-0005).** Rejected: structural problems (SDK/adapter
  conflation, scattered codegen, no conformance home) persist and grow
  as the adapter count grows. The flat-per-lang layout served well at
  spec-bootstrap but does not scale with adapter proliferation.
- **L2 shared `packages/` root (ADR-0005 alternative).** Rejected for
  the same reasons ADR-0005 rejected it — single workspace shape fights
  every language. The role-based layout proposed here preserves
  per-language workspaces inside `sdk/<lang>/` and `adapters/<lang>/`,
  avoiding L2's collapse.
- **L1 deep nested taxonomy.** Rejected as in ADR-0005.
- **Sibling repos (L4).** Rejected as in ADR-0005 — loses single-PR
  spec-bump propagation and cross-lang conformance.
- **Hybrid: keep `go/` flat, restructure only TS/PY/PHP.** Rejected:
  partial migration leaves the SDK-vs-adapter naming problem unsolved
  for the Go side (SPIFFE extension is structurally an adapter) and
  produces a layout with no governing principle.

## Related

- ADR-0001: Phase-one scope (DIR only) — informs SDK boundaries.
- ADR-0002: BSR pin + codegen via mise — `gen/` centralization depends
  on this.
- ADR-0003: SPIFFE optional extension — fits naturally under
  `sdk/<lang>/spiffe/` or `adapters/spiffe/<lang>/`; this ADR defers the
  classification to phase 4.
- ADR-0004: Per-package release-please tracks — config path updates in
  migration phase 5.
- ADR-0005: L3 flat-per-language — superseded by this ADR.
- ADR-0006: Naming + vanity + mirrors — mirror push and vanity meta
  contracts need updates in migration phase 8.
- Design spec: `docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`
  sections 5, 6, 7.
