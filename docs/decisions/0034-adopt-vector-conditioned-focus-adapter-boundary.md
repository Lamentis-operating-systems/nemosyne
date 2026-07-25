# 0034: Adopt the vector-conditioned focus-adapter boundary

Status: Accepted
Date: 2026-07-25

## Context

The current renderer contracts make a selected `FocusExpectationPlan` the
source of renderable meaning, but they prematurely select one learned
implementation family: typed projectors, a latent resampler, a soft prefix,
and a local causal decoder. They also do not expose one explicit checked
boundary at which the current numerical query context and the relevant,
weighted semantic memory vectors condition focus generation.

That omission permits two incorrect interpretations. A renderer could receive
past memory prose as prompt context and call the result vector-conditioned, or
it could treat one lossy aggregate embedding as if it preserved source,
weight, role, uncertainty, and exact-value semantics. Neither interpretation
matches the product hypothesis. Nemosyne is intended to construct new,
situation-appropriate focus from numerical memory representations, not to
reconstruct or quote the text from which those representations were derived.

The plan, authoritative source projections, and independent validator must
remain distinct. Relevance does not establish truth. Provenance does not grant
authority. A generated focus does not authorize a tool, choose an action, or
answer the original request.

## Decision

Adopt one architecture-neutral, vector-first conditioning boundary between the
selected plan and any learned focus renderer.

`FocusExpectationPlan` carries bounded
`VectorConditionedFocusSemanticsV1` whose canonical bytes are included in
`PlanCanonicalEnvelopeV1` and therefore in `PlanContentId`. For one compile,
the orchestrator combines that canonical value with request-local custody and
source-binding state in a sealed `VectorConditionedFocusInputV1<'call>`.
Request-local state is not plan content.

The live input owns a private, noncloneable, nonserializable
`ConditioningInstanceWitness<'call>` and exposes two disjoint least-privilege
borrows branded by that witness:

- `AdapterConditioningViewV1`, containing only model-visible numerical and
  closed categorical conditioning from the canonical semantics; and
- `FocusConditioningValidationViewV1`, containing authoritative source
  semantics plus request-local provenance, custody bindings, exclusions,
  authority ceilings, dependency closure, and other validator-only controls.

Formally, \(C_V=(C_V^{sem},C_V^{bind})\).
`VectorConditionedFocusSemanticsV1` contains \(C_A\) and \(C_V^{sem}\);
`PlanContentId` commits to both. \(C_V^{bind}\) and the instance witness are
excluded from canonical plan bytes. The witness can affect only live-object
join validity, never model features, canonical order, scores, output bytes, or
semantic equality.

The complete checked composite contains:

- the renderer-safe numerical task and situation projection copied from the
  sealed `BoundQuery`;
- a finite, duplicate-free, canonically ordered set of the actual semantic
  vector artifacts supporting selected memory-origin plan items;
- one finite activation weight in `[0,1]` for each memory-vector item, copied
  from the admitted activation result;
- explicit facet-presence masks, vector-space and encoder identities,
  canonical adapter handles, selected plan-item roles and relations, and
  closed missingness and uncertainty qualifiers;
- request- or situation-origin numerical items required for a valid
  empty-memory or mixed-source plan;
- safe exact-slot metadata without exact payload bytes; and
- the resolved language, finite output bounds, and the authenticated adapter
  compatibility schema.

The canonical validator semantics contain a total bijection from every dense,
contiguous `AdapterPlanItemHandleV1` to its `PlanItemSemanticKey`, required
qualifiers, permitted slots, and authoritative semantic ceiling. The adapter
sees only the handle and safe model-visible fields. Request-local provenance,
record identities, custody receipts, and source-instance bindings occur only
in \(C_V^{bind}\).

Every memory vector is copied from the verified, record-version-bound derived
artifact already present in the same `EligibleActivatedMemorySet` used to
construct the plan. The conditioning builder neither re-encodes memory text
nor performs a store lookup. Artifact identity and custody establish origin
and integrity, not the truth of the encoded proposition. Authoritative
projections and planning remain the sole owners of semantic compatibility,
validity, disclosure, uncertainty, and authority ceilings.

