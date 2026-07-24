# 0031: Complete compile and update admission handoffs

Status: Accepted
Date: 2026-07-25

## Context

Decision 0028 selected one durable compile, writer, update, recovery, and purge
admission barrier. Its detailed contracts nevertheless left five conflicting
or incomplete boundaries:

- the normative compile sequence authenticated the invocation and then resolved
  active-pair-dependent controls without acquiring the selected compile ticket;
- the product and proof contracts prohibited every persistent compile-side
  state transition while the selected barrier required durable content-free
  admission records;
- committed and rolled-back update results required access-resumption probes,
  but normal admission reopened before those probes and had no failure
  transition;
- normal update finalization retired every runtime-registration generation
  without allocating the replacement required by `Operational`; and
- successful quarantine recovery entered `Operational` before exact-generation
  registration even though normal compile and authenticated management access
  were required to remain closed until that registration.

These are contract defects rather than implementation evidence. Leaving them
open would make the compile order internally contradictory and the lifecycle
state machine impossible to implement without choosing undocumented behavior.

## Decision

Decision 0028 is superseded. This decision retains its writer exclusion,
linearized compile admission, unified store lifecycle, update/recovery/purge
mutual exclusion, publication authorization, permit terminalization, exposure
disposition, and fail-closed restart rules except where the refinements below
replace its compile and lifecycle handoffs.

Decision 0029 remains accepted. Its tuple- and cell-indexed result cardinality
applies unchanged to the retained update and release-verification contracts;
its references to Decision 0028 denote those retained contracts as superseded
and completed here.

The fixed per-call order is:

1. retain and intrinsically validate the complete request;
2. derive only configuration-independent presentation identities;
3. authenticate one sealed invocation;
4. acquire exactly one active-pair-bound `CompileAdmissionTicketV1`;
5. inside that admitted scope, resolve and pin every active-pair-dependent
   control and artifact, open the matching immutable memory snapshot, execute
   the compile core, and close every handle and snapshot; and
6. after every bound handle and snapshot closes, terminalize the durable record
   and consume that call's `CompileAdmissionTicketV1` on normal success, compile error,
   cancellation, or unwind before returning. If removal cannot be proven, the
   provisional result or core error is suppressed and the sole returning
   cleanup exception is a typed no-product admission-finalization error that
   leaves the affected store fail closed. Process loss returns no result;
   startup reconciliation instead terminalizes the record or keeps it
   conservatively generation-fenced until the old runtime cannot survive.

No active-pair-dependent configuration, policy, disclosure, artifact, runtime,
or memory handle may be resolved before step 4.

Admission rejection follows one closed `CompileAdmissionErrorV1` source
taxonomy and creates no record or coordination-state change. Coordination
availability, lifecycle, executing-program, active-pair, installation,
registry, runtime-generation, replay, and active-registry-ceiling checks run in
that fixed order. Every source except `ActiveAdmissionLimitReached` returns
public `CompileError::AdmissionUnavailable` and CLI exit `4`; the ceiling
source returns `CompileError::ResourceFailure` and exit `8`. Lifecycle closure
may become retryable only after an external lifecycle-state change, and the
ceiling source only after the active registry is observed below the configured
limit. Binding mismatch, replay, and unreconstructible coordination state are
not automatically retried by the compiler or CLI.

Terminalization uses the separate closed
`CompileAdmissionTerminalizationErrorV1` taxonomy. Binding-digest,
per-record-sequence, and live-resource violations map to
`InternalInvariantViolation` and exit `70`; missing-record, unavailable-state,
and unknown-removal-outcome failures map to
`AdmissionFinalizationFailure` and exit `4`. This mapping takes precedence over
any provisional core result or error, returns no product bytes, and permits no
automatic complete-call retry. The active record remains visible when its
presence is known; otherwise coordination is explicitly unavailable. Normal
admission and every exclusive lifecycle operation remain closed until startup
reconciliation or separately authorized repair establishes one exact state.

Authenticated empty-store provisioning and every normal update or recovery
handoff create a fresh **empty** runtime-registration generation. They do not
invent a live compiler. On every ordinary process start, `Compiler::open`
retains only bootstrap resolver/trust capabilities, authenticates its executing
program and local runtime instance, and submits one
`RegisterOperationalRuntimeV1` command. `MEM-03` atomically verifies current
`Operational` state, completed startup reconciliation, exact active
installation and executing program, current registration generation, replay,
and capacity before creating one content-free registration record and opaque
runtime ticket. Registration linearizes with lifecycle closure; it returns no
active manifest, configuration, policy, artifact, or memory handle. The atomic
registration is the last fallible open step, so successful registration is
followed only by infallible construction of the public `Compiler`.

