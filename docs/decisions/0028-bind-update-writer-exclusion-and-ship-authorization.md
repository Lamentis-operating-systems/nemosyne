# 0028: Bind update writer exclusion and ship authorization

Status: Superseded
Date: 2026-07-24
Superseded by: 0031-complete-compile-and-update-admission-handoffs.md

## Context

The update transaction introduced by Decision 0027 quiesced compiler reads but
did not exclude concurrent authenticated management writes. A publication
after backup and source-pair authentication could be lost or make the migrated
pair stale.

Shipment authorization also consumed update evidence operationally without
binding the exact release-verification receipt. Authorization could therefore
be separated from the interruption-matrix result it claimed to approve.

## Decision

`UpdateTransactionV1` acquires one durable exclusive
`UpdateExclusionLeaseV1` through the authenticated memory-management boundary
before it authenticates the current pair or creates the backup. The lease:

- binds the store, authoritative revision, monotonic writer epoch, update
  transaction, management principal, acquisition time, and expiry or recovery
  policy;
- atomically enters durable `UpdateClosing`, closes admission for normal
  management mutations and compiles, installs the complete update owner and
  captured writer/ticket/handle/snapshot set, drains or requests cancellation
  under a bounded declared policy, and enters `UpdateActive` only when every
  captured item is terminal;
- is revalidated before backup, transformation, active-pair switch, committed
  verification, and verified rollback; and
- remains effective through the crash-recoverable finalization that produces
  `UpdateCommitted` or `UpdateRolledBack`, or through the durable handoff to
  `UpdateQuarantined`.

`MEM-03` owns this lease and the underlying durable admission barrier,
monotonic writer epoch, restart reconstruction, and release protocol.
`UpdateClosing` persists the exact transaction, lease, principal, target,
implementation, mechanism/fault manifests, quiescence policy, acquisition
epoch/sequence, and complete captured set. Restart resumes that same drain;
inability to reconstruct it cannot enter `UpdateActive`, release the lease, or
admit a new mutation. Every normal management mutation is serialized through
the barrier; no independent writer path may bypass it.

Compile uses the same barrier without receiving management authority.
`API-01` acquires one non-authority-bearing `CompileAdmissionTicketV1` from
`IF-COMPILE-ADMISSION` after invocation authentication and before `MEM-02`
opens a snapshot or resolves or pins active-pair-dependent configuration or
artifact handles. The opaque, non-cloneable, non-serializable ticket binds
store, barrier generation, writer epoch, active pair, installation manifest,
configuration-registry revision, authenticated executing-program identity,
runtime-registration generation, compiler runtime, invocation, admission
sequence, cancellation registration, and drain policy without prompt or user
content. Admission rejects when the executing program or runtime registration
does not match the active pair. Pair-dependent preflight and snapshot
acquisition are available only inside the `MEM-03`-admitted scope and must
match that complete ticket binding; `API-01` cannot directly open a raw
revision snapshot. The ticket remains live until every pinned handle, snapshot,
and compile stage for the invocation has ended, and `API-01` releases it on
every success, error, and cancellation path.
A cancellation request is not drainage: completion is recorded only after the
snapshot closes and the ticket is consumed. Barrier closure and ticket
acquisition are one linearizable race: either acquisition wins and the ticket
appears in the captured set, or closure wins and acquisition fails before any
snapshot opens. Startup begins closed and reconstructs durable
exclusive-operation state before opening a new barrier generation. A prior
runtime ticket can be retired only when topology-specific evidence proves its
holder and snapshots cannot survive; ambiguous liveness remains blocked.
Active-pair switch or rollback, writer-epoch advance, old runtime-generation
retirement, and barrier reopening form one crash-recoverable handoff. An idle
old runtime is never grandfathered, including when rollback makes the same
program identity active again; it must re-register against the current
installation manifest before it can obtain a ticket. The update lease cannot
be released until every captured ticket and snapshot is terminal and the old
runtime generation is retired.

For a committed or rolled-back result, finalization atomically records the
terminal pair, clears the pending-update marker, advances the writer epoch, and
marks the lease releasable. Releasing the process-level exclusion is an
idempotent crash-recoverable continuation; compile and management mutation
remain unavailable until it completes. Candidate verification then performs
one successful compile and one authenticated fixture management mutation
against the terminal pair. A missing resumption receipt cannot count as
committed or rolled back.

Restart with a pending or quarantined update keeps compile and management
mutation unavailable. `UpdateQuarantined` is terminal for the original update
transaction. `MEM-04` implements a separate authenticated
`QuarantineRecoveryTransactionV1` that binds the quarantine record, exact old
pair and verified backup, recovery principal, writer epoch, and a dedicated
recovery lease.

