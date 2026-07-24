# 0018: Canonicalize predictive and planning identities

Status: Accepted
Date: 2026-07-24

## Context

The proposed predictive and planning contracts conflated a logical transition
chain with an individual transition-record revision, permitted overlapping
planning-error predicates, and classified language support in both request
resolution and planning. These ambiguities would make migrations, ordering,
error mapping, and replay dependent on incidental collection order.

## Decision

`TransitionId` identifies one logical transition chain.
`TransitionRecordVersionId` identifies one immutable version within that chain.
It is a domain-separated content identity over a canonical version envelope
that contains every authoritative transition field, including `TransitionId`
and the serialization-schema version, but excludes the derived
`TransitionRecordVersionId` itself.
Migration lineage, representative selection, deterministic ordering,
provenance, and tie-breaks use the version identifier whenever individual
records compete.

Planning has one closed error sum and one deterministic validation precedence.
Each predicate is disjoint: an absent source is `UnknownSource`; authority and
allowed-use expansion have their own specific predicates; residual
disagreement between an admitted projection and its located immutable source
is `SourceProjectionViolation` only after those specific checks pass and
excludes their fields. Semantic absence is classified only after source
existence and projection equality pass. When multiple failures are present,
the highest-priority class and then the smallest canonical evidence key wins,
independent of encounter or collection order.

Language support is resolved exactly once against the authenticated installed
language manifest before planning. Planning receives only a supported
`ResolvedLanguage` and has no unsupported-language variant. A later artifact
inconsistency is an artifact or internal-integrity failure, not a reclassified
caller language error.

## Rationale

Separate logical and revision identities preserve chain continuity without
making different immutable evidence records indistinguishable. Disjoint error
predicates and canonical precedence make failure behavior testable. A single
language owner prevents inconsistent public exits.

## Alternatives

- Use `TransitionId` for both chain and record revision. This loses revision
  identity and makes deterministic record selection ambiguous.
- Return the first encountered planning error. This leaks input iteration order.
- Retain language checks in both request resolution and planning. This creates
  two owners for one public failure.

## Consequences

Predictive schemas and migrations require an explicit record-version identity.
Planning tests must cover every error predicate, precedence tier, and
multi-invalid permutation. Public error mappings contain twenty planning
variants after unsupported language is removed.
