# Cognitive memory activation and focus

Status: Proposed

## Purpose

This specification defines the proposed cognitive-memory hypothesis for
Nemosyne V1. It describes how an already authorized, immutable memory revision
and one compile request can be represented numerically, activated by
situation-dependent cues, consolidated into request-local propositions, and
reduced to a bounded focus-candidate set.

Decision 0014 retains numerical memory, situation-conditioned activation, and
request-local consolidation while replacing the focus-only terminal plan
selected by superseded Decision 0012. This specification now owns only the
memory, activation, consolidation, and focus branch. The parallel expectation
branch is defined by the predictive-attention specification, and the only
renderer-facing plan is defined by the focus-and-expectation-planning
specification. The particular facet schema, derivation functions,
coefficients, and focus-candidate mechanism remain proposed pending
implementation and evaluation. This document does not select an embedding
model, vector dimension, database, index, renderer model, runtime, or
persistent consolidation policy.

The proposal is cognitively inspired. It does not claim to simulate a human
brain, consciousness, subjective experience, inner speech, or a literal chain
of thought. Human-memory research motivates the mechanisms and evaluation
hypotheses; it does not establish that the proposed equations are biological
implementations.

## Definitions

### Inputs and scope

For one compile invocation:

- `P` is the retained original prompt;
- `S` contains zero to three situation statements;
- \(\Xi\) contains caller-supplied request evidence: the declared contextual
  time `t_context`, optional declared location, and explicit metadata;
- \(\mathcal I_A\) is the sealed request-local authenticated invocation whose
  inseparable private projections include `I`, `t_auth`, \(\Gamma_A\), and
  `AuthenticatedPrompt`;
- `I` is the authenticated invocation-context projection of
  \(\mathcal I_A\);
- `t_auth` is the trusted authorization-time projection of
  \(\mathcal I_A\);
- \(\Sigma_{\mathrm{sig}}\) is the immutable minimized signal-derivation
  context projected only from \(\mathcal I_A\); it contains a private
  non-semantic call brand, the trusted instant, and a typed authenticated
  social-subject identity but no policy handle,
  authorization capability, disclosure view, store handle, or ambient clock;
- `M_A^(r,p_policy,t_auth,I)` is the immutable authorized and disclosable view
  of memory revision `r` under policy revision `p_policy`;
- `K` is the pinned compiler configuration and artifact set; and
- \(N_{\mathrm{focus}}>0\) and \(W_{\mathrm{focus}}>0\) are the finite
  focus-candidate cardinality and work budgets resolved by policy and
  configuration.

The output of this specification is a structured focus-candidate set `F`.
`F` and the independently derived expectation bundle `E` are inputs to the
combined-plan contract. Rendering, validation, and exact serialization remain
separate stages. The public product output remains the one compiled text
defined by the V1 product contract.

### Local notation

