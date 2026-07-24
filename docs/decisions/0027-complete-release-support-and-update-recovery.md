# 0027: Complete release support and update recovery

Status: Accepted
Date: 2026-07-24

## Context

The closed release graph allowed a shipped release to become `Superseded`,
`Withdrawn`, or `EndOfLife`, but provided no later lifecycle transition from
`Superseded`. A normally superseded release could therefore never record a
later emergency withdrawal or the end of its support window.

The program also required update rehearsal without defining observable update
success, interruption recovery, program-and-memory coordination, or a
fail-closed terminal state. Mechanism selection remained open, but the missing
invariants made an update receipt non-reconstructible.

## Decision

A `Superseded` release may transition to `Withdrawn` or `EndOfLife` only
through an authenticated lifecycle record that binds the release, channel
status, effective time, support disposition, reason, replacement or recovery
guidance, and retained user notification evidence. The transition never
changes prior shipment evidence.

Every supported update mechanism implements one closed `UpdateTransactionV1`
over an authenticated current program-and-memory pair and one exact target
candidate. The candidate binds the exact implementation artifact plus a finite,
nonempty, content-identified `UpdateMechanismManifestV1` and its mechanically
derived `UpdateFaultBoundaryManifestV1`. The mechanism manifest enumerates the
ordered executable steps, state transitions, durable or externally visible
effects, active-pair visibility changes, lease operations, and concurrency
boundaries. The fault manifest contains the entry and exit boundary of every
such step and transition. A boundary may be coalesced only when independent
evidence proves the enclosed operation is one atomic durable primitive with no
intermediate observable state. An empty manifest, an unbound implementation
step, or an effect without a boundary is invalid.

The transaction must:

1. authenticate current and target identities and compatibility;
2. quiesce new compiles and drain or cancel active compiles under a declared
   bounded policy;
3. retain the complete old program, readable memory revision, required keys,
   and a verified backup before target mutation;
4. stage and verify target program bytes and any source-to-target memory
   transformation without exposing a mixed active pair;
5. make exactly one verified old pair or one verified new pair active;
6. recover every frozen fault-manifest boundary to one of those complete pairs;
   and
7. enter terminal `UpdateQuarantined` with compile unavailable when neither
   complete pair can be proven.

The closed outcomes are `UpdateCommitted`, `UpdateRolledBack`, and
`UpdateQuarantined`. `UpdateCommitted` and `UpdateRolledBack` are reached only
after one crash-recoverable terminal handoff records the exact active pair,
clears the pending-update marker, advances the writer epoch, and makes the
exclusive lease releasable. The release is idempotently completed before
compile or management mutation resumes. Failure to finish that handoff becomes
`UpdateQuarantined`. Only `UpdateCommitted` is update success.
`UpdateRolledBack` proves restoration of the exact old pair and preserves the
failed-attempt record. `UpdateQuarantined` records distribution stop,
installation and memory identities that could be established, backup
location/identity, user notification, and recovery guidance without claiming
that either pair is usable. It is terminal for the original update transaction;
any later recovery is a separate transaction.

The implementation mechanism may use an atomic pointer, staged directory,
package-manager transaction, or another selected design, but it must prove the
same invariants and complete frozen fault-boundary matrix. `OD-27` selects that
mechanism and its manifest construction; it does not weaken this contract.

## Rationale

Release status evolves after supersession, and an update spans both executable
artifacts and user-owned memory. A closed outcome contract permits mechanism
choice while making success, rollback, and quarantine objectively
reconstructible.

## Alternatives

- Treat `Superseded` as terminal. This prevents truthful later withdrawal and
  EOL records.
- Call package-manager exit zero an update receipt. This does not prove memory
  compatibility, active-pair integrity, or interruption recovery.
- Modify program and memory in place. A crash can expose an unverifiable mixed
  pair.
- Claim rollback whenever recovery is incomplete. This hides data and
  installation uncertainty.

## Consequences

Release packaging carries update compatibility and recovery metadata.
Independent candidate verification checks manifest completeness against the
packaged implementation and executes the update interruption matrix, including
memory migration, terminal lease release, access resumption, and old-pair
restoration. A release cannot be authorized when update success, recovery, or
manifest completeness is ambiguous.
