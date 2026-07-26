# 0047: Adopt content-identified formal boundary fixtures

Status: Accepted
Date: 2026-07-26

## Context

BND-01 requires executable positive and counterexample fixtures for every
formal F1 through F17 boundary before implementation technology is selected.
The existing EVD-01 module provides an offline artifact identity, while TGT-00
owns the typed pre-outcome G1 envelope. Neither owns formal product-boundary
canaries.

A fixture layer that imports runtime services or pretends to implement future
domain schemas would broaden architecture prematurely. A prose-only matrix
would not detect missing coverage, incorrect expected outcomes, or identity
drift.

## Decision

Add the non-published `nemosyne-boundary-fixtures` workspace crate as the sole
BND-01 owner of `IF-BOUNDARY-FIXTURES`.

Represent each obligation through one typed abstract observation and one closed
violation. Ship exactly one positive and one targeted counterexample for every
F1 through F17 obligation. Validate declared outcomes and complete coverage at
catalog construction, order the catalog canonically, and derive an
EVD-01-compatible `ArtifactContentId` from domain-separated SHA-256 canonical
bytes.

Keep the crate outside every production dependency path. It may expose
synthetic plan roles, exact-slot joins, and prohibited capability variants only
as offline test inputs. It cannot construct runtime capabilities, G1 envelopes,
outcome access, receipts, or product results.

Do not change the TGT-00 or EVD-02 contracts, choose models or storage, or
promote any G1 outcome. Later consumers must add their concrete domain cases;
the abstract catalog does not replace implementation-specific verification.

## Rationale

A separate crate gives the fixture interface one explicit owner and keeps
test-only dependencies out of `nemosyne-core` and the existing evaluation
envelope. Closed observations make the counterexamples executable without
inventing future product types. Complete coverage validation prevents a
catalog from silently omitting one boundary, while content identity makes a
fixture correction observable to consumers.

Reusing only EVD-01's opaque artifact identity preserves the accepted evidence
boundary. It does not duplicate EVD-01 signing, custody, or admission and does
not couple BND-01 to TGT-00's G1 design.

## Alternatives

- Add integration tests directly to `nemosyne-evaluation`. Rejected because
  EVD-01 and TGT-00 would appear to own product-boundary semantics and consumers
  would have no separate `IF-BOUNDARY-FIXTURES` artifact.
- Add fixtures to `nemosyne-core`. Rejected because that would put offline
  verification artifacts on the experimental runtime-kernel path.
- Define future production request, memory, plan, renderer, or capability
  types now. Rejected because BND-01 explicitly selects no implementation
  technology and later packages own those schemas.
- Keep only a Markdown checklist. Rejected because declared outcomes,
  completeness, permutation invariance, and correction identity would not be
  executable.
- Sign the catalog or issue a promotion receipt. Rejected because BND-01 needs
  content-identified fixtures, not a new custody or gate-promotion system.

## Consequences

The repository has a checked synthetic boundary catalog that later packages
can consume as test support. A changed fixture produces a new identity and
must be reviewed as a contract change.

The catalog proves only its own structural coverage and checker behavior. It
does not prove a future implementation conforms, close empirical or security
evidence, establish a storage or transport format, authorize release, or
produce a G1 result.