The set has value semantics. Permuting its source order cannot change the
canonical input or a deterministic result. Duplicate source bindings,
nonfinite values, incompatible vector spaces, unknown facets, missing
presence masks, cross-query or cross-plan bindings, and absent required
weights fail before model execution. A zero weight is distinct from an absent
or unknown weight. Activation weight means only bounded request-local
relevance; it is not truth, probability, confidence, authority, utility,
safety, or action priority. A high weight cannot remove a qualification or
raise an authority ceiling.

The model-visible conditioning path contains no:

- raw or normalized memory text;
- nearest-neighbor recovery of stored text;
- decimal serialization of vectors as a text prompt;
- original user-prompt tokens or bytes;
- exact names, paths, dates, numbers, locations, URLs, or identifiers;
- opaque record, user, or provenance identities that could become memorized
  semantic features;
- validator-only exclusion or policy text; or
- tool, action, authorization, persistence, retrieval, or network capability.

Exact values continue to use separately authorized deterministic slots. The
original prompt continues to be appended byte-identically only after the
attention text has passed independent validation.

No public or adapter-crate API projects validator-only fields from
`AdapterConditioningViewV1`, accepts an arbitrary replacement validation view,
or reconstructs either view from identifiers. Only the compile orchestrator
may derive both borrows from the same checked composite.

Authenticated preflight continues to own one complete
`AuthenticatedRendererConfiguration`. The orchestrator derives two
nonowning, unconstructible borrows from it:

- `AdapterConfigurationViewV1`, containing only the selected adapter family,
  compatible input/output schemas, candidate checkpoint and numerical runtime,
  finite resource bounds, and candidate-required decoder/tokenizer fields
  represented by closed present/absent variants; and
- `ValidationConfigurationViewV1`, containing the structural and semantic
  validator artifacts, calibration, thresholds, corpus identities, limits,
  and the fields required to check candidate compatibility.

Both views carry the same private `RendererConfigurationId` and exact full
\(K_R\) commitment. They are not independently authenticated configurations,
cannot be widened, serialized, reconstructed, or substituted for
`AuthenticatedRendererConfiguration`, and expose no accessor for the other
view. The adapter therefore cannot inspect verifier artifacts, calibration,
thresholds, evaluation-corpus identities, or validator-only controls.

The conceptual checked interface is:

```rust
fn vector_conditioning_input<'call, 'plan>(
    plan: &'plan FocusExpectationPlan<'call>,
) -> Result<VectorConditionedFocusInputV1<'call, 'plan>, FocusConditioningError>;

trait VectorConditionedFocusAdapter {
    fn condition<'call, 'plan, 'configuration>(
        &self,
        input: &AdapterConditioningViewV1<'call, 'plan>,
        configuration: &AdapterConfigurationViewV1<'configuration>,
    ) -> Result<UntrustedBoundedFocusShapeV1<'call, 'plan>, FocusAdapterError>;
}
```

This signature fixes semantics, ownership, and failure boundaries; it does not
select a Rust crate layout. `VectorConditionedFocusInputV1`, both conditioning
views, both configuration views, and `UntrustedBoundedFocusShapeV1` have
private fields and checked constructors. The shape constructor borrows the
private conditioning witness and full-\(K_R\) commitment through the two
adapter views and retains only opaque equality bindings derived from them;
adapter code cannot supply either binding independently.

Every adapter result has one common canonical `FocusSupportTraceV1`. The trace
contains a finite, canonically ordered list of contiguous `FocusShapeNodeId`
values and a finite canonical relation list. Each assertion-bearing node
declares:

- one closed focus/expectation surface role;
- a nonempty duplicate-free set of admitted `AdapterPlanItemHandleV1` values;
- the exact required qualifier keys it claims to preserve;
- any permitted `RendererSlotId` references; and
- its candidate-output unit or shape-node range.

Relations use a closed relation tag and existing node IDs. The independent
validator resolves every handle through the total \(C_V^{sem}\) mapping before
checking support. Unknown, forged, out-of-range, duplicate, or noncanonical
handles; a handle remapped to another semantic key; duplicate nodes; empty
support for an assertion-bearing node; unknown qualifiers, slots, or relation
tags; dangling relations; overlapping or out-of-range units; missing required
qualifier coverage; and any cardinality or byte-bound excess are typed
failures. The trace contains no raw source text, semantic key, exact payload,
opaque provenance identity, validator-only control, or authority-raising
field.

