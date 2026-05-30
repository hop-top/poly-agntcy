# ADR-0004: Release — per-package release-please tracks

- Status: Accepted
- Date: 2026-05-30
- Deciders: jadb

## Context

The repo publishes 14 packages across five language ecosystems (go,
cargo, packagist, npm, pypi). Two release shapes were considered:

- Single repo-wide version that bumps everything in lockstep.
- Per-package versioning where a change to one package opens a release
  PR for only that package.

DIR spec bumps will likely touch all five languages at once (regen
stubs); but most day-to-day changes — adapter fixes, dep bumps,
docs — touch one package. Lockstep versioning would force noisy
no-op releases across 13 packages every time one moves.

## Decision

Use `googleapis/release-please` with one config file containing 14
package entries. Each entry declares its `release-type` per ecosystem
(`go`, `rust`, `php`, `node`, `python`) and a single-segment `component`
tag prefix (table in spec §6). release-please opens one standing PR per
package; merging that PR cuts a tagged release for that package only.

Publish is delegated to the shared `hop-top/.github/.github/workflows/
publish-on-tag.yml@v0` workflow, triggered by `tags: ['*/v*']`. A
preflight workflow validates three-way name alignment
(component ↔ ecosystems-map key ↔ mirror basename) at PR time.

## Consequences

Positive:

- Single-package fix → single release PR → single tag → single publish.
- Spec-bump touching all langs opens up to 14 release PRs in parallel;
  each is independently mergeable.
- Tag shape `<component>/v<x.y.z>` is unambiguous across mirrors and
  satisfies the single-segment glob constraint.

Negative:

- 14-entry config drifts easily; the preflight workflow exists to catch
  the drift before tag-push time, not after.
- Contributors must understand Conventional Commits + path-based scoping
  for release-please to attribute changes correctly.

Neutral:

- CLI tools are not separately released — they ship as bin targets
  inside an existing released package (spec §6). Keeps tag count at 14.

## Alternatives considered

- Lockstep repo version — rejected: forces no-op releases on every move.
- Manual `git tag` + hand-written changelogs — rejected: 14 packages
  guarantee inconsistency and missed tags.
- Per-ecosystem aggregate version (one tag per lang) — rejected: still
  forces no-op releases inside a language.

## Related

- Design spec: `docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`
  section 10
- Shared workflow contract: `RELEASING.md`
