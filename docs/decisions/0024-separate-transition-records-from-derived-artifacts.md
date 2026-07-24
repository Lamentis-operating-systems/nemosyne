# 0024: Separate transition records from derived artifacts

Status: Accepted
Date: 2026-07-24

## Context

The predictive-memory contract placed a rebuildable facet manifest beside
authoritative transition fields, then defined record-version identity over
“every authoritative field shown above.” Reliability migration could also
publish either a record version or a derived artifact. These ambiguities
allowed incompatible identity preimages and a cyclic dependency between a
record version and the facets derived from it.

The observation-assessment contract additionally used an undefined
`HypothesisId` instead of one of the accepted semantic or instance identities.

## Decision

`TransitionRecordVersionEnvelopeV1` is an explicit closed authoritative
projection. It contains the logical transition identifier, record and schema
versions, subject, before and after observations, condition, horizon, validity,
reliability, uncertainty, provenance, dependency group, authority,
authorization, allowed usage, and exact-sidecar reference. It excludes the
derived `TransitionRecordVersionId`, every facet or index artifact, artifact
identity, encoder output, and rebuild metadata.

Transition reliability is authoritative record content. A reliability
migration always publishes a new immutable transition record version; it
cannot replace authoritative reliability with a derived artifact.

Rebuildable facets live in a separate immutable
`TransitionFacetArtifact`. Its content identity binds the exact source
`TransitionRecordVersionId`, transform and revision, encoder and vector-space
identities, canonical facet manifest, and artifact schema. Derived artifacts
never affect authoritative record equality and may be rebuilt or discarded
without changing the source record.

The dependency-light domain constructor validates canonical artifact fields
and derives identity only; it does not prove that the named source version
exists. Encoding produces an unpersisted candidate without a store or
management capability. The privileged memory-management operation alone
verifies exact source existence and compatibility, rejects duplicate or
reverse bindings, and atomically publishes the artifact. Memory reads expose
only published artifacts.

Observation assessment keys each relation by the exact
`ExpectationHypothesisInstanceId` from the immutable prior fixture. The
constructor validates complete one-to-one coverage and canonical ordering
against that prior. Semantic keys remain for grouping and duplicate detection,
not observation-fixture identity.

## Rationale

A closed envelope removes implementation-dependent identity choices. Keeping
authoritative and rebuildable planes acyclic permits deterministic migration,
rebuild, rollback, and provenance. Instance identity is required when an
assessment claims a relation to one exact prior fixture.

## Alternatives

- Include facets in the authoritative record identity. This makes encoder
  changes rewrite source truth and creates a record-to-artifact cycle.
- Treat reliability as rebuildable. Reliability participates directly in
  eligibility and support and therefore must remain versioned authoritative
  content under the selected V1 contract.
- Key assessments by lineage-independent semantic keys. Distinct prior
  instances with equal semantics could then collapse.

## Consequences

Predictive storage and migration implementations need separate record and
facet-artifact schemas, identities, fixtures, and atomic publication rules.
Tests must reject missing, stale, cyclic, mismatched, or multiply bound
derived artifacts and incomplete assessment relation maps. This decision does
not select a physical database or encoder.
