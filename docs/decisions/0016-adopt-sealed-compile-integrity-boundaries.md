# 0016: Adopt sealed compile-integrity boundaries

Status: Accepted
Date: 2026-07-24

## Context

Decisions 0014 and 0015 establish one memory-grounded focus-and-expectation
plan and one qualified lexicalization boundary. The refined contracts now
require several joins that lifetimes, digest equality, or prose ownership
alone cannot enforce:

- every numerical consumer must observe one complete `BoundQuery`, rather than
  independently supplied projections;
- focus and expectation must borrow the exact same complete
  `EligibleActivatedMemorySet`, rather than equivalent reconstructions;
- runtime call membership must not become semantic or content identity;
- renderer candidates, exact substitution, compiler validation context, and
  validator input must remain bound to the same plan content and deployed
  renderer configuration;
- an observed equal identifier with different canonical bytes must be
  detectable and quarantined; and
- every failed join must have one closed typed owner and must produce no
  partial semantic result;
- externally configured input limits alone must not permit an oversized
  request to be materialized before the compiler can reject it; and
- the in-memory all-or-error compile contract must not be misrepresented as
  physically atomic delivery through an operating-system byte stream.

`PlanContentId` cannot safely carry both plan semantics and renderer
configuration. A Rust lifetime prevents use after the borrowed plan is gone,
but it does not prove that two separately borrowed values denote the same
content. A digest is a compact identity, not a substitute for exact-byte
comparison when both canonical envelopes are available.

The independent semantic validator must remain independent. Giving it compiler
types, call witnesses, a raw plan, or context-construction authority would let
the component being validated control its own trust boundary.

Public API values and stream-backed CLI inputs cross different allocation
boundaries. An API caller or process launcher may already have materialized an
owned value before Nemosyne receives it, whereas a CLI file or standard-input
source can be bounded while it is read. The contract must state this difference
rather than claim that every source is streamed.

## Decision

Adopt the following sealed compile-integrity boundaries for V1.

### Aggregate and shared-instance boundaries

All downstream numerical stages accept the complete sealed `BoundQuery`.
Private views are projected only inside the receiving call. No public or
sibling-private API accepts independently supplied numerical, exact-binding,
lineage, or configuration projections.

`COMP-01` constructs one complete immutable
`EligibleActivatedMemorySet<'call>`. The compiler passes the exact same object
and borrow, including its inseparable content lineage, private invocation
witness, and fresh private `EligibleSetInstanceWitness<'call>`, to both the
focus and expectation branches. The set-instance witness is minted exactly
once for that object, has only same-instance comparison, and is neither
cloneable, serializable, content-derived, nor observable as bytes. Each branch
propagates the exact witness into its result. The compiler-private planning
scope independently borrows the expected set witness from the selected set,
not from either branch, and checks both branch results against it. A projection,
copy, filter, or semantically equivalent reconstruction therefore receives a
different witness and is not an acceptable branch input, even inside the same
authenticated invocation.

### Instance witnesses and content identities

`InvocationInstanceWitness` proves only membership in one live authenticated
compile invocation. It is private, nonserializable, excluded from semantic
ordering and scoring, and excluded from content identities and product bytes.
It is propagated where current-call membership must later be checked and then
erased before independent semantic validation.

Content equality is represented separately by typed canonical content
identities. Two valid objects from different invocations may be
content-identical while their witnesses remain noninterchangeable.

### Absolute ingress boundaries

`AbsoluteIngressLimitsV1` is a versioned public contract containing finite
positive byte ceilings for every byte-bearing request field and the complete
canonical request. `TGT-01` freezes the exact V1 values before request or CLI
implementation. An authenticated installed configuration may lower a ceiling
but can never raise the public absolute maximum.

Public request constructors check already-owned values against every field and
aggregate ceiling before request construction or any further internal
allocation. This bounds compiler-owned retention, but it does not claim to
control allocations made by the caller, shell, operating system, or process
launcher before the API boundary.

