# V1 reference architecture

Status: Proposed

## Purpose

This specification proposes the logical architecture needed to implement the
Nemosyne V1 product contract. It defines component responsibilities, data-flow
boundaries, trust boundaries, memory-revision semantics, failure classes, and
the decisions that must be resolved before production implementation.

This remains a proposed logical decomposition rather than an implemented or
validated product. Decisions 0012 and 0013 nevertheless select the intended V1
implementation path: typed numerical memory and query facets, request-local
focus consolidation, a canonical numerical attention plan, and a qualified
local vector-prefix renderer. Physical database, encoder, index, process,
packaging, release-model, and production-runtime choices remain independently
evidence-gated.

The architecture has four maturity labels:

- **Accepted boundary**: behavior already selected by an accepted decision.
- **Required property**: a constraint derived from the product contract that
  every conforming architecture must preserve.
- **Proposed boundary**: the current logical decomposition to be evaluated.
- **Open choice**: an implementation or policy decision that remains unset.

## Definitions

### Compile inputs and result

A compile request contains the original prompt `P`, zero to three situation
statements `S`, and request metadata `X`, including a declared contextual time
`t_context`. An authenticated invocation context `U` identifies the trusted
local principal and caller and supplies a trusted authorization time `t_auth`
outside the untrusted request payload. A pinned compiler configuration and
policy resolve the finite attention budget `B`.

The request is evaluated against one immutable logical memory revision `M^r`
and one immutable, content-identified compiler configuration `K`.

The only successful product result is the compiled text defined by the V1
product contract. Internal plans, source bindings, scores, and diagnostics are
not additional product results.

### Logical data flow

The proposed compile path is:

```text
Compile invocation
    |
    v
Local-principal and configuration resolution
    |
    v
Ingress validation and prompt-origin check
    |
    v
Immutable artifact preflight
    |---------------------------|
    v                           v
Situation encoding       Immutable memory revision acquisition
    |                           |
    |                    Authorization and disclosure view
    |                           |
    |---------------------------|
                |
                v
Request-compatible validity and historical-scope gate
    |
    v
Authorized candidate generation
    |
    v
Signal and gate derivation
    |
    v
Activation ranking
    |
    v
Proposition support and conflict consolidation
    |
    v
Budgeted attention planning
    |
    v
Canonical numerical attention plan
    |
    v
Typed latent resampling and virtual-prefix projection
    |
    v
Local non-thinking language-model rendering
    |
    v
Exact-slot substitution
    |
    v
Faithfulness and policy validation
    |
    v
Exact compiled-text serialization
```

These are logical boundaries. They do not imply one process, one crate per
stage, or a synchronous implementation.

Every logical component in this data flow is a **proposed boundary** unless
the table below states otherwise.

| Boundary | Maturity |
| --- | --- |
| Product input, result, read-only behavior, and local trust boundary | Accepted boundary from Decision 0011 |
| Exact framing and prompt-byte preservation | Required property from the product contract |
| Numerical memory, query facets, request-local focus consolidation, and focus narrative | Accepted implementation direction from Decision 0012 |
| Vector-prefix bridge, exact slots, local renderer qualification, and non-thinking generation | Accepted implementation direction from Decision 0013 |
| Ingress, preflight, snapshot, authorization, encoding, retrieval, derivation, planning, rendering, and validation decomposition | Proposed boundaries governed by the focused specifications |
| Existing activation kernel, evaluator, and corpus | Experimental implementations and evidence |
| Physical database and schema, exact encoders and indexes, calibrated parameters, release model and quantization, production runtime, processes, and resource thresholds | Open choices |

### Configuration and artifact preflight

The invocation boundary resolves the authenticated local principal, trusted
caller context, trusted authorization time, attention budget, applicable
policy, and one content-identified compiler configuration before validation.
After basic request validation and before persistent memory access, artifact
preflight:

- verifies an authenticated artifact manifest against a pinned installation
  trust root held outside the mutable artifact bundle;
- opens immutable handles to required encoder, tokenizer, renderer, validator,
  and schema artifacts; and
- pins content or implementation identities for principal resolution,
  prompt-origin validation, authorization, disclosure, temporal validity, and
  supersession policy evaluators; and
- verifies that every artifact is present, compatible, and integrity-checked.