`MEM-03` owns one durable `StoreExclusiveLifecycleStateV1` for normal update,
compile admission, recovery, and purge exclusion. Every purge, whether it
starts from an operational or quarantined store, revalidates its exact
authorization, confirmation, principal, store, scope, and epoch, then
linearizes against that same state and barrier. From an operational store it
first enters `PurgeClosing`, atomically closes compile and writer admission,
and captures every writer, compile ticket, and snapshot. Cancellation may be
requested but does not count as drainage. No destructive effect and no
`PurgeActive` transition is permitted until the complete captured set is
terminal. Update and purge contend on the same operational-state
linearization point and cannot both own the store.

While quarantined, `QuarantineIdle` admits exactly one atomic transition to
`RecoveryActive` or `PurgeClosing`. `BeginQuarantineRecoveryV1` validates a
narrowly scoped `QuarantineRecoveryCapabilityV1`, the exact quarantine record,
old pair and backup, principal, and current epoch. Purge validates its
separately scoped authority and the same quarantine identity. Capability or
authorization is marked claimed only by the successful compare-and-swap that
creates a durable attempt-bound owner. A losing contender creates no lease,
changes no state, epoch, or data, consumes no otherwise valid authority, and
receives `ExclusiveOperationBusy`.

The durable recovery owner binds the store, quarantine record and digest,
exact old pair, verified backup, writer epoch, recovery transaction, lease,
capability identity and digest, principal, implementation, mechanism and fault
manifests, recovery policy, acquisition sequence, and current phase. The
durable purge owner binds the store, operational revision and policy or full
quarantine basis, writer epoch, purge transaction, lease, authorization
identity and digest, confirmation, principal, purge-scope digest,
implementation, mechanism and fault manifests, recovery policy, acquisition
sequence, and current phase. Restart reconstructs those exact bindings and
never infers them from process memory; missing or contradictory owner, phase,
epoch, scope, or manifest state remains blocked. Lease expiry or process loss
never silently releases ownership.

Recovery transfers durable quarantine exclusion into its dedicated owner
without reopening normal compile or management admission. Purge is not
recovery, never reopens compile, and may invalidate recovery material only
while `PurgeActive`. Before the first manifest-declared destructive effect, a
verified no-effect abort atomically records its proof, retires the purge lease,
consumes the admitted authorization, advances the epoch, and restores the exact
origin state. Admission reopens after that handoff only for an operational
origin; a quarantined origin returns to closed `QuarantineIdle`. A completed
purge atomically records an external durable receipt, verifies the deleted
scope, invalidates all targeted data and recovery material, consumes
authorization, retires the lease, advances and seals the writer epoch and
barrier generation, and enters `PurgedUninitialized`; only authenticated new
provisioning may later create an operational store.

After any destructive effect, an incomplete purge atomically records the
first and last completed effects, deleted and remaining material digests,
continuation scope, immutable original-owner digest, exact
implementation/mechanism/fault-manifest continuation contract, retired lease,
consumed authorization, next epoch, and terminal handoff receipt, then enters
`PurgeOnlyBlocked`. Its only successor is a new `PurgeClosing` created by a
`BeginPurgeContinuationV1` capability bound to that handoff, immutable
continuation contract, original owner digest, remaining material, continuation
scope, principal, and epoch. Recovery, compile, and normal management can never
resume from that state. An interrupted
`PurgeActive` resumes the exact same attempt and phase after restart. The
lifecycle state, exclusion record, and terminal receipt live outside the bytes
being purged. Every purge binds its exact implementation and complete nonempty
mechanically derived `PurgeMechanismManifestV1` and
`PurgeFaultBoundaryManifestV1`. Any mismatch leaves the current state and
authority unchanged.

Every advertised update tuple binds a finite nonempty content-identified
`QuarantineInputCoverageManifestV1`. Its canonical cells are mechanically
derived from every update step and fault boundary that can emit
`UpdateQuarantined`. Each cell has an exact-one membership predicate over the
emitting boundary and failed stage, established program-and-memory state,
quarantine-record schema, verified exact-old-pair backup state, writer epoch,
and exclusion or lease-handoff state, plus at least one exact reachable
fixture. Every admissible quarantine input matches exactly one cell before
recovery; zero or multiple matches, an unreachable cell, or an unrepresented
quarantine-emitting boundary invalidates candidate freeze.