The CLI reads file and standard-input prompt sources into a bounded buffer of
at most the prompt ceiling plus one detection byte. The extra byte proves
overflow; the adapter closes the source, does not read or retain the remainder,
and returns `AbsoluteInputLimitExceeded`. Direct argument text is already
materialized at process entry and is checked before request construction. All
paths use the same exact maximum and maximum-plus-one boundary fixtures, while
only stream-backed paths claim bounded pre-materialization reads.

### Semantic result and transport boundary

The compiler returns one complete in-memory compiled prompt or one typed error.
No compile, renderer, substitution, or validation failure returns a partial
semantic value.

An adapter completely serializes and buffers the successful value before
delivery and writes zero standard-output bytes before delivery begins. Once an
operating-system write starts, physical rollback is not promised. A
`write_all` or `flush` failure may expose a byte prefix; the adapter stops,
returns transport exit `10`, and requires the caller to discard every emitted
byte. Such a call is unsuccessful even though the transport may retain a
prefix. “No partial result” therefore describes the semantic API result and
pre-delivery behavior, not atomic external-stream delivery.

### Plan and renderer identities

`PlanCanonicalEnvelopeV1` is the versioned, domain-separated, injectively
encoded canonical projection of all product-relevant plan semantics, controls,
structure, cost, language, budget, and exact-surface identities and bytes. It
excludes invocation witnesses, request-local instance identifiers, and
receipt-only runtime lineage.

The complete compiler configuration \(K\) is projected into a separately
authenticated plan-semantic configuration \(K_S=\pi_{\mathrm{plan}}(K)\).
`SemanticConfigurationId` commits to every schema, registry, encoder,
retrieval, signal, activation, focus, expectation, and planning field that can
change plan meaning or selection. It excludes \(K_R\), renderer and validator
execution fields, serializer and transport settings, and the full
configuration identity. The canonical plan source projection uses the
configuration-independent request and situation content digests \(d_R,d_S\),
the complete lineage-independent semantic content actually selected into the
plan, and `SemanticConfigurationId`. It excludes configuration-bound
`request_id`, `situation_id`, `BoundQueryContentId`, \(B_Q\), \(\Lambda_A\),
branch instance IDs, receipts, and the full `configuration_id`. Consequently,
a \(K_R\)-only change cannot change `PlanCanonicalEnvelopeV1` or
`PlanContentId`; a \(K_S\) change may.

`PlanContentId` is derived from those canonical bytes. Trusted constructors
retain a private exact-byte comparison commitment whenever a later same-ID
join can occur. An observed equal `PlanContentId` with different canonical
bytes is `PlanContentIdentityCollision`; it is quarantined and never treated as
equality.

Renderer implementation, tokenizer, lexical grammar or model, precision,
decoding, artifacts, exact platform-dependent execution selection, and every
configured resource limit or other field capable of changing emitted bytes
are not plan content. Together they form the exact authenticated renderer
configuration \(K_R\) and are sealed separately as
`RendererConfigurationId`. Equal `RendererConfigurationId` plus byte-identical
authenticated canonical \(K_R\) content means equal byte-affecting renderer
configuration and, for equal plan content, byte-identical deterministic
output. ID equality alone is insufficient. A target-platform class groups
claims and evidence only; it is not an execution identity and cannot stand in
for \(K_R\) or `RendererConfigurationId`.
`RenderedAttention`, `SubstitutedAttention`, and the compiler-private
`ValidationContext` carry both `PlanContentId` and
`RendererConfigurationId`. A renderer-configuration mismatch is never hidden
inside plan identity.

`AuthenticatedRendererConfiguration` is the sole immutable shared-domain
representation of \(K_R\). Authenticated compiler preflight constructs it once
from the selected trusted renderer, validator, tokenizer, artifacts, settings,
and limits. Correctness is defined by exact authenticated canonical-content
equality, not Rust referent identity. The compiler normally lends the one
preflight value throughout a call, but a separately authenticated value is
equivalent exactly when both its canonical \(K_R\) bytes and
`RendererConfigurationId` are equal. A projection, narrower validation
configuration, unauthenticated reconstruction, caller-created replacement, or
same-ID/different-byte value is invalid.

