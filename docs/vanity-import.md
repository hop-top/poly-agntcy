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
<meta name="go-source" content="hop.top/agntcy https://github.com/hop-top/poly-agntcy https://github.com/hop-top/poly-agntcy/tree/main/go{/dir} https://github.com/hop-top/poly-agntcy/blob/main/go{/dir}/{file}#L{line}">
```

The `go-import` tag tells the toolchain that the module rooted at
`hop.top/agntcy` is a Git repository hosted at
`https://github.com/hop-top/poly-agntcy`. The Go module proxy
clones that repo, then resolves the module by walking subdirectories
that contain a `go.mod` (here: `go/` and `go/spiffe/`).

The `go-source` tag is consumed by godoc-style browsers
(pkg.go.dev, godocs.io) to link directly to source on GitHub.

## Path → repo subtree mapping

| Module path | Repo subtree |
|---|---|
| `hop.top/agntcy` | `poly-agntcy/go/` |
| `hop.top/agntcy/spiffe` | `poly-agntcy/go/spiffe/` |

Tags follow the `<component>/v<version>` convention (e.g.
`go/v0.1.0`, `go-spiffe/v0.1.0`). The Go toolchain reads tags
matching the prefix corresponding to each module's subdirectory.

## Public vs private source

The source repo `hop-top/poly-agntcy` is private. Until that
changes, the vanity record MUST point at a public-readable
location for module resolution to succeed for external consumers.
Two options:

1. **Point at the public Go mirror** `hop-top/agntcy`.
   Once the publish pipeline lands and the Go mirror exists,
   switch the `go-import` repo URL to
   `https://github.com/hop-top/agntcy`. This is the only path
   that works for unauthenticated `go get`.
2. **Keep pointing at `poly-agntcy`** for development against
   private builds with `GOPRIVATE=hop.top/agntcy` set on
   maintainer workstations.

The mirror-pointing record is the production default once the
Go mirror is published.

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
