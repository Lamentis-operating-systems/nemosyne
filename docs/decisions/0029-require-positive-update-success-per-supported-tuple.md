# 0029: Require positive update success per supported tuple

Status: Accepted
Date: 2026-07-24

## Context

The update contract distinguished committed, rolled-back, and quarantined
outcomes, but shipment checks could be read as accepting a test campaign in
which every update attempt failed and merely restored the old pair. Recovery
evidence does not establish that the advertised update can complete.

The first release has no shipped update source, so a blanket positive-update
requirement would recreate the predecessor cycle closed by Decision 0026.

## Decision

Every release candidate binds a finite canonical
`SupportedUpdateTupleSetV1`. Each tuple contains an authenticated shipped
source release, source program identity, source memory schema plus
migration/compatibility class, platform/support domain, exact target candidate,
selected update mechanism and implementation identity, compatibility identity,
one `UpdateMechanismManifestV1`, its complete nonempty
`UpdateFaultBoundaryManifestV1`, and one `UpdateCoverageManifestV1` identity.
It also binds the exact `QuarantineRecoveryTransactionV1` implementation,
one complete nonempty `QuarantineRecoveryMechanismManifestV1`, and its
complete nonempty `QuarantineRecoveryFaultBoundaryManifestV1`.
The tuple never freezes a user-specific authoritative memory revision.

The coverage manifest prospectively partitions only the advertised
compatibility domain into a finite canonical set of mutually exclusive cells.
Each cell has a content-identified membership predicate over
compatibility-relevant authenticated source properties and at least one exact
fixture revision. Canonical predicate evaluation must yield exactly one cell
for every claimed source state. Zero or multiple matches are
unsupported and fail before target mutation. No result may claim support
outside the manifest. Every advertised tuple has at least one cell; an empty
coverage manifest is invalid.

For every advertised tuple and every cell in its frozen coverage manifest,
`RCV-01` first verifies that the packaged implementation, mechanism manifest,
and derived fault-boundary manifest are complete and content-identical. It then
retains:

- at least one uninterrupted execution ending in `UpdateCommitted` with the
  exact verified target program-and-memory pair active; and
- one fault-injection execution for every frozen fault-manifest and concurrent
  writer boundary, each ending in either the same exact
  `UpdateCommitted` target pair or an exact-old-pair `UpdateRolledBack`.

Every execution receipt binds its advertised tuple and coverage-cell identity
plus the exact authenticated source and target memory revision identities used
by that finite fixture or runtime attempt. A runtime update performs the same
membership check before mutation. Fixture results and the independently
verified `MEM-04` transformation invariants establish evidence only for the
declared cells and predicates; they do not prove arbitrary unmodeled content or
make the fixture revisions the only permitted user revisions.

An `UpdateRolledBack` fault case proves recovery only. It never substitutes for
the uninterrupted positive-success case. A missing positive case, missing
fault case, ambiguous pair, lease/epoch failure without exact-old restoration,
or `UpdateQuarantined` blocks authorization.

`NoShippedPredecessor` requires the supported update tuple set to be empty and
therefore has no tuple-scoped update or quarantine-recovery manifests. It uses
clean-install plus initial-publication recovery evidence instead.
`EligiblePredecessor` requires at least the selected predecessor-to-target
tuple. Once a shipped release exists in the same product, channel, platform,
and support domain, a later release in that domain cannot declare an empty
tuple set to avoid update verification. A first release in a newly supported
domain remains eligible for `NoShippedPredecessor` only when authenticated
complete domain-scoped history proves the absence of a predecessor.

`IF-RCV-RECEIPT` and `IF-SHIP-AUTHORIZATION` bind the complete tuple set,
update and recovery mechanism/fault-boundary manifests, and all positive,
update-fault, and recovery-fault dispositions. This aggregate tuple-indexed
result makes explicit the cardinality jointly required by Decision 0028; it
does not permit one transaction result to stand for the complete advertised
set.

## Rationale

Update capability and update recovery are separate claims. Positive evidence
proves the target can be reached; fault injection proves failures do not expose
a mixed or silently corrupted pair. Explicit first-release applicability keeps
the initial shipment reachable without a vacuous self-update.

## Alternatives

- Count exact-old rollback as update success. This proves only recovery.
- Require one positive update across the whole support matrix. Other advertised
  source/platform tuples could remain untested.
- Require a self-update for the first release. There is no shipped source and
  the test would not represent a user transition.

## Consequences

Candidate manifests enumerate update support rather than implying it.
Verification cost grows with the supported tuple set and fault-boundary matrix,
so a release may narrow support before candidate freeze. It may not omit an
implemented durable or externally visible boundary. Authorization binds the
exact resulting evidence.