The authenticated manifest establishes which identities are authorized;
content digests then establish that the opened bytes have those identities.
An unsigned self-consistent manifest is insufficient. No artifact may be
downloaded or replaced during compilation. Trust-root rotation, installation,
and update occur through a separately authenticated management path. A version
label without provenance, content identity, and an immutable handle is
insufficient because the underlying file could change during a call.

### Ingress validation

Ingress owns:

- original-prompt preservation;
- prompt-origin and caller preconditions;
- zero-to-three situation-statement validation;
- required contextual-time validation;
- metadata, language, and configured size validation;
- resolution of one output language for the complete call; and
- creation of an immutable request-local input object.

Ingress retains the original prompt bytes separately from every decoded,
normalized, tokenized, or numerical representation. No later stage may
reconstruct the product prompt from an encoder output.

### Principal and disclosure policy

V1 runs for one local user principal. Principal resolution establishes the
caller and ownership context before persistent memory is read. After revision
acquisition, the policy gate derives the revision-scoped view that determines
which records the caller may cause Nemosyne to read and disclose in derived
form. The architecture separates:

- permission to read;
- permission to disclose to the caller;
- source authenticity;
- current validity;
- confidence or uncertainty; and
- instruction authority.

None of these properties implies another. Authorization is evaluated before
candidate generation. A high relevance value cannot restore an excluded
record. Authorization, disclosure expiry, current normative validity, and
supersession are evaluated at `t_auth`. The caller-controlled `t_context` may
select explicitly historical context but cannot make historical instructions
currently authoritative.

The operating-system identity or another concrete ownership mechanism remains
an open choice. A V1 implementation must not silently share one memory universe
across principals.

### Immutable memory revision

One logical memory revision `M^r` is a self-consistent read view containing:

- authoritative records and stable record-version identities;
- provenance, authority, validity, uncertainty, and supersession state;
- authorization and derived-disclosure policy facts with a policy revision;
- exact values required for faithful reconstruction;
- manifests for rebuildable numerical representations; and
- every index used for candidate generation.

For one call, the compiler pins `t_auth`, invocation context `U`, memory
revision `r`, and policy revision `p`. It derives one call-specific authorized
view `M_A^(r,p,t_auth,U)`. Authorization expiry and disclosure decisions use
that same `t_auth`; current normative validity and supersession are also
resolved at `t_auth`. They do not use `t_context` or reread the wall clock.

Every derived artifact is bound to the authoritative record version, encoder
or transform version, and revision for which it is valid. A stale derived
artifact cannot be combined silently with a newer authoritative revision.

A concurrent management operation may publish `M^(r+1)`, but an in-flight
compile using `M^r` never observes it. Re-encoding, re-indexing, consolidation,
access-history updates, and cache publication are write or maintenance
operations; they are not hidden effects of compilation.

The proposed V1 rule is snapshot-stable authorization: a revocation published
after `M_A^(r,p,t_auth,U)` is acquired applies to later calls and does not
rewrite the authorization view of the in-flight call. Compile duration must
remain bounded. Immediate cancellation on revocation is an alternative that
requires a later privacy and concurrency decision before implementation.

### Memory planes

Decision 0012 selects a two-plane logical memory model.

The **authoritative exact plane** preserves immutable record-version and
canonical-proposition identities, provenance, validity, authority,
authorization, supersession, source-dependency groups, conflicts, and
loss-sensitive values. Its representation is numerical in the broad machine
sense: typed identifiers, enums, booleans, scalars, timestamps, coordinates,
relations, and byte-preserving payloads. It is lossless for every claim the
compiler may emit and never depends on inversion of an embedding.

The **derived numerical plane** contains versioned, rebuildable typed facet
vectors, calibrated scalars, numerical relations, and search indexes. It is
the sole computational state for similarity, activation, propagation,
consolidation, and adapter input, but it is not an independent source of
truth. Deleting or rebuilding this plane must not change the meaning of the
authoritative exact plane.

The exact physical representation remains open, but its contract must expose:

- stable memory identity and immutable record-version identity;
- source and import provenance;
- observed, created, valid-from, valid-until, and superseded times;
- authority and authorization labels;
- uncertainty and unresolved conflicts;
- exact entities, names, paths, numbers, and other loss-sensitive values;
- typed numerical facets and relations;
- transform, encoder, tokenizer, and index manifests; and
- logical deletion, physical erasure, export, migration, and repair state.