The tuple also binds the exact recovery implementation artifact, a finite
nonempty content-identified
`QuarantineRecoveryMechanismManifestV1`, and its mechanically derived nonempty
`QuarantineRecoveryFaultBoundaryManifestV1`. They enumerate and cover the
entry and exit of every executable recovery step, durable or externally
visible effect, quarantine-marker transition, epoch or lease operation, and
access-visibility change. Boundary coalescing requires the same
atomic-durable-primitive evidence as `UpdateTransactionV1`; an unbound step,
effect, or empty manifest is invalid. Its closed results are:

- `QuarantineRecovered`, after the exact old pair is restored and verified,
  one crash-atomic terminal handoff binds the restored pair, installation
  manifest, configuration registry, old and new writer/barrier generations,
  retired runtime-generation set, and a newly allocated empty runtime-
  registration generation; clears the quarantine/pending marker; retires every
  prior runtime registration; installs that empty generation as the terminal
  operational generation; releases the lease; and requires a new registration
  in that exact generation before compile plus authenticated management access
  resume; or
- `QuarantineRecoveryFailed`, which preserves the quarantine record and keeps
  compile and normal management mutation blocked without weakening the exact
  old-pair backup or future recovery basis. Its crash-recoverable terminal
  handoff atomically re-establishes the durable quarantine exclusion, retires
  the dedicated recovery lease, consumes the attempt-bound recovery
  capability, and advances the writer epoch before a later capability may be
  authorized. Failure to complete that handoff remains recovery-in-progress
  and admits neither normal access nor another recovery attempt.

Quarantine recovery never counts as update success and cannot choose an
unverified target pair. For every advertised update tuple and every
quarantine-input cell, `RCV-01` runs one uninterrupted recovery plus every
frozen recovery fault boundary on an exact candidate-bound fixture. Every
execution either
restores the exact old pair and resumes access or preserves the original
quarantine and verified recovery basis while access remains blocked after the
failed-recovery handoff retires the lease and capability. Restart at every
handoff boundary must complete the same terminal state before new admission.
The update matrix still rejects `UpdateQuarantined`; fixtures produced for
recovery-input coverage cannot satisfy a per-cell update-success or
update-fault obligation.
The authenticated `NoShippedPredecessor` branch has no update-quarantine
recovery matrix; its initial-publication recovery path is separate. Lease loss,
epoch drift, expiry without safe renewal, or an unreconstructible writer state
cannot commit, claim rollback, or claim quarantine recovery.

`IF-RCV-RECEIPT` binds the exact lease/epoch evidence, every tuple-scoped
update, quarantine-input, and recovery implementation/manifest identity, and
the complete update and recovery fault dispositions.

Every release candidate also binds one branch-matching
`PublicationRecoveryContractV1`, the exact branch implementation, a complete
nonempty `PublicationRecoveryMechanismManifestV1`, its mechanically derived
complete nonempty `PublicationRecoveryFaultBoundaryManifestV1`, and a finite
nonempty `PublicationSurfaceCoverageManifestV1`. The surface manifest
partitions every advertised product, channel, platform, support endpoint, and
publication surface into canonical exact-one cells with at least one exact
fixture. Each cell also binds one closed
`PublicationExposureDispositionV1`: either complete evidence that no external
installation obtained the candidate, or a privacy-preserving complete
inventory in which every exposed installation has exactly one verified
exact-predecessor restoration or local-quarantine disposition. An unknown,
offline, delayed, still-active, merely notified, or otherwise unresolved
installation cannot count as `PublicationRolledBack`. That result requires
either complete no-exposure evidence or exact predecessor restoration at every
exposed installation. Any local-quarantine disposition requires the terminal
result `PartialPublicationQuarantined` after complete containment. Each local
receipt binds the candidate/predecessor branch, installation, store,
authoritative memory revision, lifecycle transaction, admission drain, runtime
retirement, quarantine basis when applicable, and terminal handoff. The
mechanism enumerates every effectful distribution stop, endpoint/channel mutation,
eligible-predecessor rollback, exposure enablement, retrieval, download or
installation receipt, inventory, local restoration or quarantine, byte/status
verification, authoritative-memory preservation, incident, notification,
mitigation, and terminal-receipt step. Boundary coalescing again requires one
proven atomic durable primitive.

`RCV-01` executes the selected publication-recovery implementation
uninterrupted and at every frozen fault boundary for every surface cell.
Eligible-predecessor success requires verified predecessor restoration on
every publication surface plus no exposure or exact predecessor restoration at
every exposed installation. First-release containment, failed predecessor
rollback, and any locally quarantined installation require verified
distribution stop, withdrawal, complete conservative inventory,
authoritative-memory preservation, notification, mitigation, and quarantine.
An interruption, nonterminal effect permit, or unresolved exposure remains
durable `PublicationRecoveryPending`, admits no distribution, new publication
attempt, or G10 success, and resumes the same branch after restart. An
incomplete or empty implementation, mechanism, fault, or surface manifest, an
unrepresented surface, permit, local lifecycle transaction, or affected
installation, or a missing terminal result blocks authorization.

