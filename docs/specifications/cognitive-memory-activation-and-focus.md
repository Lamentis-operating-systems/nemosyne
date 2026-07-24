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
- `I` is the authenticated invocation context;
- `t_auth` is the trusted authorization time;
- `M_A^(r,p,t_auth,I)` is the immutable authorized and disclosable view of
  memory revision `r` under policy revision `p`;
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
| \(F_Q,X_Q,G_Q,Z_Q\) | Query facets, exact values, active goals, and typed absence/quality state |
| \(\widehat B_{\mathrm{in}},B_Q,J_{QA}\) | Compiler-owned ingress content binding, its query projection, and its validated join with shared-set lineage |
| \(c_{i,f},C_i\) | One calibrated direct cue and the complete direct-cue vector |
| \(k_{\mathrm{hist}},n_i^{\mathrm{hist}},t_{i,k_{\mathrm{hist}}},\delta_{i,k_{\mathrm{hist}}}^{\mathrm{hist}},\beta_{\mathrm{decay}},b_i^{\mathrm{raw}},b_i\) | History-event index and count, event time, dimensionless age, decay exponent, raw base availability, and calibrated availability |
| \(t_i,x_i,g_i,u_i,s_i\) | Temporal, spatial, goal, procedural, and social relevance channels |
| \(k_{\mathrm{haz}},n_Q^{\mathrm{haz}},z_{k_{\mathrm{haz}}}^{\mathrm{haz}},h_{i,k_{\mathrm{haz}}},h_i\) | Hazard index and count, one represented hazard, per-hazard relevance, and aggregate hazard relevance |
| \(a^{(k)},W,\rho,K_{\mathrm{spread}}\) | Spreading-activation state, relation matrix, restart factor, and fixed iteration count |
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

Compiler ingress first validates the complete public compile request against
the authenticated, pinned configuration `K` and retains immutable exact input
buffers. The request schema does not accept `request_id`, `situation_id`,
`configuration_id`, an identity digest, or an identity-algorithm override.
Unknown fields attempting to supply any of those values are rejected rather
than ignored. The compiler-owned `SIT-01` boundary then constructs exactly one
sealed `IngressRequestBinding`:

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
compiler-configuration manifest selected before request identity derivation;
that manifest enumerates the complete artifact set. The installation trust
root authenticates the manifest, and the compile call pins it for its full
lifetime. A caller may request one installed configuration through the public
untrusted claims contract, but cannot supply, substitute, or override the
resolved authoritative `configuration_id`; the compiler authenticates,
authorizes, and resolves that request before ingress derives the identity.
Missing authentication, a manifest digest mismatch, an unauthorized or absent
requested configuration, or a configuration change after pinning fails before
`Q`, retrieval, or authorization-view construction.
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

The sealed binding is forked from compiler ingress to two independent
consumers. Situation encoding copies its three-field projection

\[
B_Q=(request\_id,situation\_id,configuration\_id)
\]

into `X_Q`. Shared-set construction independently copies the same three fields
from \(\widehat B_{\mathrm{in}}\) into the corresponding projection of
\(\Lambda_A\). It must not copy them from `Q`, derive them from lossy query
facets, reconstruct them from ambient state, or accept them from the caller.
This common compiler-owned source makes the later join meaningful: the two
branches can detect corruption, reuse, or cross-request mixing without giving
the focus branch an authorization dependency.

Situation encoding produces:

\[
Q=\operatorname{encode}(P,S,\Xi;K)
\]

with:

\[
Q=(F_Q,X_Q,G_Q,Z_Q)
\]

where:

- `F_Q` contains typed prompt, situation, temporal, spatial, task, social,
  procedural, and risk-related facets;
- `X_Q` contains exact request-local values, typed prompt, situation, and
  metadata source bindings, and exactly one validated canonical query binding
  \(B_Q=(request\_id,situation\_id,configuration\_id)\);
- `G_Q` contains explicitly active goal states and their declared priorities;
  and
- `Z_Q` contains absence, uncertainty, language, and observation-quality
  metadata.

`Q` retains `t_context` only as caller-supplied, untrusted request evidence.
The same qualification applies to declared location and metadata. Trusted
authorization time `t_auth` is neither an encoder input nor a facet, exact
binding, identifier, or hidden dependency of `Q`. Authorization, current
normative validity, expiry, and supersession use `t_auth` on their separately
owned memory path.

