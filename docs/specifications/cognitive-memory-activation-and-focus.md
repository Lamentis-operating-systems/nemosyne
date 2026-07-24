# Cognitive memory activation and focus

Status: Proposed

## Purpose

This specification defines the proposed cognitive-memory hypothesis for
Nemosyne V1. It describes how an already authorized, immutable memory revision
and one compile request can be represented numerically, activated by
situation-dependent cues, consolidated into request-local propositions, and
reduced to a bounded focus plan.

Decision 0012 accepts numerical memory, situation-conditioned activation,
request-local consolidation, and bounded focus planning as the intended V1
direction. This specification makes that path testable while its particular
facet schema, derivation functions, coefficients, and planning mechanism remain
proposed pending implementation and evaluation. It refines the
signal-derivation and attention-planning boundaries in the V1 reference
architecture. It does not select an embedding model, vector dimension,
database, index, renderer model, runtime, or persistent consolidation policy.

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
- `X` contains request metadata and the declared contextual time `t_context`;
- `U` is the authenticated invocation context;
- `t_auth` is the trusted authorization time;
- `M_A^(r,p,t_auth,U)` is the immutable authorized and disclosable view of
  memory revision `r` under policy revision `p`;
- `K` is the pinned compiler configuration and artifact set; and
- `B > 0` is the finite attention budget resolved by policy and configuration.

The output of this specification is a structured focus plan `L`. Rendering,
validation, and exact serialization remain separate stages. The public product
output remains the one compiled text defined by the V1 product contract.

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
- provenance roots and source-independence groups;
- confidence and uncertainty values under a declared calibration contract; and
- encoder, transform, schema, and index identities.

Bytes are data, not instructions. A renderer may reproduce exact bytes only
through an explicit focus-plan binding whose disclosure and authority checks
have already passed.

### Numerical memory unit

The compile-time representation of one immutable memory record version `i` is:

\[
m_i=
(
\operatorname{id}_i,
F_i,
X_i,
H_i,
R_i,
\Pi_i
)
\]

where:

- `F_i` is a finite collection of typed numerical facets;
- `X_i` is the collection of exact sidecars;
- `H_i` is a versioned history of relevant real observations, uses, and
  outcomes;
- `R_i` is a finite set of typed numerical relations to other immutable record
  versions; and
- `Pi_i` is provenance, authority, validity, uncertainty, and policy metadata.

The authoritative memory plane may retain exact source material or canonical
propositions as defined by a future memory-management contract. The
compile-time relevance path operates on typed numerical representations and
lossless sidecars derived from that authoritative plane. Numerical
representations remain rebuildable computational state, not an independent
source of truth.

### Numerical query state

Situation encoding produces:

\[
Q=\operatorname{encode}(P,S,X,t_{\mathrm{auth}};K)
\]

with:

\[
Q=(F_Q,X_Q,G_Q,Z_Q)
\]

where:

- `F_Q` contains typed prompt, situation, temporal, spatial, task, social,
  procedural, and risk-related facets;
- `X_Q` contains exact request-local values and source bindings;
- `G_Q` contains explicitly active goal states and their declared priorities;
  and
- `Z_Q` contains absence, uncertainty, language, and observation-quality
  metadata.

`Q` retains distinct facets for `t_context` and `t_auth`. Contextual time may
affect relevance. It cannot change authorization, current normative validity,
expiry, or supersession, which use `t_auth`.

Situation encoding does not retrieve memory, assign instruction authority, or
modify `P`.

### Eligible memory view

Hard policy and integrity predicates define whether a record may enter the
request-local candidate graph:

\[
\operatorname{eligible}(m_i)=
\operatorname{readable}(m_i,U)
\land
\operatorname{disclosable}(m_i,U,t_{\mathrm{auth}})
\land
\operatorname{revisionCompatible}(m_i,r,K)
\land
\operatorname{representationValid}(m_i,K)
\land
\operatorname{notDeleted}(m_i,r)
\]

The eligible set is:

\[
\mathcal{M}_{E}=
\{m_i\in M_A^{(r,p,t_{\mathrm{auth}},U)}
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
\mathcal{M}_{Q}=
\{m_i\in\mathcal{M}_{E}\mid
\operatorname{usageCompatible}(m_i,Q)\}
\]

`currentUsable` applies the record's validity and supersession contract.
Historical admission is explicit, carries a `historical_only` qualification,
and cannot support a current normative instruction. Candidate generation and
graph expansion operate on `mathcal{M}_Q`; a stale record cannot enter an
ordinary current request merely because it is semantically similar.

### Direct cue activation

Candidate generation derives a bounded direct candidate set from `Q` and
`mathcal{M}_Q`. A direct cue score is a collection of independently inspectable
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
in the same units as timestamps and let `delta_min > 0` be a dimensionless
minimum age. For every configured history event `t_(i,h) <= t_auth`, define:

\[
\delta_{i,h}=
\max
\left(
\frac{t_{\mathrm{auth}}-t_{i,h}}{u_{\mathrm{time}}},
\delta_{\min}
\right)
\]

For `n_i > 0` and pinned `d > 0`, define:

\[
b_i^{raw}=
\ln
\left(
\sum_{h=1}^{n_i}\delta_{i,h}^{-d}
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

For each currently represented hazard `h`, the separately derived
`severity(h)`, `causalRelevance(m_i,h)`, and `confidence(h)` values are finite
and lie in `[0,1]`. Define:

\[
h_{i,h}=
\operatorname{severity}(h)
\cdot
\operatorname{causalRelevance}(m_i,h)
\cdot
\operatorname{confidence}(h)
\]

and:

\[
h_i=\max_h h_{i,h}\in[0,1]
\]

An empty hazard set disables the risk channel. Arousal, emotional language, or
negative sentiment does not by itself establish severity or causal relevance.
Risk relevance and truth confidence remain separate values.

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
versioned relations. Let `n` be the number of records in `mathcal{M}_Q`
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

The existing kernel computes:

\[
E_i=
\frac{\sum_c w_cg_c(Q)e_{i,c}}
{\sum_c w_cg_c(Q)}
\]

\[
R_i=
\prod_j(1-\lambda_jp_{i,j})
\]

\[
A_i=E_iR_i
\]

The evidence denominator must be positive. Kernel input validation, canonical
channel order, floating-point behavior, explanation contract, and tie-breaking
remain governed by the situation-conditioned activation specification.

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

### Request-local proposition consolidation

Ranking selects memory record versions, but rendering requires coherent
propositions. Request-local proposition consolidation groups compatible
activated facets without modifying persistent memory.

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

To prevent repeated paraphrases or duplicated imports from creating artificial
confidence, memory support records are grouped by provenance root. For a
request-supported proposition, a pinned request-support derivation produces:

\[
q_{\phi}=
\operatorname{cal}_{request}
\left(
\operatorname{supportFit}(Q,\phi);K
\right)
\in[0,1]
\]

It exists only when `support_phi^Q` is nonempty and retains the exact request
source binding. The proposition's activation score is conservatively:

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
Independent corroboration count, source diversity, and conflicts are recorded
as separate planning features. They do not silently increase the activation
score. A later accepted decision may replace this conservative aggregation
only after source-dependence and calibration evidence exists.

Consolidation creates request-local computational state only. Persistent
episodic-to-semantic consolidation, reconsolidation, deletion, correction, and
learning belong to a separately authorized memory-management path.

### Budgeted focus-plan selection

The planner receives request-supported propositions and request-local memory
propositions. It first partitions the applicable, authorized propositions into
an inclusion candidate set \(\Phi_{\mathrm{include}}\) and a control-only
exclusion set \(\Phi_{\mathrm{exclude}}\). A proposition enters
\(\Phi_{\mathrm{exclude}}\) only through an explicit request, policy,
supersession, or conflict-resolution rule in the pinned configuration. A
record removed by authorization or usage gating never enters either set.

Each proposition in \(\Phi_{\mathrm{include}}\) is classified as one of:

- current focus;
- dominant goal;
- dominant constraint;
- relevant background;
- secondary influence;
- conflict; or
- uncertainty.

Let
\(\Phi_{\mathrm{mandatory}}\subseteq\Phi_{\mathrm{include}}\) be the
propositions whose faithful inclusion is mandatory. Each inclusion candidate
has:

- activation `score_phi` in `[0,1]`;
- deterministic estimated rendering cost `cost_phi > 0`;
- coverage `cov_(phi,d)` in `[0,1]` for focus dimension `d`;
- pairwise redundancy `red_(phi,psi)` in `[0,1]`;
- mandatory qualifications and exact bindings; and
- a stable identifier for canonical tie-breaking.

All scores, costs, coverage values, and redundancy values are finite.
Redundancy is symmetric with a zero diagonal. Dimension weights satisfy:

\[
\omega_d\geq0,
\qquad
\sum_{d\in D}\omega_d=1
\]

The finite planning coefficients satisfy
`kappa >= 0`, `eta >= 0`, `alpha >= 0`, and `zeta >= 0`, with at least one of
`alpha` or `zeta` positive. They are compiler parameters, not inferred from the
current candidate set.

For selected set
\(U\subseteq\Phi_{\mathrm{include}}\), define:

\[
\operatorname{coverage}(U)
=
\sum_{d\in D}
\omega_d
\min
\left(
1,
\sum_{\phi\in U}\operatorname{cov}_{\phi,d}
\right)
\]

\[
\operatorname{redundancy}(U)
=
\sum_{\substack{\phi,\psi\in U\\\operatorname{id}_\phi<
\operatorname{id}_\psi}}
\operatorname{red}_{\phi,\psi}
\]

The proposed selection objective is:

\[
U^\*=
\arg\max_{U\in\mathcal{F}}
\left[
\sum_{\phi\in U}\operatorname{score}_{\phi}
+
\kappa\operatorname{coverage}(U)
-
\eta\operatorname{redundancy}(U)
-
\alpha|U|
-
\zeta
\frac{\sum_{\phi\in U}\operatorname{cost}_{\phi}}{B}
\right]
\]

subject to:

\[
\Phi_{\mathrm{mandatory}}\subseteq U,
\qquad
\sum_{\phi\in U}\operatorname{cost}_{\phi}\leq B
\]

`mathcal{F}` contains the empty set when `Phi_mandatory` is empty and otherwise
only sets that include every mandatory proposition and preserve mandatory
qualification, authority, exact-value, and conflict constraints. A proposition
that requires a qualifier is infeasible without that qualifier. If one side of
an unresolved material conflict is selected, the plan must include the conflict
status and every side required for a faithful representation.

Exclusions are not members of \(\mathcal F\), receive no positive utility, and
cannot be removed by the optimization. Every member of
\(\Phi_{\mathrm{exclude}}\) becomes a canonical control record in the plan.
These records do not consume the attention output budget `B` because they can
never be rendered. They count against a separate schema-bound maximum
`N_exclude`. Exceeding that maximum returns `AttentionPlanCapacityExceeded`
rather than omitting an exclusion. Exclusion records are available only to
validation and never enter the generative renderer prefix.

Planning uses the pinned finite dtype, canonical accumulation order, and
rounding contract in `K`. Exact optimization, deterministic greedy selection,
and a tested approximation remain open implementation choices. Equal
objectives resolve first by lower total cost, then fewer propositions, then
canonical proposition-identifier order. When `Phi_mandatory` is empty and no
nonempty set has objective strictly above the empty set's value of zero,
\(U^\*\) is empty. The item and normalized-cost penalties prevent a positive but
negligible score from automatically filling otherwise unused budget.

If mandatory faithful focus cannot fit within `B`, planning returns
`InsufficientAttentionBudget`. It must not silently truncate, weaken a
qualification, or substitute empty attention. Empty attention is valid only
when no additional focus is justified.

### Structured focus plan

The selected plan is one immutable authoritative envelope:

\[
L=
\left(
\kappa_L,
(u_1,\ldots,u_n),
\mathcal X_L,
\mathcal{R}_L,
V_L,
\Phi_{\mathrm{mandatory}},
\Phi_{\mathrm{optional}},
\ell_L,
B_L
\right)
\]

where `kappa_L` identifies the complete plan schema, `u_i` are canonically
ordered plan items corresponding exactly to \(U^\*\), `X_L` is the complete
canonically ordered control view of \(\Phi_{\mathrm{exclude}}\), `R_L`
contains typed dominant, secondary, conflict, and qualification relations,
`V_L` is the output-authorized exact-value sidecar, `Phi_mandatory` and
`Phi_optional` partition the selected plan items, `ell_L` is the resolved
output language, and `B_L` is the post-substitution rendering budget. A
renderer has no independent language, budget, or inclusion source of truth and
rejects a configuration that is incompatible with `kappa_L`.

Each entry in `X_L` retains the numerical proposition meaning, scope,
qualification, control-rule identity, and any exact surfaces required for
deterministic or semantic exclusion checks. It is visible only to validators.
It is not a plan item, an exact substitution source, or a model-prefix input.

Before the planning objective is evaluated, the pinned deterministic formatter
materializes every authorized exact surface for `ell_L` and its contribution to
the declared cost function. `V_L` retains those approved bytes, lengths,
permitted bindings, and minimum and maximum occurrence counts for substitution
and post-render verification, but the language model never receives the bytes.
Estimated proposition cost reserves the schema-declared maximum permitted
exact-value expansion rather than treating a one-token slot as the final cost.

Each `u_i` carries the complete lossless renderer view of one selected
proposition:

- the ordered focus categories;
- typed numerical proposition meanings and role-bound facets;
- source and provenance bindings;
- authority ceilings;
- confidence, validity, and uncertainty qualifications;
- exact-value bindings;
- dominant and secondary priority relations;
- unresolved conflicts;
- required inclusions and permitted omissions;
- exact-slot identities and binding roles; and
- mandatory or optional disposition.

The plan contains no answer to `P`, no executable tool call, and no free-form
draft reasoning. Role identity is retained so a renderer cannot mistake
background for a constraint or uncertainty for a fact.

### Focus narrative

A renderer may lexicalize `L` into a concise focus narrative containing:

- what the current situation makes salient;
- which goals or constraints dominate;
- which background materially changes interpretation;
- which secondary pressures remain relevant;
- which conflicts or uncertainties must remain visible; and
- how the downstream model should orient its response without answering `P`.

The focus narrative is an interface artifact. It is not a transcript of
internal human cognition, model reasoning, hidden reasoning, or chain of
thought. It must not claim emotions, beliefs, motives, or subjective experience
that are not represented by authorized evidence in `L`.

### Action-selection boundary

This specification ends at focus-plan construction. Nemosyne V1 does not:

- choose or execute a downstream action;
- authorize a tool;
- override the downstream model's safety or instruction hierarchy;
- infer that the highest-activation memory prescribes the correct action; or
- guarantee that a focus narrative improves the downstream response.

Action competition, evidence accumulation, and urgency-gating research motivate
separating possible actions and time pressure from general semantic similarity.
They do not place action selection inside the V1 compiler.

## Preconditions

A conforming experiment requires:

- the accepted local, read-only V1 product boundary;
- a pinned immutable request, compiler configuration, and authorized memory
  revision;
- valid trusted `t_auth` distinct from caller-controlled contextual time;
- typed numerical schemas with explicit dimensions, metrics, normalization,
  absence semantics, and artifact identities;
- a pinned numerical execution contract covering dtype, backend, elementary
  functions, reduction order, rounding, and tie semantics;
- lossless sidecars for every exact value required downstream;
- an authorization and disclosure view established before candidate
  generation;
- a bounded candidate, graph, relation, and planning budget;
- a channel schema with versioned parameters and no implicit default weights;
- a positive effective evidence denominator for every activation call;
- stable provenance roots and record-version identities;
- declared language and focus-budget limits; and
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
- Required exact values flow through explicit sidecar bindings.
- Planning respects the finite attention budget without truncating required
  meaning.
- The focus plan contains neither an answer nor an executable action.
- Activation values are not presented as truth, probability, safety,
  instruction authority, or biological measurements.
- The focus narrative is not presented as human inner speech or chain of
  thought.
- For fixed input, revision, configuration, canonical order, declared random
  tape, and pinned dtype, backend, elementary functions, reduction order,
  rounding, and tie semantics, every numerical intermediate and focus plan is
  reproducible within the declared bitwise or tolerance contract.

## Edge cases

- An empty authorized memory revision may still produce request-supported focus.
- No justified additional focus produces a valid empty plan.
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
- A material qualification that does not fit the budget causes an explicit
  budget failure.
- Equal activation or planning objectives use stable numeric identifiers rather
  than input order.
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
6. selecting a budgeted focus plan; and
7. lexicalizing a fixed plan into a focus narrative.

Training only an end-to-end vector-to-text target would obscure which boundary
failed and could teach the renderer to retrieve, rerank, invent, or answer the
prompt. Each boundary requires independent labels and evaluation before joint
optimization is considered.

### Example schema

A complete corpus and evaluation-harness record should retain at least:

- one stable scenario-family identifier;
- the original prompt and declared situation;
- typed numerical query facets and exact request sidecars;
- eligible and excluded memory-version identifiers;
- candidate signals, gates, parameters, activations, and derivation receipts;
- relation paths used by bounded spreading activation;
- request-local proposition support and provenance groups;
- dominant, secondary, conflicting, uncertain, and mandatory plan items plus
  control-only exclusion records;
- exact-value bindings;
- `must_include` and `must_exclude` proposition identifiers;
- output language and maximum rendering budget;
- one evidence-bound target focus narrative; and
- downstream outcome labels kept separate from renderer faithfulness labels.

Only the generator view of the canonical plan envelope—selected inclusion
items, their relations, language, budget, and exact-slot identities—is renderer
model input. Control-only exclusions, exact surface bytes, the original prompt,
situation, raw memory, eligible and excluded candidates, and derivation
receipts remain outside the renderer as validator input, corpus provenance,
leakage labels, or evaluation context.

All generated variants from one semantic scenario family belong to the same
train, validation, or test partition.

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
- independent corroboration without hidden source duplication;
- unresolved contradiction and supersession;
- empty memory and request-only focus;
- no justified attention;
- exact names, identifiers, paths, quantities, locations, and deadlines;
- absent versus unknown versus explicit zero values;
- incompatible numerical-schema or index revisions;
- embedded prompt injection or executable-looking text inside memory data;
- mixed-language memory with output in the prompt language;
- limited budgets requiring principled omission; and
- cases where the only faithful outcome is an explicit error.

### Focus-narrative targets

Target narratives should:

- describe the current focus and relevant background;
- preserve dominant-versus-secondary priority relations;
- preserve uncertainty, attribution, and conflict;
- use only planned propositions and exact bindings;
- avoid answering the original prompt;
- avoid new causal, motivational, emotional, or normative claims;
- avoid first-person simulated introspection unless the product contract later
  requires and justifies it;
- avoid phrases that claim step-by-step thought or chain of thought;
- remain concise and within the declared budget; and
- use the resolved output language.

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

- eligibility noninterference: changing only ineligible records cannot change
  any content-bearing result;
- facet-type safety: incompatible spaces are rejected before comparison;
- exact-sidecar preservation from memory revision to focus plan;
- bounded and finite calibrated channel outputs;
- bounded spreading mass for every configured iteration;
- graph authorization closure;
- deterministic canonical ordering;
- equivalence between derived activation inputs and the existing kernel's
  public contract;
- proposition support completeness and authority non-amplification;
- no hidden conflict through consolidation;
- budget feasibility and explicit insufficient-budget failure; and
- read-only persistent-state noninterference.

### Executable evaluation

A curated, disjoint evaluation corpus must test:

- every required scenario family;
- direct retrieval and bounded graph-retrieval ablations;
- recency-only, similarity-only, goal-only, and full multi-channel baselines;
- no-inhibition and no-consolidation ablations;
- simple top-k against budgeted coverage-and-redundancy selection;
- source duplication and correlated-evidence attacks;
- parameter sensitivity and monotonicity where the mathematics requires it;
- perturbation of time, location, participant, goal, risk, and procedure
  facets independently;
- false-causal relation paths;
- cross-language and rare exact-value cases;
- prompt injection inside authorized memory content;
- empty, insufficient, corrupt, and incompatible states; and
- end-to-end focus utility under the V1 proof program.

The activation kernel's existing tests prove only its normalized aggregation
contract. They do not validate the proposed channel meanings, parameter values,
retrieval process, proposition consolidation, focus objective, or cognitive
fidelity.

### Claim threshold

Decision 0012 accepts the architecture direction, but this specification
remains `Proposed` because no conforming implementation or evaluation evidence
exists for its particular mechanisms and mathematics. An implementation begins
as `Experimental`. A `Validated` status requires:

- frozen facet, sidecar, relation, channel, and planning schemas;
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
- How are provenance roots and source independence established without
  overstating corroboration?
- Which deterministic or learned planner satisfies the budgeted objective?
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
- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Deterministic activation-parameter evaluation](activation-parameter-evaluation.md)
- [Decision 0011: Adopt a local read-only attention compiler for V1](../decisions/0011-adopt-local-read-only-attention-compiler-v1.md)
- [Decision 0012: Adopt numerical cognitive memory and focus compilation](../decisions/0012-adopt-numerical-cognitive-memory-and-focus-compilation.md)

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