`UntrustedBoundedFocusShapeV1` consists of the plan and renderer-configuration
bindings, an opaque `ConditioningBinding<'plan>` derived from the private
conditioning witness, resolved language and finite bounds, this common support
trace, one registered adapter-output schema identity, and a finite
candidate-specific payload accepted by that authenticated schema. The
validator accepts it only with validation and configuration views carrying
matching opaque bindings. Shared render-domain code represents the witness
join only as an equality-comparable `ConditioningBinding<'plan>` in the
candidate and validation view. The
validator can test equality but cannot read, construct, clone, serialize,
order, hash, or use the witness as a semantic feature. It is ephemeral,
untrusted, and incapable of raising input authority. A deterministic
lexicalizer may consume a registered structural-shape payload. A fused
adapter-decoder instead returns a registered bound-text payload inside the
same `UntrustedBoundedFocusShapeV1` envelope. Every adapter family therefore
uses the one checked trait and common trace/binding contract. No adapter
constructs `RenderedAttention<'plan>` directly: a compiler-owned deterministic
wrapper validates the registered payload, recomputes the plan and complete
renderer-configuration bindings, and seals the slot-bearing candidate without
performing learned generation. Neither output is a new truth record, memory,
plan, answer, action, authorization, or chain of thought.

The empirical implementation family remains open. Direct projection, weighted
pooling, set encoders, cross-attention, learned resampling, soft prompting, and
other registered bounded architectures are candidates. Each candidate
configuration binds its complete input schema, architecture family, tensor
inventory, dimensions, precision, decoder integration, deterministic runtime,
resource functions, and checkpoint bytes into the authenticated renderer
configuration. No projection, resampler, prefix length, decoder, model family,
or checkpoint is selected by this decision.

Training examples use the same field-level runtime adapter view and pair it
with the validator view and both least-privilege configuration views only in
the harness. Raw
memory text may exist only in a separately governed annotator or evaluation
custody plane and is never a feature, hidden teacher input, control prompt,
lookup target, or decoder context. Targets contain a bounded focus shape or
focus text plus source, qualifier, exclusion, and leakage labels. Dataset
splitting occurs by shared semantic and source root before encoding,
translation, paraphrase, augmentation, or teacher rendering. All derivations
of one memory record, proposition root, query scenario, or exact surface stay
in one split.

Frozen qualification compares at least:

1. the deterministic controlled renderer;
2. query-only conditioning;
3. memory-only conditioning;
4. the identical vector set with uniform weights;
5. the identical vector set with permuted order and shuffled or zeroed
   weights;
6. a registered weighted pooling or linear projection;
7. a small registered set adapter; and
8. every more complex learned family proposed for selection.

Raw-memory-text prompting, vector-to-text reconstruction, and
vector-to-nearest-text-to-decoder pipelines are forbidden product candidates,
not selectable baselines. A sealed offline oracle may measure the information
gap only when it cannot enter training or deployment selection.

Acceptance evidence binds the exact query and vector schemas, encoder
revisions, dataset roots, candidate and decoder artifacts, seeds, resource
target, validator, baselines, multiplicity procedure, thresholds, and
counterexamples before sealed outcomes are read. It must demonstrate:

- permutation invariance and duplicate non-amplification;
- query conditionality and relevant-memory contribution beyond query-only;
- rejection before adapter execution of unauthorized, unselected, or
  incompatible input members;
- noninterference of unauthorized or incompatible records outside the sealed
  valid input, plus noninterference from admitted irrelevant and zero-weight
  members inside it;
- preservation of uncertainty, missingness, conflict, authority ceilings, and
  required source attribution;
- no unsupported claim, original-text reconstruction, exact-byte invention,
  raw-source canary copying, answer leakage, action language, tool
  authorization, or authority escalation;
- finite language, byte, token, latency, memory, cold-load, and unload bounds;
  and