The registration ticket is revalidated at each later compile admission and is
retired atomically with its generation on update or recovery. Each record has a
canonical content-free binding digest and monotonic record-state sequence; its
ticket privately carries the expected generation, digest, and sequence.
Conditional close, recovery-fence, and whole-generation-retirement commands
compare those values before changing state; generation retirement additionally
compares the lifecycle-state sequence and complete generation-binding digest.
Fencing advances the record sequence, close removes only the exact record after
all work closes, and generation retirement atomically removes every old record
and generation from the live registry. No per-runtime terminal row remains.
Runtime state retains
only the single bounded content-free retirement receipt required by the current
or pending lifecycle handoff; longer-lived audit evidence is external.

Public `Compiler::close(self)` returns idempotent success only when an
authenticated receipt proves that the ticket's complete generation was already
retired. Absence from a still-current generation is a binding failure. The CLI
closes before stdout delivery. Its close failure suppresses a provisional
compile result or error; binding or active-work failures map to exit `70`, while
coordination or unknown-removal failures map to exit `4`. Non-panicking `Drop`
is best effort and cannot claim removal. Abrupt loss or failed drop leaves a
bounded recovery-fenced registration until liveness proof permits removal or a
lifecycle handoff retires the generation.

Registration source checks run in this fixed order: coordination availability,
`Operational`, completed startup reconciliation, executing program, active
pair, installation, registry, current generation, replay, then capacity. Close
checks coordination availability, exact retirement proof, generation, record
presence, digest, sequence, active work, then durable removal outcome. The
first applicable source wins. Stale, cross-binding, replayed, or unregistered
runtimes cannot compile. Registration coordination/lifecycle failure maps to
`RuntimeRegistrationUnavailable` and exit `4`; binding/replay failure maps to
`InvalidInstallation` and exit `5`; capacity maps to
`OpenResourceFailure` and exit `8`. No open, close, or compile path
automatically retries.

The persistent state contract is partitioned into:

- **semantic product state**, containing memory, provenance, policy, derived
  representations, indexes, caches, installed artifacts, semantic diagnostics,
  and any state visible to retrieval, planning, rendering, or the product
  result; and
- **operational coordination state**, containing only the content-free
  lifecycle, barrier, epoch, runtime-registration, and admission records needed
  to exclude updates, recovery, purge, and admitted work across restart.

Compilation is read-only over semantic product state. It may perform only the
closed operational admission transitions required to create and terminalize
one ticket record. Terminalization crash-atomically removes the active record;
no per-invocation terminal row accumulates, and a configured concurrent-
admission ceiling bounds the active registry. Persistent record and
cancellation identities are domain-separated derivations of store, barrier,
and monotonic operational sequence values only; the exact invocation is bound
by a runtime-private call brand that is never persisted. Such a record contains
no prompt, situation statement,
metadata payload, memory content, derived semantic value, output, or authority;
it is unavailable to semantic computation and cannot trigger memory creation,
learning, caching, indexing, telemetry publication, or artifact mutation. The
proof obligation is semantic-state invariance plus membership of every durable
compile-side transition in that closed coordination allowlist.
Any runtime diagnostic produced during compile is bounded, redacted,
request-local, and nonpersistent and cannot affect the result or
terminalization. Persisting a diagnostic requires a later separately
authorized post-call operation selected by its own accepted decision.

Normal commit or rollback first enters durable
`UpdateTerminalVerificationPending`. The transition:

- keeps normal compile and authenticated management admission closed and
  retains the update exclusion owner;
- binds the exact candidate outcome and terminal pair;
- advances writer and barrier generations;
- retires every captured runtime-registration generation; and
- allocates and installs one fresh empty runtime-registration generation.

Only an attempt-bound terminal-verification capability may register a runtime
in that exact generation and execute the prescribed full compile and
authenticated management-readiness probes while this state owns the store.
`MEM-03` owns the `UpdateCompileProbeDriverV1` dependency-inversion callback
and narrow verification-scope contracts inside `IF-MEMORY-MANAGEMENT`;
`API-01` supplies the sole production driver, and `MEM-04`
consumes it without importing the compiler crate or receiving normal compile
authority. G5 verifies the update state machine with the contract's conformance
driver; G8 verifies the exact compiler producer; release support requires their
exact packaged binding and RCV execution. The compile probe accepts only the
candidate-frozen non-user fixture, reuses the exact registered compiler
pipeline, closes every handle and snapshot, discards all product bytes, and
persists neither fixture nor output-derived content. Management-readiness
performs only the closed authenticated `OpenManagementReadinessView`
operation. Neither probe returns a semantic payload, creates a normal caller
admission or general management capability, or mutates semantic state.

