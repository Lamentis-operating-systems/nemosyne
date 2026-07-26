# 0043: Derive delivery-fixture inventories

Status: Accepted
Date: 2026-07-26

## Context

Decision 0042 made the post-DOC finding, conformance, specification, and
decision inventories extensible. Several delivery-program regressions still
expected frozen DOC-00 inventory values. Adding the next valid decision
therefore made the mandatory regression suite fail even though the checker
accepted the extended inventory.

The regression script is a DOC-00-bound governance program. Fixing it requires
a replacement source/evidence pair rather than an ordinary descendant change.

## Decision

Derive the active finding and conformance numbers from the delivery program.
Derive the current specification count and the active, accepted, and
superseded decision inventories from the canonical documentation tree.
Require numbered decision identifiers to be contiguous before using those
values.

Use the derived inventories in fixtures that intentionally address the
current boundary or its next valid extension. Keep frozen-history fixtures,
the checker, and all validation semantics unchanged.

## Rationale

Current-boundary fixtures should test the active contiguous inventory, not
restate historical inventory sizes. Derivation keeps their mutations and
expected diagnostics aligned with the source under test without weakening
the checker or changing protected historical boundaries.

## Alternatives

- Update the literal after every new decision. Rejected because it recreates
  the same governance-only maintenance cycle.
- Loosen or remove the assertion. Rejected because the fixture must still
  prove exact decision continuity.
- Omit otherwise required decisions. Rejected because that would make
  implementation governance depend on a stale test literal.

## Consequences

Future valid post-DOC inventory extensions do not require editing these
regressions. Malformed or noncontiguous decision inventories fail before
fixture execution. This source still requires the complete DOC-00 replacement
attestation flow.
