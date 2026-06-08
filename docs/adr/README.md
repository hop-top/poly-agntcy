# Architecture Decision Records

Decisions captured at design time for the `poly-agntcy` polyglot DIR SDK
suite. Format: MADR-lite — Status, Date, Deciders, Context, Decision,
Consequences, Alternatives, Related.

| # | Title | Status | Date |
|---|-------|--------|------|
| [ADR-0001](0001-scope-dir-only-phase-one.md) | Scope — DIR only, phase 1 of polyglot AGNTCY family | Accepted | 2026-05-30 |
| [ADR-0002](0002-protobuf-bsr-pin-codegen-mise.md) | Protobuf — BSR pin, codegen via mise, no checked-in stubs | Accepted | 2026-05-30 |
| [ADR-0003](0003-spiffe-optional-extension-per-language.md) | SPIFFE — optional extension package per language | Accepted | 2026-05-30 |
| [ADR-0004](0004-release-per-package-release-please.md) | Release — per-package release-please tracks | Accepted | 2026-05-30 |
| [ADR-0005](0005-layout-l3-flat-per-language.md) | Layout — L3 flat-per-lang, TS/Py adapter-only | Superseded by ADR-0007 | 2026-05-30 |
| [ADR-0006](0006-naming-source-vanity-mirrors.md) | Naming — `poly-agntcy` source, `hop.top/agntcy` Go vanity, per-language mirror | Partially superseded by ADR-0008 | 2026-05-30 |
| [ADR-0007](0007-role-based-layout-sdk-adapter-split.md) | Layout — role-based (sdk/adapter/cmd) over flat-per-language | Accepted | 2026-06-08 |
| [ADR-0008](0008-vendor-package-names.md) | Vendor-rooted package names — `hop-top/agntcy-*` | Accepted | 2026-06-08 |
| [ADR-0009](0009-mirror-publish-topology.md) | Mirror publish topology — defer multi-component-per-mirror fix | Proposed | 2026-06-08 |

## Conventions

- Filename: `NNNN-kebab-slug.md`. Four-digit zero-padded sequence.
- Status values: `Proposed | Accepted | Superseded by ADR-NNNN | Deprecated`.
- Date is the decision date, not the file-edit date.
- New ADR supersedes an old one: edit the old ADR's Status to point at
  the new one; do not delete.
- Reference the design spec under `docs/superpowers/specs/` for the
  evidence trail; reference upstream specs (RFCs, BSR modules) where
  relevant.