- downstream benefit against the strongest passing simple non-oracle baseline
  without exceeding the frozen harm ceiling.

Thresholds and comparison rules are frozen before sealed evaluation. Failure
selects a simpler passing renderer, narrows the contract prospectively, or
stops G4. It never weakens truth, authority, disclosure, uncertainty, leakage,
or action boundaries.

Interface evidence includes compile-fail or equivalent visibility tests
showing that an adapter implementation cannot read validator-only provenance,
authority projections, exclusion controls, or exact payloads and cannot
construct or replace either conditioning or configuration view. Tests also
prove that validator artifacts, thresholds, calibration, and corpus identities
cannot affect adapter execution; that foreign or reconstructed conditioning
witnesses fail; and that forged, out-of-range, duplicate, remapped, or
noncanonical adapter handles fail. Output conformance tests exercise every
`FocusSupportTraceV1` structural error independently and prove equivalent trace
enforcement for structural-shape and bound-text payloads.

This decision supersedes Decisions 0015, 0016, 0019, and 0023. It re-adopts
their combined focus-and-expectation plan, sealed compile-integrity and
canonical-content identities, dependency-light render domain, exact slots,
independent validation, deterministic baseline, exhaustive configuration
identity, complete learned-state inventory, and evidence-based selection
rules. It narrows the former full-configuration access to commitment-bound
least-privilege views, adds a live conditioning-instance join, and replaces
the premature latent vector-prefix choice with empirical adapter candidates.

## Rationale

A canonical vector-first input makes the actual product hypothesis testable:
the generated focus must depend on the current query and admitted numerical
memory evidence without receiving the source prose. Including the input in the
existing plan identity prevents a candidate or validator from silently using
a different vector set while avoiding another request-local identity domain.

Keeping authoritative propositions and independent validation outside the
adapter prevents a fluent model from turning relevance into truth or
authorization. Keeping exact values outside the lossy path avoids asking a
small model to reconstruct bytes that embeddings do not preserve.

An architecture-neutral boundary permits the simplest sufficient mapping to
win. A projection or small set adapter may be adequate; a cross-attention or
soft-prefix family must earn its additional resource and verification cost.

## Alternatives

- **Pass memory text to the decoder.** Rejected because it tests retrieval plus
  prompt injection rather than vector-conditioned focus and exposes stored
  prose to the model.
- **Invert each embedding back into its source text.** Rejected because the
  mapping is lossy, privacy-sensitive, model-specific, and contrary to the
  goal of constructing a new focus.
- **Use one untyped aggregate vector.** Rejected because it erases source,
  weight, role, missingness, vector-space, and qualification boundaries.
- **Keep the latent resampler and soft prefix as the selected V1 design.**
  Rejected because no Nemosyne-specific evidence yet shows that this family is
  superior to simpler or differently structured adapters.
- **Let the adapter select relevant memories or decide truth.** Rejected
  because retrieval, activation, authoritative projection, and planning own
  those decisions and have separate evidence.
- **Generate only free text without support trace.** Rejected because
  independent validation could not bind claims and qualifiers back to the
  admitted numerical inputs and authoritative plan.
- **Expose exact values as ordinary vector features.** Rejected because their
  byte-exact recovery is not guaranteed and could introduce invention or
  privacy leakage.

## Consequences

The plan schema and renderer dataset must gain an explicit vector-conditioning
view. Existing latent-resampler formulas remain useful only as one registered
candidate family and no longer define the universal renderer contract.
Renderer qualification must compare query, memory, weight, order, leakage,
and simple-adapter counterfactuals on byte-identical frozen inputs.

Any implementation that accepts raw memory prose or reconstructs stored prose
before decoding is nonconforming even if its final text appears useful. Any
learned candidate must publish a complete adapter-specific tensor and resource
inventory; artifacts from the superseded bridge partition cannot silently
qualify a different family.

The additional plan fields increase bounded request-local state and
qualification work. They do not change the public one-call API, the
byte-identical prompt suffix, local database ownership, semantically read-only
compile contract, or the separation among relevance, truth, authorization,
and action. No adapter implementation, model, training result, or product
benefit is claimed by this documentation decision.