Situation encoding does not retrieve memory, assign instruction authority, or
modify `P`. It does not decide that a facet is a renderable proposition. That
decision belongs to the focus branch's validated request-proposition
construction below. `Q` contains no `policy_revision_id`,
`authorization_view_id`, principal, authorization decision, disclosure view,
or memory-eligibility result. Those values are not situation facts and cannot
be supplied to the focus branch through a request-source binding.

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
\{m_i\in M_A^{(r,p,t_{\mathrm{auth}},I)}
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
\operatorname{usageCompatible}(m_i,Q)=
\operatorname{currentUsable}(m_i,t_{\mathrm{auth}})
\lor
\left(
\operatorname{historicalScopeRequested}(Q)
\land
\operatorname{historicallyApplicable}(m_i,Q)
\right)
\]

The request-local candidate universe is:

\[
\mathcal M_Q=
\{m_i\in\mathcal M_E\mid
\operatorname{usageCompatible}(m_i,Q)\}
\]

`currentUsable` applies the record's validity and supersession contract.
Historical admission is explicit, carries a `historical_only` qualification,
and cannot support a current normative instruction. Candidate generation and
graph expansion operate on \(\mathcal M_Q\); a stale record cannot enter an
ordinary current request merely because it is semantically similar.

### Direct cue activation

Candidate generation derives a bounded direct candidate set from `Q` and
\(\mathcal M_Q\). A direct cue score is a collection of independently inspectable
facet matches rather than one undifferentiated embedding similarity.

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

The direct cue vector for candidate `i` is:

\[
C_i=(c_{i,f})_{f\in F_{\mathrm{comparable}}}
\]

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
b_i^{raw}=
\ln
\left(
\sum_{k_{\mathrm{hist}}=1}^{n_i^{\mathrm{hist}}}
\left(\delta_{i,k_{\mathrm{hist}}}^{\mathrm{hist}}\right)^{-\beta_{\mathrm{decay}}}
\right)
\]

The logarithm receives a positive dimensionless quantity. With no qualifying
history event, the base-availability channel is disabled rather than assigned
an invented zero. A versioned calibrator maps the unbounded value into:

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
\operatorname{roleCompatibility}(Q,m_i);
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
\operatorname{conditionMatch}(Q,m_i),
\operatorname{roleMatch}(Q,m_i),
\operatorname{actionClassMatch}(Q,m_i),
\operatorname{outcomeMatch}(Q,m_i);
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

For a future deadline with remaining duration `Delta_i >= 0`, a proposed
urgency feature is:

\[
u_i^{deadline}=\exp(-\Delta_i/\tau)
\]

for pinned `tau > 0` in declared units. This feature approaches one as the
deadline approaches. It never overrides a hard constraint or authorization
rule. Deadline urgency, immediate response window, and severity if ignored are
separate inputs.

### Social and common-ground relevance

Social relevance is partner- and interaction-specific. A record may distinguish
private knowledge, attributed belief, assertion, acknowledgment, and
jointly-established common ground.

For the authenticated principal or interaction partner `p`:

\[
s_i=
\operatorname{cal}_{social}
\left(
\operatorname{participantMatch}(m_i,p),
\operatorname{interactionMatch}(m_i,Q),
\operatorname{groundingState}(m_i,p),
\operatorname{perspectiveCompatibility}(m_i,Q);
K
\right)
\in[0,1]
\]

One assertion does not prove mutual knowledge. A partner match does not grant
authorization, establish truth, or elevate instruction authority.

### Bounded spreading activation

Direct candidates may activate usage-compatible related records through typed,
versioned relations. Let `n` be the number of records in \(\mathcal M_Q\)
admitted to a bounded request-local graph. Let:

\[
a^{(0)}\in[0,1]^n,\qquad
\lVert a^{(0)}\rVert_1\leq1
\]

be the normalized direct seed activation. Let `W` be a nonnegative
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

The V1 experiment executes exactly `K_spread` configured iterations and admits
at most the configured node, edge, and hop budgets. Because `W` is
row-substochastic and `a^(0)` has unit-or-lower `L1` mass, every iteration
remains finite, nonnegative, and bounded by total activation mass one.

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

Signal derivation maps the query, direct matches, bounded graph activation, and
eligible candidates into the existing activation-kernel contract.

For evidence channel `c` and candidate `i`:

\[
e_{i,c}\in[0,1]
\]

Candidate-independent parameter weight `w_c` and situation-dependent gate
`g_c(Q)` satisfy:

\[
w_c,g_c(Q)\in[0,1]
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

Situation encoding owns typed facets and exact request-source bindings in
`Q`. It does not decide proposition identity, renderability, or authority.
Inside the existing `focusCandidates(Q, A; K)` stage, the focus branch alone
owns the logical sub-boundary:

\[
\operatorname{deriveRequestPropositions}(Q,\Lambda_A;K)
\rightarrow
\mathcal R_Q
\;\mid\;
\operatorname{RequestPropositionError}.
\]

This is a logical interface, not a committed public Rust API.
\(\Lambda_A\) is copied from the validated
`EligibleActivatedMemorySet`, including when its activated-record collection
is empty. The interface receives no principal, policy object, authorization
view, authorization service, or second authorization result.

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
invalid. Before equality, both branch projections must validate against the
one sealed \(\widehat B_{\mathrm{in}}\) retained by the compiler call. The
`Q` projection and the \(\Lambda_A\) projection were copied independently
from that ingress value; neither validates the other merely by echoing its
fields.

A missing, duplicate, malformed, non-canonical, recomputation-inconsistent, or
configuration-inconsistent field in \(B_Q\) is `InvalidQueryBinding`.
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
reconstructed from `Q`, \(\widehat B_{\mathrm{in}}\), or ambient process
state. Conversely, the request, situation, and configuration identities in
\(\Lambda_A\) originate only from compiler ingress, not from `Q` or the
authorization view.

\(\mathcal R_Q\) is a finite canonical `RequestPropositionSet`:

```text
RequestPropositionSet
├── schema_id
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
    ├── request_proposition_id
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
\operatorname{supportFit}(Q,r_k^Q);K
\right)
\in[0,1].
\]

The metric, calibrator, numerical policy, input facets, missingness behavior,
and accumulation order are content-identified configuration artifacts. The
score measures request-local focus relevance only. It is not truth,
confidence, instruction strength, probability, safety, or permission.
Non-finite output or a value outside the declared domain is an explicit
representation error.

`RequestPropositionId` is content-derived from the source-receipt projection,
source-kind tag, canonical locator, proposition schema and meaning identity,
derivation identity, exact-binding identities, qualifiers, authority ceiling,
and allowed-use ceiling. Display prose, insertion order, and \(q_k\) do not
participate in identity.

`RequestPropositionSourceOrderKey` is the closed lexicographic tuple:

1. source-kind rank: `AuthenticatedPrompt`, `SituationStatement`, then
   `RequestMetadata`;
2. the source-kind locator defined above;
3. ascending proposition-schema and meaning identities;
4. ascending derivation identity; and
5. ascending `RequestPropositionId`.

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
- every required exact slot agrees byte-for-byte or is explicitly represented
  as a compatible set;
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

Every consolidated proposition receives one content-derived identity over its
canonical meaning, complete tagged support identities, exact bindings,
qualifiers, authority and allowed-use ceilings, and derivation/configuration
identity. A request-only proposition does not borrow a persistent provenance
root or memory identity. A mixed proposition preserves request attribution and
memory provenance separately.

Consolidation creates request-local computational state only. Persistent
episodic-to-semantic consolidation, reconsolidation, deletion, correction, and
learning belong to a separately authorized memory-management path.

### Focus-candidate construction

The focus branch receives `Q` and the shared eligible activated-memory set
before renderer-budget pruning. It first constructs \(\mathcal R_Q\) under the
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
exact-sidecar bindings, stable identifier, and deterministic cost estimate.
Control-only exclusions retain enough evidence for validation but are never
renderer input.

The output is:

```text
FocusCandidateSet
├── schema_id
├── candidates[]
│   ├── proposition_id
│   ├── focus_roles[]
│   ├── numerical_meaning
│   ├── activation
│   ├── support[]
│   │   ├── RequestPropositionSource
│   │   └── ActivatedMemorySource
│   ├── provenance_roots[]
│   ├── qualifications[]
│   ├── exact_bindings[]
│   ├── estimated_cost
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

The `RequestPropositionSet` receipt must equal its five-field projection from
this same \(\Lambda_A\), and its three-field `query_binding` must equal
\(\pi_Q(\Lambda_A)\). Request-only focus therefore retains the same memory,
policy, authorization, retrieval, activation, and configuration lineage as
the empty activated-memory set used by the call. It does not replace
\(\Lambda_A\) with a shorter receipt, import an authorization-view identity
into `Q`, or perform authorization again.

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
4. ascending `proposition_id`.

The key is derived, never caller-authored. Its activation component uses the
exact bounded activation value already carried by the candidate under the
pinned numerical policy; it introduces no new score. An unknown role,
duplicate role, non-total role table, mismatch between a candidate and its
derived key, or duplicate complete candidate key is a structural error.
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
- \(c_{\mathrm{facet}}\) be the sum of declared per-candidate direct,
  temporal, spatial, procedural, and social metric/calibrator costs;
- \(h_\Sigma\) be the total qualifying history-event count traversed for the
  \(n_r\) candidates;
- \(n_G\) and \(n_H\) be the active-goal and represented-hazard counts, with
  worst-case per-pair costs \(c_G\) and \(c_H\);
- \(n_g,e_g,k_g\) be the bounded spreading graph's node count, edge count, and
  fixed iteration count;
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
O\!\left(n_M(c_{\mathrm{policy}}+c_{\mathrm{facet}}+\log n_r)\right)
\]
time and \(O(n_r)\) selection workspace, where
\(c_{\mathrm{policy}}\) is the pinned worst-case per-record authorization,
validity, and usage-gate cost. A selected approximate index may replace this
oracle only when its own build/query/storage bounds, immutable-revision
binding, and recall contract are declared and it passes the proof program's
false-negative gates. Authorization still precedes retrieval competition.