This list does not require one universal memory-object row or one physical
schema. The complete logical record and facet contract is defined in
[`cognitive-memory-activation-and-focus.md`](cognitive-memory-activation-and-focus.md).

### Situation encoding

Situation encoding converts `P`, `S`, `X`, and `t_auth` into a versioned
numerical query state `Q`. The query contains separate facets for the declared
contextual time and trusted authorization time so relevance can use temporal
context without granting the caller control over authorization or expiry. The
query state contains typed vectors, scalars, identifiers, presence masks, and
numerical relations corresponding to available memory facets, while retaining
source references and exact values outside lossy representations.

The encoder contract must define:

- input normalization that does not affect original-prompt preservation;
- vector spaces, dimensions, types, and normalization;
- exact scalar and categorical encodings;
- treatment of absent, unknown, and uncertain values;
- model and transform versions;
- deterministic or stochastic behavior;
- supported languages and modalities; and
- failure behavior for unavailable or incompatible artifacts.

The encoder does not decide instruction authority and does not retrieve memory.

### Authorized candidate generation

Candidate generation searches only the authorized view of `M^r`. It accepts
`Q` and produces a bounded candidate set `C^r` with source bindings and
retrieval diagnostics:

\[
C^r = G(Q, M^r_{authorized}; K_G)
\]

Project, workspace, application, time, and location may affect search and
ordering but are not undocumented exclusion predicates. Logical eligibility
does not require an exhaustive physical scan. Approximate retrieval therefore
requires a declared candidate budget and measured false-negative behavior.
Authorization is applied before bounded top-k or nearest-neighbor competition.
Adding, removing, or changing an unauthorized record must not crowd out an
authorized candidate or alter content-bearing diagnostics.

The retrieval contract must distinguish:

- no eligible or relevant candidate found;
- a successful bounded search;
- a known incomplete or degraded search; and
- a failed or incompatible index.

Empty candidates and retrieval failure are not equivalent.

### Signal and gate derivation

Signal derivation maps `Q` and every candidate to the normalized inputs required
by an activation mechanism:

\[
N = D(Q, C^r; K_D)
\]

It owns channel semantics, gates, evidence signals, inhibition signals, and
their provenance. It must not assign arbitrary numbers without an authored or
learned derivation contract and independent evaluation targets. Decision 0012
selects cue, temporal-context, base-availability, active-goal, procedural,
hazard, and social-perspective fit as the initial channel hypotheses when the
required facets exist. The focused specification defines their candidate
mathematics and the separation between hard policy gates and soft inhibition.

The five channels in the revision-1 coding-agent corpus are experimental
evidence labels. They are not the V1 memory ontology or an accepted runtime
channel set.

### Activation ranking

The existing deterministic activation kernel is the current implemented
candidate for this boundary. It accepts already normalized signals and returns
a complete bounded ranking of aggregate scores. A separate operation explains
one candidate with a per-channel breakdown. The kernel remains replaceable
until a later decision adopts it for V1 using end-to-end evidence.

If adopted without changing its current contract, candidate `i` receives:

\[
A_i =
\frac{\sum_c w_c g_c e_{i,c}}
     {\sum_c w_c g_c}
\cdot
\prod_j(1-\lambda_jp_{i,j})
\]

The formula is defined only when the evidence denominator is positive. The
kernel's values are activation scores, not truth, probability, safety, or
instruction authority.

Runtime compilation may depend on an adopted runtime kernel. It must not
depend on the offline evaluation or corpus crates.

### Proposition consolidation and attention planning

Request-local proposition consolidation first groups activated memory facets by
canonical proposition and source-dependency group. Duplicates from one
dependency group cannot multiply support. Independent corroboration remains
visible, and unresolved conflicting propositions remain separate. This stage
does not create, update, or delete persistent memory.

Attention planning then converts source-bound query evidence, consolidated
propositions, ranked candidates, and the resolved budget into a bounded
structured plan. Request, situation, and metadata evidence can therefore
support attention even when the memory candidate set is empty. The planner
owns:

- proposition selection;
- redundancy and diversity control;
- conflict and uncertainty preservation;
- authority ceilings;
- exact-value bindings;
- abstention and empty-attention decisions;
- inclusion priority; and
- allocation of the attention budget.

The planner produces empty attention only when no additional focus is
justified. If materially required context cannot be expressed faithfully
within `B`, planning fails with insufficient budget.

Decision 0012 selects a deterministic coverage-and-nonredundancy objective as
the first planning hypothesis. The focused specification defines its terms,
canonical tie-breaking, conflict-preservation rule, and mandatory baseline
comparisons. Its coefficients, budget, and promotion thresholds remain frozen
experiment parameters rather than defaults inferred at runtime.

### Canonical numerical attention plan

The internal attention plan is the single source of meaning for rendering and
diagnostics. It is a canonical set of typed numerical plan items rather than a
draft prose block. Each planned proposition contains at least:

- a stable proposition identity;
- the intended meaning;
- essential source references;
- whether each source comes from the request or authorized memory revision;
- source authority and the proposition's authority ceiling;
- validity, confidence, uncertainty, and conflict qualification;
- exact values that must survive rendering;
- inclusion priority and budget estimate; and
- allowed omission or mandatory-inclusion status.

The plan contains canonical control-only exclusions for content that must not
appear. They cannot be removed by output-budget optimization, but they are not
renderable items, substitution sources, or generative-prefix inputs. They are
available only to validation. The plan does not contain a draft answer to the
original prompt. Its renderable roles distinguish current situation, dominant
goal, immediate constraint or hazard, relevant background, secondary
influence, conflict, uncertainty, and social perspective. Exact output values
are referenced through authorized slots rather than encoded as content to
reconstruct.

The plan envelope also owns its schema identity, canonical item order,
mandatory and optional sets, exact-value sidecar, resolved output language, and
post-substitution rendering budget. These values have no second renderer-side
source of truth.

The plan is internal. It does not change the one-text successful product
contract.

### Vector-prefix adapter and renderer

The renderer accepts only the bounded numerical attention-plan envelope and
the compatible rendering configuration. It reads output language and budget
from that envelope and rejects a configuration-schema mismatch. It does not
receive the whole memory universe, raw memory prose, or decimal serializations
of plan vectors. It does not retrieve, rerank, select new facts, invent policy,
or answer the original prompt.

Decision 0013 selects a typed latent resampler followed by direct virtual input
embeddings as the first generative renderer hypothesis. The renderer
specification owns the experimental dimensions, tensor mapping, training
phases, and required simple baselines.

Its internal result is a `RenderedAttention` value containing:

- the slot-bearing attention text and token-origin map;
- a complete segmentation into output units; and
- untrusted bindings from every assertion-bearing output unit to planned
  proposition identities.

A closed surface-only class permits only whitespace, punctuation, and
configuration-listed structural delimiters; it cannot carry a connective,
relation, exact value, or independent semantic claim. Bindings are validation
input, not proof that the text expresses the identified propositions. They are
omitted from the successful product result.

The renderer emits only registered placeholder tokens for loss-sensitive exact
values. A deterministic resolver rejects unauthorized, unknown, omitted,
duplicated, or invented slots and substitutes the approved surface bytes
before final faithfulness validation.

A model-based renderer remains a fallible, untrusted transformation even when
it runs locally. Qwen3 is the first integration family, but the model
qualification specification owns the candidate slate, selection rule, resource
protocol, and release evidence. A deterministic template renderer remains a
mandatory baseline and may be a separately qualified renderer configuration
selected before a request. It is not an automatic substitute after another
renderer fails.

Renderer artifacts must be provisioned, versioned, integrity-checked, and
available before compilation. Download and update mechanisms run outside the
no-network compile path.

### Faithfulness and policy validation

Validation compares the post-substitution `RenderedAttention` with the
structured plan and receives a read-only view of the retained original prompt
and prompt-derived intent. It reads output language and budget from the plan
envelope. It rejects:

- unsupported propositions;
- omitted mandatory qualifications;
- authority escalation;
- answer leakage;
- forbidden or excluded content;
- language mismatch;
- budget overflow;
- malformed leading or trailing line breaks; and
- output that cannot be mapped back to planned propositions.

Validation verifies complete, nonoverlapping segmentation and known proposition
identities. It accepts the exact rendered text unchanged or returns an error.
It is not a second renderer.