One candidate-frozen versioned probe contract determines each exact request
identity, finite resource envelope, closed success predicate, and receipt
derivation. Each in-flight operation is durably registered before a handle or
view opens and removed only after all resources close and the exact receipt is
durable. If restart observes both the execution record and its exact durable
receipt, it never reruns the probe: after old resources provably cannot survive,
a matching passed receipt removes the record and advances verification, while a
matching failed receipt removes the record and enters quarantine. Without a
receipt, restart reattaches or conservatively fences the execution and may not
retry until old resources provably cannot survive. Both receipt variants and
the exact registration receipt bind the same request, transaction, pair,
installation manifest, configuration registry, writer/barrier generations,
and runtime-registration generation. A passed receipt carries the observed
binding digest and proves byte- and identity-equal semantic state before and
after. A failed receipt carries a closed failure reason plus explicit observed
binding and semantic-state comparison dispositions: every available digest or
before/after identity is retained, while an unavailable or unequal observation
is represented as such and is never inferred as equality. The receipt identity
binds the result discriminant and complete canonical result payload. Any
mutation, mismatch, unavailable required observation, missing result,
unresolved prior execution, or failed resource bound is a failed probe.
The V1 reference schema closes the failure and observation-unavailability
reason domains, fixes first-applicable precedence, and assigns explicit
canonical discriminants. Platform error text and implementation enum layout
never enter receipt identity.

When every probe succeeds, one crash-atomic terminal handoff records the
resumption receipt, clears the pending marker, retires the update lease, and
enters `Operational` with that registered generation. Only then does normal
compile and authenticated management admission reopen. A missing, failed,
mismatched, or unreconstructible registration or probe transitions
deterministically to `UpdateQuarantined`; it never opens normal admission or
claims `UpdateCommitted` or `UpdateRolledBack`. Later restoration remains the
separate exact-old-pair quarantine-recovery transaction.

Successful quarantine restoration transitions
`RecoveryActive → RecoveryRegistrationPending`, not directly to
`Operational`. The crash-atomic restoration handoff binds the exact old pair,
advances writer and barrier generations, retires every prior runtime
registration, and allocates and installs one fresh empty runtime-registration
generation while retaining exclusive lifecycle ownership. Only a narrow
attempt-bound registration capability may add a runtime to that exact
generation. Normal compile and authenticated management access remain closed.
Exact registration then permits one crash-atomic
`RecoveryRegistrationPending → Operational` handoff that retires the recovery
lease and reopens both normal paths. Missing, stale, mismatched, failed, or
unreconstructible registration remains pending or reaches the existing
fail-closed `QuarantineRecoveryFailed` handback; it cannot reopen access.

Every registration, probe, pending-state, success, failure, restart, and access-
visibility boundary is part of the appropriate complete mechanism and fault-
boundary manifest and is exercised by candidate verification.

## Rationale

Admission before pair-dependent resolution gives update closure and compile
acquisition one enforceable linearization order. Separating semantic state from
content-free coordination preserves the product's no-learning and immutable-
memory boundary without pretending that a crash-recoverable exclusion protocol
has no durable state.

Pending states make access visibility an outcome of verified handoffs rather
than an assumption between state transitions. Fresh generations remove stale
runtime ambiguity after both normal update and quarantine recovery. Keeping
exclusive ownership until exact registration, and for normal update until the
required probes complete, provides a deterministic failure path without
exposing an unverified pair.

## Alternatives

- Keep every persistent state byte unchanged during compile. This cannot provide
  the selected restart-reconstructible active-ticket registry and was rejected.
- Treat admission records as semantic memory writes. This would unnecessarily
  weaken the read-only memory and no-learning product boundary.
- Reopen normal access and run probes afterward. This leaves no owner or
  transition able to contain a failed probe.
- Reuse or infer the prior runtime generation after pair switch. This permits
  stale runtimes and leaves `Operational` without a canonical current
  generation.
- Enter `Operational` after quarantine restoration and block only compile.
  This contradicts the selected management-access barrier and permits a normal
  mutation before exact-generation registration.

## Consequences

The product and proof specifications must define semantic-state invariance and
the closed operational-coordination exception precisely. The compiler API must
acquire admission immediately after authentication and before control
resolution.

The store lifecycle gains `UpdateTerminalVerificationPending` and
`RecoveryRegistrationPending`, a normal-update terminal handoff that installs a
fresh empty generation, and typed registration, probe, success, and failure
receipts. `MEM-03`, `MEM-04`, `API-01`, `RCV-01`, release packaging, and
verification must cover every new boundary and restart state.

The extra durable coordination and verification steps add implementation and
test cost. They add no product output, memory truth, semantic score, release
claim, or evidence that V1 is implemented.