For the bounded candidates, reference facet, availability, goal, procedural,
risk, and social derivation costs
\[
O\!\left(
n_r c_{\mathrm{facet}}+h_\Sigma+n_r(n_Gc_G+n_Hc_H)+n_r(c_e+c_j)
\right)
\]
before graph propagation. A selected metric or calibrator with a greater cost
replaces its corresponding term explicitly; no encoder, nearest-neighbour, or
calibrator cost is hidden inside an assumed constant. Bounded spreading costs
\[
O\!\left(k_g(n_g+e_g)\right)
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
  callers cannot supply them, and neither `Q` nor an authorization view is
  allowed to mint or overwrite them.
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
complete \(\Lambda_A\). The resulting `FocusCandidateSet` contains exactly one
request-only candidate with:

- a three-field \(B_Q\) exactly equal to \(\pi_Q(\Lambda_A)\) under the
  canonical join;
- the roles `CurrentSituation` and `RelevantBackground` in canonical role
  order;
- activation equal to that source's \(q_k\);
- one tagged request source and no activated-memory source;
- no persistent provenance root or dependency group;
- the `CallerReported` qualification and descriptive-data authority ceiling;
- the exact complete \(\Lambda_A\) as its source receipt; and
- a `FocusCandidateOrderKey` derived normally from role, activation, and
  proposition identity.

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

- deterministic \(Q=\operatorname{encode}(P,S,\Xi;K)\): identical prompt,
  ordered zero-to-three situation statements, contextual-time, location, and
  metadata presence states, and pinned encoder/configuration inputs produce
  identical facets, \(B_Q\), locators, and source-buffer content identities;
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
  change `Q`, while attempting to place any such field in request evidence is
  rejected before encoding;
- eligibility noninterference: changing only ineligible records cannot change
  any content-bearing result;
- facet-type safety: incompatible spaces are rejected before comparison;
- exact-sidecar preservation from memory revision to focus-candidate set;
- bounded and finite calibrated channel outputs;
- bounded spreading mass for every configured iteration;
- graph authorization closure;
- deterministic canonical ordering;
- request-proposition source-locator validity, receipt equality, authority
  ceilings, order-key correctness, and permutation invariance;
- exact \(B_Q=\pi_Q(\Lambda_A)\) joining over independently copied ingress
  projections, full typed-content-identity comparison, absence of policy and
  authorization-view fields from `Q`, and absence of a direct authorization
  dependency or repeated authorization call in focus construction;
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
- a cross-request swap that pairs `Q` from request A with \(\Lambda_A\) from
  request B, including requests with identical prompts but different
  situation evidence;
- constant-ID, prior-request-ID reuse, caller-supplied-ID, outer-digest
  collision, and full collision-witness injection fixtures; every detected
  recomputation mismatch or same-identity/different-canonical-bytes witness
  must fail closed, while the true-digest-collision case remains an explicit
  cryptographic assumption rather than a tested uniqueness proof;
- authenticated configuration pinning, manifest mismatch, post-pin
  substitution, and branch-configuration mismatch fixtures;
- parameter sensitivity and monotonicity where the mathematics requires it;
- perturbation of time, location, participant, goal, risk, and procedure
  facets independently;
- false-causal relation paths;
- cross-language and rare exact-value cases;
- prompt injection inside authorized memory content;
- empty, insufficient, corrupt, and incompatible states; and
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