Validation establishes conformance to a bounded plan, not truth of the source
memory. Decision 0013 selects a fail-closed hybrid contract: deterministic
structural, slot, and literal checks followed by an independently trained and
calibrated dual-branch semantic verifier. The focused renderer specification
fixes its inputs, independence boundary, classifier heads, threshold-selection
procedure, and failure semantics. Its exact encoder, dimensions, confidence
targets, and resulting thresholds remain frozen qualification-manifest
choices. Renderer self-attribution without independent checks is insufficient
evidence.

### Serializer and adapters

The serializer performs only the exact byte concatenation defined by the
product contract and uses the retained original prompt buffer directly. It
adds no suffix.

The programmatic API is the canonical semantic operation. A CLI is the proposed
first adapter for one-call local use, not yet an accepted V1 requirement. If
adopted, its exact flags and input transport must support:

- exactly one prompt, supplied without shell-induced mutation;
- zero to three repeated situation statements;
- explicit contextual time and optional metadata;
- the finite budget resolved by configuration and policy;
- compiled text only on standard output; and
- diagnostics and errors only on standard error.

The CLI, library, and any later application adapter share the same compile
orchestrator and error taxonomy.

### Configuration and reproducibility

One immutable compiler configuration `K`, together with its pinned artifact
handles, binds all behavior that can change an output:

- request and budget limits;
- memory-schema and revision compatibility;
- principal-resolution, prompt-origin, authorization, disclosure,
  temporal-validity, and supersession policy schema and evaluator identities;
- encoder and numerical-schema versions;
- index and retrieval configuration;
- signal schema and parameters;
- activation implementation and parameters;
- selection policy;
- renderer and tokenizer artifacts;
- decoding configuration;
- language support; and
- validator and serializer versions.

Diagnostics and evaluation receipts identify the content of `K` and its
artifacts without exposing private memory content. A change that can alter
semantics creates a new configuration revision and receives the required
specification and decision review.

### Failure taxonomy

A compile failure returns no compiled prompt. `CompileError` preserves a stable
class and an inspectable underlying stage or cause.

| Class | Representative causes |
| --- | --- |
| `InvalidRequest` | Invalid UTF-8, empty prompt, invalid time, excessive input, or too many statements |
| `PromptOrigin` | Caller cannot satisfy the authenticated prompt-origin precondition |
| `UnsupportedLanguage` | Language is absent, undetermined, or outside declared support |
| `AuthorizationUnavailable` | Caller trust or disclosure view cannot be established |
| `MemoryUnavailable` | Uninitialized, locked, unreadable, incompatible, or corrupt memory |
| `SnapshotUnavailable` | No coherent revision or a representation/index revision mismatch |
| `RepresentationFailure` | Missing encoder or schema artifact, incompatible schema, or invalid numerical state |
| `RetrievalFailure` | Search cannot meet its declared completeness contract |
| `ActivationFailure` | Invalid profile, signal, parameter, or numerical evaluation |
| `PlanningFailure` | Unresolvable selection, qualification, conflict, or abstention state |
| `InsufficientAttentionBudget` | Mandatory qualified attention cannot fit the resolved budget |
| `RendererFailure` | Missing renderer artifact, malformed output, or unsupported generation |
| `FaithfulnessFailure` | Unsupported claim, lost qualification, escalation, or answer leakage |
| `ResourceFailure` | A declared memory, deadline, or compute budget is exceeded at any stage |
| `PolicyViolation` | A compile component attempts prohibited network or persistent write access |

Adapter delivery errors are separate from `CompileError`. A
`TransportFailure` means compilation succeeded but an adapter could not deliver
the complete text. It remains an unsuccessful invocation. CLI standard-output
failure is one possible adapter-specific mapping.

Error ownership is stage-specific. A missing artifact belongs to the stage that
owns it; an externally enforced deadline or resource ceiling is
`ResourceFailure`; and an attempted prohibited action is always
`PolicyViolation`, even when it also causes another stage to fail. Focused
error specifications must resolve any remaining overlap before public error
codes are stabilized.

Memory import, correction, migration, deletion, and index-build failures belong
to the separate management plane.

### Decision register

The following decisions are already accepted and constrain this proposal:

