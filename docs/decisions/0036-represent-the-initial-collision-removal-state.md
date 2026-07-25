# 0036: Represent the initial collision-removal state

Status: Accepted
Date: 2026-07-25

## Context

Decision 0032 made collision containment and physical resource cleanup
separate. It required every durable `CollisionTerminalRemovalStateV1` to carry
one last-step outcome: `Committed`, `Aborted`, or
`ReconciliationRequired`.

The removal state is created with containment, before a removal step has been
attempted. None of those three outcomes truthfully represents that interval. A
crash after containment but before the first step would therefore force
recovery either to fabricate an outcome or to persist a state outside the
closed schema.

## Decision

`CollisionTerminalRemovalStateV1` carries one closed `step_state`:

- `NotStarted { initial_cursor }`;
- `Committed { prior_cursor, next_cursor,
  exact_resource_closure_receipts }`;
- `Aborted { unchanged_cursor, verified_no_effect }`; or
- `ReconciliationRequired {
  unchanged_or_last_committed_cursor, durable_recovery_fence_id }`.

The crash-atomic containment transaction creates the removal state as
`NotStarted`. Its `initial_cursor` and top-level `next_cursor` are exactly the
canonical start cursor committed by the immutable removal scope. For
`CanonicalRevokeSet`, that cursor is ordinal zero in the canonical revoked
resource sequence. For `FencedGeneration`, it is the scope's canonical initial
generation cursor. `NotStarted` carries no closure receipt, no no-effect
witness, and no recovery fence.

The top-level `next_cursor` is a redundant integrity check, never an
independent source of progress. It equals the active variant's effective
cursor in every valid state:

- `NotStarted.initial_cursor = next_cursor`;
- `Committed.next_cursor = next_cursor`;
- `Aborted.unchanged_cursor = next_cursor`; and
- `ReconciliationRequired.unchanged_or_last_committed_cursor = next_cursor`.

Before a first-step effect can become durable or externally visible, execution
atomically replaces `NotStarted` with exactly one step outcome or durably
installs `ReconciliationRequired`. A committed step advances from the stored
cursor, requires `Committed.prior_cursor` to equal the pre-state
`next_cursor`, and atomically writes both post-state cursor copies to the same
advanced value while binding its exact receipts. An aborted step proves no
effect and writes `Aborted.unchanged_cursor`, pre-state `next_cursor`, and
post-state `next_cursor` as the same value. An ambiguous step enters
`ReconciliationRequired` and blocks another step until reconciliation proves
the last committed cursor; that cursor equals the post-state `next_cursor`,
while any unproved effect remains covered by the durable recovery fence. Later
steps replace one step outcome with the next outcome under the same rules.

Once the state leaves `NotStarted`, it cannot return. Idempotent containment
replay preserves any existing progress and never resets it. Restart from
`NotStarted` repeats the first-step attempt at the same initial cursor only
after proving that no step effect could have occurred. Restart from another
variant resumes or reconciles that exact state. Construction and replay reject
a `NotStarted` value with an advanced or mismatched cursor, step-result
evidence, or a transition history that has already attempted a step. It also
rejects every mismatch between the top-level cursor and active variant, a
committed `prior_cursor` unequal to the pre-state cursor, an abort that changes
the cursor, or a reconciliation cursor other than the last proven committed
cursor.

This decision refines only the initial-state omission in Decision 0032. Its
quarantine basis, limits, bounded-step, terminal-origin, fence, recovery,
authority, and no-product rules remain accepted unchanged.

## Rationale

A closed initial variant makes every durable point between containment and
cleanup representable without weakening boundedness or idempotence. Creating it
in the containment transaction eliminates an unjournaled handoff. A canonical
start cursor makes replay deterministic for both removal scopes.

## Alternatives

- **Treat initial state as `Aborted`.** Rejected because no step was attempted
  and no no-effect witness exists.
- **Use an optional last outcome.** Rejected because optional fields admit
  combinations that the closed state machine excludes.
- **Create the removal record lazily at the first step.** Rejected because a
  crash before that step would leave contained resources without durable
  cleanup ownership.
- **Add a synthetic committed zero-work step.** Rejected because it fabricates
  an execution and closure evidence.

## Consequences

Memory, architecture, proof, package, interface, and recovery tests must cover
containment-to-first-step crashes, repeated restart from `NotStarted`, both
scope-specific initial cursors, every legal first transition, cursor
monotonicity, equality of every redundant cursor copy, and rejection of
malformed or regressive states.

Because this accepted refinement changes reviewed source and the governed
finding and decision inventories, the same revision appends `DOC-CONF-26` and
advances only the corresponding checker constants and regression fixtures.
It does not rewrite Decision 0032, rebaseline protected history, rename the
stable `DOC-CONF-24` G0 record, or claim implementation evidence.