The sealed type exposes only bounded read access, its
`RendererConfigurationId`, and a private exact canonical-byte comparison
commitment. `RenderedAttention`, `SubstitutedAttention`, and
`ValidationContext` retain that commitment beside the ID. Substitution
compares its candidate with the supplied authenticated configuration; the
validator compares the candidate, the least-privilege read-only view projected
from the private context, and the supplied authenticated configuration. Both
comparisons use the ID and exact canonical content. Any disagreement, including
equal ID with different bytes, is
`RendererConfigurationMismatch`, quarantines the configuration path, and
fails closed. The type carries no trust-resolution, update, filesystem,
network, registry, installation, or artifact-mutation capability, and
downstream consumers cannot refresh or mutate it.

### Candidate construction, substitution, and pre-validation

The checked renderer constructor recomputes the plan canonical envelope and
seals its ID, comparison commitment, and the resolved
`RendererConfigurationId`. Exact substitution compares, in fixed order, the
candidate's sealed renderer configuration, plan identity, and available exact
canonical-byte commitment before interpreting slots or content.

`RendererSubstitutionError` is a closed eleven-variant error:

1. `RendererConfigurationMismatch`;
2. `PlanIdentityMismatch`;
3. `PlanContentIdentityCollision`;
4. `UnknownSlot`;
5. `ForbiddenSlot`;
6. `SlotBindingMismatch`;
7. `SlotOccurrenceMismatch`;
8. `ExactSurfaceUnavailable`;
9. `InvalidExactSurface`;
10. `InvalidExactPlacement`; and
11. `RendererCostBoundViolation`.

`RendererCostBoundViolation` is owned by substitution because the substituted
candidate is the first complete value whose exact product cost can be checked.
It is not reclassified as a validator failure.

Before the independent validator is called, one compiler-owned pre-validation
adapter compares retained exact canonical-plan bytes only when candidate and
private context carry the same `PlanContentId`. Equal ID with different bytes
is the standalone typed `PlanContentIdentityCollision`, maps to
`InternalInvariantViolation` and CLI exit `70`, quarantines the attempt, and
prevents the validator call. A different `PlanContentId` is not converted into
a compiler collision or mismatch; it proceeds to the independent validator.
The adapter does not compare renderer configuration and does not perform
semantic validation.

The independent validator keeps no compiler dependency. It receives only an
opaque substituted candidate, a least-privilege witness-free read-only view,
and an authenticated renderer configuration with exact canonical content equal
to the preflight-selected \(K_R\), after the compiler-owned plan collision
check has passed. There is no narrower, projected, or unauthenticated
validation configuration. It derives or verifies both
`RendererConfigurationId` and the exact canonical-content commitment from
\(K_R\), then compares candidate, validation view, and supplied configuration.
It owns
`RendererValidationError::PlanIdentityMismatch` for different
`PlanContentId` values and
`RendererValidationError::RendererConfigurationMismatch` for configuration
disagreement; both map to `InternalInvariantViolation` and CLI exit `70`.
It cannot construct the context, receive a call witness or raw plan, import
compiler-private types, or return an accepted result that callers can inject
back into compilation. `ValidationContextError` remains the closed
five-variant builder domain; the builder retains the exact-byte comparison
capsule but cannot observe a candidate/context collision by itself.

Every constructor, substitution, pre-validation join, and semantic-validation
failure has one closed typed source, one exhaustive public `CompileError` and
CLI-exit mapping, fixed precedence, and preserved source information. No
failure returns rendered bytes, a substituted candidate, a verdict, compiled
prompt bytes, or any other partial success.

This decision refines Decisions 0014 and 0015. It does not supersede either
decision and does not change the observable single-result product contract.

## Rationale

Aggregate-only inputs make mixed-projection states unrepresentable at public
and sibling-private boundaries. Passing one shared-set instance prevents
branch-local reconstruction from silently changing eligibility, order,
lineage, or witness membership.

Separating invocation witnesses, plan content, and renderer configuration
gives each equality relation one meaning. Exact-byte comparison where both
canonical envelopes exist turns a digest-collision assumption into a
fail-closed observable boundary without exposing canonical plan content to the
validator.