- one local user principal and trusted caller;
- local persistent memory and one logical memory universe;
- authorization before relevance;
- read-only compilation over one immutable logical revision;
- structured numerical relevance computation after ingress;
- exact combined text with byte-identical prompt bytes;
- no required network service, autonomous discovery, downstream model
  invocation, or automatic learning during compilation; and
- coding agents as the first domain eligible for a supported V1 claim;
- typed numerical memory and query facets with a parallel authoritative exact
  plane;
- request-local proposition consolidation into a bounded numerical focus plan;
- an evidence-bound focus narrative rather than a claimed chain of thought;
- a direct vector-prefix bridge with deterministic exact-value slots; and
- a frozen, task-specific local-model qualification path before any release
  model is selected.

The proposed product contract additionally requires compilation without any
network access. This stricter boundary is a required property of this proposed
architecture, not an accepted decision, until a focused decision record adopts
an implementation that enforces it.

The following contracts must be decided before their production components are
implemented:

| Decision area | Required evidence before acceptance |
| --- | --- |
| Request and API | Boundary cases, exact time and metadata semantics, stable error behavior |
| Memory read and authority model | Provenance, validity, supersession, authorization, conflict, and exact-value cases over supplied revisions |
| Snapshot and derived indexes | Concurrent publication, revision binding, recovery, and corruption tests |
| Physical numerical representation | Encoder-specific reconstruction limits, perturbation tests, and artifact versioning under the accepted logical representation |
| Candidate generation | Recall, false-negative, cross-context, scale, and authorization measurements |
| Signal derivation | Grounded channel semantics, independent labels, sensitivity, and robustness |
| Activation adoption | Improvement over simpler ranking baselines on disjoint evidence |
| Attention planning | Accepted objective instantiated with coverage, exclusion, conflict, redundancy, abstention, and budget evidence |
| Renderer and validation | Accepted vector-prefix path compared with templates and MLP; model selected by faithfulness, leakage, language, exact-slot, downstream, and resource evidence |
| Runtime topology | Offline enforcement, packaging, failure isolation, and reference-hardware measurements |
| Release claim | Sealed end-to-end evaluation and all predeclared gates |

Database engine, physical schema, concrete facet encoders and dimensions,
index, release renderer model and quantization, production model runtime,
caching strategy, and process topology are chosen only after their owning
contracts and minimum evidence exist. The logical numerical representation and
renderer qualification path are no longer open.
Initialization, create/import, correction, revocation, deletion, export,
consolidation, migration, and recovery are separately scoped management
operations. Each requires a contract before its own implementation, but this
proposal does not make all of them prerequisites for compile V1.

## Preconditions

A conforming implementation requires:

- the accepted V1 product boundary;
- one initialized local memory universe for one principal;
- an authorization and disclosure view;
- a coherent immutable memory revision;
- installed compatible numerical and rendering artifacts with immutable
  content identities;
- one pinned versioned compiler configuration and artifact set;
- declared language, input, resource, and attention-budget limits; and
- a compile dependency boundary that exposes no network capability and
  performs no network access.

## Invariants

- Original prompt bytes flow only from ingress retention to serialization.
- One call uses one immutable compiler configuration and artifact set.
- Every artifact identity is authorized by an authenticated manifest anchored
  outside the mutable artifact bundle before its content digest is trusted.
- Every source used after authorization belongs to the pinned authorized
  revision.
- No derived representation or proposition has greater instruction authority
  than its essential supporting sources.
- Every planned and rendered proposition has source bindings and preserves
  material qualifications.
- No compile stage writes any persistent compiler state or performs a network
  call.
- Every index and numerical representation is compatible with the pinned
  authoritative revision.
- Empty attention, retrieval failure, renderer failure, and insufficient
  budget remain distinct outcomes.
- No stage silently substitutes missing data, guessed metadata, stale indexes,
  unsupported language, or truncated content.
- Offline evaluation artifacts are not runtime dependencies.
- One failure aborts compilation without a partial successful result.

## Edge cases

- An empty memory universe may still produce situation-supported attention.
- No useful request or memory context produces valid empty attention.
- An unauthorized record with perfect numerical similarity remains excluded.
- A cross-project record may be selected when it is relevant.
- A current-project record may be omitted when it is irrelevant.
- A stale index cannot silently supply candidates for a newer revision.
- A correction published concurrently affects only later compile calls.
- Under the proposed snapshot-stable rule, authorization revocation published
  after snapshot acquisition affects later calls; immediate cancellation
  remains an explicit alternative to decide.