The global cross-stage registry is in the
[V1 proof program](v1-proof-program.md#canonical-notation-and-derivation-ownership).
This table owns symbols reused only inside this specification:

| Symbol | Local meaning |
| --- | --- |
| \(f=(\tau_f,v_f,\kappa_f,\nu_f)\) | One typed numerical facet and its type, payload, transform identity, and presence state |
| \(F_i,X_i,H_i\) | Memory-unit facets, exact sidecars, and history/transition references |
| \(Q,Q_{\mathrm{num}},F_Q,X_Q,G_Q,Z_Q\) | Sealed bound query, its private pure numerical projection, numerical facets, exact request-local values, active goals, and typed absence/quality state |
| \(\widehat B_{\mathrm{in}},B_Q,C_{\mathrm{boundQ}},J_{QA}\) | Compiler-owned ingress content binding, the bound query's private exact projection and canonical content envelope, and its validated join with shared-set lineage |
| \(\Sigma_{\mathrm{sig}},b_{\mathrm{sig}},V_{\mathrm{sig}},u_{\mathrm{auth}},U_{\mathrm{decl}},\kappa_{\mathrm{social}},\operatorname{subjectKey}\) | Minimized immutable signal context, carried private call brand, validated semantic signal values, authenticated social-subject identity, canonical declared-partner set, pinned social-identity schema, and its private registry-backed comparison projection |
| \(c_{i,f},F_{\mathrm{cue}},C_i,R_{\mathrm{abs}}^C,L_C,\eta_{i,f}^C,\mu_i^C,C^r\) | One calibrated direct cue, the registered cue schema, one sparse typed cue-value vector, the closed cue-absence reason and lineage-reference domains, one disjoint cue-presence state, its complete presence/lineage mask, and the finite unique direct-retrieval set |
| \(k_{\mathrm{hist}},n_i^{\mathrm{hist}},t_{i,k_{\mathrm{hist}}},\delta_{i,k_{\mathrm{hist}}}^{\mathrm{hist}},\beta_{\mathrm{decay}},\ell_{i,k_{\mathrm{hist}}},m_i^\ell,b_i^{\mathrm{raw}},b_i\) | History-event index and count, event time, dimensionless age, decay exponent, log-domain term and maximum, raw base availability, and calibrated availability |
| \(t_i,x_i,g_i,u_i,\operatorname{socialMatch},s_i^{\mathrm{auth}},s_i^{\mathrm{decl}},a_{\mathrm{auth}},a_{\mathrm{decl}},A_{\mathrm{social}}\) | Temporal, spatial, goal, procedural, social-match function, two stable social channels, their effective weights, and the pinned social-family budget |
| \(k_{\mathrm{haz}},n_Q^{\mathrm{haz}},z_{k_{\mathrm{haz}}}^{\mathrm{haz}},h_{i,k_{\mathrm{haz}}},h_i\) | Hazard index and count, one represented hazard, per-hazard relevance, and aggregate hazard relevance |
| \(\Delta_i,\tau,x_{\mathrm{zero}},u_i^{deadline}\) | Finite remaining deadline duration, pinned positive scale, explicit dimensionless exact-zero boundary, and deadline-urgency feature |
| \(D_h,V_g,E_g,k_i^V,k_e^{\mathrm{raw}},k_e^E,e_{\mathrm{scan}},N_g^{\max},E_g^{\max},H_g^{\max},b_W,\widetilde a_i^{(0)},\widetilde s,a_{\mathrm{ref}}^{(0)},d_0,s_0\) | Canonical expansion frontier, graph-node and edge sets, node, raw relation-entry, and accepted edge order keys, scanned relation-entry count, node/edge/hop ceilings, common fixed-point matrix scale, direct calibrated seed, its exact binary-rational mass, ideal reference normalization, executable denominator, and exact stored seed mass |
| \(a^{(k)},\widehat a^{(k)},\widehat s,d_k,W,\rho,K_{\mathrm{spread}},\varepsilon_{\mathrm{mass}}\) | Accepted and uncorrected spreading states, exact stored mass, executable correction denominator, exactly row-validated relation matrix, restart factor, fixed iteration count, and configured forward-error bound |
| \(e_{i,c},w_c,g_c,p_{i,j},\lambda_j\) | Evidence signals, weights, gates, inhibition signals, and strengths passed to activation |
| \(\mathcal R_Q,r_k^Q,q_k\) | Canonical request-proposition source set, one validated source entry, and its bounded request-support score |
| \(\phi,q_\phi,\operatorname{score}_\phi\) | Request-local proposition, aggregated request support, and conservative proposition activation |
| \(N_{\mathrm{focus}},W_{\mathrm{focus}}\) | Focus candidate-count and conservative work ceilings |

Symbols with the same bare Latin letter in another specification are unrelated
unless the global registry explicitly connects their complete qualified form.

### Typed numerical facets

A typed numerical facet is a tuple:

\[
f=(\tau_f,v_f,\kappa_f,\nu_f)
\]

where:

- `tau_f` is a stable facet-type identifier;
- `v_f` is a finite numeric vector or scalar payload;
- `kappa_f` identifies the encoder, transform, schema, and normalization
  contract that produced the payload; and
- `nu_f` records whether the value is observed, derived, absent, unknown, or
  uncertain.

Vectors from different facet types or incompatible `kappa_f` identities are
not directly comparable. Missing and unknown values are states, not zero
vectors. A comparison exists only when the pinned configuration defines a
compatible metric and a declared absent-value policy.

The proposed V1 facet families are:

- semantic content;
- situation and environment;
- task and procedural conditions;
- active goal and desired state;
- temporal context;
- spatial context;
- participant and social perspective;
- risk and consequence relevance;
- entity and relation roles; and
- source-dependency and evidence-role relationships, excluding authority and
  confidence from generic relevance.

This list defines logical families, not one fixed database row, universal
ontology, or required vector dimension.

### Exact sidecars

Lossy numerical representations do not carry the sole copy of information that
must be reconstructed exactly. An exact sidecar is a typed, lossless numeric,
byte, or identifier value bound to an immutable record version. Sidecars
include, where applicable:

- stable record, version, entity, source, and relation identifiers;
- integer timestamps, durations, intervals, sequence numbers, and time-zone
  identifiers;
- finite exact-domain numbers and their units;
- coordinates and declared uncertainty;
- UTF-8 or opaque bytes for names, paths, identifiers, quotations, and other
  loss-sensitive values;
- authority, authorization, disclosure, validity, and supersession labels;
- provenance roots and dependency groups;
- confidence and uncertainty values under a declared calibration contract; and
- encoder, transform, schema, and index identities.

Bytes are data, not instructions. A renderer may reproduce exact bytes only
through an explicit `FocusExpectationPlan` binding whose disclosure and
authority checks have already passed.

Every sidecar occurrence that may reach planning has two deliberately
separate identities:

```text
ExactSlotSemanticDescriptor
├── ExactSlotOwnerSemanticDescriptor
│   ├── Item(BranchItemOwnerSemanticDescriptor, ExactSlotOwnerRole)
│   └── Shared(SharedExactSlotMeaningKey)
├── schema-owned ExactSlotSemanticLocator
├── registered exact-value type
├── semantic role
├── finite occurrence bounds
├── exact-value schema identity
└── deterministic formatter identity

ExactSlotBinding
├── ExactSlotSemanticDescriptor
├── lineage-bearing binding instance identity
├── authoritative exact value
├── exact-surface content identity
├── byte-preserving formatted surface
└── permitted owning semantic item-role bindings
```

`ExactSlotOwnerSemanticDescriptor` is the branch-owned, pre-planning
description of which semantic owner contains or intentionally shares an exact
occurrence. It is the closed tagged sum displayed above.

`BranchItemOwnerSemanticDescriptor` is the canonical branch-tagged semantic
description of the owning item before exact-slot descriptors are added. It
contains the owner's non-slot meaning, scope, role, qualifiers, derivation
semantics, and authority/allowed-use classes needed to distinguish that owner.
It excludes every exact value, exact-surface byte or content identity,
exact-binding instance identity, provenance or lineage identity,
request/configuration ID, insertion position, and runtime witness. This
two-phase construction prevents recursion: derive the non-slot owner
descriptor first, then build the slot descriptors, then build the complete
branch item semantic key.

`SharedExactSlotMeaningKey` is a registered, content-identified semantic key
for one explicitly declared shared conceptual slot across multiple owners.
Sharing may be assigned only by the authenticated pinned schema registry; it
is never caller-authored or inferred from equal values, equal schema, equal
locator/path, or vector similarity. Two independent owners using the same
schema and locator retain different `BranchItemOwnerSemanticDescriptor`
values and therefore distinct upstream slots. Two otherwise identical
independent items are duplicate semantic items and must be consolidated or
rejected; lineage is never introduced merely to make them distinct.

The upstream branches do not construct the planning-owned
`ExactSlotOwnerSemanticKey`. After plan-item selection, the planner alone maps
each validated upstream descriptor together with the selected
`PlanItemSemanticKey`:

```text
mapExactSlotOwner(
  plan_item_key,
  Item(branch_owner_descriptor, owner_role)
)
  -> Item(plan_item_key, owner_role)

mapExactSlotOwner(
  plan_item_key,
  Shared(shared_exact_slot_meaning_key)
)
  -> Shared(shared_exact_slot_meaning_key)
```

For the `Item` branch, planning first verifies that
`branch_owner_descriptor` equals the descriptor independently derived from the
selected source item. At this later planner-owned boundary, a mismatch is
`SourceProjectionViolation`: the already admitted immutable branch projection
disagrees with its own selected source. `InvalidExactSlotSemanticDescriptor`
remains the upstream construction and schema/shape error for a descriptor that
cannot validly enter a branch projection. The mapping is deterministic and
cannot inspect the exact value, surface content identity, lineage, request
identity, insertion position, or runtime witness. Its result is precisely the canonical
`ExactSlotOwnerSemanticKey::Item(PlanItemSemanticKey, ExactSlotOwnerRole)` or
`ExactSlotOwnerSemanticKey::Shared(SharedExactSlotMeaningKey)` defined by the
planning contract; this specification defines no competing key type.

The planning specification owns the closed
`ExactSlotSemanticLocator` schema and later constructs `SlotSemanticKey` by
adding the canonical permitted item-role bindings. The descriptor contains no
authoritative value, exact bytes, surface content identity, \(B_Q\),
\(\Lambda_A\), source receipt, insertion position, or request-local identity.
The binding contains those privileged value and provenance fields but never
turns them into semantic grouping, priority, or model-visible slot identity.
The semantic slot identity before planning-role binding is exactly
`(ExactSlotOwnerSemanticDescriptor, ExactSlotSemanticLocator)`. Reusing that
pair with an incompatible descriptor is
`InvalidExactSlotSemanticDescriptor`; reusing it with different authoritative
exact values or surface bytes is the typed `ExactSlotValueConflict`. The
conflict is never resolved by choosing one value, merging bytes, or adding
lineage. The same locator under different independent owner descriptors
coexists without conflict. An explicit `Shared` descriptor with the same
`SharedExactSlotMeaningKey` makes all participating bindings one slot and
therefore requires byte-for-byte equal authoritative value and surface
content. Planning repeats this agreement check after mapping, keyed by its
canonical `(ExactSlotOwnerSemanticKey, ExactSlotSemanticLocator)`.

### Numerical memory unit

The compile-time representation of one immutable memory record version `i` is:

\[
m_i=
(
\operatorname{id}_i,
	F_i,
	X_i,
	H_i,
	\mathcal G_i,
	\Pi_i
	)
\]

where:

- `F_i` is a finite collection of typed numerical facets;
- `X_i` is the collection of exact sidecars;
- `H_i` contains immutable references to relevant history and transition
  records governed by the predictive-attention specification;
- \(\mathcal G_i\) is a finite set of typed numerical relations to other
  immutable record versions; and
- \(\Pi_i\) is provenance, authority, validity, uncertainty, and policy
  metadata.

The authoritative memory plane may retain exact source material or canonical
propositions as defined by a future memory-management contract. The
compile-time relevance path operates on typed numerical representations and
lossless sidecars derived from that authoritative plane. Numerical
representations remain rebuildable computational state, not an independent
source of truth.

### Numerical query state

After intrinsic validation and exact prompt/request-origin authentication,
the compiler-owned `SIT-01` boundary receives the same retained complete
request, the sealed `AuthenticatedInvocation`, and the resolved pinned
configuration `K`. It revalidates the aggregate-to-request pairing and may
borrow its private `AuthenticatedPrompt` projection only inside that aggregate
lifetime; no caller or downstream stage can supply the prompt projection or
authenticated call binding separately. It then retains immutable exact input
buffers. The request schema does not accept
`request_id`, `situation_id`,
`configuration_id`, an identity digest, or an identity-algorithm override.
Unknown fields attempting to supply any of those values are rejected rather
than ignored. `SIT-01` then constructs exactly one sealed
`IngressRequestBinding`:

\[
\widehat B_{\mathrm{in}}=
(
identity\_schema\_id,
request\_id,
situation\_id,
configuration\_id
).
\]

`configuration_id` is the authenticated content identity of the canonical
compiler-configuration manifest selected before configuration-bound
`request_id` derivation; it is not used by the earlier
configuration-independent `request_presentation_identity`;
that manifest enumerates the complete artifact set. The installation trust
root authenticates the manifest, and the compile call pins it for its full
lifetime. A caller may request one installed configuration through the public
untrusted claims contract, but cannot supply, substitute, or override the
resolved authoritative `configuration_id`; the compiler authenticates,
authorizes, and resolves that request before ingress derives the identity.
Missing authentication, a manifest digest mismatch, an unauthorized or absent
requested configuration, or a configuration change after pinning fails before
bound `Q`, retrieval, or authorization-view construction.
`identity_schema_id` and `digest_algorithm_id` are authenticated members of
that pinned manifest; an unknown algorithm, caller-selected override, or
downgrade is the same pre-ingress failure class.

`request_id` and `situation_id` are distinct typed content identities, not
caller labels and not bare interchangeable hash strings. Each contains:

```text
TypedContentIdentity
├── identity_schema_id
├── digest_algorithm_id
├── content_digest
└── bound_digest
```

The two identities use different closed type tags and different
domain-separation tags. Let `CE_v1` be the versioned canonical identity
encoding selected by the authenticated `identity_schema_id`. It is an
injective, length-delimited type-length-value encoding: field tags are
registered, lengths and unsigned integers use the one schema-defined
big-endian representation, optional fields carry an explicit absent or present
tag, maps are sorted by registered field identity, and sequences retain their
semantic order. Unknown or duplicate fields are errors. A serialized envelope
or binding that is not in the unique canonical field order is rejected on
validation. Exact prompt and situation UTF-8 bytes are copied without Unicode
normalization, whitespace rewriting, newline conversion, or case folding.

Define the canonical situation-evidence envelope:

\[
C_S =
CE_{v1}(
  situation\_statements_{\mathrm{ordered}},
  t_{\mathrm{context}},
  declared\_location,
  host\_application,
  workspace,
  project,
  remaining\_registered\_situation\_metadata
).
\]

Every situation statement is encoded with its zero-based ordinal and exact
validated bytes. Contextual time uses the schema's exact parsed instant and
offset representation; declared location and every other registered metadata
value retain their typed presence state and exact canonical value. Input map
iteration order is not semantic.

Define the canonical complete-request envelope:

\[
C_R =
CE_{v1}(
  original\_prompt\_bytes,
  C_S,
  declared\_output\_language,
  attention\_budget,
  remaining\_registered\_compile\_controls
).
\]

`C_R` therefore commits to every validated public compile-request field, while
`C_S` commits specifically to the ordered situation and contextual evidence.
Neither envelope contains principal, `t_auth`, policy, authorization-view
state, ambient clock data, or any derived identifier.

For the pinned collision-resistant digest `H`, ingress derives the following;
every displayed concatenation operand is itself `CE_v1` length framed, so no
two operand sequences have the same byte encoding:

\[
d_R=H(\texttt{"nemosyne/v1/request-content"}\parallel C_R),
\qquad
d_S=H(\texttt{"nemosyne/v1/situation-content"}\parallel C_S),
\]

\[
b_R=H(
\texttt{"nemosyne/v1/request-id"}
\parallel identity\_schema\_id
\parallel digest\_algorithm\_id
\parallel configuration\_id
\parallel d_R
),
\]

\[
b_S=H(
\texttt{"nemosyne/v1/situation-id"}
\parallel identity\_schema\_id
\parallel digest\_algorithm\_id
\parallel configuration\_id
\parallel d_S
).
\]

`request_id` is the typed tuple
`(identity_schema_id, digest_algorithm_id, d_R, b_R)` and `situation_id` is the
distinct typed tuple
`(identity_schema_id, digest_algorithm_id, d_S, b_S)`. Consequently, identical
validated request content under the same authenticated schema and
configuration produces identical identities. A prompt, situation ordering,
context value, metadata presence/value, language, budget, schema, or
configuration change changes the corresponding bound identity subject to the
stated digest-collision assumption. Situation changes change both
`situation_id` and `request_id`; non-situational request-control changes need
change only `request_id`.

The construction validates every embedded field by recomputing it from the
retained canonical envelopes. A supplied, constant, cross-request, or
otherwise non-derived identity, including reuse of a prior request's identity
with nonidentical canonical content, is `InvalidIngressBinding`. If the
compiler or verification harness observes the same complete typed identity
associated with different canonical bytes, it reports
`ContentIdentityCollision` and fails closed. A true collision of `H` that is
not exposed by retained-byte comparison cannot be ruled out mathematically;
changed-content separation is therefore conditional on canonical-encoding
injectivity and the collision-resistance assumption for `H`, not an absolute
uniqueness claim.

The sealed ingress binding is forked to exactly two compiler-owned
constructors: `bindQuery` and shared-set construction. Shared-set construction
independently copies its fields into the corresponding projection of
\(\Lambda_A\). It must not copy them from bound `Q`, derive them from lossy
query facets, reconstruct them from ambient state, or accept them from the
caller.

`bindQuery` is the sole constructor of the sealed `BoundQuery` aggregate. Its
logical constructor receives the same retained validated compile request,
sealed ingress binding, and authenticated pinned `K` that entered `SIT-01`:

\[
Q=
\operatorname{bindQuery}
(\&\operatorname{ValidatedCompileRequest},
\&\widehat B_{\mathrm{in}},
\&K).
\]

Inside that one constructor, situation encoding produces the authority-free
private numerical projection:

\[
Q_{\mathrm{num}}=\operatorname{encode}(P,S,\Xi;K)
\]

with:

\[
Q_{\mathrm{num}}=(F_Q,X_Q,G_Q,Z_Q),
\]

and the constructor independently projects:

\[
B_Q=(request\_id,situation\_id,configuration\_id)
\]

from that same \(\widehat B_{\mathrm{in}}\). The aggregate is:

```text
BoundQuery
├── <private> numerical_projection: Q_num
├── <private> exact_projection: B_Q
└── <private> content_id: BoundQueryContentId
```

Before sealing, `bindQuery` revalidates that the retained request canonical
envelopes and authenticated `K` reproduce every request, situation, schema,
digest, and configuration field in \(\widehat B_{\mathrm{in}}\). A
cross-request or cross-configuration pairing fails before either projection is
observable.

Let `CanonicalNumericalQuery` and `CanonicalQueryBinding` be the unique
canonical encodings pinned by `K`. The aggregate's canonical content envelope
is:

\[
C_{\mathrm{boundQ}}=
CE_{v1}(
\operatorname{CanonicalNumericalQuery}(Q_{\mathrm{num}};K),
\operatorname{CanonicalQueryBinding}(B_Q)
),
\]

`BoundQueryContentId` is the closed, domain-separated typed wrapper over the
`TypedContentIdentity` of \(C_{\mathrm{boundQ}}\), derived under the same
authenticated identity schema, digest algorithm, and `configuration_id`
contract used above. It cannot be substituted by another typed content
identity even when the underlying digest bytes match. Validation recomputes
that identity from the retained canonical fields.
Observed same-identity/different-content is `ContentIdentityCollision` under
the same stated digest assumption.

There is no public field constructor, struct literal, deserializer, projection
pair constructor, or `bindQuery(Q_num, B_Q)` overload. The two projections are
private and can be borrowed only by registered compiler stages after
`BoundQuery` validation; no downstream focus, expectation, planning, or
validation interface accepts them as separate arguments. A test-only corrupt
aggregate with a projection, request, configuration, or content-identity
mismatch is `InvalidQueryBinding`.

`bindQuery` performs semantic encoding and exact envelope construction once;
it does not let the exact projection alter numerical semantics. Therefore a
language, budget, or other complete-request control that changes `request_id`
may change \(B_Q\) and `BoundQueryContentId` while leaving
\(Q_{\mathrm{num}}\) bit-identical. Conversely, no field of \(B_Q\) may
change a facet, goal, exact source value, or quality state in
\(Q_{\mathrm{num}}\). This common compiler-owned ingress source makes the
later independent join meaningful: the branches can detect corruption, reuse,
or cross-request mixing without giving the focus branch an authorization
dependency.

where:

- `F_Q` contains typed prompt, situation, temporal, spatial, task, social,
  procedural, and risk-related facets;
- `X_Q` contains exact request-local values plus typed prompt, situation, and
  metadata source bindings that participate in numerical encoding; it does
  not contain \(B_Q\), resolved call controls, or authorization state;
- `G_Q` contains explicitly active goal states and their declared priorities;
  and
- `Z_Q` contains absence, uncertainty, source-language observations derived
  from the request evidence, and observation-quality metadata. Resolved output
  language is a compile control outside \(Q_{\mathrm{num}}\).

`Q_{\mathrm{num}}` retains `t_context` only as caller-supplied, untrusted
request evidence.
The same qualification applies to declared location and metadata. Trusted
authorization time `t_auth` is neither an encoder input nor a facet, exact
binding, identifier, or hidden dependency of `Q_{\mathrm{num}}` or \(B_Q\).
Authorization, current
normative validity, expiry, and supersession use `t_auth` on their separately
owned memory path.

Situation encoding does not retrieve memory, assign instruction authority, or
modify `P`. It does not decide that a facet is a renderable proposition. That
decision belongs to the focus branch's validated request-proposition
construction below. Neither \(Q_{\mathrm{num}}\) nor bound `Q` contains a
`policy_revision_id`,
`authorization_view_id`, principal, authorization decision, disclosure view,
or memory-eligibility result. Those values are not situation facts and cannot
be supplied to the focus branch through a request-source binding.

### Minimized trusted signal context

After prompt-origin authentication and before signal derivation, the
compiler's sole registered projection constructs:

\[
\Sigma_{\mathrm{sig}}=
\operatorname{projectSignalContext}(\mathcal I_A;K)
=
(signal\_context\_schema\_id,b_{\mathrm{sig}},
t_{\mathrm{auth}},\kappa_{\mathrm{social}},u_{\mathrm{auth}}).
\]

\[
V_{\mathrm{sig}}=
\operatorname{validateSignalContext}
(\mathcal I_A,\Sigma_{\mathrm{sig}};K)
=(t_{\mathrm{auth}},u_{\mathrm{auth}}).
\]

The logical wire contract is:

```text
SignalDerivationContext
├── signal_context_schema_id
├── carried_authenticated_call_brand
├── trusted_authorization_time
├── social_subject_identity_schema_id
└── authenticated_social_subject
```

All fields are private, immutable, and request-local. The schema identity is a
pinned member of `K`; `trusted_authorization_time` is copied exactly from the
authenticator output; and `authenticated_social_subject` is an
`AuthenticatedSocialSubjectId<social_subject_identity_schema_id>` produced
from the authenticated installation-principal registry under the pinned
social-identity schema \(\kappa_{\mathrm{social}}\). It is an opaque,
privacy-preserving comparison identity, not a raw principal, credential, or
caller-authored digest. The
private brand \(b_{\mathrm{sig}}\) is an opaque reference to the fresh
generative capability identity \(b_A\) sealed into \(\mathcal I_A\). It is
used only to prove that the context belongs to the current aggregate supplied
independently to validation; it is excluded from every channel, gate, score,
renderer input, diagnostic, and public result. The projector accepts the
complete aggregate and cannot accept or combine independently supplied `I`,
`t_auth`, or \(\Gamma_A\). Brand comparison uses the current
\(\mathcal I_A\)'s own \(b_A\), not a second value returned beside the
context. Equality is instance membership, not byte, digest, or numerical
equality.
The brand is never serialized, hashed into a semantic identity, sampled as
random model input, or included in determinism comparisons. The concrete
private lifetime or shared-object representation remains owned by `SEC-00`
and `OD-04`. The object
contains no raw principal credential, caller claim, policy or
disclosure handle, authorization result, memory/store capability, location,
prompt content, bound-query identity, or ambient clock.

The authenticator is the sole producer of \(\mathcal I_A\) and \(b_A\); the
projection is the sole producer of the trusted context from that aggregate.
`validateSignalContext` receives the already validated current
\(\mathcal I_A\) separately, compares its brand with
\(b_{\mathrm{sig}}\), rechecks the copied trusted time and registry-derived
social subject against that same aggregate and pinned registry, and validates
both schemas before all signal mathematics. Supplying a complete valid context
from another call therefore fails against the current aggregate instead of
forming a self-validating pair. A missing, duplicate, malformed, or cross-call
brand or copied value fails closed. Only
\(V_{\mathrm{sig}}=(t_{\mathrm{auth}},u_{\mathrm{auth}})\) crosses into signal
mathematics, so the brand cannot influence signals. The signal kernel may
read `t_auth` only for registered temporal/availability formulas
and `u_auth` only for registered social comparisons. It cannot authorize a
record, widen disclosure, create a request fact, expose either value to the
renderer, or query another source. Missing, duplicate, malformed, cross-call,
or schema-incompatible context is `InvalidSignalDerivationContext`; there is
no ambient or request-supplied fallback. Mixed-\(\Gamma_A\), mixed-`I`, or
mixed-time construction is unrepresentable through the private API and is
also retained as a compile-fail or forged-internal-state counterexample.

The social-identity schema is shared by authenticated subjects, declared
interaction partners, and memory participant references while preserving
their source tags as disjoint sum types:

```text
SocialIdentity
├── AuthenticatedSocialSubjectId<schema>
├── DeclaredInteractionPartnerId<schema>
└── MemoryParticipantId<schema>

SocialComparisonSubject<schema>
├── Authenticated(AuthenticatedSocialSubjectId<schema>)
└── Declared(DeclaredInteractionPartnerId<schema>)
```

Equal opaque payloads across tags do not erase source attribution. The pinned
authenticated registry owns one private total projection
\(\operatorname{subjectKey}\) from each valid tagged identifier to a common
opaque `SocialSubjectComparisonKey<schema>`. That key is available only inside
the registered social comparator; it is never a public identity, authority
claim, rendered value, diagnostic payload, or semantic ordering key. The
closed comparison

\[
\operatorname{sameSocialSubject}(m,u;\kappa_{\mathrm{social}})
=
\left[
\operatorname{subjectKey}(m)=\operatorname{subjectKey}(u)
\right]
\]

accepts exactly one validated `MemoryParticipantId<schema>` and one
`SocialComparisonSubject<schema>`, retains both source tags in its explanation,
and rejects every other type pairing. `participantMatch` may consume only this
result plus registered relation semantics; it cannot compare raw tagged bytes
or coerce one tag into another.

Within one schema, the registry must return the same comparison key for the
same represented subject and distinct keys for distinct registered subjects,
subject to its explicit collision/integrity assumption. A schema rotation is
accepted only through a pinned, authenticated, one-to-one migration artifact
that binds every affected tagged registry revision and preserves comparison-key
equality without exposing raw principal material. Missing, ambiguous,
many-to-one, stale, unauthenticated, or incompatible migration is
`SocialIdentityArtifactUnavailable`; the compiler does not silently treat it
as no match. A memory participant whose schema cannot be validated or migrated
is excluded from that social comparison with a typed diagnostic before
calibration; it is never converted to numeric zero. Declared partners are
canonical request-evidence identities in the same schema, remain untrusted,
and cannot alias the authenticated tag for authorization or authority.

### Eligible memory view

Hard policy and integrity predicates define whether a record may enter the
request-local candidate graph:

\[
\operatorname{eligible}(m_i)=
\operatorname{readable}(m_i,I)
\land
\operatorname{disclosable}(m_i,I,t_{\mathrm{auth}})
\land
\operatorname{revisionCompatible}(m_i,r,K)
\land
\operatorname{representationValid}(m_i,K)
\land
\operatorname{notDeleted}(m_i,r)
\]

The eligible set is:

\[
\mathcal M_E=
\{m_i\in M_A^{(r,p_{\mathrm{policy}},t_{\mathrm{auth}},I)}
\mid\operatorname{eligible}(m_i)\}
\]

These are hard gates. An ineligible record receives no score, occupies no
candidate budget, contributes no graph activation, appears in no diagnostic
that discloses its content, and cannot be restored by a later numerical stage.

Validity, supersession, and authority are usage predicates rather than blanket
historical-erasure rules. A superseded or no-longer-current record may remain
eligible when the request explicitly needs historical context, but it carries
its temporal and supersession qualification and cannot support a current
normative instruction. Current instructions require current validity and
non-supersession at `t_auth`; caller-controlled `t_context` cannot revive them.

Source confidence, contextual mismatch, ambiguity, unresolved contradiction,
and redundancy are not authorization predicates. They are represented as
qualifications, evidence, or soft inhibition after eligibility.

Eligibility alone does not make a historical revision relevant to every
request. Define a pinned usage-compatibility predicate:

\[
\operatorname{usageCompatible}(m_i,Q_{\mathrm{num}})=
\operatorname{currentUsable}(m_i,t_{\mathrm{auth}})
\lor
\left(
\operatorname{historicalScopeRequested}(Q_{\mathrm{num}})
\land
\operatorname{historicallyApplicable}(m_i,Q_{\mathrm{num}})
\right)
\]

The request-local candidate universe is:

\[
\mathcal M_Q=
\{m_i\in\mathcal M_E\mid
\operatorname{usageCompatible}(m_i,Q_{\mathrm{num}})\}
\]

`currentUsable` applies the record's validity and supersession contract.
Historical admission is explicit, carries a `historical_only` qualification,
and cannot support a current normative instruction. Candidate generation and
graph expansion operate on \(\mathcal M_Q\); a stale record cannot enter an
ordinary current request merely because it is semantically similar.

### Direct cue activation

Candidate generation accepts one sealed `&BoundQuery`, validates the aggregate,
and privately borrows \(Q_{\mathrm{num}}\) while deriving a bounded direct
candidate set from \(\mathcal M_Q\). It has no boundary accepting an
independent numerical projection, binding projection, or caller-assembled
pair. Let \(C^r\) denote that finite set. It contains each admitted immutable
memory-record version at most once, satisfies \(C^r\subseteq\mathcal M_Q\) and
\(0\leq |C^r|\leq n_r\), and is canonically ordered by
\(\operatorname{GraphNodeKey}(\operatorname{id}_i)\), defined below from the
complete immutable record-version identity. An input containing two
members with the same complete identity but different payloads is
`InvalidDirectCandidateSet`; an exact duplicate is also rejected rather than
silently deduplicated. The reference exhaustive retriever and every selected
approximate retriever must produce the same typed set contract. Their retrieval
and bounded-selection policies remain separately versioned artifacts; neither
insertion order nor a request-local allocation identity may choose or order a
member.

A direct cue score is a collection of independently inspectable facet matches
rather than one undifferentiated embedding similarity. The pinned configuration
defines one finite, duplicate-free, canonically ordered cue schema
\(F_{\mathrm{cue}}\).

For a compatible facet family `f`:

\[
c_{i,f}=
\operatorname{cal}_{f}
\left(
\operatorname{metric}_{f}(q_f,m_{i,f});K
\right)
\in[0,1]
\]

`metric_f` may be cosine similarity, an exact numeric distance, a categorical
match, or another versioned metric. `cal_f` maps its domain into the unit
interval and must be evaluated independently. It must not be inferred from the
activation formula itself.

For every evaluated record `i`, the numerical cue value collection is the
sparse canonical map:

\[
C_i=
\left((f,c_{i,f})\right)_{
f\in F_{\mathrm{cue}}\;:\;
\eta_{i,f}^C=\mathrm{Present}(\ell),\ \ell\in L_C}
\]

and the corresponding complete typed presence/lineage mask is:

\[
\mu_i^C=
\left((f,\eta_{i,f}^C)\right)_{f\in F_{\mathrm{cue}}}.
\]

`K` pins the finite closed cue-absence reason set
\(R_{\mathrm{abs}}^C\), which contains `AbsentByRelationExpansion`, and the
typed immutable lineage-reference domain \(L_C\). The disjoint state is:

\[
\eta_{i,f}^C\in
\{\mathrm{Present}(\ell)\mid\ell\in L_C\}
\mathbin{\dot\cup}
\{\mathrm{Absent}(r)\mid r\in R_{\mathrm{abs}}^C\}.
\]

Every cue-schema identity occurs exactly once in \(\mu_i^C\).
`Present(ell)` requires exactly one finite \(c_{i,f}\in[0,1]\) in \(C_i\);
`ell` is its source-tagged immutable cue-lineage reference. Every typed
`Absent(r)` state permits neither a numeric entry nor a lineage reference;
its registered `r` is the explanation. Thus a present numeric zero is the
numeric entry \((f,0)\) in \(C_i\) paired with
\((f,\mathrm{Present}(\ell))\) in \(\mu_i^C\), whereas an absent cue has no
numeric payload. The key set of \(C_i\) must equal the `Present` key set of
\(\mu_i^C\); a missing, additional, duplicate, unknown, lineage-inconsistent,
or unregistered absence-reason entry is `InvalidDirectCueState`. Both
structures are
canonically sorted by the cue-schema order after validation; caller order is
never semantic. No implementation may materialize an absent cue as a numeric
zero.

Candidate generation may use an approximate index, but its output remains
bound to immutable record versions and the pinned index revision. Approximate
retrieval requires measured recall and cannot replace authorization.

### Base availability from frequency and recency

Human-memory research motivates a prior availability term based on the timing
of real observations or uses. Let `u_time > 0` be a pinned reference duration
in the same units as timestamps, let `delta_min > 0` be a dimensionless
minimum age, and let \(n_i^{\mathrm{hist}}\) be the finite nonnegative count of
qualifying configured history events in canonical order. For
\(n_i^{\mathrm{hist}}>0\) and each
\(k_{\mathrm{hist}}\in\{1,\ldots,n_i^{\mathrm{hist}}\}\), require
\(t_{i,k_{\mathrm{hist}}}\leq t_{\mathrm{auth}}\) and define:

\[
\delta_{i,k_{\mathrm{hist}}}^{\mathrm{hist}}=
\max
\left(
\frac{t_{\mathrm{auth}}-t_{i,k_{\mathrm{hist}}}}{u_{\mathrm{time}}},
\delta_{\min}
\right)
\]

When \(n_i^{\mathrm{hist}}>0\), for pinned
\(\beta_{\mathrm{decay}}>0\), define:

\[
\ell_{i,k_{\mathrm{hist}}}
=
-\beta_{\mathrm{decay}}
\ln\delta_{i,k_{\mathrm{hist}}}^{\mathrm{hist}},
\qquad
m_i^{\ell}=\max_k\ell_{i,k},
\]

\[
b_i^{raw}=
m_i^{\ell}
+
\ln
\left(
\sum_{k_{\mathrm{hist}}=1}^{n_i^{\mathrm{hist}}}
\exp(\ell_{i,k_{\mathrm{hist}}}-m_i^{\ell})
\right).
\]

This log-sum-exp form is the required finite-arithmetic evaluation, not a
permission to sum the raw powers first. Every age, logarithm, multiplication,
subtraction, exponential, canonical-order partial sum, and final result is
checked finite under pinned input ceilings. At least one shifted exponent is
exactly \(\exp(0)=1\), so a valid finite input cannot underflow the sum to zero.
Any nonfinite intermediate or violated ceiling is
`InvalidAvailabilityArithmetic`; it is never clamped or interpreted as
missingness. With no qualifying history event, the base-availability channel
is disabled rather than assigned an invented zero. A versioned calibrator maps
the unbounded value into:

\[
b_i=\operatorname{cal}_{base}(b_i^{raw};K)\in[0,1]
\]

History events must distinguish:

- real observation or import;
- user-confirmed use;
- successful outcome attribution;
- internal candidate retrieval; and
- rendered inclusion.

An internal retrieval or rendering event does not automatically count as a new
real observation. Otherwise repeated compiler use can create an unearned
self-reinforcing activation loop. Compilation remains read-only and does not
append any history event.

### Temporal and spatial context

Temporal-context relevance combines a learned context match with exact
time-domain features:

\[
t_i=
\operatorname{cal}_{temporal}
\left(
\operatorname{sim}(q_{\mathrm{temporal}},
m_{i,\mathrm{temporal}}),
\Delta t_i,
\operatorname{intervalRelation}(X_Q,X_i);
K
\right)
\in[0,1]
\]

`Delta t_i` is computed from exact timestamps rather than reconstructed from a
vector. A recent event is not necessarily relevant, and a remote procedural
memory is not necessarily irrelevant.

Spatial relevance is:

\[
x_i=
\operatorname{cal}_{spatial}
\left(
\operatorname{sim}(q_{\mathrm{spatial}},
m_{i,\mathrm{spatial}}),
\operatorname{exactDistance}(X_Q,X_i),
\operatorname{roleCompatibility}(Q_{\mathrm{num}},m_i);
K
\right)
\in[0,1]
\]

An unavailable location is unknown, not zero distance. Exact coordinates and
their uncertainty remain sidecars.

### Active-goal relevance

Only explicitly active goals in `G_Q` contribute goal relevance. For active
goal `g`, the declared priority satisfies `pi_g in [0,1]` under a pinned
priority contract:

\[
g_i=
\max_{g\in G_Q}
\left[
\pi_g
\operatorname{cal}_{goal}
\left(
\operatorname{metric}_{goal}(m_i,g);K
\right)
\right]
\in[0,1]
\]

The maximum over an empty active-goal set is undefined and disables the goal
channel; it is not silently interpreted as zero evidence.

Completed, cancelled, rejected, superseded, or merely historical goals are not
active goals. Long-term interests do not become active goals solely because
they are semantically related.

### Procedural and action relevance

Procedural relevance measures whether a memory's conditions, applicable role,
action class, and expected outcome match the current task:

\[
u_i=
\operatorname{cal}_{procedure}
\left(
\operatorname{conditionMatch}(Q_{\mathrm{num}},m_i),
\operatorname{roleMatch}(Q_{\mathrm{num}},m_i),
\operatorname{actionClassMatch}(Q_{\mathrm{num}},m_i),
\operatorname{outcomeMatch}(Q_{\mathrm{num}},m_i);
K
\right)
\in[0,1]
\]

This signal concerns the usefulness of remembered procedural information. It
does not execute an action, establish that an action is safe, or authorize a
tool.

### Risk and urgency relevance

Let
\((z_1^{\mathrm{haz}},\ldots,z_{n_Q^{\mathrm{haz}}}^{\mathrm{haz}})\)
be the finite canonical sequence of currently represented hazards. When
\(n_Q^{\mathrm{haz}}>0\), each
\(k_{\mathrm{haz}}\in\{1,\ldots,n_Q^{\mathrm{haz}}\}\) has separately derived
`severity`, `causalRelevance`, and `confidence` values that are finite and lie
in `[0,1]`. Define:

\[
h_{i,k_{\mathrm{haz}}}=
\operatorname{severity}(z_{k_{\mathrm{haz}}}^{\mathrm{haz}})
\cdot
\operatorname{causalRelevance}(m_i,z_{k_{\mathrm{haz}}}^{\mathrm{haz}})
\cdot
\operatorname{confidence}(z_{k_{\mathrm{haz}}}^{\mathrm{haz}})
\]

and:

\[
h_i=
\max_{1\leq k_{\mathrm{haz}}\leq n_Q^{\mathrm{haz}}}
h_{i,k_{\mathrm{haz}}}
\in[0,1]
\]

When \(n_Q^{\mathrm{haz}}=0\), the maximum is not evaluated and the risk
channel is disabled. Arousal, emotional language, or negative sentiment does
not by itself establish severity or causal relevance. Risk relevance and truth
confidence remain separate values.

For a future deadline, the validated remaining duration and scale use one
declared unit and satisfy
\(0\leq\Delta_i\leq\Delta_{\max}<\infty\) and
\(0<\tau_{\min}\leq\tau\leq\tau_{\max}<\infty\). The proposed urgency feature
is:

\[
u_i^{deadline}=\exp(-\Delta_i/\tau)
\]

The implementation computes \(x=\Delta_i/\tau\) with checked finite
arithmetic. `K` pins one deterministic exponential implementation and one
finite positive dimensionless underflow boundary
\(x_{\mathrm{zero}}\). It returns exactly
zero when \(x\geq x_{\mathrm{zero}}\); for
\(0\leq x<x_{\mathrm{zero}}\) it evaluates \(\exp(-x)\) and requires a
finite strictly positive result in `(0,1]`. Nonfinite input,
overflow, an out-of-domain library result, or a configuration whose declared
forward-error and underflow bounds are incomplete is
`InvalidUrgencyArithmetic`; there is no saturation or general clamping. This
feature approaches one as the deadline approaches. It never overrides a hard
constraint or authorization rule. Deadline urgency, immediate response
window, and severity if ignored are separate inputs.

### Social and common-ground relevance

Social relevance is partner- and interaction-specific. A record may distinguish
private knowledge, attributed belief, assertion, acknowledgment, and
jointly-established common ground.

Let \(V_{\mathrm{sig}}=(t_{\mathrm{auth}},u_{\mathrm{auth}})\), and let
\(U_{\mathrm{decl}}(Q_{\mathrm{num}})\) be the finite canonical set of unique
`DeclaredInteractionPartnerId<kappa_social>` values. Every participant
reference used below is a validated
`MemoryParticipantId<kappa_social>`. Define the source-tagged match function:

\[
\operatorname{socialMatch}(i,u)=
\operatorname{cal}_{social}
\left(
\operatorname{participantMatch}(m_i,u),
\operatorname{interactionMatch}(m_i,Q_{\mathrm{num}}),
\operatorname{groundingState}(m_i,u),
\operatorname{perspectiveCompatibility}(m_i,Q_{\mathrm{num}});
K
\right)
\in[0,1]
\]

The stable authenticated-subject channel is:

\[
s_i^{\mathrm{auth}}=
\operatorname{socialMatch}
(i,u_{\mathrm{auth}}).
\]

The stable declared-partner channel is disabled when
\(U_{\mathrm{decl}}=\varnothing\). Otherwise:

\[
s_i^{\mathrm{decl}}=
\max_{u\in U_{\mathrm{decl}}}
\operatorname{socialMatch}
(i,u).
\]

The maximizing partner is retained only as source-tagged explanation
provenance; exact ties select the lowest canonical partner identity. These are
two fixed registered channel identities, not dynamically created partner
channels. Let
\(a_{\mathrm{auth}}=w_{\mathrm{auth}}g_{\mathrm{auth}}\) and
\(a_{\mathrm{decl}}=w_{\mathrm{decl}}g_{\mathrm{decl}}\). When the declared
channel is disabled, \(a_{\mathrm{decl}}=0\) exactly. The validated pinned
channel profile must satisfy
\(a_{\mathrm{auth}}+a_{\mathrm{decl}}\leq A_{\mathrm{social}}\leq1\);
violation is `InvalidSocialFamilyBudget` before activation, never
normalization or clamping.
Canonical set construction, the maximum, and the family budget make the result
invariant to partner order and duplication and prevent partner cardinality
from multiplying social-channel mass. A declared interaction partner is
untrusted request evidence and never aliases the authenticated principal.
Signal derivation never substitutes one source for the other or reads an
ambient user identity. Missing authenticated subject is a structural context
error; empty declared-partner input disables only its stable channel rather
than scoring it as zero.
One assertion does not prove mutual knowledge. A subject match does not grant
authorization, establish truth, or elevate instruction authority.

### Canonical spreading-graph construction

The bounded relation expansion consumes the finite unique \(C^r\), the exact
revision-bound eligible relation projection, and no other record source. `K`
pins integer ceilings \(N_g^{\max},E_g^{\max}\in\mathbb N_0\) and
\(H_g^{\max}\in\mathbb N_0\), with \(N_g^{\max}\geq n_r\), together with the
closed set of traversable relation types and one registered traversal-direction
rule per type. Applying that rule produces the oriented `source` and `target`
used by expansion, the edge key, and \(W\); stored direction and oriented
direction are never implicitly interchanged. `K` also pins
one closed, total, injective `RelationTypeRank` table over that exact type
domain. An unknown type, missing or duplicate rank, non-total table, invalid
direction, negative limit, or configured limit above the installed supported
ceiling fails before expansion.

The revision-bound relation projection exposes a snapshot-consistent bounded
iterator over a finite source-node set. Before semantic validation, every
syntactically decoded relation entry has the injective total raw key:

\[
k_e^{\mathrm{raw}}=
\operatorname{CanonicalRawRelationEntry}(e).
\]

This canonical byte representation covers every stored field, including raw
endpoint identities, relation-type identity, direction, version, and
fixed-point weight, and remains defined for syntactically valid but
semantically unknown values. The iterator merges the source ranges in
ascending \(k_e^{\mathrm{raw}}\) order and can return at most a requested
`limit` plus a Boolean `has_more` indicator without materializing later
entries. Malformed serialization fails projection validation before expansion;
the canonical artifact locator and a `K`-pinned field-validation priority
determine the reported error when more than one malformed field or entry is
present. An implementation that first materializes an unbounded frontier to
synthesize this iterator does not satisfy this contract.

For one immutable record version `i`, define the graph-node order key:

\[
k_i^V=\operatorname{GraphNodeKey}(\operatorname{id}_i).
\]

`GraphNodeKey` is the injective canonical serialization of the complete
immutable record-version identity. It excludes retrieval score, insertion
position, allocation identity, and traversal path. Two different record
payloads with one node key are an integrity error. The canonical node order is
ascending \(k_i^V\).

Expansion is deterministic breadth-by-depth set construction. Let
\(D_0=V_0=C^r\), already in canonical node order, and initialize
\(e_{\mathrm{scan}}=0\). For each
\(0\leq h<H_g^{\max}\):

1. request the next canonical entries whose configured source is in \(D_h\)
   from the revision-bound eligible relation projection, with limit
   \(E_g^{\max}-e_{\mathrm{scan}}\);
2. increment \(e_{\mathrm{scan}}\) by the returned count using checked
   arithmetic; if the iterator reports `has_more`, return
   `GraphEdgeLimitExceeded` before semantically validating or admitting that
   batch;
3. otherwise, in
   ascending \(k_e^{\mathrm{raw}}\) order, validate each source and target as a
   unique member of
   \(\mathcal M_Q\), validate its type, version, direction, and weight, and
   derive its complete accepted edge key, using the pinned field-validation
   priority within an entry;
4. reject a duplicate accepted edge key and extend the cumulative accepted
   edge collection in ascending accepted edge-key order;
5. define \(D_{h+1}\) as the unique target nodes not already in \(V_h\), sorted
   by \(k_i^V\); and
6. define \(V_{h+1}=V_h\cup D_{h+1}\) in ascending node-key order.

An empty frontier terminates expansion early. Reaching \(H_g^{\max}\) is the
declared semantic depth bound, not an invitation to inspect another hop; the
receipt records the number of completed hops. If a completed step would make
\(|V_{h+1}|>N_g^{\max}\), construction fails before admitting any member of
that step. It never retains a traversal-order-dependent prefix.

For every traversed relation entry `e`, define:

\[
k_e^E=
\left(
k_{\operatorname{source}(e)}^V,
k_{\operatorname{target}(e)}^V,
\operatorname{RelationTypeRank}(\operatorname{type}(e)),
\operatorname{RelationVersionKey}(\operatorname{version}(e))
\right).
\]

`RelationVersionKey` has one pinned total canonical order. \(E_g\) is the
final unique cumulative traversed-edge collection sorted by ascending
\(k_e^E\). Two entries
with the same complete edge key are rejected, even when their weights are
equal. The bounded iterator guarantees
\(e_{\mathrm{scan}}=|E_g|\leq E_g^{\max}\) for every successful expansion and
\(e_{\mathrm{scan}}\leq E_g^{\max}\) for every limit failure. It never
truncates a successful graph by input order. The resulting graph satisfies
\(C^r\subseteq V_g\subseteq\mathcal M_Q\), where \(V_g\) is the final
canonically ordered \(V_h\). Empty \(C^r\) produces empty \(V_g\), empty
\(E_g\), and the unique zero-dimensional graph. Every nonempty isolated node
produces an explicit zero matrix row.

Graph-bound failures have one typed source contract:
`InvalidGraphLimit` covers malformed or unsupported node, edge, or hop
configuration; `GraphNodeLimitExceeded` covers a completed next frontier above
the node ceiling; `GraphEdgeLimitExceeded` covers `has_more` from the bounded
relation iterator; and `GraphIntegerOverflow` covers any checked counter or
fixed-point integer overflow. Reaching a valid \(H_g^{\max}\) is successful
semantic truncation at the declared depth and is not a resource error.

The closed `SpreadingGraphConstructionError` contains exactly these distinct
variants: `InvalidDirectCandidateSet`, `InvalidGraphLimit`,
`InvalidRelationRankArtifact`, `InvalidGraphEndpoint`,
`InvalidGraphRelation`, `DuplicateGraphNode`, `DuplicateGraphEdge`,
`GraphNodeLimitExceeded`, `GraphEdgeLimitExceeded`, `InvalidGraphWeight`,
`GraphIntegerOverflow`, and `InvalidGraphShape`. Public routing preserves the
exact source variant:

- `InvalidGraphLimit` and `InvalidRelationRankArtifact` map to
  `ArtifactUnavailable`, because a malformed or unsupported authenticated
  graph artifact must be repaired rather than interpreted as request data;
- `GraphNodeLimitExceeded`, `GraphEdgeLimitExceeded`, and
  `GraphIntegerOverflow` map to `ResourceFailure`, return no partial graph,
  and remain non-retryable for the same request, revision, and configuration;
  and
- every remaining variant maps to `ActivationFailure`.

An absent, wrong-revision, or ineligible endpoint is
`InvalidGraphEndpoint`; it is never silently dropped or converted to a
zero-weight edge. No routing step flattens or discards the source variant.

Each configured edge weight is an exact nonnegative fixed-point value
\(z/2^b\), where \(0\leq b\leq53\), \(0\leq z\leq2^b\), and the conversion
to binary64 is exact. `K` pins one common matrix scale \(b_W\leq53\).
An input with \(b\leq b_W\) is converted exactly to numerator
\(z\,2^{b_W-b}\) by checked integer shift; an input with \(b>b_W\), or any
conversion that is not exact in the registered integer domain, is invalid.
Multiple distinct typed edges from one source to one target are accumulated
by checked common-scale integer numerators in canonical edge-key order. For
every source row, the exact integer numerator sum must be at most \(2^{b_W}\).
Duplicate keys, mixed scales without that registered exact conversion,
invalid weights, checked-integer overflow, a row budget above one, or any
dimension/order mismatch returns its corresponding
`SpreadingGraphConstructionError`. There is no silent row renormalization and
no nearest-rounded `f64` row-sum test.

Matrix row and column index `i` is exactly the zero-based position of the
corresponding node in ascending \(k_i^V\) order. The resulting matrix is:

\[
W_{ij}=
\sum_{\substack{e\in E_g\\source(e)=i\\target(e)=j}}weight(e),
\qquad
\sum_jW_{ij}\leq1.
\]

### Direct spreading-seed derivation

Spreading activation starts from direct request-to-record cues and must not call
the later activation kernel recursively. Let \(V_g\) be the canonical unique
graph-node set admitted by the construction above, with
\(C^r\subseteq V_g\subseteq\mathcal M_Q\). Its order is exactly ascending
\(k_i^V\), which is also the row and column order of \(W\). For each
\(i\in V_g\), the pinned seed calibrator consumes both the sparse typed
direct-cue values \(C_i\) and their complete typed presence/lineage mask
\(\mu_i^C\):

\[
\widetilde a_i^{(0)}
=
\operatorname{cal}_{\mathrm{seed}}(C_i,\mu_i^C;K)
\in[0,1].
\]

The calibrator, cue schema, missingness behavior, and numerical policy are
content-identified artifacts. The calibrator may use cue identities, presence
states, and numeric values. It may use lineage only to validate uniqueness and
produce an explanation receipt; changing a valid opaque lineage reference
without changing its cue identity, source kind, presence state, or value cannot
change \(\widetilde a_i^{(0)}\). A relation-only node carries the explicit
`Absent(AbsentByRelationExpansion)` state for every unavailable direct cue,
has no numeric entry in \(C_i\), and retains the reason in \(\mu_i^C\). A mask
with no `Present` cue maps to exact zero with the typed explanation
`NoDirectCue`. A mask with at least one `Present` cue whose corresponding
numeric values are all zero also maps to exact zero, but retains the distinct
typed explanation
`AllPresentCuesZero`. No bias, default, missing-to-zero conversion, relation
weight, or activation output may create a seed.

Each graph-node identity and cue-schema identity occurs exactly once in its
respective canonical collection. Duplicate records, duplicate cue identities,
a disagreement between \(C_i\) and \(\mu_i^C\), missing required cue
provenance, or a nonfinite/out-of-range calibrated seed is
`InvalidSpreadingSeed`. Canonical \(V_g\) order is fixed before arithmetic.
The output dimension and index order must equal \(W\)'s node dimension and
order exactly; a missing, extra, or permuted node is
`InvalidSpreadingSeed`.

Let \(\widetilde s\) be the exact sum of the nonnegative finite binary64 seed
values, computed by the pinned exact binary-rational superaccumulator. The
ideal real-arithmetic reference normalization is:

\[
a_{\mathrm{ref},i}^{(0)}
=
\begin{cases}
0,&\widetilde s=0,\\[2pt]
\dfrac{\widetilde a_i^{(0)}}{\max(1,\widetilde s)},
&\widetilde s>0.
\end{cases}
\]

This reference has exact mass \(\min(1,\widetilde s)\), but it is not the
stored binary64 vector when \(\widetilde s>1\) and the exact denominator is not
representable. The executable denominator is:

\[
d_0=
\begin{cases}
1_{\mathrm{f64}},&\widetilde s\leq1,\\
\operatorname{up}_{\mathrm{f64}}(\widetilde s),&\widetilde s>1,
\end{cases}
\]

where \(\operatorname{up}_{\mathrm{f64}}\) returns the least finite binary64
value not below the exact binary-rational argument. With the pinned
round-to-nearest, ties-to-even binary64 division
\(\operatorname{fl\_div}_K\), the actually stored seed is:

\[
a_i^{(0)}=
\begin{cases}
0_{\mathrm{f64}},&\widetilde s=0,\\
\operatorname{fl\_div}_K
\left(\widetilde a_i^{(0)},d_0\right),&\widetilde s>0.
\end{cases}
\]

The implementation then recomputes:

\[
s_0=\operatorname{exactBinarySum}\left(a^{(0)}\right)
\]

from the stored binary64 components and accepts only \(s_0\leq1\). Checked
division must preserve every positive seed as positive. Positive-to-zero
underflow, overflow, nonfinite output, an unavailable upward denominator, or
\(s_0>1\) is `InvalidSpreadingSeedArithmetic`; one nearest-rounded `f64` sum
or one unchecked componentwise normalization pass is never accepted as proof.
For \(\widetilde s=0\), every component of both vectors is canonical positive
zero and spreading remains zero. This stage retains one cue-lineage
explanation per seed and cannot read \(B_Q\), policy, authorization, an ambient
clock, relation edges, or any activation-kernel output.

### Bounded spreading activation

Direct candidates may activate usage-compatible related records through typed,
versioned relations. Let \(n=|V_g|\) for the exact bounded request-local
graph-node set and canonical matrix constructed above. Let:

\[
a^{(0)}\in[0,1]^n,\qquad
\lVert a^{(0)}\rVert_1\leq1
\]

be the output of the direct spreading-seed derivation above. Let `W` be a nonnegative
row-substochastic relation matrix:

\[
W_{ij}\in[0,1],
\qquad
\sum_j W_{ij}\leq1
\]

where every edge type and weight has a pinned semantic contract. For restart
factor `rho` with `0 <= rho < 1`:

\[
a^{(k+1)}
=
(1-\rho)a^{(0)}
+
\rho W^\top a^{(k)}
\]

The real-valued recurrence proves
\(\lVert a^{(k)}\rVert_1\leq1\) by induction because `W` is
row-substochastic and \(a^{(0)}\) has unit-or-lower `L1` mass. That proof does
not by itself prove an ordinary floating-point implementation has the same
postcondition.

V1 therefore pins one checked `f64` policy in `K`. Each sparse matrix product
and sum uses canonical node/edge order and checked finite multiplication and
addition. For the uncorrected finite nonnegative iterate
\(\widehat a^{(k+1)}\), the implementation computes its exact binary-rational
mass \(\widehat s\) from the stored binary64 components with the same pinned
superaccumulator. If \(\widehat s\leq1\), it accepts the iterate
unchanged. If
\(1<\widehat s\leq
1+\varepsilon_{\mathrm{mass}}(n,|E_g|,\rho;K)\), it derives:

\[
d_{k+1}=\operatorname{up}_{\mathrm{f64}}(\widehat s),
\qquad
a_i^{(k+1)}=
\operatorname{fl\_div}_K
\left(\widehat a_i^{(k+1)},d_{k+1}\right).
\]

The positive \(\varepsilon_{\mathrm{mass}}\) is a configuration-owned forward
rounding-error bound derived from the declared operation count and arithmetic
policy; it is not an empirical tolerance or an epsilon equality rule. A
negative or nonfinite component, nonfinite mass, missing error bound,
unavailable finite upward-rounded denominator, positive-to-zero division,
mass above that bound, or exact post-correction binary-rational mass above one
is `InvalidSpreadingArithmetic`. The post-correction exact re-sum is mandatory;
the real quotient
\(\widehat a^{(k+1)}/\widehat s\), the quotient by the upward binary64
denominator, and the stored componentwise
\(\operatorname{fl\_div}_K\) result are not treated as identical. One
componentwise division is not assumed to establish the invariant. Signed zero
is canonicalized to positive zero. No component is otherwise clamped, and
every accepted iteration satisfies the finite, nonnegative, exact
stored-mass-at-most-one postcondition constructively.

The V1 experiment executes exactly `K_spread` configured iterations and admits
at most the configured node, edge, and hop budgets. Its explanation receipt
retains whether the narrowly defined mass correction occurred and the
uncorrected mass; neither value is renderer-visible.

Spreading activation:

- can propose an additional candidate;
- records every traversed relation and source version;
- cannot cross the eligible-memory boundary;
- cannot increase truth, authority, or confidence;
- cannot turn association into causation; and
- cannot replace direct evidence for an exact proposition.

Relations such as `supports`, `contradicts`, `supersedes`, `caused_by`, and
`applies_to` are not interchangeable. Negative or inhibitory relations are
represented through explicit typed signals, not negative graph weights.

### Signal derivation and existing kernel composition

Signal derivation accepts one sealed `&BoundQuery`, validates that aggregate,
then privately borrows \(Q_{\mathrm{num}}\) and maps it with validated
\(V_{\mathrm{sig}}\), direct matches, bounded graph activation, and eligible
candidates into the existing activation-kernel contract. It exposes no
split-query overload and cannot replace or retain either query projection.
\(V_{\mathrm{sig}}\) is the sole trusted signal-value input: it supplies
`t_auth` to base availability and the authenticated subject to social
relevance without placing either in \(Q_{\mathrm{num}}\), \(B_Q\), or bound
`Q`. It cannot be used to reopen authorization or discover ambient state.

For evidence channel `c` and candidate `i`:

\[
e_{i,c}\in[0,1]
\]

Candidate-independent parameter weight `w_c` and situation-dependent gate
`g_c(Q_num, V_sig)` satisfy:

\[
w_c,g_c(Q_{\mathrm{num}},V_{\mathrm{sig}})\in[0,1]
\]

The proposed channel families are:

- cue and semantic-context match;
- temporal-context relevance;
- spatial-context relevance;
- base availability;
- active-goal relevance;
- procedural relevance;
- risk and deadline relevance;
- social-perspective relevance;
- bounded relation activation.

Source confidence, authority, validity, and qualifications remain separate
features attached to the candidate and plan. They must not be folded into a
generic relevance channel that could turn a highly activated observation into
a more authoritative instruction.

The channel schema specifies whether closely related features are separate
channels or inputs to one calibrated channel. It must avoid counting one
underlying signal repeatedly under different names.

Soft inhibition channel `j` supplies:

\[
p_{i,j},\lambda_j\in[0,1]
\]

Appropriate soft inhibitions include:

- contextual mismatch within an otherwise eligible record;
- ambiguity or unresolved qualification;
- competition from a better-matched eligible interpretation;
- request-local redundancy; and
- an explicitly modeled, unresolved contradiction.

Authorization, disclosure, deletion, revision compatibility, and
representation integrity are never soft inhibitions. Current normative use
also requires current validity and non-supersession as a hard authority rule.
Historical relevance is represented explicitly rather than by weakening that
rule.

The existing kernel consumes those evidence and inhibition channel values.
Its formulas, denominator precondition, canonical channel order,
floating-point behavior, explanation contract, tie-breaking, and notation are
owned only by the
[situation-conditioned activation specification](situation-conditioned-activation.md).
This specification supplies candidate meaning and signal lineage but does not
restate the kernel derivation.

`A_i` is a bounded relative activation score. It is not:

- a probability that the memory is true;
- a calibrated probability that it will improve an answer;
- a safety score;
- an instruction-authority level;
- a biological firing rate; or
- a measure of consciousness.

Weights, gates, calibrators, relation parameters, and inhibition strengths are
versioned experimental parameters. Cognitive research motivates their
qualitative roles but does not supply production-ready values.

### Validated request-proposition sources

The focus branch may derive focus from the validated request even when the
eligible activated-memory set contains no records. This path is explicit
rather than an implicit special case.

Situation encoding owns sealed `BoundQuery` \(Q\), including its private typed
\(Q_{\mathrm{num}}\) and independent exact \(B_Q\) projections. It does not
decide proposition identity, renderability, or authority. The focus branch
receives only aggregate references:

```text
focusCandidates(
    query: &BoundQuery,
    activated: &EligibleActivatedMemorySet<'call>,
    K
)
```

Inside that boundary it alone owns the logical sub-boundary:

```text
deriveRequestPropositions(
    query: &BoundQuery,
    activated: &EligibleActivatedMemorySet<'call>,
    K
)
```

\[
\operatorname{deriveRequestPropositions}
(\&Q,\&\mathcal A;K)
\rightarrow
\mathcal R_Q
\;\mid\;
\operatorname{RequestPropositionError}.
\]

This is a logical interface, not a committed public Rust API. Only after
validating both sealed aggregates may the implementation privately borrow
\(Q_{\mathrm{num}}\) and \(B_Q\) from \(Q\), and \(\Lambda_A\) plus the private
nonserializable `InvocationInstanceWitness` \(\omega_A\) and private
noncloneable `EligibleSetInstanceWitness<'call>` from \(\mathcal A\). None is
an independently constructible or caller-supplied argument. The same rule
holds when the activated-record collection is empty. Both witnesses are copied
unchanged and neither can affect proposition derivation, identity, score,
order, diagnostics, or bytes. The interface receives no principal, policy
object, authorization view, authorization service, current-invocation anchor,
expected-set anchor, or second authorization result.

`PLAN-01` can validate aggregate structure, query-to-lineage content equality,
and internal receipt consistency, but it cannot establish that one otherwise
valid \(\mathcal A\) came from the current invocation or is the exact set
selected for both branches. A whole foreign set from a separate same-content
call may carry equal \(B_Q\) and \(\Lambda_A\) plus a different internally
consistent invocation witness; a reconstructed set inside the same call may
carry the same invocation witness but a different set-instance witness.
Without independently borrowed current-call and expected-set anchors, those
cases are intentionally unclassified here. `PLAN-01` must preserve both
witnesses; the later combined planner compares each with the corresponding
independently anchored value in its private planning scope and rejects a
foreign invocation or reconstructed set. Branch-to-branch or query-to-lineage
agreement alone proves neither property.

The canonical join is:

\[
B_Q=
(request\_id_Q,situation\_id_Q,configuration\_id_Q),
\]

\[
\pi_Q(\Lambda_A)=
(request\_id_A,situation\_id_A,configuration\_id_A),
\]

\[
J_{QA}=
\operatorname{joinRequestLineage}(B_Q,\Lambda_A)
\quad\text{iff}\quad
B_Q=\pi_Q(\Lambda_A).
\]

The comparison is exact and follows the displayed field order. Equality of
`request_id` and `situation_id` compares their complete typed identity values,
including identity schema, digest algorithm, content digest, and
configuration-bound digest; comparing only an outer digest or display form is
invalid. Their owning constructors independently anchored them to their
respective sealed ingress values; `PLAN-01` receives neither ingress aggregate
and does not re-anchor them. It validates `BoundQuery` and the shared set, then
checks exact projection equality. Neither projection validates the other
merely by echoing its fields, and equality does not prove current-call witness
membership.

A missing, duplicate, malformed, non-canonical, recomputation-inconsistent, or
configuration-inconsistent private \(B_Q\) field or `BoundQueryContentId` is
`InvalidQueryBinding`.
Malformed or recomputation-inconsistent shared-set lineage is a shared-set
structural error. An otherwise valid equality failure, cross-request swap, or
source-content mismatch is `LineageMismatch`. Detected reuse of one complete
typed identity for different retained canonical content is
`ContentIdentityCollision`. Every failure is terminal for that compile call;
there is no ID regeneration, caller fallback, lossy-facet comparison, or
best-effort join.

On success, \(J_{QA}\) retains \(B_Q\) and the source-receipt projection from
the same \(\Lambda_A\). `policy_revision_id` and `authorization_view_id` come
exclusively from \(\Lambda_A\); they are never read from, compared against, or
reconstructed from bound `Q`, \(\widehat B_{\mathrm{in}}\), or ambient process
state. Conversely, the request, situation, and configuration identities in
\(\Lambda_A\) originate only from compiler ingress, not from bound `Q` or the
authorization view.

\(\mathcal R_Q\) is a finite canonical `RequestPropositionSet`:

```text
RequestPropositionSet
├── schema_id
├── invocation_instance_witness: private, nonserializable
├── query_binding
│   ├── request_id
│   ├── situation_id
│   └── configuration_id
├── source_receipt
│   ├── request_id
│   ├── situation_id
│   ├── policy_revision_id
│   ├── authorization_view_id
│   └── configuration_id
└── sources[]
    ├── request_proposition_instance_id
    ├── request_proposition_semantic_key
    ├── source_kind
    │   ├── AuthenticatedPrompt
    │   ├── SituationStatement
    │   └── RequestMetadata
    ├── source_locator
    ├── numerical_meaning
    ├── exact_bindings[]
    ├── qualifiers[]
    ├── authority_ceiling
    ├── allowed_use_ceiling
    ├── derivation_id
    ├── request_support_score
    └── order_key: RequestPropositionSourceOrderKey
```

`query_binding` is the exact ordered copy of \(B_Q\). Its field order is
`request_id`, `situation_id`, then `configuration_id`; it contains no policy
or authorization-view identity. Each content-identity field retains its typed
inner schema, algorithm, content-digest, and bound-digest components; a
serialization that drops those components is not the same binding.

The five-field `source_receipt` is created only after the canonical join and
is the exact ordered projection of \(\Lambda_A\), not a second lineage
authority. Its field order is
`request_id`, `situation_id`, `policy_revision_id`,
`authorization_view_id`, then `configuration_id`. A missing field, mismatch,
receipt assembled from different calls, or attempt to use a request source
with another \(\Lambda_A\) is structural failure. The repeated
request/situation/configuration identities are audit evidence for the checked
join; they do not authorize data or repeat an authorization decision.

`source_locator` is total by source kind:

- `AuthenticatedPrompt` uses one nonempty canonical set of valid UTF-8 byte
  ranges in the retained original prompt;
- `SituationStatement` uses the zero-based statement ordinal plus one
  nonempty canonical set of valid UTF-8 byte ranges in that exact statement;
  and
- `RequestMetadata` uses one registered metadata-schema identity and field
  identity. Its exact value remains in `X_Q`.

Situation encoding checks ranges against the original immutable request
buffers, verifies that they do not split a UTF-8 code point, sorts them by
`(start, end)`, rejects overlap, and retains the source-buffer content identity
in `X_Q`. Request-proposition construction consumes those validated bindings;
it does not reread or reconstruct the original text. The original prompt and
situation bytes remain validator evidence and are not copied into the
generative plan. Arbitrary prompt or situation passages cannot be smuggled
through exact-value slots; only registered exact-value types with an
authorized binding may enter the plan sidecar.

Each source entry represents one typed proposition that a pinned derivation
attributes to those exact request locations. A derivation may expose an
explicit user request, explicit constraint, caller-reported situation,
declared contextual time, or other registered metadata meaning. It may not
infer an unstated preference, goal, fact, causal relation, emotion, intent, or
world state. When a supported meaning cannot be established under the pinned
derivation, the source is omitted; malformed input, an unknown derivation, or
a derivation output outside its declared schema is an explicit error.

Authority is source-kind bounded:

- `AuthenticatedPrompt` may carry at most the authenticated current
  user-prompt authority established by ingress and the invocation context. It
  supports only the meaning actually bound to the cited prompt ranges.
- `SituationStatement` is caller-supplied descriptive data. It is qualified
  as `CallerReported`, cannot establish an independent world fact, and cannot
  create a user instruction, goal, preference, permission, or action.
- `RequestMetadata` is a caller-declared typed value. It may support a
  qualified contextual statement under its registered schema, but it cannot
  establish external truth or instruction authority.

The installed source-ceiling artifact maps each source kind and derivation
class to one authority and allowed-use ceiling. The artifact is authenticated,
total over the supported schema, content-identified by `configuration_id`, and
compatible with the `policy_revision_id` copied from \(\Lambda_A\).
`authorization_view_id` remains an opaque shared-set lineage identity; the
focus branch neither receives nor opens the corresponding view.

Applying this mapping is a pure authority-lowering classification over
request-local sources. It does not determine memory readability, admit a
record, expand disclosure, or repeat authorization. A source may be
`FocusUse` or excluded; no request source is eligible for the expectation
branch's observed-transition use. A missing, unauthenticated, or
policy-incompatible mapping is `AuthorityMappingUnavailable`; there is no
fallback authorization path. Downstream consolidation takes the meet of all
essential source ceilings and never promotes a caller report into an
authenticated instruction or observed memory fact.

For each canonical source entry \(r_k^Q\), the pinned request-support
derivation computes:

\[
q_k=
\operatorname{cal}_{\mathrm{request}}
\left(
\operatorname{supportFit}(Q_{\mathrm{num}},r_k^Q);K
\right)
\in[0,1].
\]

The metric, calibrator, numerical policy, input facets, missingness behavior,
and accumulation order are content-identified configuration artifacts. The
score measures request-local focus relevance only. It is not truth,
confidence, instruction strength, probability, safety, or permission.
Non-finite output or a value outside the declared domain is an explicit
representation error.

`RequestPropositionInstanceId` is content-derived from the source-receipt projection,
source-kind tag, canonical locator, proposition schema and meaning identity,
derivation identity, exact-binding identities, qualifiers, authority ceiling,
and allowed-use ceiling. Display prose, insertion order, and \(q_k\) do not
participate in identity.

`RequestPropositionSemanticKey` is derived only from the source-kind tag,
canonical locator class, proposition schema and numerical meaning identity,
derivation class, the canonical set of `ExactSlotSemanticDescriptor` values,
qualifiers, authority ceiling, and allowed-use ceiling. It excludes
authoritative exact values, exact-surface bytes and content identities,
exact-binding instance identities, \(B_Q\), \(\Lambda_A\), every
request-local/source-receipt identity, and support score. It is the only
request-proposition identity permitted in semantic grouping or priority.
`RequestPropositionInstanceId` and exact bindings retain exact provenance and
content after those decisions.

`RequestPropositionSourceOrderKey` is the closed lexicographic tuple:

1. source-kind rank: `AuthenticatedPrompt`, `SituationStatement`, then
   `RequestMetadata`;
2. the source-kind locator defined above;
3. ascending proposition-schema and meaning identities;
4. ascending derivation identity; and
5. ascending `RequestPropositionSemanticKey`.

The set is sorted by this key before scoring or consolidation. Unknown tags,
invalid locators, duplicate complete keys, duplicate identities, incompatible
exact bindings, or a key that disagrees with its entry are structural errors.
Repeated statements at different ordinals remain separately attributable, but
their repetition cannot increase a later proposition score because request
support is aggregated by `max`.

`RequestPropositionError` has closed internal reason codes for:

- `InvalidQueryBinding`;
- `LineageMismatch`;
- `InvalidSourceLocator`;
- `UnknownSourceKind`;
- `UnknownDerivation`;
- `InvalidPropositionMeaning`;
- `AuthorityMappingUnavailable`;
- `InvalidExactBinding`;
- `InvalidSupportScore`;
- `DuplicateSourceIdentity`;
- `DuplicateSourceOrderKey`; and
- `RequestPropositionLimitExceeded`.

`InvalidExactBinding` preserves the closed typed source
`InvalidExactSlotSemanticDescriptor` or `ExactSlotValueConflict`; it is not a
message-only flattening of either condition.

A derivation with no supported request proposition returns a valid empty set,
not an error.
The focus branch preserves these typed causes and does not map them to a public
compiler class itself. Missing or invalid authenticated artifacts, defensive
lineage failures, invalid numerical derivation, and configured resource-limit
failures remain distinct so the reference architecture can map them without
inspecting message text.

\(\mathcal R_Q\) is ephemeral focus-branch state. Its entries are not inserted
into the memory revision, retrieval index, activated-memory set, transition
store, access history, or expectation bundle. They receive no persistent
record identity, provenance root, dependency group, observation status, or
memory validity merely by being derived. Renderer output and downstream agent
output cannot turn them into memory truth. Persistent adoption requires a
separate authorized memory-management operation and a later compile.

### Request-local proposition consolidation

Ranking selects memory record versions, while \(\mathcal R_Q\) supplies
validated request sources. Rendering requires coherent propositions.
Request-local proposition consolidation groups compatible sources from both
paths without modifying persistent memory.

A request-local proposition is:

\[
\phi=
(
\operatorname{id}_{\phi},
\operatorname{meaning}_{\phi},
\operatorname{support}_{\phi},
\operatorname{exact}_{\phi},
\operatorname{qualifiers}_{\phi},
\operatorname{authority}_{\phi},
\operatorname{score}_{\phi}
)
\]

where `support_phi` is a nonempty set partitioned into request support
`support_phi^Q` and usage-compatible immutable memory support
`support_phi^M`.

`support_phi^Q` contains only entries from the canonical
\(\mathcal R_Q\) bound to this call. `support_phi^M` contains only record
versions from the exact shared `EligibleActivatedMemorySet`. A proposition may
be request-only, memory-only, or jointly supported. The two source namespaces
remain tagged and disjoint even when their numeric identifiers have equal
underlying values.

`meaning_phi` is a typed numerical proposition structure with role-bound
facets and exact-sidecar references. It is not a prose sentence, draft
attention text, or text chunk. Natural-language realization first occurs in
the renderer defined by the vector-to-attention specification.

Records may consolidate only when:

- a pinned equivalence contract judges their proposed meanings compatible;
- every exact slot with the same
  `(ExactSlotOwnerSemanticDescriptor, ExactSlotSemanticLocator)` agrees
  byte-for-byte or returns `ExactSlotValueConflict`; the same locator under
  different independent owner descriptors remains separate, and an explicitly
  shared `SharedExactSlotMeaningKey`
  deliberately invokes the equality requirement;
- temporal, social, and modal scopes are compatible;
- no unresolved contradiction is hidden;
- all essential support remains disclosable; and
- the output authority is permitted by every essential supporting source.

Incomparable, contradictory, differently scoped, or differently attributed
claims remain distinct propositions. Their vectors are not averaged into a
false compromise.

The consolidation oracle begins with the canonically ordered
\(\mathcal R_Q\), then the activated memory records in their canonical stable
record order. The order determines only deterministic traversal. The pinned
equivalence and conflict contracts determine grouping, and every optimized
implementation must produce the same partition, support bindings, authority
ceilings, qualifications, and final canonical order.

To prevent repeated paraphrases or duplicated imports from creating artificial
confidence, memory support records are grouped by provenance root. Request
sources retain their exact source identities and locators, but repeated
request support is not summed. For a request-supported proposition, define:

\[
q_{\phi}=
\max
\{q_k\mid r_k^Q\in\operatorname{support}_{\phi}^{Q}\}.
\]

It exists only when `support_phi^Q` is nonempty. The set inside this `max` is
therefore nonempty, and every \(q_k\) retains its exact request-source binding
and derivation identity. The proposition's activation score is
conservatively:

\[
\operatorname{score}_{\phi}
=
\max
\left(
\{A_i\mid i\in\operatorname{support}_{\phi}^{M}\}
\cup
\{q_\phi\mid\operatorname{support}_{\phi}^{Q}\ne\varnothing\}
\right)
\]

The set inside `max` is nonempty because every proposition has support.
Dependency-group diversity and conflicts are recorded as separate planning
features. They do not silently increase the activation score. A later accepted
decision may replace this conservative aggregation only after source-dependence
and calibration evidence exists.

Every consolidated proposition receives two identities. Its
`PropositionInstanceId` is content-derived over canonical meaning, complete
tagged support identities, exact bindings, qualifiers, authority and
allowed-use ceilings, and derivation/configuration identity. Its
`PropositionSemanticKey` is derived from meaning, lineage-independent
qualifiers, the canonical `ExactSlotSemanticDescriptor` set, authority and
allowed-use classes, and stable derivation semantics; it excludes
authoritative exact values, exact-surface bytes and content identities,
exact-binding instance identities, support identities, \(B_Q\),
\(\Lambda_A\), and every request-local instance identity. A
request-only proposition does not borrow a persistent provenance root or
memory identity. A mixed proposition preserves request attribution and memory
provenance separately. Consolidation emits at most one proposition per
`PropositionSemanticKey`; a duplicate after registered equivalence resolution
is a structural error.

Consolidation creates request-local computational state only. Persistent
episodic-to-semantic consolidation, reconsolidation, deletion, correction, and
learning belong to a separately authorized memory-management path.

### Focus-candidate construction

The focus branch receives `&BoundQuery` and
`&EligibleActivatedMemorySet<'call>` before renderer-budget pruning. The
private \(Q_{\mathrm{num}}\), \(B_Q\), \(\Lambda_A\), \(\omega_A\), and
\(\sigma_A\) projections are available only through the aggregate validation
and internal destructuring described above; no focus constructor accepts them
separately.
It first constructs \(\mathcal R_Q\) under the
validated boundary above, consolidates compatible request and memory support
into propositions, then partitions the applicable propositions into:

- inclusion candidates;
- mandatory inclusion candidates; and
- control-only exclusions created by an explicit request, policy,
  supersession, or conflict-resolution rule.

Authorization-rejected records never enter any partition. Each inclusion
candidate is classified as current focus, dominant goal, dominant constraint,
relevant background, secondary influence, conflict, or uncertainty. It retains
its activation, support, provenance roots, authority ceiling, qualifications,
exact-sidecar bindings, and stable identities.
Control-only exclusions retain enough evidence for validation but are never
renderer input.

The output is:

```text
FocusCandidateSet
├── schema_id
├── invocation_instance_witness: exact private copy from shared set
├── eligible_set_instance_witness: exact private handle from shared set
├── candidates[]
│   ├── proposition_instance_id
│   ├── proposition_semantic_key
│   ├── focus_roles[]
│   ├── numerical_meaning
│   ├── activation
│   ├── support[]
│   │   ├── RequestPropositionSource
│   │   └── ActivatedMemorySource
│   ├── provenance_roots[]
│   ├── qualifications[]
│   ├── exact_bindings[]
│   ├── mandatory
│   └── order_key: FocusCandidateOrderKey
├── control_exclusions[]
└── source_receipt: exact copy of Lambda_A
```

The predictive-attention specification solely owns the complete immutable
lineage tuple \(\Lambda_A\). The focus branch copies it field-for-field from the
one shared `EligibleActivatedMemorySet`; it does not omit, extend, or
reconstruct lineage from ambient state. A mismatch with the shared set or a
receipt assembled from more than one set is a structural error. The receipt is
lineage metadata only: it contains no raw evidence, diagnostic prose, or
independent semantic truth.

The focus branch also copies the shared set's
`InvocationInstanceWitness` field-for-field into
`RequestPropositionSet` and `FocusCandidateSet`, and propagates the shared
set's distinct `EligibleSetInstanceWitness` into `FocusCandidateSet`. With no
independent planning scope, it cannot classify either witness as current,
foreign, selected, or reconstructed. It may not serialize, hash, order, score,
render, diagnose, or derive a semantic identity from either witness. Planning
later compares the invocation witness with its independently anchored current
invocation and the set witness with its independently anchored exact shared
set; mutual branch agreement alone remains insufficient.

The `RequestPropositionSet` receipt must equal its five-field projection from
this same \(\Lambda_A\), and its three-field `query_binding` must equal
\(\pi_Q(\Lambda_A)\). Request-only focus therefore retains the same memory,
policy, authorization, retrieval, activation, and configuration lineage as
the empty activated-memory set used by the call. It does not replace
\(\Lambda_A\) with a shorter receipt, import an authorization-view identity
into either query form, or perform authorization again.

`provenance_roots` and dependency groups contain only persistent-memory
provenance. They are empty for a request-only candidate; request attribution
remains complete through its tagged `RequestPropositionSource` bindings.
Neither an empty provenance-root list nor a request-source identity is
reinterpreted as independent memory evidence.

The schema's versioned total focus-role order sorts every candidate's
nonempty, duplicate-free `focus_roles` list. The first role is the primary
role. `FocusCandidateOrderKey` is the closed lexicographic tuple:

1. primary focus-role rank;
2. the complete sorted focus-role rank vector;
3. descending finite canonical activation; and
4. ascending `PropositionSemanticKey`.

The key is derived, never caller-authored. Its activation component uses the
exact bounded activation value already carried by the candidate under the
pinned numerical policy; it introduces no new score. An unknown role,
duplicate role, non-total role table, mismatch between a candidate and its
derived key, duplicate complete candidate key, or duplicate
`PropositionSemanticKey` is a structural error.
Missing qualifications, incompatible exact values, unresolved contradictions,
or an unbounded candidate set are also structural errors. No candidate is
silently truncated merely to satisfy the final output budget.

The focus branch does not own final plan selection. The
[focus-and-expectation-planning specification](focus-and-expectation-planning.md)
combines this complete candidate set with the independently produced
expectation bundle, applies the one authoritative rendering budget and
lexicographic selection contract, and creates the sole renderer-facing plan.
This separation prevents a focus-only objective from deleting alternative or
counterevidential transition support before expectation formation.

### Focus contribution to the attention text

When selected into the combined plan, focus candidates may support language
about:

- what the current situation makes salient;
- which goals or constraints dominate;
- which background materially changes interpretation;
- which secondary pressures remain relevant; and
- which conflicts or uncertainties must remain visible.

The combined attention text is an interface artifact. It is not a transcript
of internal human cognition, model reasoning, hidden reasoning, or chain of
thought. The focus branch does not create expectations, answers, executable
actions, or unsupported claims about emotions, beliefs, motives, or subjective
experience.

### Action-selection boundary

This specification ends at focus-candidate construction. Nemosyne V1 does not:

- choose or execute a downstream action;
- authorize a tool;
- override the downstream model's safety or instruction hierarchy;
- infer that the highest-activation memory prescribes the correct action; or
- guarantee that a focus contribution improves the downstream response.

Action competition, evidence accumulation, and urgency-gating research motivate
separating possible actions and time pressure from general semantic similarity.
They do not place action selection inside the V1 compiler.

## Computational complexity

This specification owns the conservative reference bounds for memory
eligibility, numerical cue and signal derivation, spreading activation,
request-local proposition consolidation, and focus-candidate construction. Let:

- \(n_M\) be the number of records in the acquired immutable revision;
- \(n_r\) be the bounded direct candidate count;
- \(c_{\mathrm{retrieve}}\) be the declared worst-case per-record direct
  retrieval-cue and candidate-competition cost;
- \(c_{\mathrm{facet}}\) be the sum of declared per-graph-node direct,
  temporal, spatial, procedural, and social metric/calibrator costs;
- \(h_\Sigma\) be the total qualifying history-event count traversed for the
  \(n_g\) graph nodes;
- \(n_G\) and \(n_H\) be the active-goal and represented-hazard counts, with
  worst-case per-pair costs \(c_G\) and \(c_H\);
- \(n_g,e_g,k_g\) be the bounded spreading graph's node count, edge count, and
  fixed iteration count;
- \(c_{\mathrm{seed}}\) be the worst-case complete cue/presence assembly and
  seed-calibrator cost per graph node, \(c_{\mathrm{edge}}\) the worst-case
  membership, type, version, and fixed-point validation cost per graph edge,
  and \(c_{\mathrm{acc}}\) the worst-case cost of one exact binary-rational
  accumulator insertion or finalization under the configured ceilings;
- \(c_e,c_j\) be the activation evidence and inhibition channel counts;
- \(n_a\) be the activated records admitted to consolidation;
- \(n_Q\) be the bounded count of validated entries in
  \(\mathcal R_Q\), and \(c_Q\) the declared worst-case validation,
  derivation, and scoring cost per entry;
- \(d_{\mathrm{eq}}\) be the declared worst-case cost of one proposition
  compatibility/equivalence comparison, including exact-slot checks; and
- \(n_\phi,s_\phi\) be the resulting proposition count and total retained
  source-binding count.

The exhaustive reference candidate generator authorizes and scores every
eligible record, keeps a bounded top-\(n_r\) heap, and therefore costs
\[
O\!\left(
n_M(c_{\mathrm{policy}}+c_{\mathrm{retrieve}}+\log(1+n_r))
\right)
\]
time and \(O(n_r)\) selection workspace, where
\(c_{\mathrm{policy}}\) is the pinned worst-case per-record authorization,
validity, and usage-gate cost. A selected approximate index may replace this
oracle only when its own build/query/storage bounds, immutable-revision
binding, and recall contract are declared and it passes the proof program's
false-negative gates. Authorization still precedes retrieval competition.

For the complete bounded graph-node set, reference facet, availability, goal,
procedural, risk, social, and seed derivation costs
\[
O\!\left(
n_g c_{\mathrm{facet}}+h_\Sigma+n_g(n_Gc_G+n_Hc_H)
+n_g(c_e+c_j+c_{\mathrm{seed}}+c_{\mathrm{acc}})
\right)
\]
before propagation. Canonical graph construction, edge sorting, exact
fixed-point row validation, and matrix materialization cost at most
\[
O\!\left(
n_g\log(1+n_g)+e_g\log(1+e_g)
+e_{\mathrm{scan}}(c_{\mathrm{edge}}+c_{\mathrm{acc}})
\right)
\]
time and \(O(n_g+e_{\mathrm{scan}})\) workspace, with
\(e_{\mathrm{scan}}=e_g\) on success and
\(e_{\mathrm{scan}}\leq E_g^{\max}\) on a bounded edge-limit failure. A
selected metric or calibrator with a
greater cost replaces its corresponding term explicitly; no encoder,
nearest-neighbour, calibrator, graph-membership, or exact-accumulator cost is
hidden inside an assumed constant. Bounded spreading, including exact mass
postconditions on every iterate, costs
\[
O\!\left(k_g(n_g+e_g+n_g c_{\mathrm{acc}})\right)
\]
time and \(O(n_g+e_g)\) workspace. Activation evaluation and full ordering use
the exact bounds owned by the
[situation-conditioned activation specification](situation-conditioned-activation.md#computational-complexity).

Request-proposition construction costs \(O(n_Qc_Q+n_Q\log n_Q)\) time and
\(O(n_Q)\) request-local state, including canonical ordering. The conservative
consolidation oracle then compares every request source and activated record
with each existing compatible proposition representative and therefore costs
at most
\[
O\!\left(
(n_a+n_Q)^2d_{\mathrm{eq}}
+n_Qc_Q
+n_Q\log n_Q
+s_\phi
+n_\phi\log n_\phi
\right)
\]
time and \(O(n_a+n_Q+s_\phi+n_\phi)\) request-local state. An optimized
grouping index may reduce comparisons only if it produces the same proposition
partition, conflicts, support bindings, authority ceilings, and canonical
order as the oracle. Focus-candidate construction is linear in the proposition
and source state after that canonical sort.

All counts have authenticated finite configuration ceilings and use checked
arithmetic before allocation or iteration. These expressions are complexity
contracts, not scale, latency, memory, or suitability claims. Cold encoder and
database-open costs, persistent index construction, rendering, validation, and
transport are owned by the reference architecture and their focused
specifications.

## Preconditions

A conforming experiment requires:

- the accepted local, read-only V1 product boundary;
- a pinned immutable request, compiler configuration, and authorized memory
  revision;
- valid trusted `t_auth` for the separate authorization and memory-validity
  path, distinct from caller-controlled contextual time and absent from
  situation encoding;
- typed numerical schemas with explicit dimensions, metrics, normalization,
  absence semantics, and artifact identities;
- a pinned numerical execution contract covering dtype, backend, elementary
  functions, reduction order, rounding, and tie semantics;
- lossless sidecars for every exact value required downstream;
- an authorization and disclosure view established before candidate
  generation;
- bounded candidate, graph, relation, cardinality, and work budgets;
- a channel schema with versioned parameters and no implicit default weights;
- a positive effective evidence denominator for every activation call;
- stable provenance roots and record-version identities;
- a finite authenticated request-proposition schema with total source-kind,
  locator, authority, allowed-use, derivation, scoring, and canonical-order
  contracts;
- declared focus-candidate limits; and
- no network access or persistent write capability on the compile path.

## Invariants

- An unauthorized, deleted, revision-incompatible, or representation-invalid
  record cannot affect candidates, activations, focus propositions, output
  content, or content-bearing diagnostics.
- A historically eligible but currently invalid or superseded record cannot
  support a current normative instruction and must retain its historical
  qualification.
- Caller-controlled context cannot change authorization, disclosure expiry,
  current normative validity, or supersession.
- Lossy vectors never replace exact identifiers, timestamps, numbers, units,
  coordinates, names, paths, quotations, or source bytes required for faithful
  reconstruction.
- Missing, unknown, and zero are distinct states.
- Every vector comparison uses compatible typed spaces and pinned transforms.
- Every evidence and inhibition value has a traceable derivation and source.
- `request_id`, `situation_id`, and `configuration_id` are compiler-owned
  typed identities derived under one authenticated pinned configuration;
  callers cannot supply them, and neither bound `Q` nor an authorization view
  is allowed to mint or overwrite them.
- The query and shared-set branches receive independent projections of the
  same sealed ingress binding. Exact typed-identity equality, configuration
  equality, canonical-envelope recomputation, and any observed collision
  evidence are checked before request-derived focus can be constructed.
- Every request-derived focus source is bound to exact validated prompt,
  situation, or metadata evidence, the exact three-field \(B_Q\), and a
  successful canonical join with \(\Lambda_A\). Policy and
  authorization-view identity originate only in \(\Lambda_A\).
- Focus construction has no direct authorization-view input and cannot
  retrieve, reopen, repeat, or widen the shared-set authorization decision.
- Association and graph reachability do not establish truth, causation,
  authority, or confidence.
- Hard policy and integrity failures are never represented as soft inhibition.
- Compilation does not update access history, consolidation state, weights,
  vectors, indexes, or any persistent memory value.
- Request-local consolidation retains all essential support and never hides a
  material contradiction or qualification.
- Duplicated or correlated sources do not increase proposition activation
  through repeated counting.
- Every selected focus proposition has source bindings and an authority ceiling
  permitted by its essential support.
- A situation or metadata source remains a qualified caller assertion and
  cannot become a user instruction, observed fact, persistent memory, or
  expectation transition through focus construction.
- Required exact values flow through explicit sidecar bindings.
- Focus-candidate construction respects its finite candidate bound without
  truncating required meaning.
- The focus-candidate set contains neither an answer nor an executable action.
- Activation values are not presented as truth, probability, safety,
  instruction authority, or biological measurements.
- A rendered focus contribution is not presented as human inner speech or
  chain of thought.
- For fixed input, revision, configuration, canonical order, and pinned dtype,
  backend, elementary functions, reduction order, rounding, and tie semantics,
  every numerical intermediate and focus-candidate set is reproducible within
  the declared bitwise or tolerance contract. A V1-deployable compile uses no
  request-time random input.

## Edge cases

- An empty authorized memory revision may still produce request-supported
  focus from a validated \(\mathcal R_Q\) bound to the empty shared set's
  complete lineage.
- Empty memory plus no supported request proposition produces an empty
  focus-candidate set rather than a generic invented focus.
- No justified additional focus produces a valid empty focus-candidate set.
- A perfect vector match in an unauthorized record has no effect.
- A recent irrelevant record may rank below an old but strongly matching
  procedural record.
- A highly emotional record without causal risk relevance receives no automatic
  risk advantage.
- An approaching deadline remains secondary to a separately enforced hard
  constraint.
- A memory associated with the right topic but the wrong person or interaction
  may be down-ranked or excluded from a proposition.
- One unacknowledged assertion is not treated as shared common ground.
- Unknown location does not become zero distance or a matching location.
- Incompatible encoder versions cannot be compared or silently re-encoded
  during compilation.
- A relation path that leaves the usage-compatible request graph terminates at
  the boundary.
- A graph cycle remains bounded by restart, iteration, node, and edge budgets.
- Two paraphrases from one provenance root cannot create artificial
  corroboration.
- Compatible sources with different exact values remain separate or explicitly
  conflicting.
- A consolidated semantic proposition does not delete or replace its episodic
  support.
- Candidate construction retains every material qualification required to
  interpret a candidate; an overflow of its own finite bound is explicit.
- Equal activation or candidate-order values use stable numeric identifiers
  rather than input order.
- A renderer that produces plausible but unsupported introspection fails
  faithfulness validation outside this specification.

## Training implications

### Separation of learning problems

The following learning or calibration problems are distinct:

1. producing compatible typed facets from prompt, situation, and memory;
2. calibrating facet metrics into bounded evidence signals;
3. deriving situation-dependent gates;
4. calibrating evidence weights and soft inhibition strengths;
5. identifying proposition equivalence, contradiction, and scope;
6. constructing a complete focus-candidate set;
7. combining focus candidates with independently derived expectations; and
8. lexicalizing the fixed combined plan.

Training only an end-to-end vector-to-text target would obscure which boundary
failed and could teach the renderer to retrieve, rerank, invent, or answer the
prompt. Each boundary requires independent labels and evaluation before joint
optimization is considered.

### Example schema

A complete corpus and evaluation-harness record should retain at least:

- one stable scenario-family identifier;
- the original prompt and declared situation;
- typed numerical query facets and exact request sidecars;
- the canonical `RequestPropositionSet`, validated source locators, source
  kinds, authority and use ceilings, scores, order keys, and receipt;
- eligible and excluded memory-version identifiers;
- candidate signals, gates, parameters, activations, and derivation receipts;
- relation paths used by bounded spreading activation;
- request-local proposition support and provenance groups;
- dominant, secondary, conflicting, uncertain, and mandatory focus candidates
  plus
  control-only exclusion records;
- exact-value bindings;
- `must_include` and `must_exclude` proposition identifiers;
- output language and maximum rendering budget;
- one evidence-bound target combined attention text; and
- downstream outcome labels kept separate from renderer faithfulness labels.

The focus-candidate record is not renderer input. Only the generator view of
the canonical combined plan—selected focus and expectation items, their
relations, language, budget, and exact-slot identities—is renderer model input.
Control-only exclusions, exact surface bytes, the original prompt, situation,
raw memory, unselected candidates, and derivation receipts remain outside the
renderer as validator input, corpus provenance, leakage labels, or evaluation
context.

All generated variants from one semantic scenario family belong to the same
train, validation, or test partition.

### Normative empty-memory situation-only fixture

The conformance corpus includes one frozen fixture whose semantic content is:

```text
original prompt:
What matters in this situation?

situation statement 0:
The build failure first appeared after the dependency update.

authorized memory revision:
empty
```

The fixture's pinned request-proposition derivation yields exactly one
`SituationStatement` source for the explicit caller report that the failure
first appeared after the dependency update. It binds statement ordinal `0`,
the exact supporting byte ranges, one registered numerical meaning, the
`CallerReported` qualifier, a descriptive-data authority ceiling,
`FocusUse`, its derivation identity, and a finite \(q_k\). The generic prompt
does not yield an additional focus proposition under this fixture's registered
derivation.

The shared `EligibleActivatedMemorySet` contains zero activated records but a
complete \(\Lambda_A\), one private invocation witness, and one fresh private
`EligibleSetInstanceWitness<'call>` for that exact set object. The resulting
`FocusCandidateSet` contains exactly one
request-only candidate with:

- a three-field \(B_Q\) exactly equal to \(\pi_Q(\Lambda_A)\) under the
  canonical join;
- the roles `CurrentSituation` and `RelevantBackground` in canonical role
  order;
- activation equal to that source's \(q_k\);
- one tagged request source and no activated-memory source;
- no persistent provenance root or dependency group;
- the `CallerReported` qualification and descriptive-data authority ceiling;
- the exact complete \(\Lambda_A\) as its source receipt;
- the same private invocation witness and set-instance witness, with no
  semantic or ordering effect; and
- a `FocusCandidateOrderKey` derived normally from role, activation, and
  lineage-independent `PropositionSemanticKey`.

The fixture creates no transition, expectation, memory record, access-history
event, or persistent write. It does not authorize the stronger statements
that the dependency update caused the failure or that the downstream agent
must change dependencies. A permutation of request-source construction inputs
must produce the identical `RequestPropositionSet` and `FocusCandidateSet`.

### Required scenario families

Training and evaluation should include:

- recent but irrelevant memory versus remote procedural relevance;
- high semantic similarity with temporal, spatial, social, or task mismatch;
- an immediate hazard competing with a later appointment;
- active, completed, cancelled, and historical goals;
- emotional salience without task or risk relevance;
- partner-specific information used with the wrong participant;
- unacknowledged assertions versus established common ground;
- redundant imports and paraphrases with one provenance root;
- additional dependency-group support without hidden source duplication;
- unresolved contradiction and supersession;
- the normative empty-memory situation-only fixture above, plus other empty
  memory and request-only focus cases;
- no justified attention;
- exact names, identifiers, paths, quantities, locations, and deadlines;
- absent versus unknown versus explicit zero values;
- incompatible numerical-schema or index revisions;
- embedded prompt injection or executable-looking text inside memory data;
- mixed-language memory with output in the prompt language;
- limited budgets requiring principled omission; and
- cases where the only faithful outcome is an explicit error.

### Focus-candidate label requirements

Focus labels identify current situation, goal, constraint, relevant background,
secondary influence, conflict, uncertainty, and social perspective. Each label
retains source, authority, qualification, exact-binding, and mandatory/optional
status. This specification does not own target prose. The combined-plan and
renderer specifications own selection and lexicalization targets.

Free-form human think-aloud reports are not ground truth for hidden cognition.
If human inner-experience data is collected, it requires informed consent and
must be labeled as fallible self-report rather than direct access to cognitive
mechanisms.

### Evaluation targets

Primary evaluation measures are:

- candidate recall against required supporting propositions;
- exclusion of unauthorized, invalid, and irrelevant records;
- pairwise ranking preferences and calibrated channel sensitivity;
- required-proposition coverage;
- request-source attribution, authority-ceiling preservation, and false
  proposition derivation rates by source kind;
- unsupported-claim and lost-qualification rates;
- exact-value preservation;
- conflict, attribution, and social-perspective preservation;
- answer leakage;
- language match;
- attention-budget compliance;
- reproducibility under input permutation;
- downstream response quality with and without attention; and
- conditional harm or reversal relative to the no-attention baseline.

Human-likeness, eloquence, and apparent introspective depth are not sufficient
success measures.

## Verification

### Formal and property verification

Verification must establish:

- deterministic
  \(Q_{\mathrm{num}}=\operatorname{encode}(P,S,\Xi;K)\): identical prompt,
  ordered zero-to-three situation statements, contextual-time, location, and
  metadata presence states, and pinned encoder/configuration inputs produce
  identical facets, locators, and source-buffer content identities, with no
  \(B_Q\), resolved call control, or authorization state in any component;
- deterministic exact binding:
  \(\operatorname{bindQuery}(\&request,\&\widehat B_{\mathrm{in}},\&K)\)
  is the sole constructor, derives both private projections from that same
  request/configuration, changes no encoded numerical component, and produces
  identical `BoundQuery` fields and `BoundQueryContentId` for identical
  inputs;
- sealed query aggregation: public or downstream construction of
  \(Q_{\mathrm{num}}\), \(B_Q\), or their pair is unrepresentable; every
  retrieval, signal, focus, expectation, and validation boundary accepts
  `&BoundQuery`, and a private corruption harness swapping either projection,
  canonical request, `K`, or content identity fails `InvalidQueryBinding`;
- deterministic ingress identity: repeated canonical encoding of the exact
  same validated compile request under the same authenticated identity schema
  and configuration produces byte-identical typed `request_id` and
  `situation_id`; metadata map insertion-order permutations do not change
  them;
- mutation separation under the digest assumption: a one-byte prompt mutation
  changes `request_id` while preserving `situation_id`, and a statement byte,
  statement-order, contextual-time, declared-location, or registered
  situation-metadata mutation changes both `situation_id` and `request_id`;
- configuration binding: the same request under another authenticated
  configuration produces different bound identities, while an unauthenticated,
  caller-selected, stale, or post-pin configuration identity is rejected;
- changing only `t_auth`, principal, policy, or authorization-view state cannot
  change \(Q_{\mathrm{num}}\), \(B_Q\), or bound `Q`, while attempting to
  place any such field in request evidence is rejected before encoding;
- exact minimized signal-context projection: one sealed
  `AuthenticatedInvocation` yields one context carrying a reference to its
  opaque generative brand plus pinned context and social-identity schema
  identities, trusted authorization time, and typed authenticated
  social-subject identity; validation independently anchors every copied
  value and that brand to the current aggregate, so a complete context from
  another valid call fails; mixed-\(\Gamma_A\),
  mixed-`I`, mixed-time, wrong-owner/lifetime, missing, duplicate, malformed,
  cross-call, and cross-schema values fail before signal derivation, with no
  ambient or request fallback;
- trusted-input provenance: base availability reads `t_auth` only from
  validated \(V_{\mathrm{sig}}\), and each social channel names exactly one
  authenticated or declared subject source; perturbing excluded principal
  credentials, policy, disclosure, store state, ambient time, or unrelated
  declared partners cannot affect the corresponding fixed channel;
- signal-brand erasure: neither opaque brand reference, object allocation
  identity, nor raw \(\Sigma_{\mathrm{sig}}\) reaches a channel, score,
  semantic order, renderer input, diagnostic payload, or product byte;
- signal-context separation: signal derivation has no \(B_Q\) input; at a
  fixed retained request and configuration, any corrupt private \(B_Q\)
  projection rejects during aggregate/join validation rather than changing a
  signal or gate, while changing a registered trusted-time or
  authenticated-subject input can affect only the channel families whose
  contracts name it;
- eligibility noninterference: changing only ineligible records cannot change
  any content-bearing result;
- facet-type safety: incompatible spaces are rejected before comparison;
- exact-sidecar preservation from memory revision to focus-candidate set;
- bounded and finite calibrated channel outputs, including checked deadline
  division/exponential boundaries and exact zero-underflow behavior;
- two stable social channels invariant to declared-partner order, duplication,
  and cardinality, with deterministic tie attribution, unknown-ID handling,
  source-tag separation, memory-side schema compatibility, authenticated
  one-to-one rotation/migration, and enforced social-family budget;
- finite unique \(C^r\), including empty-set validity, immutable
  record-version-key ordering, arbitrary input-order invariance, exact
  duplicate rejection, same-key/different-payload rejection, and the
  \(|C^r|\leq n_r\) bound;
- direct spreading seeds over exactly \(V_g\), including distinct all-absent
  and all-present-zero masks and explanations, explicit relation-only absence,
  closed absence-reason validation, absent-value and absent-lineage rejection,
  exact equality between the `Present` mask keys and numeric cue keys,
  lineage-only noninterference, node-order and dimension equality with \(W\),
  positive preservation, ideal-reference reconstruction, upward-denominator
  reconstruction, stored `fl_div` reconstruction, and exact binary-rational
  stored mass at most one after normalization;
- canonical graph construction from unique typed eligible edges, including
  deterministic breadth-depth expansion, total/injective relation ranks,
  immutable node keys, raw and accepted edge keys, canonical invalid-input
  error precedence, bounded relation iteration, exact power-of-two fixed-point
  conversion, empty and isolated graphs, orientation, row-budget validation
  without rounded-sum acceptance, no prefix truncation, and node/edge/hop
  ceilings with source-preserving typed failures and declared failed-run
  resource bounds;
- bounded spreading mass for every configured iteration under canonical
  checked `f64`, including the exact-stored-mass-downround counterexample, the
  nonrepresentable exact-denominator counterexample, normalization-still-above-
  one counterexample, derived roundoff correction bound over \(|E_g|\), exact
  stored `fl_div` reconstruction, exact post-correction re-sum, correction
  receipt, and failures above that bound;
- graph authorization closure;
- deterministic canonical ordering;
- request-proposition source-locator validity, receipt equality, authority
  ceilings, lineage-independent semantic-key correctness, and permutation
  invariance;
- exact private \(B_Q=\pi_Q(\Lambda_A)\) joining through
  `(&BoundQuery, &EligibleActivatedMemorySet<'call>)`, full typed-content-
  identity comparison, absence of policy and authorization-view fields from
  either query projection, semantic equality across separately valid
  same-content calls, rejection of a corrupted \(B_Q\)-only mutation, and
  absence of a direct authorization dependency or repeated authorization call
  in focus construction;
- aggregate-only focus APIs: \(\Lambda_A\), \(\omega_A\), and the set-instance
  witness can be borrowed only by destructuring the validated shared set
  internally; `PLAN-01` preserves but does not authenticate either witness,
  and a mutually consistent foreign or same-call reconstructed set remains
  detectable only at the later independently anchored planning boundary;
- exact-slot owner semantics: mutating only an authoritative exact value or
  its lineage preserves every upstream owner descriptor and every downstream
  owner, item, relation, planning, and renderer-slot semantic key; independent
  same-schema/locator items map to distinct
  `ExactSlotOwnerSemanticKey::Item` values; an explicit
  `SharedExactSlotMeaningKey` maps to one shared owner intentionally; and equal
  descriptor-plus-locator or mapped owner-plus-locator with differing
  descriptor or value fails with the corresponding typed exact-slot cause;
- equivalence between derived activation inputs and the existing kernel's
  public contract;
- proposition support completeness and authority non-amplification;
- no hidden conflict through consolidation;
- finite focus-candidate construction and explicit capacity failure; and
- read-only persistent-state noninterference.

### Executable evaluation

A curated, disjoint evaluation corpus must test:

- every required scenario family;
- direct retrieval and bounded graph-retrieval ablations;
- recency-only, similarity-only, goal-only, and full multi-channel baselines;
- no-inhibition and no-consolidation ablations;
- simple top-k against complete bounded `FocusCandidateSet` construction;
- source duplication and correlated-evidence attacks;
- the normative empty-memory situation-only fixture, including exact request
  attribution, request-only scoring, empty persistent provenance, unchanged
  \(\Lambda_A\), and absence of memory or expectation creation;
- prompt-, situation-, and metadata-source authority tests proving that
  caller-reported data cannot become an instruction or observed fact;
- request/source-receipt mismatch, invalid UTF-8 range, duplicate source key,
  invalid query binding, query/shared-set lineage mismatch, unknown
  derivation, non-finite \(q_k\), and unsupported exact-slot cases;
- prompt, situation, context, metadata-presence, language, budget,
  identity-schema, and configuration mutation fixtures with their exact
  expected `request_id`/`situation_id` preservation or separation behavior;
- compile-fail/private-constructor fixtures proving that
  \(Q_{\mathrm{num}}\) and \(B_Q\) cannot be supplied or paired independently,
  that retrieval and signal APIs accept only `&BoundQuery`, plus
  corrupt-aggregate fixtures that swap either private projection across
  requests/configurations and fail `InvalidQueryBinding`;
- `BoundQuery`/shared-set cross-request joins, including identical prompts with
  different situation evidence, fail `LineageMismatch`; separate valid
  same-content calls may join content-equal projections at `PLAN-01`, whose
  output must preserve the foreign witness unchanged until the independently
  anchored planning/validation boundary rejects it;
- constant-ID, prior-request-ID reuse, caller-supplied-ID, outer-digest
  collision, and full collision-witness injection fixtures; every detected
  recomputation mismatch or same-identity/different-canonical-bytes witness
  must fail closed, while the true-digest-collision case remains an explicit
  cryptographic assumption rather than a tested uniqueness proof;
- authenticated configuration pinning, manifest mismatch, post-pin
  substitution, and branch-configuration mismatch fixtures;
- forged private states mixing \(\Gamma_A\), `I`, or `t_auth` from distinct
  successful invocations; wrong-instance/lifetime signal scopes and contexts;
  and proof that the generative brand is absent from semantic/output equality;
- extreme finite availability ages and decay exponents, raw-power underflow,
  every deadline duration/scale boundary, checked division, exponential
  domain failure, and exact-zero underflow;
- declared-partner permutation, duplication, cardinality, exact-tie,
  unknown-identity, empty-set, authenticated/declared/memory source-tag,
  social-schema compatibility, rotation/migration, collision/integrity,
  and social-family-budget fixtures;
- empty, singleton, maximum-size, duplicate, same-key/different-payload, and
  permuted direct-retrieval sets, with exact immutable record-version order and
  no traversal-order-dependent bounded prefix;
- all-absent, all-present-zero, mixed-presence, relation-only, subunit, and
  superunit seed fixtures; unknown absence reason; absent cue carrying a value
  or lineage; cue/mask key disagreement; lineage-only renaming;
  seed/matrix dimension and order mismatches; exact mass whose nearest
  sequential `f64` sum rounds down to one; the nonrepresentable exact
  denominator fixture
  \(\widetilde a^{(0)}=(1,2^{-54})\); exact reconstruction of
  \(a_{\mathrm{ref}}^{(0)}\), \(d_0\), stored
  \(\operatorname{fl\_div}_K\), and \(s_0\); and a normalization whose first
  componentwise division still has exact mass above one;
- canonical spreading-graph fixtures for empty and isolated graphs,
  relation-only nodes, breadth-depth frontier and input permutation, edge
  orientation, raw-key ordering, identical error selection under permutations
  containing multiple invalid entries, missing/duplicate/non-total
  relation-rank artifacts, duplicate/mixed-version edges, invalid endpoints,
  a mock bounded iterator proving that an oversized frontier is not
  materialized, exact public/source-preserving error routing for invalid
  limits, node overflow, iterator `has_more`, and checked integer overflow,
  node and edge counts immediately below, at, and above their ceilings, zero
  and maximum supported
  hop configurations, unsupported hop configuration, a path continuing beyond
  the configured semantic hop bound, fixed-point scale conversion, integer
  overflow, row mass below/at/above one, and ineligible neighbors;
- the floating spreading-activation overshoot counterexample, exact mass
  whose nearest sum rounds down to one, a nonrepresentable correction
  denominator, exact stored `fl_div` reconstruction, correction that still
  exceeds one, mass exactly at and above the derived
  \(\varepsilon_{\mathrm{mass}}(n,|E_g|,\rho;K)\) bound, signed zero,
  nonfinite intermediates, and canonical sparse-order permutations;
- paired valid branch fixtures with equal sealed `BoundQuery` content,
  equal semantic inputs, and only a consistent valid correspondence between
  nonsemantic
  \(\Lambda_A\)-derived request-local instance identities, including equal-role,
  equal-activation focus candidates under a one-of-two budget; selected
  semantic meaning and product bytes must remain equal;
- parameter sensitivity and monotonicity where the mathematics requires it;
- perturbation of time, location, participant, goal, risk, and procedure
  facets independently;
- false-causal relation paths;
- cross-language and rare exact-value cases;
- prompt injection inside authorized memory content;
- exact-slot fixtures proving value- and lineage-only mutation preserves all
  upstream descriptors and downstream semantic keys; deterministic planner
  mapping produces `Item(PlanItemSemanticKey, ExactSlotOwnerRole)` for an item
  descriptor and `Shared(SharedExactSlotMeaningKey)` for a shared descriptor;
  the same schema/path under separate owner descriptors coexists and maps to
  separate item owners; explicit shared descriptors converge; mutation of the
  exact value alone preserves both descriptor and mapped key; the same
  descriptor/path or mapped owner/path with differing values returns
  `ExactSlotValueConflict`; an upstream descriptor with an invalid schema or
  shape returns `InvalidExactSlotSemanticDescriptor`; and a planner-owned
  independent rederivation mismatch against an already admitted immutable
  branch projection returns `SourceProjectionViolation`;
- empty, insufficient, corrupt, and incompatible states;
- zero \(n_r\) and zero \(n_g,e_g\) complexity-bound fixtures; and
- focus-branch utility and combined-plan utility under the V1 proof program.

The activation kernel's existing tests prove only its normalized aggregation
contract. They do not validate the proposed channel meanings, parameter values,
retrieval process, proposition consolidation, focus-candidate construction, or cognitive
fidelity.

### Claim threshold

Decision 0014 retains this focus branch inside the predictive architecture.
Superseded Decision 0012 records the earlier focus-only terminal-plan design.
This specification remains `Proposed` because no conforming implementation or
evaluation evidence exists for its particular mechanisms. An implementation
begins as `Experimental`. A `Validated` status requires:

- frozen facet, sidecar, relation, channel, and focus-candidate schemas;
- implementation and public-boundary tests;
- disjoint curated evaluation data;
- comparison with simpler baselines and named ablations;
- sealed downstream evaluation under the V1 proof program;
- measured failure and harm rates; and
- explicit evidence for every supported product claim.

No evaluation can validate that the system reproduces human consciousness,
inner experience, or literal thought.

## Open questions

- Which facet families, dimensions, encoders, and metrics form the smallest
  sufficient V1 schema?
- Which exact representation is authoritative in the memory-management plane?
- How are observed, inferred, imported, and user-confirmed history events
  distinguished?
- Which calibrators map raw facet metrics and base availability into `[0,1]`?
- Which gate-derivation contract is deterministic and sufficiently
  interpretable?
- Which channel set avoids correlated double counting while retaining useful
  distinctions?
- Which relation types may participate in bounded spreading activation?
- What restart factor, graph budget, and iteration count provide useful recall
  without semantic drift?
- How is request-local proposition equivalence established across languages and
  vector spaces?
- How are provenance roots and dependency groups established without
  overstating corroboration?
- Which deterministic focus-candidate policy gives the combined planner enough
  coverage without unbounded candidate growth?
- Which qualified dimensions and model artifact satisfy the accepted
  vector-prefix renderer contract?
- Which parts require learned models, and which remain deterministic?
- What empirical thresholds justify adoption over similarity-only retrieval and
  simple top-k selection?
- What separate contract governs persistent episodic-to-semantic consolidation
  and reconsolidation?

## References

### Project contracts

- [V1 product contract](v1-product-contract.md)
- [V1 reference architecture](v1-reference-architecture.md)
- [V1 proof program](v1-proof-program.md)
- [V1 delivery program](v1-delivery-program.md)
- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Deterministic activation-parameter evaluation](activation-parameter-evaluation.md)
- [Predictive attention and expectation](predictive-attention-and-expectation.md)
- [Focus and expectation planning](focus-and-expectation-planning.md)
- [Superseded Decision 0011: Adopt a local read-only attention compiler for V1](../decisions/0011-adopt-local-read-only-attention-compiler-v1.md)
- [Decision 0012: Adopt numerical cognitive memory and focus compilation
  (superseded)](../decisions/0012-adopt-numerical-cognitive-memory-and-focus-compilation.md)
- [Decision 0014: Adopt memory-grounded predictive attention](../decisions/0014-adopt-memory-grounded-predictive-attention.md)
- [Decision 0016: Adopt sealed compile-integrity boundaries](../decisions/0016-adopt-sealed-compile-integrity-boundaries.md)

### Human-memory and cognition research

- Tulving, E., and Thomson, D. M. (1973), “Encoding specificity and retrieval
  processes in episodic memory,”
  [doi:10.1037/h0020071](https://doi.org/10.1037/h0020071).
- Godden, D. R., and Baddeley, A. D. (1975), “Context-dependent memory in two
  natural environments: On land and underwater,”
  [doi:10.1111/j.2044-8295.1975.tb01468.x](https://doi.org/10.1111/j.2044-8295.1975.tb01468.x).
- Morris, C. D., Bransford, J. D., and Franks, J. J. (1977), “Levels of
  processing versus transfer appropriate processing,”
  [doi:10.1016/S0022-5371(77)80016-9](https://doi.org/10.1016/S0022-5371(77)80016-9).
- Meyer, D. E., and Schvaneveldt, R. W. (1971), “Facilitation in recognizing
  pairs of words: Evidence of a dependence between retrieval operations,”
  [doi:10.1037/h0031564](https://doi.org/10.1037/h0031564).
- Collins, A. M., and Loftus, E. F. (1975), “A spreading-activation theory of
  semantic processing,”
  [doi:10.1037/0033-295X.82.6.407](https://doi.org/10.1037/0033-295X.82.6.407).
- Raaijmakers, J. G. W., and Shiffrin, R. M. (1981), “Search of associative
  memory,”
  [doi:10.1037/0033-295X.88.2.93](https://doi.org/10.1037/0033-295X.88.2.93).
- Anderson, J. R., and Schooler, L. J. (1991), “Reflections of the environment
  in memory,”
  [doi:10.1111/j.1467-9280.1991.tb00174.x](https://doi.org/10.1111/j.1467-9280.1991.tb00174.x).
- Anderson, J. R., et al. (2004), “An integrated theory of the mind,”
  [doi:10.1037/0033-295X.111.4.1036](https://doi.org/10.1037/0033-295X.111.4.1036).
- Howard, M. W., and Kahana, M. J. (2002), “A distributed representation of
  temporal context,”
  [doi:10.1006/jmps.2001.1388](https://doi.org/10.1006/jmps.2001.1388).
- Polyn, S. M., Norman, K. A., and Kahana, M. J. (2009), “A context maintenance
  and retrieval model of organizational processes in free recall,”
  [doi:10.1037/a0014420](https://doi.org/10.1037/a0014420).
- Goschke, T., and Kuhl, J. (1993), “Representation of intentions: Persisting
  activation in memory,”
  [doi:10.1037/0278-7393.19.5.1211](https://doi.org/10.1037/0278-7393.19.5.1211).
- Marsh, R. L., and Hicks, J. L. (1998), “Event-based prospective memory and
  executive control of working memory,”
  [doi:10.1037/0278-7393.24.2.336](https://doi.org/10.1037/0278-7393.24.2.336).
- Cowan, N. (2001), “The magical number 4 in short-term memory,”
  [doi:10.1017/S0140525X01003922](https://doi.org/10.1017/S0140525X01003922).
- Oberauer, K. (2002), “Access to information in working memory: Exploring the
  focus of attention,”
  [doi:10.1037/0278-7393.28.3.411](https://doi.org/10.1037/0278-7393.28.3.411).
- Dehaene, S., Kerszberg, M., and Changeux, J. P. (1998), “A neuronal model of
  a global workspace in effortful cognitive tasks,”
  [doi:10.1073/pnas.95.24.14529](https://doi.org/10.1073/pnas.95.24.14529).
- Anderson, M. C., Bjork, R. A., and Bjork, E. L. (1994), “Remembering can
  cause forgetting,”
  [doi:10.1037/0278-7393.20.5.1063](https://doi.org/10.1037/0278-7393.20.5.1063).
- Anderson, M. C., and Green, C. (2001), “Suppressing unwanted memories by
  executive control,”
  [doi:10.1038/35066572](https://doi.org/10.1038/35066572).
- McGaugh, J. L. (2004), “The amygdala modulates the consolidation of memories
  of emotionally arousing experiences,”
  [doi:10.1146/annurev.neuro.27.070203.144157](https://doi.org/10.1146/annurev.neuro.27.070203.144157).
- Mather, M., and Sutherland, M. R. (2011), “Arousal-biased competition in
  perception and memory,”
  [doi:10.1177/1745691611400234](https://doi.org/10.1177/1745691611400234).
- Ratcliff, R. (1978), “A theory of memory retrieval,”
  [doi:10.1037/0033-295X.85.2.59](https://doi.org/10.1037/0033-295X.85.2.59).
- Cisek, P. (2007), “Cortical mechanisms of action selection: The affordance
  competition hypothesis,”
  [doi:10.1098/rstb.2007.2054](https://doi.org/10.1098/rstb.2007.2054).
- Cisek, P., Puskas, G. A., and El-Murr, S. (2009), “Decisions in changing
  conditions: The urgency-gating model,”
  [doi:10.1523/JNEUROSCI.1844-09.2009](https://doi.org/10.1523/JNEUROSCI.1844-09.2009).
- Shenhav, A., Botvinick, M. M., and Cohen, J. D. (2013), “The expected value
  of control,”
  [doi:10.1016/j.neuron.2013.07.007](https://doi.org/10.1016/j.neuron.2013.07.007).
- McClelland, J. L., McNaughton, B. L., and O'Reilly, R. C. (1995), “Why there
  are complementary learning systems in the hippocampus and neocortex,”
  [doi:10.1037/0033-295X.102.3.419](https://doi.org/10.1037/0033-295X.102.3.419).
- Nader, K., Schafe, G. E., and LeDoux, J. E. (2000), “Fear memories require
  protein synthesis in the amygdala for reconsolidation after retrieval,”
  [doi:10.1038/35021052](https://doi.org/10.1038/35021052).
- Dudai, Y., Karni, A., and Born, J. (2015), “The consolidation and
  transformation of memory,”
  [doi:10.1016/j.neuron.2015.09.004](https://doi.org/10.1016/j.neuron.2015.09.004).
- Clark, H. H., and Brennan, S. E. (1991), “Grounding in communication,”
  [doi:10.1037/10096-006](https://doi.org/10.1037/10096-006).
- Brennan, S. E., and Clark, H. H. (1996), “Conceptual pacts and lexical choice
  in conversation,”
  [doi:10.1037/0278-7393.22.6.1482](https://doi.org/10.1037/0278-7393.22.6.1482).
- Keysar, B., Barr, D. J., Balin, J. A., and Brauner, J. S. (2000), “Taking
  perspective in conversation,”
  [doi:10.1111/1467-9280.00211](https://doi.org/10.1111/1467-9280.00211).
- Brown-Schmidt, S. (2009), “Partner-specific interpretation of maintained
  referential precedents during interactive dialog,”
  [doi:10.1016/j.jml.2009.04.003](https://doi.org/10.1016/j.jml.2009.04.003).
- Alderson-Day, B., and Fernyhough, C. (2015), “Inner speech: Development,
  cognitive functions, phenomenology, and neurobiology,”
  [doi:10.1037/bul0000021](https://doi.org/10.1037/bul0000021).
- Heavey, C. L., and Hurlburt, R. T. (2008), “The phenomena of inner
  experience,”
  [doi:10.1016/j.concog.2007.12.006](https://doi.org/10.1016/j.concog.2007.12.006).
- Nisbett, R. E., and Wilson, T. D. (1977), “Telling more than we can know,”
  [doi:10.1037/0033-295X.84.3.231](https://doi.org/10.1037/0033-295X.84.3.231).
- Ericsson, K. A., and Simon, H. A. (1980), “Verbal reports as data,”
  [doi:10.1037/0033-295X.87.3.215](https://doi.org/10.1037/0033-295X.87.3.215).