`IF-SHIP-AUTHORIZATION` binds the exact candidate, claim envelope, complete
`IF-RCV-RECEIPT` identity and digest, all tuple/cell and publication-surface
results, and `RollbackDispositionV1`. Any identity or disposition change
expires authorization.

Before the first distribution effect, `REL-03` atomically derives a
least-privilege `PublicationRecoveryCapabilityV1`. It is dormant until the same
attempt and state epoch enter `PublicationRecoveryPending`, valid only there
for stop, rollback, inventory, notification, mitigation, and quarantine, and
atomically consumed by `Shipped`, `PublicationRolledBack`, or
`PartialPublicationQuarantined`. Cleanup therefore never depends on continued
ship authority and no recovery authority survives a terminal state.

Every distribution effect consumes one bounded, single-use
`DistributionEffectPermitV1` issued from a durable
`PublicationAuthorizationLeaseV1`. The lease binds the exact authorization
identity and digest, candidate, recovery disposition, attempt, authorization
status epoch, permitted surfaces and effects, issuance time, and an expiry no
later than the authorization. Lease issue, renewal, revocation, expiry, effect
permit issue, permit completion, and terminal consumption share one
authority-owned linearization state and durable permit ledger. Revocation or
expiry blocks later permits; a permit that wins first is explicitly
non-revocable only for its one declared effect and bounded lifetime. Every
permit binds one execution adapter and effect-commit generation. The effect
can commit only through that adapter. The adapter atomically acknowledges
either effect commit or abort-before-commit; recovery may instead advance the
surface generation and acknowledge a durable commit fence proving the old
generation can never commit. Expiry, process loss, timeout, or conservative
exposure classification alone is not a fence and leaves the permit
outstanding. A surface without an acknowledged commit-or-fence boundary is
unsupported. Shipment and recovery inventory cutoff require every issued
permit to have a terminal commit, abort, or fence receipt. No free check/use
interval exists.

`Shipped` is created only by an atomic terminal compare-and-consume against the
same live lease and status epoch after every issued permit is terminal; the
successful revalidation, lease consumption, recovery-capability retirement,
state transition, and receipt share one linearization point. Lease loss,
expiry, uncertain or unfenced permit, or failed terminal comparison enters
`PublicationRecoveryPending`. Rollback and quarantine continue through the
same attempt/epoch-bound recovery capability. Their terminal transitions also
consume that capability atomically. `IF-SHIPMENT`, `PublicationRolledBack`,
and `PartialPublicationQuarantined` bind the authorization and lease
identities, complete permit ledger, terminal linearization result, retired
recovery capability, recovery contract, typed local lifecycle receipts, and
exposure disposition. A terminal publication receipt without those bindings
cannot close G10 or become post-V1 release evidence.

## Rationale

Read quiescence cannot protect an authoritative store from a concurrent
writer. A durable writer epoch and exclusive lease make the source pair stable
and restart behavior fail closed. Binding the verification receipt into
authorization preserves the evidence chain through shipment.

## Alternatives

- Rely on process-local compile cancellation. This does not exclude a separate
  management process or restart.
- Let compile open a snapshot without admission. The update barrier could
  close after its read-side scan and miss a newly opened snapshot.
- Admit purge and recovery independently. Either path could invalidate the
  other's recovery basis or restore data after a completed purge.
- Recheck only the revision ID before commit. A writer may already have
  produced effects or invalidated the backup basis.
- Copy update fields into authorization without the receipt identity. This
  permits a self-consistent authorization detached from the verified run.

## Consequences

Memory management exposes narrowly scoped update and purge exclusion, durable
pending state, active-pair-bound non-authority-bearing compile admission, and
one store-lifecycle recovery-or-purge admission state. `MEM-03` owns their
admission, epoch, lease, restart, and release mechanics.
`MEM-04` owns update execution, terminal finalization, and the separate
old-pair quarantine-recovery transaction and manifests. Candidate verification
must test concurrent writers, preflight/admission ordering, idle old runtimes,
compile-admission races, healthy and quarantined purge, purge/recovery races,
lease loss, epoch drift, expiry, restart, every quarantine-input and recovery
boundary, every publication surface, exposure disposition, effect permit,
revocation race and recovery boundary, access resumption, and every terminal
handoff. Shipment authorization becomes invalid when its exact verification
evidence changes.
