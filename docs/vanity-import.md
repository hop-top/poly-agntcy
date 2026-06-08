# Vanity import: `hop.top/agntcy`

The Go module path `hop.top/agntcy` is a vanity import. Resolution
relies on the `hop.top` host responding to a `?go-get=1` query with
`<meta name="go-import">` and `<meta name="go-source">` tags per the
[Go remote-import-paths spec](https://pkg.go.dev/cmd/go#hdr-Remote_import_paths).

## Redirect contract

When a Go client requests `https://hop.top/agntcy?go-get=1`, the
response MUST contain (at minimum) the following meta tags:

```html
<meta name="go-import" content="hop.top/agntcy git https://github.com/hop-top/agntcy">
<meta name="go-source" content="hop.top/agntcy https://github.com/hop-top/agntcy https://github.com/hop-top/agntcy/tree/main{/dir} https://github.com/hop-top/agntcy/blob/main{/dir}/{file}#L{line}">
```

The `go-import` tag points at the **per-language Go mirror**
(`hop-top/agntcy`), not the polyglot source repo
(`hop-top/poly-agntcy`). Reason: the source repo's `go.mod` for
`hop.top/agntcy` lives under `sdk/go/`, not at repo root.
`go get hop.top/agntcy@<tag>` would not resolve against the source
repo. The mirror's tree, by construction, has `go.mod` at root —
populated by the publish pipeline subtree-splitting `sdk/go/` onto
the mirror's `main`.

The `go-source` tag is consumed by godoc-style browsers
(pkg.go.dev, godocs.io) to link directly to source on GitHub.
Pointing it at the mirror (not source) means deep-links resolve
without the `sdk/go/` prefix.

## Path → repo subtree mapping (published modules)

Per ADR-0010, each module path has its own mirror (no shared trees).

| Module path | Mirror | Source subtree |
|---|---|---|
| `hop.top/agntcy` | `hop-top/agntcy/` | `poly-agntcy/sdk/go/` |
| `hop.top/agntcy/spiffe` | `hop-top/agntcy-go-spiffe/` | `poly-agntcy/sdk/go/spiffe/` |

Each module has its own vanity meta tag (`hop.top/agntcy/spiffe` is
not a subdirectory of `hop-top/agntcy`; the hop.top infra serves
two `<meta name="go-import">` records).

Tags follow the `<component>/v<version>` convention (e.g.
`go/v0.1.0`, `go-spiffe/v0.1.0`).

**Internal-only modules** (not part of the vanity/release contract,
not safe to depend on from outside the repo):

- `hop.top/agntcy/gen/go` — generated-code module under
  `gen/go/go.mod`. Used in-repo via the root `go.work`.
- `hop.top/agntcy/cmd/agntcy` — CLI module under
  `cmd/agntcy/go.mod`. Uses `replace hop.top/agntcy => ../../sdk/go`
  for development; ships as a built binary (Homebrew / GitHub
  releases), not as a `go install`-able module.

If either is ever published as a consumer-facing module, this table
moves it from "internal-only" to the published row.

## Mirror status

Topology fixed per [ADR-0010](adr/0010-one-mirror-per-package.md):
each module path resolves to its own mirror; no overwrite races.
The shared `mirror-subtree.yml@main` calls `gh repo create` on
first push, so new mirrors auto-provision when their component's
first tag fires.

Two operational steps remain (out-of-repo):

1. **Vanity infra**: add a second `<meta name="go-import">` record
   for `hop.top/agntcy/spiffe` pointing at `hop-top/agntcy-go-spiffe`.
2. **Archive orphans**: `gh repo archive hop-top/agntcy-ts hop-top/agntcy-py`
   — these were named optimistically under the old "per-language
   mirror" framing and are no longer referenced by `publish.yml`.

## Verification

```bash
curl -sH "Accept: text/html" "https://hop.top/agntcy?go-get=1" \
  | grep -E 'go-(import|source)'
```

Expected: both meta tags present, `content` values matching the
contract above.

```bash
GOPROXY=direct go list -m hop.top/agntcy@latest
```

Expected (once releases exist + ADR-0010 lands): resolves to the
latest `go/vX.Y.Z` tag from the mirror.

## Operational ownership

`hop.top` infra serves the redirect (out of repo scope). This
document captures the contract that the infra serves; changes to
mirror URL, tag prefix convention, or subtree mapping land here
first, then propagate to the hop.top vanity-record source of
truth.
