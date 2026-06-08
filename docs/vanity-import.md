# Vanity import: `hop.top/agntcy`

The Go module path `hop.top/agntcy` is a vanity import. Resolution
relies on the `hop.top` host responding to a `?go-get=1` query with
`<meta name="go-import">` and `<meta name="go-source">` tags per the
[Go remote-import-paths spec](https://pkg.go.dev/cmd/go#hdr-Remote_import_paths).

## Redirect contract

When a Go client requests `https://hop.top/agntcy?go-get=1`, the
response MUST contain (at minimum) the following meta tags:

```html
<meta name="go-import" content="hop.top/agntcy git https://github.com/hop-top/poly-agntcy">
<meta name="go-source" content="hop.top/agntcy https://github.com/hop-top/poly-agntcy https://github.com/hop-top/poly-agntcy/tree/main/sdk/go{/dir} https://github.com/hop-top/poly-agntcy/blob/main/sdk/go{/dir}/{file}#L{line}">
```

The `go-import` tag tells the toolchain that the module rooted at
`hop.top/agntcy` is a Git repository hosted at
`https://github.com/hop-top/poly-agntcy`. The Go module proxy
clones that repo, then resolves the module by walking subdirectories
that contain a `go.mod` (here: `sdk/go/` and `sdk/go/spiffe/`).

The `go-source` tag is consumed by godoc-style browsers
(pkg.go.dev, godocs.io) to link directly to source on GitHub.

## Path → repo subtree mapping

| Module path | Repo subtree |
|---|---|
| `hop.top/agntcy` | `poly-agntcy/sdk/go/` |
| `hop.top/agntcy/spiffe` | `poly-agntcy/sdk/go/spiffe/` |
| `hop.top/agntcy/gen/go` | `poly-agntcy/gen/go/` |
| `hop.top/agntcy/cmd/agntcy` | `poly-agntcy/cmd/agntcy/` |

Tags follow the `<component>/v<version>` convention (e.g.
`go/v0.1.0`, `go-spiffe/v0.1.0`). The Go toolchain reads tags
matching the prefix corresponding to each module's subdirectory.

The generated-code module (`hop.top/agntcy/gen/go`) and CLI module
(`hop.top/agntcy/cmd/agntcy`) were introduced by the role-based
layout migration (ADR-0007). These have their own `go.mod` files
and resolve via the same vanity host.

## Public vs private source

The source repo `hop-top/poly-agntcy` is public, so the current
vanity record can point at it directly. The eventual production
default is to point at the per-language Go mirror (`hop-top/agntcy`)
once the publish pipeline reliably populates it.

> **Status (2026-06-08): blocked.** The mirror push for
> `hop-top/agntcy` is broken under multi-component tags
> (see [ADR-0009](adr/0009-mirror-publish-topology.md) — each tag
> force-pushes only its own subtree, so subsequent tags overwrite
> prior ones). Until ADR-0010 decides the topology and the
> execution PR lands, the `hop-top/agntcy` mirror is not safe to
> point the vanity record at. The `go-import` URL must stay on the
> source repo `hop-top/poly-agntcy`.

Once ADR-0010 lands and the Go mirror reliably contains the SDK
tree:

1. Switch the `go-import` repo URL to `https://github.com/hop-top/agntcy`.
2. Update the `go-source` paths to drop the `sdk/` prefix (since
   the mirror's root *is* `sdk/go/`).

## Verification

```bash
curl -sH "Accept: text/html" "https://hop.top/agntcy?go-get=1" \
  | grep -E 'go-(import|source)'
```

Expected: both meta tags present, `content` values matching the
table above.

```bash
GOPROXY=direct go list -m hop.top/agntcy@latest
```

Expected: resolves to the latest `go/vX.Y.Z` tag from whichever
repo the vanity record points at.

## Operational ownership

`hop.top` infra serves the redirect (out of repo scope). This
document captures the contract that the infra serves; changes to
mirror URL, tag prefix convention, or subtree mapping land here
first, then propagate to the hop.top vanity-record source of
truth.
