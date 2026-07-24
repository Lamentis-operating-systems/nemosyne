# 0026: Distinguish initial release from predecessor rollback

Status: Accepted
Date: 2026-07-24

## Context

The release contract required every candidate to name an eligible rollback
target. Eligibility required a previously shipped Nemosyne release. That made
the first shipment impossible even when no predecessor had ever existed.

Removing rollback evidence entirely would create the opposite defect:
subsequent releases could silently ship without a recoverable predecessor, and
an initial publication failure could be mislabeled as a successful rollback.

## Decision

Every release candidate binds exactly one closed `RollbackDispositionV1`:

- `EligiblePredecessor`, containing one currently supported, previously
  shipped, non-withdrawn, non-EOL, memory-compatible target and its verified
  rollback procedure; or
- `NoShippedPredecessor`, containing an authenticated complete channel-history
  commitment proving that no Nemosyne release has previously reached
  `Shipped` in the candidate's product, channel, platform, and support domain.

`NoShippedPredecessor` is valid only for the first release in that domain. A
later candidate with an ineligible predecessor stops; it cannot use the
initial-release disposition.

Package verification rehearses the selected branch. The initial-release branch
proves distribution stop, endpoint withdrawal, channel inventory, preservation
of any authoritative user memory, affected-user notification, and quarantine
of a partial publication. It never claims that rollback occurred. Publication
failure with no shipped predecessor enters `PartialPublicationQuarantined`
directly and cannot produce `Shippable`.

The exact disposition, history commitment, and branch-specific recovery
artifacts are part of the immutable release-candidate identity.

## Rationale

Rollback is defined relative to an earlier shipped release. A closed,
authenticated first-release disposition removes the impossible predecessor
cycle without weakening later-release recovery requirements or inventing a
false rollback receipt.

## Alternatives

- Require an eligible predecessor for every release. This makes the first
  release unreachable.
- Treat an unpublished development build as shipped. This corrupts the release
  state machine and support history.
- Permit a generic “no rollback available” flag. This lets later releases evade
  predecessor eligibility and provides no verifiable failure-recovery plan.
- Report initial publication withdrawal as `PublicationRolledBack`. This makes
  a false transition claim.

## Consequences

Release candidate, verification, authorization, shipment, support, and
quarantine receipts must carry the selected disposition. Channel history is a
trusted release input. The first release can proceed without a predecessor,
but it still cannot pass shipment when publication or branch-specific recovery
verification fails.