- A relevant exact name, path, timestamp, or number must not be guessed from a
  lossy vector.
- Two copies derived from one source must not masquerade as independent
  corroboration.
- Conflicting propositions must not be averaged into a false compromise.
- A renderer must not expose a reasoning trace or label its focus narrative as
  human thought.
- Conflicting sources remain conflicting unless an accepted authority and
  supersession rule resolves them.
- Renderer inability to preserve a necessary qualification is a failure, not
  permission to weaken the claim.
- A budget just below the faithful minimum is an error; it does not justify
  truncation or empty attention.
- Missing model or encoder artifacts fail locally without a network download.

## Verification

Architecture conformance requires:

- request and serializer boundary tests;
- prompt-buffer aliasing or copy-path tests proving byte preservation;
- authorization-before-retrieval tests;
- memory-snapshot model tests with concurrent revision publication;
- authorization-revocation timing tests for the selected cancellation policy;
- representation and index revision-mismatch tests;
- cross-context candidate-generation tests and measured retrieval recall;
- signal-provenance and channel-grounding tests;
- existing activation-kernel verification where that kernel is used;
- attention-plan coverage, exclusion, conflict, abstention, and budget tests;
- renderer faithfulness, language, leakage, and qualification evaluation;
- network-blocked and persistent-write-detection integration tests;
- result isolation and transport-failure tests for every adopted adapter;
- resource measurements on frozen reference hardware; and
- end-to-end evaluation under the V1 proof program.

Any management operation added to the product requires its own specification
and evidence. Compile-path verification proves only that those capabilities are
absent from compilation and that supplied revisions obey the selected read
contract; it does not validate unimplemented management features.

The exact proof obligations, empirical hypotheses, metrics, gates, and stop
conditions are defined in
[`v1-proof-program.md`](v1-proof-program.md).

## Open questions

- Exact request types, whether the proposed CLI is adopted, its syntax, input
  limits, and budget unit.
- Time, location, project, workspace, application, and language metadata.
- Authoritative memory representation and minimum provisioning operation.
- Storage engine, encryption policy, filesystem ownership, and physical
  deletion.
- Artifact-manifest signature format, installation trust root, rollback
  protection, and authenticated update and recovery path.
- Snapshot-stable versus immediate in-flight authorization revocation.
- Concrete vector spaces, dimensions, encoders, and relation-learning method
  within the accepted typed-facet and exact-value boundary.
- Retrieval indexes, candidate budgets, and permitted false-negative rates.
- Calibrated runtime channel parameters, inhibition strengths, and
  normalization artifacts.
- Planning coefficients, item and cost penalties, mandatory-set policy, and
  attention budget.
- Public diagnostic authorization for the internal plan and bindings.
- Release renderer checkpoint, adapter configuration, quantization, validator,
  supported languages, and reproducibility level selected by qualification.
- Crate, process, service, packaging, caching, and platform topology.
- Resource budgets, release thresholds, and artifact distribution.

Each open choice requires a focused specification and, when selected for
implementation, a decision record. This proposal must not be treated as one
omnibus acceptance of those choices.

## References

- [V1 product contract](v1-product-contract.md)
- [V1 proof program](v1-proof-program.md)
- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Activation parameter evaluation](activation-parameter-evaluation.md)
- [Curated activation evidence](curated-activation-evidence.md)
- [Cognitive memory activation and focus](cognitive-memory-activation-and-focus.md)
- [Vector-to-attention renderer](vector-to-attention-renderer.md)
- [Local renderer model qualification](local-renderer-model-qualification.md)
- [Decision 0011: Adopt a local read-only attention compiler for V1](../decisions/0011-adopt-local-read-only-attention-compiler-v1.md)
- [Decision 0012: Adopt numerical cognitive memory and focus compilation](../decisions/0012-adopt-numerical-cognitive-memory-and-focus-compilation.md)
- [Decision 0013: Adopt a vector-prefix local renderer qualification path](../decisions/0013-adopt-a-vector-prefix-local-renderer-qualification-path.md)
