# ADR-0001: Scope — DIR only, phase 1 of polyglot AGNTCY family

- Status: Accepted
- Date: 2026-05-30
- Deciders: jadb

## Context

The AGNTCY ecosystem covers multiple specs: Agent Directory Service (DIR),
Agent Connect Protocol (ACP), Identity, SLIM transport, OASF schema, and a
runtime/orchestration layer. Official SDK coverage is uneven — Go, Rust,
and PHP have no first-party DIR SDK at all; JavaScript and Python ship
official DIR SDKs but lack typed framework adapters.

Trying to cover every spec across five languages in a single repo blows
the surface area past any realistic phase-1 budget. Coverage must narrow.
DIR is the foundational spec — register, discover, describe, publish,
verify — and the gap is most acute there.

## Decision

Phase 1 of `poly-agntcy` covers the DIR spec only across Go, Rust, PHP,
TypeScript, and Python. Orchestration, runtime, ACP, Identity, SLIM, and
OASF are explicitly out of scope and deferred to sibling repos
(`poly-agntcy-acp`, `poly-agntcy-identity`, `poly-agntcy-slim`,
`poly-agntcy-oasf`, `poly-agntcy-runtime`). Sibling vs sub-directory
shape is a deferred decision.

## Consequences

Positive:

- Narrow surface keeps a five-language matrix shippable.
- DIR is the right anchor — every other AGNTCY spec depends on agent
  identity + discovery.
- Adapter-only TS/Py avoids reimplementing maintained upstream code.

Negative:

- Users wanting ACP/runtime get nothing from phase 1.
- Multi-repo phase-2 sprawl risk if sibling-vs-subdir decision drifts.

Neutral:

- Each sibling repo will reuse the same A3+L3+P2+S2+R1 shape captured in
  ADR-0002 through ADR-0005.

## Alternatives considered

- Monolith covering all AGNTCY specs in one repo — rejected: surface area
  blows budget; release coupling forces lockstep across unrelated specs.
- DIR + ACP in phase 1 — rejected: ACP spec velocity higher than DIR;
  bundling delays DIR ship.
- Adapter-only for all five languages — rejected: no upstream Go/Rust/PHP
  DIR SDK exists to adapt against.

## Related

- Design spec: `docs/superpowers/specs/2026-05-30-poly-agntcy-design.md`
  sections 1, 2, 3, 17