The compiler-owned pre-validation adapter keeps exact-byte collision detection
close to the authority that possesses both commitments without stealing
ordinary identity or configuration mismatch ownership from the validator. The
validator remains a genuinely independent semantic checker with a one-way,
least-privilege dependency.

A closed error domain and no-partial-success rule make precedence, retryability,
telemetry, CLI behavior, and adversarial tests reconstructible.

## Alternatives

- **Use lifetimes as referent identity.** Rejected because lifetimes constrain
  validity and detachment, not content equality or object identity.
- **Compare only `PlanContentId`.** Rejected because an observed equal ID with
  different available canonical bytes must fail closed.
- **Put `RendererConfigurationId` inside `PlanContentId`.** Rejected because
  plan semantics and deployable renderer configuration have different owners,
  compatibility rules, and rollback lifecycles.
- **Include configuration-bound query or lineage IDs in `PlanContentId`.**
  Rejected because the full compiler configuration transitively includes
  \(K_R\); canonical plan identity instead uses \(d_R,d_S\), \(K_S\), and
  lineage-independent selected semantic content.
- **Let each branch receive a projection of the activated set.** Rejected
  because equivalent-looking projections can omit, reorder, or rebind content,
  lineage, or invocation membership.
- **Use only the invocation witness to prove shared-set identity.** Rejected
  because two reconstructed sets inside one invocation would share it; the
  separate set-instance witness proves the exact branch source.
- **Require one Rust renderer-configuration referent.** Rejected because
  referent identity is not observable through the current APIs and is not a
  semantic property of immutable authenticated configuration; exact canonical
  content equality is enforceable and sufficient.
- **Rely only on installed input limits.** Rejected because a mutable
  installation could enlarge the admissible public request domain and because
  stream-backed sources need a context-independent bound before configuration
  resolution.
- **Describe every input as streamed.** Rejected because direct arguments and
  owned API values may be allocated before Nemosyne receives them; the enforceable
  guarantee is bounded reading for stream-backed sources and no further
  internal allocation after an oversized owned value is detected.
- **Claim atomic standard-output delivery.** Rejected because an
  operating-system stream may accept a prefix before reporting failure;
  buffering before delivery plus exit-`10` invalidation is truthful and
  testable.
- **Let the validator receive the raw plan or compiler context.** Rejected
  because it breaks the independent dependency boundary and grants
  unnecessary authority and private data.
- **Let the compiler classify every candidate/context mismatch.** Rejected
  because only the compiler-private exact-byte capsule can establish the
  collision case, while ordinary plan/configuration mismatch belongs to the
  independent validator contract.
- **Use free-form error messages or partial candidates.** Rejected because
  callers could not exhaustively map failures and partial bytes could escape a
  failed trust boundary.

## Consequences

The plan, renderer, validator-view, compiler, CLI, proof, test, and delivery
contracts must carry the three distinct concepts of runtime instance,
canonical plan content, and renderer configuration.

Implementations require checked private constructors, canonical-envelope
metamorphic and collision fixtures, cross-configuration corruption cases,
compile-fail dependency tests, exhaustive eleven-variant substitution tests,
pre-validation join tests, fixed error-precedence tests, and proof that failed
calls expose no partial result.

Request and adapter implementations additionally require exact field and
aggregate max/max-plus-one fixtures, ceiling-plus-one bounded-read tests for
file and standard input, proof that an oversized owned value causes no further
internal allocation, zero-standard-output-before-delivery tests, broken-write
and flush-failure tests, exit-`10` prefix invalidation, and documentation that
caller-side preallocation and external-stream rollback remain outside the
guarantee.

Rollback must treat the exact authenticated \(K_R\), renderer and validator
artifacts, and `RendererConfigurationId` as one compatibility selection while
leaving plan content identity stable across compatible renderer changes.

These boundaries make invalid joins observable; they do not prove semantic
fidelity, usefulness, collision resistance of an unselected hash, or end-to-end
product safety. Those claims remain gated by implementation and retained
evidence.
