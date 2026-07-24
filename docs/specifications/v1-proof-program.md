# V1 proof program

Status: Proposed

## Purpose

This specification defines how Nemosyne will determine whether the V1 idea is
internally coherent, correctly implemented, empirically useful, and
operationally viable before a supported release claim.

Not every V1 claim can be proved mathematically. The program separates formal
obligations, executable conformance, empirical hypotheses, and operational
measurements so that evidence from one class is never used as proof of another.
It is a verification plan, not evidence that the current repository implements
or validates the V1 product.

## Definitions

### Evidence classes

| Class | Can establish | Cannot establish |
| --- | --- | --- |
| Formal derivation | Properties that follow from explicit mathematical assumptions | Semantic relevance, renderer fidelity, downstream usefulness, or universal runtime behavior |
| Executable conformance | Agreement between an implementation and a contract on tested inputs, states, platforms, and adversarial cases | Statistical generalization or the universal absence of defects |
| Empirical evaluation | Measured utility, harm, retrieval quality, language behavior, and robustness for a frozen population and configuration | Mathematical certainty, biological fidelity, or untested-domain support |
| Operational measurement | Latency, memory, model-load, scaling, crash, and offline behavior on declared hardware and software | Performance or isolation on other systems |

The existing activation kernel has formal and executable evidence for its
already-normalized numeric boundary. The evaluator and revision-1 corpus add
deterministic regression evidence. None of them supplies observations for the
end-to-end V1 product-success estimate.

### Canonical notation and derivation ownership

This table is the sole cross-specification notation registry. A focused
specification owns the formula named in the final column; other documents may
state consequences or link to that derivation but must not copy the normative
formula. A symbol not registered here must remain local to one focused
specification and cannot carry meaning across documents. That specification
maintains a local notation table for every symbol reused across more than one
of its derivations or sections. Symbol identity is the complete rendered symbol,
including namespace-defining subscripts and superscripts; reuse of a bare base
letter is allowed only when the qualified forms are visually distinct and
their owning tables make the distinction explicit. The same complete symbol
cannot acquire a second meaning.

Bound dummy indices and local asymptotic-cardinality metavariables are not
cross-stage semantic identities. A focused specification may reuse them only
inside an explicitly quantified finite scope that defines their domain and
does not reference them as named state outside that scope. This exception does
not permit a local variable to reuse any complete symbol registered below, nor
does it permit two focused specifications to exchange an unregistered
semantic value under the same notation. Cross-stage inputs, outputs, artifacts,
and claims must use registered or visibly qualified symbols.

| Symbol | Type or domain | Canonical meaning | Formula or contract owner |
| --- | --- | --- | --- |
| \(P\) | bytes | Retained original prompt | [Product contract](v1-product-contract.md) |
| \(S\) | zero to three statements | Caller-supplied situation statements | [Product contract](v1-product-contract.md) |
| \(\Xi\) | typed request evidence | Caller-supplied contextual time, optional declared location, and explicit metadata | [Reference architecture](v1-reference-architecture.md) |
| \(I\) | authenticated context | Trusted invocation principal, caller, and authorization facts | [Reference architecture](v1-reference-architecture.md) |
| \(t_{\mathrm{context}}\) | exact contextual instant | Caller-declared situational time | [Product contract](v1-product-contract.md) |
| \(t_{\mathrm{auth}}\) | exact trusted instant | One pinned authorization time | [Reference architecture](v1-reference-architecture.md) |
| \(M^r\) | immutable logical revision | Authoritative memory revision \(r\) | [Reference architecture](v1-reference-architecture.md) |
| \(M_A^{r,p,t_{\mathrm{auth}},I}\) | authorized view | Policy- and invocation-scoped memory view | [Reference architecture](v1-reference-architecture.md) |
| \(\mathcal M_E\) | finite eligible view | Authorized records passing hard integrity and eligibility gates | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\mathcal M_Q\) | finite usage-compatible view | Eligible records admitted for this request use | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(m_i\) | cognitive memory unit | One immutable exact-plus-numerical record version | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\mathcal G_i\) | finite relation set | Typed numerical memory relations for \(m_i\) | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\Pi_i\) | typed metadata | Provenance, authority, validity, uncertainty, and policy of \(m_i\) | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(Q\) | numerical query state | Encoding of \(P,S,\Xi\) under pinned \(K\); no trusted authorization input | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\widehat B_{\mathrm{in}}\) | sealed ingress binding | Compiler-owned typed request, situation, identity-schema, and authenticated configuration identities derived once from exact canonical request content | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(B_Q\) | exact query binding | \((request\_id,situation\_id,configuration\_id)\), independently projected from \(\widehat B_{\mathrm{in}}\) into \(Q\); the request and situation fields are complete typed content identities, not caller labels or bare digests | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(C^r\) | finite retrieved set | Authorized bounded retrieval result | [Reference architecture](v1-reference-architecture.md) |
| \(N\) | finite numerical candidates | Derived activation candidates and signals | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\mathbb U\) | \([0,1]\) | Predictive derivation's closed finite unit interval | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(U_{\mathrm{act}}\) | \([0,1]\) | Activation kernel's finite unit interval, called `U` in its implemented contract | [Activation specification](situation-conditioned-activation.md) |
| \(C_{\mathrm{act}}\) | finite channel set | Activation evidence-channel set, called `C` in its implemented contract | [Activation specification](situation-conditioned-activation.md) |
| \(J_{\mathrm{act}}\) | finite channel set | Activation inhibition-channel set, called `J` in its implemented contract | [Activation specification](situation-conditioned-activation.md) |
| \(D_{\mathrm{act}}\) | positive finite scalar | Activation evidence denominator, called `D` in its implemented contract | [Activation specification](situation-conditioned-activation.md) |
| \(E_i^{\mathrm{act}}\) | \(U_{\mathrm{act}}\) | Candidate evidence aggregate, called \(E_i\) in the implemented contract | [Activation specification](situation-conditioned-activation.md) |
| \(R_i^{\mathrm{act}}\) | \(U_{\mathrm{act}}\) | Candidate retention, called \(R_i\) in the implemented contract | [Activation specification](situation-conditioned-activation.md) |
| \(A_i\) | \(U_{\mathrm{act}}\) | Final activation for candidate \(i\) | [Activation specification](situation-conditioned-activation.md) |
| \(q_{i,c}^{\mathrm{act}}\) | \(U_{\mathrm{act}}\) | Evidence-channel contribution, called \(q_{i,c}\) in the implemented contract | [Activation specification](situation-conditioned-activation.md) |
| \(\mathcal A\) | canonical finite set | One shared `EligibleActivatedMemorySet` | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\Lambda_A\) | immutable lineage receipt | Shared request, memory, policy, retrieval, activation, and configuration identity | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\psi\) | expectation query | One validated prediction-frame query | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\tau_i\) | observed transition | Eligible transition record \(i\) | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\chi_{i,\psi}\) | \(\{0,1\}\) | Expectation-query eligibility indicator | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(C_i^x,C_i^c,C_i^h\) | finite unit interval | State-facet, condition, and horizon compatibility | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\mathcal F_x\) | finite facet set | Registered comparable state facets for prediction | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\omega_f\) | \((0,1]\) | Registered normalized comparable-facet weight | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\delta_{i,f}^{\mathrm{cmp}}\) | \(\{0,1\}\) | Comparable-facet presence indicator | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\gamma_i^{\mathrm{cov}},\eta_i^{\mathrm{match}}\) | finite unit interval | Comparable-facet coverage and conditional match | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\varrho_i\) | compatible typed finite unit interval | Transition reliability admitted under one versioned schema, derivation, calibration-domain, missingness, compatibility, and migration contract | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\alpha_i\) | finite unit interval | Qualified transition support weight | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(a,h,d\) | typed identifiers | Alternative family, outcome group, and dependency group | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\mathcal K_a,\bot_a,\mathcal H_a\) | finite tagged sets | Known, unknown, and complete outcome groups for family \(a\) | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(u_{a,h,d},b_{a,d},\bar u_{a,h,d}\) | finite unit interval | Raw, total, and dependency-budgeted group support | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(s_{a,h}\) | bounded nonnegative scalar | Dependency-budgeted support for outcome \(h\) in family \(a\) | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(Z_a\) | bounded nonnegative scalar | Complete known-plus-unknown family support | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(r^{\mathrm{share}}_{h\mid a}\) | finite unit interval | Relative evidence share within one family | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(D_+\) | nonnegative integer | Count of dependency groups with positive budget | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(N_{\mathrm{support},a}\) | bounded nonnegative scalar | Effective dependency-group support size | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(N_{a,h}\) | bounded nonnegative scalar | Effective support-group count for one hypothesis | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(N_{\mathrm{frame}},N_{\mathrm{frame,max}}\) | bounded nonnegative integers | Actual and configured maximum expectation-frame counts | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\Gamma,\nu\) | finite unit interval or absent | Nearest coverage-qualified compatible-case score and novelty \(1-\Gamma\) | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\Delta_a^{\mathrm{disp}}\) | finite unit interval or absent | Report-only within-family support dispersion | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\mathcal M_{f,a},\mathcal M_f,K_{\max}\) | finite sets and positive bound | Material family groups, complete material frame union, and per-frame limit | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(F\) | canonical finite set | Complete `FocusCandidateSet` | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(E\) | canonical finite bundle | Per-frame `ExpectationBundle` | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\mathcal C_{\mathrm{plan}}\) | finite closure set | Planning candidate closures, called \(\mathcal C\) in its focused contract | [Planning specification](focus-and-expectation-planning.md) |
| \(X\) | subset of \(\mathcal C_{\mathrm{plan}}\) | One candidate plan selection | [Planning specification](focus-and-expectation-planning.md) |
| \(X_{\min}\) | subset of \(\mathcal C_{\mathrm{plan}}\) | Direct mandatory closures plus mandatory frame dispositions | [Planning specification](focus-and-expectation-planning.md) |
| \(X^*\) | subset of \(\mathcal C_{\mathrm{plan}}\) | Canonical selected closure subset | [Planning specification](focus-and-expectation-planning.md) |
| \(G(X),V(X)\) | tagged finite projections | Renderer-visible and validator-only projections of a closure selection | [Planning specification](focus-and-expectation-planning.md) |
| \(\mathcal J\) | finite plan set | Structurally feasible nonempty renderable plans before budget | [Planning specification](focus-and-expectation-planning.md) |
| \(\Phi_{\mathrm{plan}}\) | finite bit vector | Unified optional-closure priority objective | [Planning specification](focus-and-expectation-planning.md) |
| \(\widehat c_K,cost_K\) | checked integer functions | Conservative bound and measured attention cost | [Planning specification](focus-and-expectation-planning.md) |
| \(B\) | checked nonnegative integer | Resolved post-substitution attention budget | [Planning specification](focus-and-expectation-planning.md) |
| \(\ell\) | supported language identity | Resolved declared output language | [Reference architecture](v1-reference-architecture.md) |
| \(L\) | canonical plan | Sole `FocusExpectationPlan` | [Planning specification](focus-and-expectation-planning.md) |
| \(\zeta_i\) | renderer item | One canonical renderer projection item | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal F_R\) | finite facet set | Versioned renderer-facet vocabulary | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathbf n_i,\varphi_i,\kappa_i,\mathbf m_i,\iota_i,\delta_i\) | typed renderer fields | Scalar features, item role, canonical rank, presence mask, proposition identity, and disposition | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal R_i,\mathcal U_i,\mathcal B_i^{\mathrm{src}},\mathcal X_i^{\mathrm{slot}}\) | typed finite renderer fields | Required relations, authority ceiling, source bindings, and exact-slot bindings | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal V_{\mathrm{exact}},\mathcal V_{\mathrm{slotmeta}}\) | exact sidecar and safe projection | Authoritative exact values and model-visible metadata without payload bytes | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal P_{\mathrm{soft}}\) | continuous prefix | Learned model-input projection of the canonical plan | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal B_{\mathrm{attr}}\) | finite attribution-label set | Nonempty token-to-proposition training labels | [Renderer specification](vector-to-attention-renderer.md) |
| \(V_{\mathrm{ctx}}\) | validation context | Prompt- and policy-bound validator-only context | [Renderer specification](vector-to-attention-renderer.md) |
| \(Z_{\mathrm{slot}}\) | internal render value | Pre-substitution text, segments, bindings, and origins | [Renderer specification](vector-to-attention-renderer.md) |
| \(Z_{\mathrm{exact}}\) | internal render value | Exact-slot-substituted render value | [Renderer specification](vector-to-attention-renderer.md) |
| \(T'\) | attention bytes | Validated attention text | [Renderer specification](vector-to-attention-renderer.md) |
| \(O\) | compiled bytes | Exact successful product output | [Product contract](v1-product-contract.md) |
| \(K\) | immutable artifact identity | Pinned compiler configuration and execution envelope | [Reference architecture](v1-reference-architecture.md) |
| \(K_R\) | immutable renderer identity | Exact renderer configuration bound inside \(K\) | [Renderer specification](vector-to-attention-renderer.md) |

The derivation path below is the sole cross-stage composition. Focused
specifications may refine one stage, but no document may bypass a stage, invent
an alternate authoritative intermediate, or redefine another stage's formula.

### Formal compile model

Let:

- `P` be the retained original prompt bytes;
- `S` be zero to three situation statements;
- \(\Xi\) be validated caller-supplied request evidence containing declared
  contextual time `t_context`, optional declared location, and explicit
  metadata;
- `I` be the authenticated invocation context outside the request payload,
  including trusted authorization time `t_auth`;
- \(\ell\) be the resolved declared output language;
- `B` be the attention budget resolved by configuration and policy;
- `M^r` be immutable memory revision `r` with policy revision `p`; and
- `K` be one pinned content-identified compiler configuration, immutable
  artifact set, and supported execution envelope, including exactly one
  renderer configuration, runtime, precision policy, and target platform class
  selected before the call;
- \(K_R\) be the exact renderer configuration bound inside \(K\). A
  V1-deployable \(K\) permits no request-time random input.

The proposed logical stages are:

\[
M_A^{r,p,t_{\mathrm{auth}},I} =
authorize(M^r,I,t_{\mathrm{auth}};K)
\]

\[
Q = encode(P,S,\Xi;K)
\]

\[
\mathcal M_E =
eligibilityGate(M_A^{r,p,t_{\mathrm{auth}},I};K)
\]

\[
\mathcal M_Q =
usageGate(Q,\mathcal M_E,t_{\mathrm{auth}};K)
\]

\[
C^{r} = retrieve(Q,\mathcal M_Q;K)
\]

\[
N = derive(Q,C^{r};K)
\]

\[
\mathcal A = activate(N;K)
\]

\[
F = focusCandidates(Q,\mathcal A;K)
\]

\[
E = expectations(Q,\mathcal A;K)
\]

\[
L = plan(Q,F,E,\ell,B;K)
\]

\[
V_{\mathrm{ctx}}=
buildValidationContext(P,Q,I;K)
\]

\[
Z_{\mathrm{slot}}=
(T_{\mathrm{slot}},segments_{\mathrm{slot}},bindings_{\mathrm{slot}},origins)
=render_{K_R}(L)
\]

\[
Z_{\mathrm{exact}} =
substitute(Z_{\mathrm{slot}},exactSlots(L),K_R)
\]

\[
T' =
validate(Z_{\mathrm{exact}},L,V_{\mathrm{ctx}},K_R)
\]

\[
O=serialize_{\mathrm{product}}(T',P).
\]

`serialize_product` invokes the sole normative successful-output contract in
the [product specification](v1-product-contract.md#successful-output). This
proof program does not duplicate its headers, separators, or byte-concatenation
formula.

Each stage is partial: it returns either its complete value or an explicit
error. It does not return a plausible substitute after a failed precondition.
In particular, renderer or validation failure ends the call; the orchestrator
does not retry the same request with another renderer.
`\mathcal A` is the one canonical `EligibleActivatedMemorySet` shared by both
semantic branches. `F` is the complete bounded `FocusCandidateSet`; `E` is the
qualified `ExpectationBundle` of canonical per-frame sets, including
abstention and control state. Both bind
the same query, memory, policy, authorization, retrieval, and configuration
lineage. `L` is the canonical `FocusExpectationPlan` and contains its resolved
language, exact-slot table, budget, focus items, expectation items, mandatory
relations, and validator controls. `segments_slot` and
`bindings_slot` are untrusted renderer claims. `origins` records whether each
generated token came from ordinary vocabulary or one registered slot.
Substitution validates slot identity and authorization, inserts approved exact
surface bytes, and deterministically recomputes byte offsets without semantic
rewriting. Validation verifies the post-substitution structure and returns the
exact substituted text component unchanged as `T'` or returns an error; it is
not another rendering stage.

### Formula and algorithm obligation registry

This registry gives every cross-stage normative formula family or
deterministic algorithm one stable `ALG-*` identity. It does not relocate a
formula from its owner. `ALG-*` identifiers, predictive-local derivation IDs
such as `EXP-SUP-004`, and delivery work-package IDs such as `EXP-02` are
disjoint namespaces; a local derivation ID never becomes an executable
work-package ID. Each `ALG-EXP-*` row aggregates one or more local
`EXP-<family>-NNN` owners linked through that row's focused section.
`Proposed` properties remain obligations, not current evidence. Anchors are
normative and move only with a same-change registry update.

| ID | Sole owner | Domain, codomain, totality, and numeric contract | Required property and counterexample class | Executable evidence owner | Obligation and gate |
| --- | --- | --- | --- | --- | --- |
| `ALG-SER-01` | [Product output](v1-product-contract.md#successful-output) | Validated attention bytes plus retained prompt bytes to one compiled byte string; exact byte serialization; total after validation | Prompt is the final byte-identical suffix; test normalization, line endings, nested headers, and trailing bytes | `API-01`, `CLI-01`, `SYS-01` | F1, F9; G7 |
| `ALG-MEM-01` | [Numerical query state](cognitive-memory-activation-and-focus.md#numerical-query-state) | Authenticated pinned \(K\) plus the exact validated compile request produce one sealed \(\widehat B_{\mathrm{in}}\), independently projected \(B_Q=(request,situation,configuration)\), and \(Q\); request/situation IDs are domain-separated typed content identities over injective canonical envelopes; typed failure; presence differs from numeric zero; exact prompt remains outside \(Q\); no caller-owned ID, principal, trusted time, policy, or authorization-view input | Same content/configuration is deterministic; changed content/configuration separates subject to the digest-collision assumption; recomputation mismatch and observed collision evidence fail closed; test prompt/situation/context/control mutations, map permutation, cross-request branch swaps, constant/reused/caller IDs, collision witnesses, configuration substitution, locators, content identities, and ambient/trusted-time noninterference | `SIT-01`, `ENC-01` | F2, F3, F10, F12; G3 |
| `ALG-MEM-02` | [Eligible memory view](cognitive-memory-activation-and-focus.md#eligible-memory-view) | Authorized snapshot to \(\mathcal M_E\), then \(Q,\mathcal M_E\) to \(\mathcal M_Q\); total for valid policy artifacts; hard gates precede scores | Excluded records cannot crowd candidates; mutate only unauthorized, deleted, invalid, and usage-incompatible records | `MEM-03`, `RET-01`, `SEC-01` | F2, F3, F12; G2-G3 |
| `ALG-MEM-03` | [Direct cue activation](cognitive-memory-activation-and-focus.md#direct-cue-activation) | Compatible query-memory facets to finite calibrated cues; metric, calibration, missingness, and canonical accumulation are pinned | Inspectable bounded cue lineage; test incompatible spaces, nonfinite values, and duplicated evidence | `SIG-01`, `ACT-00` | F8, F10; G4 |
| `ALG-MEM-04` | [Base availability](cognitive-memory-activation-and-focus.md#base-availability-from-frequency-and-recency) | Valid history statistics to bounded availability; checked finite arithmetic in canonical event order | Recency and frequency affect accessibility, not truth or authority; test future and duplicate events | `SIG-01`, `EVAL-01` | F4, F8; G4 |
| `ALG-MEM-05` | [Risk and urgency relevance](cognitive-memory-activation-and-focus.md#risk-and-urgency-relevance) | Typed temporal, spatial, goal, procedure, hazard, and social facets to separate finite channels; missingness stays typed | Context affects relevance without overriding policy; test deadline-over-constraint, social-as-authorization, and risk-as-probability errors | `SIG-01`, `EVAL-01`, `SEC-01` | F2, F5, F8; G4 |
| `ALG-MEM-06` | [Bounded spreading activation](cognitive-memory-activation-and-focus.md#bounded-spreading-activation) | Bounded seed and substochastic typed relation matrix to bounded propagation; finite depth and canonical sparse order | No unbounded mass or ineligible graph crossing; test cycles, duplicate edges, and unauthorized neighbors | `CORE-01`, `CORE-02`, `PERF-01` | F2, F8; G4 |
| `ALG-MEM-07` | [Kernel composition](cognitive-memory-activation-and-focus.md#signal-derivation-and-existing-kernel-composition) | Derived channels and gates to activation candidates; exact channel identities, no defaults, typed errors | Preserve kernel bounds and explainability; test hidden zero fill and reused channel lineage | `ACT-00`, `ACT-01`, existing kernel tests | F8, F10; G4 |
| `ALG-MEM-08` | [Proposition consolidation](cognitive-memory-activation-and-focus.md#request-local-proposition-consolidation) | Eligible activated records to complete bounded `FocusCandidateSet`; registered equivalence and conflict only | Deduplicate without losing conflict, authority, or provenance; test embedding-only equivalence and cloned sources | `PLAN-01`, `EVAL-01` | F5, F6, F12; G4 |
| `ALG-ACT-01` | [Activation mathematics](situation-conditioned-activation.md#mathematics) | Valid profile and candidates to complete ranking; implemented `f64`, canonical channels, exact score tie then `CandidateId` | \(E_i,R_i,A_i\in[0,1]\), monotonicity, deterministic reconstruction | Existing `nemosyne-core` tests, `ACT-00` | F8; G4 |
| `ALG-EXP-01` | [Expectation hard eligibility](predictive-attention-and-expectation.md#expectation-projection-hard-eligibility) | \(\psi,\tau_i,K\) to \(\chi_{i,\psi}\); closed use, observation, frame, condition, missing-facet, and typed-reliability policy | Ineligible evidence cannot regain support through scores; test predicted evidence, unknown passive conditions, unavailable reliability, and incompatible reliability contracts | `EXP-01`, `EXP-02` | F2, F12, F15; G4 |
| `ALG-EXP-02` | [Facet compatibility](predictive-attention-and-expectation.md#facet-compatibility-and-missing-values) | Comparable facets, condition, and horizon to compatibility plus diagnostics; real semantics and a frozen executable numerical policy | Preserve coverage versus mismatch; test missing values, unit mismatch, and each boundary | `EXP-02`, `EVAL-01` | F10, F15; G4 |
| `ALG-EXP-03` | [Qualified support](predictive-attention-and-expectation.md#qualified-transition-support) | Eligible activation, compatibility, and compatible typed reliability to \(\alpha_i\); finite factors, explicit unavailable states, target-state derivation/calibration identity, optional authenticated source/target migration lineage only for migrated values, and named feature interactions | Bounded inspectable support, never probability; test native values without invented migration, migrated values with complete lineage, derived zero versus unavailable reliability, incompatible schemas and calibration domains, every cross-state migration class, exact rollback, unregistered migration, and duplicated raw-feature influence | `EXP-02`, `EVAL-01` | F8, F13; G4 |
| `ALG-EXP-04` | [Dependency-budgeted support](predictive-attention-and-expectation.md#dependency-budgeted-support) | Grouped support to \(s_{a,h},Z_a,r^{\mathrm{share}}_{h\mid a}\); one canonical budget per dependency group and family | Duplicates cannot multiply family budget; test cloned and split provenance roots | `EXP-02`, `EVAL-01`, `DATA-01` | F13, F14; G4 |
| `ALG-EXP-05` | [Effective support count](predictive-attention-and-expectation.md#effective-support-group-count) | Positive group budgets to participation-ratio diagnostics; explicit zero case, canonical finite sums | Count is bounded by positive groups; test concentration and duplication | `EXP-02`, `EVAL-01` | F13-F15; G4 |
| `ALG-EXP-06` | [Representative medoid](predictive-attention-and-expectation.md#representative-medoid) | Canonical outcome group and registered dissimilarity to one representative; finite limits and total fallback key | Deterministic representative without truth or metric claims; test unavailable and nontransitive distances | `EXP-02`, `PERF-01` | F10, F15; G4 |
| `ALG-EXP-07` | [Bounded alternatives](predictive-attention-and-expectation.md#bounded-alternatives-and-diversity) | Complete frame families to positive set or typed abstention; closed inclusive thresholds, proximity restricted to coverage-qualified transitions, and tagged family/outcome identities | Retain all material alternatives or abstain; test every threshold, no-qualified-case semantics, the split-maxima counterexample, and frame-limit boundaries | `EXP-03`, `EVAL-01`, `TGT-01` | F14, F15; G4 |
| `ALG-EXP-08` | [Observation assessment](predictive-attention-and-expectation.md#observation-and-prediction-error-contract) | Immutable prior and independent sealed observation to typed assessment or error; validation precedes classification | Assessment cannot rewrite prior or memory; test predicted evidence, prior abstention, and compatible co-outcomes | `OBS-01`, `EVAL-02` | F4, F16; G9 |
| `ALG-EXP-09` | [Learned-predictor research boundary](predictive-attention-and-expectation.md#learned-predictor-research-boundary) | Frozen supported envelope and detector artifacts to a pre-invocation disposition, then bounded in-domain slots to typed research candidates, abstention, or null; total through structural errors and independent OOD outcomes | Learned research cannot invent open-world outcomes, bypass eligibility, expose logits as probability, or become a silent V1 fallback | `ML-01`-`ML-03`, `EVAL-02` | P2 research only; F10, F12, F17 |
| `ALG-EXP-10` | [Predictive-stage complexity](predictive-attention-and-expectation.md#computational-complexity) | Finite configured cardinalities and authenticated distance-cost bounds to explicit time and peak-additional-memory bounds | No hidden cross-frame all-pairs work; reject inputs or artifacts outside every declared finite ceiling | `EXP-02`, `PERF-01` | F8, F10; G4, G6 |
| `ALG-PLAN-01` | [Structural projection](focus-and-expectation-planning.md#structural-plan-and-renderable-projection) | Immutable branch-owned `PlanningSourceProjection` values and a minimized permissionless exact-surface inventory to closure selection, lowering-only ceiling meets, exact-slot joins, tagged \(G(X)\), and \(V(X)\); no principal, policy, authorization/disclosure view, or live grant | Renderer and validator truths cannot cross; planning cannot raise authority, allowed-use, or surface ceilings or turn inventory presence into permission; test projection mismatch, ambient-authority noninterference, exact-slot/content identity, inventory minimization, exact-payload, and exclusion leakage | `PLAN-02`, `VAL-01` | F5-F7, F12, F17; G4 |
| `ALG-PLAN-02` | [Mandatory closure](focus-and-expectation-planning.md#mandatory-closure) | Candidates to atomic semantic closures; registered relations; overlapping tagged members counted once | Preserve qualifiers, conflicts, and complete material families; test dropped horizons and alternatives | `PLAN-02`, `EVAL-01` | F5, F6, F15; G4 |
| `ALG-PLAN-03` | [Cost contract](focus-and-expectation-planning.md#cost-contract) | Renderable projection and rendered bytes to checked integer bound and measured cost; one unit, canonical accumulation, no saturation | Accepted bound is conservative and zero iff \(G=\varnothing\); test slot expansion, overflow, and tokenizer mismatch | `PLAN-02`, `REN-04`, `PERF-01` | F7; G4, G6 |
| `ALG-PLAN-04` | [Feasible subsets](focus-and-expectation-planning.md#feasible-subsets) | Finite closures to structural and budget-feasible sets; explicit \(\mathcal J\) and cardinality ceilings | Budget cannot create false empty success; test control-only and all-nonempty-over-budget cases | `PLAN-02`, `EVAL-01` | F7, F9; G4 |
| `ALG-PLAN-05` | [Unified selection](focus-and-expectation-planning.md#canonical-unified-selection) | Finite feasible closure subsets to \(X^*\); complete unified bit objective, no epsilon or cost utility | Unique deterministic cross-branch result without cross-scale score comparison; permute every input and test repairable dependencies | `PLAN-02` exhaustive oracle and equivalence tests | F9, F10, F15; G4 |
| `ALG-REND-01` | [Renderer projection](vector-to-attention-renderer.md#renderer-projection-view) | \(G(L)\), safe slot metadata, and language to typed tensors; fixed facets, masks, and canonical rank | Preserve selected semantics without exact payload access; alter masked and validator-only fields | `REN-01`, `REN-02`, `VAL-01` | F5, F6, F17; G5 |
| `ALG-REND-02` | [Latent resampler](vector-to-attention-renderer.md#typed-latent-resampler) | Bounded item tensors to bounded continuous prefix; fixed dimensions and deterministic runtime identity | Canonical-order determinism and no hidden state; test rank permutation, missing facets, and precision drift | `REN-02`, `REN-03`, `PERF-01` | F10, F17; G5-G6 |
| `ALG-REND-03` | [Generation](vector-to-attention-renderer.md#generation) | Nonempty prefix to slot-bearing claim, then substitution and validation to \(T'\) or error; frozen decoding and checked slots | No answer, action, unsupported claim, or exact-value invention; test injection, smuggling, language, and cost violations | `REN-04`, `VAL-01`, `SEC-01`, `SYS-01` | F7, F17; G5-G7 |
| `ALG-REND-04` | [Training objective](vector-to-attention-renderer.md#combined-objective) | Frozen examples to candidate parameters; research-only; manifest-bound weights, seeds, splits, masks, and numeric policy | Training does not itself prove faithfulness; test leakage, attribution gaming, and nonfinite runs | `ML-01`-`ML-03`, `REN-05`, `REN-06` | F17 empirical support only; G5-G6 |

The predictive specification keeps finer derivation identities because several
proof obligations share one executable stage. Their aggregation is complete
and fixed as follows:

| Cross-stage identity | Predictive-local derivation identities |
| --- | --- |
| `ALG-EXP-01` | `EXP-QRY-001`, `EXP-LIN-001`, `EXP-ELIG-001` |
| `ALG-EXP-02` | `EXP-CMP-001`-`EXP-CMP-003`, `EXP-HOR-001` |
| `ALG-EXP-03` | `EXP-REL-001`, `EXP-WGT-001` |
| `ALG-EXP-04` | `EXP-FAM-001`, `EXP-SUP-001`-`EXP-SUP-004`, `EXP-OMIT-001`, `EXP-OMIT-002` |
| `ALG-EXP-05` | `EXP-ESS-001`, `EXP-ESS-002`, `EXP-NUM-001` |
| `ALG-EXP-06` | `EXP-REP-001`, `EXP-REP-002` |
| `ALG-EXP-07` | `EXP-CTR-001`, `EXP-COV-001`, `EXP-COV-002`, `EXP-DSP-001`, `EXP-MAT-001`-`EXP-MAT-003`, `EXP-ABS-001` |
| `ALG-EXP-08` | `EXP-OBS-001` |
| `ALG-EXP-09` | `EXP-OOD-001`, `EXP-LRN-001` |
| `ALG-EXP-10` | `EXP-CPLX-001`, `EXP-CPLX-002` |

The documentation checker must eventually verify unique IDs, one owner link,
valid anchors, and referenced work-package and gate IDs. `DOC-00` owns that
governance extension. Until implemented, each documentation change retains a
manual conformance receipt in the delivery program.

### Formal obligations

The architecture must discharge these obligations under explicit assumptions.

#### F1: Prompt preservation

Because `P` is retained independently and appended as the final operand of
`O`, the final `|P|` bytes of every successful `O` equal `P`, and no byte
follows them. This proof depends on the serializer using the retained buffer
directly and on every adapter delivering the original bytes without
normalization.

#### F2: Authorization before relevance

Candidate generation receives only the request-compatible \(\mathcal M_Q\),
which is a subset of \(\mathcal M_E\), itself a subset of
\(M_A^{r,p,t_{\mathrm{auth}},I}\). Every downstream source reference must be
constructed from the candidate source set or from the compile request.
Therefore, if constructors prevent forged references and no stage has ambient
memory access, every memory-derived source in a successful plan belongs to
\(M_A^{r,p,t_{\mathrm{auth}},I}\).

Authorization also requires semantic noninterference. For two physical memory
states whose authorized projections for the call are identical, changing only
unauthorized records must not change candidate crowding, ranking, planned
meaning, rendered attention, or content-bearing diagnostics when the request,
invocation context, contextual and authorization times, and configuration are
held fixed:

\[
projection_A(M_1)=projection_A(M_2)
\Rightarrow
semanticCompile(M_1;P,S,\Xi,I,\ell,B,K)
=
semanticCompile(M_2;P,S,\Xi,I,\ell,B,K)
\]

The physical index and search procedure must therefore enforce authorization
before bounded nearest-neighbor or top-k competition, not retrieve a crowded
global top-k and filter afterward. Timing and other side channels require a
separate security model. These properties establish information-flow
eligibility and noninterference, not truth of an authorized record.

#### F3: Snapshot consistency

One immutable memory revision `r`, policy revision `p`, trusted authorization
time `t_auth`, declared context time `t_context`, invocation context `I`, and
authorized view are pinned before memory-dependent work. Authoritative records,
numerical representations, and indexes are checked against `r`; authorization
and disclosure expiry, current normative validity, and supersession use
`t_auth` and `p`; temporal relevance receives both times explicitly, but
`t_context` cannot revive historical instructions as current authority.
`Q` carries only caller-supplied `t_context`; `t_auth` reaches authorization,
validity, and later memory-relevance derivations through pinned private
compile state and is never an input to `encode`.
Downstream stages receive no ambient store handle or wall clock. One immutable
`K` pins the identities of every policy evaluator and content-identified
artifact handle, execution runtime, precision policy, supported platform
class, and deterministic inference policy for the same call. Stochastic
encoding, retrieval, planning, renderer decoding, or validation is not
V1-deployable. Therefore, for fixed `P`, `S`, \(\Xi\), `I`, `ell`, and `B`, a
successful call is a function of one logical memory-policy revision, both
pinned time values, and one compiler-artifact and execution set even when a
writer or updater publishes a later revision concurrently.

Compiler ingress authenticates and pins `K` before constructing
\(\widehat B_{\mathrm{in}}\). Under the injectivity of the registered
canonical encoding and collision resistance of its authenticated digest
algorithm:

\[
C_R=C_R' \land K=K'
\Rightarrow request\_id=request\_id',
\]

\[
C_S=C_S' \land K=K'
\Rightarrow situation\_id=situation\_id',
\]

and unequal canonical content or configuration yields a different applicable
typed identity except with digest-collision probability. This is a
computational assumption, not an injective-hash proof. The retained canonical
buffers permit recomputation checks and make any observed
same-identity/different-content witness a fail-closed collision error.
Principal, `t_auth`, policy, and authorization-view changes cannot affect
either content identity because they are absent from both canonical envelopes.

#### F4: Read-only compilation

Let `Sigma` be all persistent compiler state, including memory, derived
representations, indexes, caches, logs, telemetry, and installed artifacts.
Compilation has the transition:

\[
(\Sigma,request) \rightarrow (\Sigma,result)
\]

The proof requires read-only storage capabilities in the compile dependency
graph and treats request-local allocation as nonpersistent state. Access logs,
cache publication, re-indexing, and consolidation would violate this property
if performed by compile.

#### F5: Authority non-amplification

Let `x <= y` mean that `x` grants no greater authority than `y` in one declared
authority dimension. For an emitted proposition `q`, let `support(q)` contain
every essential premise. The derivation rule must establish:

\[
authority(q) \leq authority(s)
\quad\text{for every }s \in support(q)
\]

If essential source labels are incomparable, the rule must reject the
derivation or map them through an explicitly verified typed composition rule.
Factual support, normative authority, authenticity, and permission to disclose
remain separate dimensions. A normative proposition additionally requires an
authenticated, still-valid source whose authority permits that normative type.
The exact labels and derivation rules remain a required future contract.

#### F6: Structural provenance completeness

Every planned proposition has a nonempty essential support set. Every
assertion-bearing output unit maps to one or more existing planned proposition
identities, and validation rejects missing or unknown identities. A closed
surface-only class may contain only whitespace, punctuation, and structural
delimiters enumerated by the rendering configuration; it cannot carry a noun,
verb, modifier, relation, connective, exact value, or independent semantic
claim. By construction, every structurally accepted assertion-bearing unit has
request or authorized-memory provenance.

This formal property does not prove that free natural language is semantically
equivalent to the mapped proposition. Semantic support, qualification
preservation, answer leakage, and validator false positives or negatives remain
executable and empirical obligations. Provenance establishes support, not
factual truth.

#### F7: Budget safety

Under `ALG-PLAN-03` and `ALG-PLAN-04`, validation accepts only a measured
post-substitution attention value inside the same checked unit and finite
budget domain used by planning. Planning, substitution, rendering, and
validation never truncate semantic content. Mandatory content that cannot fit
fails. When any otherwise justified nonempty faithful plan exists but none
fits, the call also fails rather than returning budget-driven empty attention.
One over-budget optional closure remains skippable when another faithful
nonempty result fits. Invalid units, incomplete domains, unsupported
configurations, or overflow fail before selection. `ALG-REND-03` maps a
post-substitution bound violation to public `FaithfulnessFailure` and
invalidates the exact renderer qualification.

#### F8: Activation properties

If the existing activation kernel is adopted unchanged and its documented
preconditions hold:

\[
0 \leq E_i,R_i,A_i \leq 1
\]

Positive evidence is nondecreasing and positive inhibition is nonincreasing
when all other inputs remain fixed. Canonical input order gives the defined
evaluation order.

These are real-number properties refined by explicit finite-`f64` behavior.
Underflow rejection, clamping, exact ties, and platform behavior remain
executable obligations unless modeled at IEEE-754 level.

#### F9: Atomic semantic result

Every stage returns one complete value or an error, and serialization runs only
after validation. Therefore no compile error yields a partial compiled prompt.
Adapter transport may fail after compilation; it remains a distinct
unsuccessful adapter outcome.

#### F10: Single derivation path

Ranking owns activation scores, the structured plan owns selected meaning, and
the retained prompt owns original prompt bytes. Reports and diagnostics are
derived views. An implementation must not maintain independently editable
copies of these truths.

#### F11: Artifact authenticity before artifact identity

Content digests establish identity only after a trusted manifest establishes
which identities are authorized. A successful compile pins an authenticated
artifact manifest through an installation trust root held outside the mutable
artifact bundle, verifies the manifest's authenticity and policy scope, and
then verifies every opened artifact against its manifest digest. Replacing a
model, encoder, tokenizer, policy evaluator, formatter, renderer, or validator
together with an unsigned self-consistent manifest therefore cannot satisfy
preflight. Trust-root rotation and artifact installation occur outside
compilation and require a separate authenticated update path.

#### F12: Shared-set branch coherence

Focus and expectation receive projections of the same immutable
`\mathcal A`. Each output carries the same request, situation, memory, policy,
authorization-view, retrieval, and configuration identities. The combined
planner validates those identities before selection:

\[
lineage(L)=lineage(F)=lineage(E)=lineage(\mathcal A)
\]

Here, every lineage value is exactly the complete `source_receipt` tuple
\(\Lambda_A\) owned by the predictive-attention specification. The focus set,
expectation bundle, every contained expectation set, plan envelope, and
\(\mathcal A\) expose the same typed fields, and equality is checked field by
field rather than inferred from a newly derived aggregate digest or
reconstructed ambient state. `request_id` and `situation_id` are themselves
complete typed content identities; their schema, algorithm, content-digest,
and configuration-bound-digest components are compared rather than only an
outer display value.

Before that downstream equality, compiler ingress forks one sealed
\(\widehat B_{\mathrm{in}}\) to situation encoding and shared-set
construction. `Q` receives \(B_Q\) from the first projection, while
\(\Lambda_A\)'s request, situation, and configuration projection receives the
same binding directly from ingress. It is forbidden to initialize
\(\Lambda_A\) from `Q` or to initialize `Q` from authorization state. The
focus boundary validates:

\[
B_Q=\pi_Q(\Lambda_A)=
\pi_{request,situation,configuration}(\widehat B_{\mathrm{in}})
\]

using the independently retained branch projections. A cross-request swap,
constant or reused identifier, recomputation mismatch, configuration mismatch,
or observed collision witness fails before request-proposition construction.

If any identity differs, focus construction or planning fails. Neither branch
may retrieve ambient memory, repeat authorization with a different policy, or
mutate the shared set. This establishes one evidence boundary while preserving
distinct focus and expectation semantics.

Planning receives authority information only through the immutable
branch-owned `PlanningSourceProjection` fields that already accompany each
consumable item. For essential support \(s\), let \(A_s\), \(U_s\), and \(R_s\)
be its authority, allowed-use, and surface-authority ceilings. A planned
proposition may derive only the closed-schema meets

\[
A_p=\bigwedge_s A_s,\qquad
U_p=\bigwedge_s U_s,\qquad
R_p=\bigwedge_s R_s
\]

and may copy or lower those results; it cannot authorize, reauthorize, widen
disclosure, or consult ambient principal, policy, authorization, or disclosure
state. The separate exact-surface inventory is the canonical minimum union of
upstream-referenced slots and carries only bytes, content identity, and display
metadata. Inventory presence grants no use: the matching upstream slot/item
binding and identical content identity are both mandatory. Projection,
ceiling, slot, or surface mismatch fails closed. Therefore ambient authority
changes that do not change the immutable branch inputs and pinned planning
configuration cannot change the plan.

#### F13: Dependency-budget and duplicate non-amplification

Under `ALG-EXP-04`, each dependency group receives one finite budget inside one
mutually exclusive alternative family. Exact duplicate transitions are
content-canonicalized before representatives are selected. Consequently, an
exact duplicate or a record whose qualified weight does not exceed the current
group maximum cannot change aggregate support. A fully saturated dependency
budget cannot increase.

An additional record with a higher qualified weight may legitimately increase
an unsaturated \(b_{a,d}\); the invariant limits one dependency group's total
budget rather than freezing its current evidence.
This property depends on correct provenance-root and dependency-group labels.
It prevents known duplicate amplification; it does not prove that all hidden
dependencies were discovered.

#### F14: Alternative-family normalization without probability promotion

`ALG-EXP-04` owns the relative-support derivation.
It computes support separately inside each explicit mutually exclusive
alternative family. Every canonical known outcome group, including a group
later classified as non-material and omitted from rendering, and the one
explicit unknown group enter \(Z_a\) exactly once through their canonical
group support \(s_{a,h}\). The derived `OmittedSupport` control aggregates
already-counted non-material known groups after materiality evaluation; it is
not a member of \(\mathcal H_a\) and never enters the denominator again.
Compatible co-outcomes belong to separate families and never compete for one
normalization budget. Under that specification's preconditions, every defined
\(r^{\mathrm{share}}_{h\mid a}\) is bounded in \([0,1]\) and the complete
family shares sum to one. It is still only an evidence share under the
retrieved support model. No renderer, API, metric, or release claim may rename
it probability, confidence, truth, or expected utility without a separately
accepted calibration contract and disjoint evidence.

#### F15: Bounded alternatives, abstention, and no action selection

Under `ALG-EXP-03` and `ALG-EXP-07`, a positive expectation requires
structurally valid support from an eligible direct observation or an explicitly
permitted registered derivation, a compatible typed reliability value under
its pinned schema, derivation, calibration-domain, and migration contract,
positive known support, and every frozen materiality, coverage, dependency,
retrieval, conflict, canonicalization, and faithful frame-local \(K_{\max}\)
representation precondition. Coverage and proximity must be established by at
least one jointly qualifying transition: proximity is evaluated only among
transitions that pass the coverage minimum. Separate transitions cannot
combine a coverage maximum and a proximity maximum.

The expectation branch preserves material mutually exclusive alternatives and
required unknown or omitted-support qualification together within that limit
or abstains. Global plan-budget failure follows `ALG-PLAN-04`: it remains a
typed planning error whenever justified nonempty attention cannot fit. An
abstention is a successful semantic result, not a fabricated negative
prediction.

Expectation kinds describe a hidden present state, passive successor, or
conditional outcome. They contain no action-selection field. Conditions may
name an observed action but never recommend it. The combined plan and renderer
must preserve that type boundary.

#### F16: Offline observation-assessment evidence

Under `ALG-EXP-08`, the conformance harness compares an immutable prior expectation
fixture \(E_{\mathrm{prior}}\) with one independently authenticated observation
fixture \(o\) under a pinned frame-relation configuration
\(K_{\mathrm{obs}}\). These fixtures and the assessment record are
sealed evaluation artifacts, not compile request fields, public product
results, persistent user memory, or a second V1 API. The total assessment
function reports a per-hypothesis support, contradiction,
compatible-co-outcome, different-frame, or ambiguous relation, or a typed
prior-abstained or observation-ambiguous disposition. It does not change the
expectation's support, relative support, ordering, or persistent memory.

The harness retains both input fixture identities, the pinned configuration
identity, the assessment result, and evidence-receipt identity so the result is
reproducible. Product recomputation requires a new explicit compile query after
a separately authorized memory-management operation. Renderer or
downstream-agent output is never an observation merely because it follows the
prediction. No runtime observation-assessment endpoint or retention contract is
part of V1.

#### F17: Renderer semantic non-amplification

Every assertion-bearing rendered segment must bind to one selected focus or
expectation proposition and every mandatory relation or qualifier for that
proposition. Validation rejects an unknown hypothesis, promoted fact,
probability statement, answer, action recommendation, lost condition or
horizon, hidden material alternative, or unsupported connective. Therefore a
successful renderer result is a lexicalized view of `L`, not a second semantic
inference path. Neural validation remains empirical and fallible; F17 defines
the contract to test rather than proving a model can satisfy it.

### Requirement traceability

| ID | Product requirement | Owning boundary | Required evidence |
| --- | --- | --- | --- |
| `V1-R01` | Exact prompt/request origin authentication, zero to three situations, caller-supplied contextual time, optional declared location, explicit request metadata, compiler-owned content identities, and separate private trusted caller and authorization time that do not enter \(Q\) | Invocation context, authenticated prompt, compiler ingress, and situation encoding | Exact prompt/request presentation freshness, substitution, cross-pair, and replay; origin, count, canonical-envelope, contextual-time/location/metadata presence, deterministic \(Q\), independently projected \(B_Q\), same-content identity, content/control/configuration mutation, map-permutation, cross-request swap, constant/reused/caller-ID, recomputation/collision-witness, locator/content-identity, invalid-input, trusted-time noninterference, and forged-time authorization-isolation tests |
| `V1-R02` | One deterministic complete result or explicit error with separate transport failure and monotonic authority-free cancellation | Orchestrator and adapters | F3, F9, repeatability, prohibited-random-input, cancellation source-drop/token-clone/concurrent-idempotence/monotonic-visibility/every-stage/final-return-race, failure-injection, and adapter-delivery tests |
| `V1-R03` | Byte-identical original prompt and exact framing | Ingress and serializer | F1, golden tests, and arbitrary UTF-8 property tests |
| `V1-R04` | Read-only one-revision compilation | Snapshot and compile capability graph | F3, F4, concurrency, configuration-pinning, and write-detection tests |
| `V1-R05` | Authorization before unified cross-context relevance | Policy gate and candidate generation | F2, canary exclusions, cross-context recall, and revocation policy tests |
| `V1-R06` | Source support, qualification, immutable branch-owned planning projections, lowering-only authority/allowed-use/surface ceilings, and no authority promotion | Plan and validation | F5, F6, F12, adversarial provenance, projection/ceiling/slot/content-identity/inventory-minimization failures, ambient-authority noninterference, and semantic-fidelity cases |
| `V1-R07` | Evidence-bound focus and/or qualified expectation context, or faithful empty attention | Planner, renderer, and validation | Plan-shape, proposition-label, leakage, support, and raw-copy metrics |
| `V1-R08` | Declared language, finite budget, faithful empty attention, and budget error | Planner, renderer, and validation | Per-language evaluation and exact budget-boundary tests |
| `V1-R09` | Local memory and no compile network access or compiler-initiated disclosure beyond the authorized local result channel | Runtime and packaging | F11, network-denied integration, capability audit, result-channel authorization, artifact-authenticity, and storage-location tests |
| `V1-R10` | No caller-supplied trust or internal identity, no focus/planning authorization capability, no discovery, downstream AI invocation, or automatic learning | Compile dependency boundary | Private trusted-type and ingress construction, all-field forgery, cancellation authority-noninterference, static no-authority-edge, capability, and prohibited-call tests |
| `V1-R11` | Numerical relevance after ingress with retained exact evidence | Encoding through planning | Schema, reconstruction-limit, provenance, and perturbation tests |
| `V1-R12` | Coding agents are the first supported domain and claims remain bounded | End-to-end harness and release process | Sealed coding-task outcomes and frozen evidence receipts |
| `V1-R13` | Memory management remains separate from compile | Compile capability boundary | Absence of management dependencies, persistent-write detection, and explicit rejection of management requests |
| `V1-R14` | Versioned transition evidence with outcome representation, observation status, and state-typed reliability lineage | Transition schema and memory read boundary | Constructor, provenance, condition, horizon, status, version, every cross-state migration pair, and exact rollback fixtures |
| `V1-R15` | Focus, expectation, goal, action, answer, fact, and probability remain distinct | Domain types, combined plan, renderer, and validator | Type-boundary, corruption, semantic-fidelity, and leakage tests |
| `V1-R16` | Zero to a finite number of competing expectations | Expectation kernel | Empty, single, tied, alternative, cardinality, and canonical-order properties |
| `V1-R17` | Complete expectation qualification | Expectation kernel and combined planner | Condition, horizon, support, counterevidence, uncertainty, exact-slot, and authority-closure reconstruction |
| `V1-R18` | Duplicate and dependency non-amplification | Transition canonicalization and expectation support | F13, duplicate metamorphism, saturated-group, and hidden-dependency sensitivity cases |
| `V1-R19` | Relative support is not probability | Expectation schema, renderer, API, and documentation | F14, schema-label, forbidden-lexeme, and calibration-claim audits |
| `V1-R20` | Valid-but-insufficient evidence abstains while malformed canonicalization, dependency, lineage, and global-budget states fail | Expectation kernel and combined planner | Every abstention reason, error-versus-abstention, and boundary-threshold fixture |
| `V1-R21` | Material alternatives and unknown or omitted support remain visible | Expectation kernel, closure selection, and validator | Alternative-preservation, top-k-renormalization, conflict-budget, and one-sided-rendering cases |
| `V1-R22` | Observation assessment remains outside compile and cannot mutate a prior result | Offline conformance harness and compile API boundary | F16, API-absence checks, complete relation matrix, immutable-fixture, and explicit-rerun tests |
| `V1-R23` | Generated outputs never become memory truth implicitly | Compile and memory-management capability boundaries | Taint, provenance-origin, prohibited-write, and generated-output ingestion tests |
| `V1-R24` | Renderer is a local lexicalizer only | Combined plan, renderer, exact-slot resolver, and validator | F17, model corruptions, semantic verifier, no-answer, no-action, and no-probability gates |
| `V1-R25` | Every stage has finite configured limits | Ingress, retrieval, activation, expectation, planning, rendering, and runtime | Limit-boundary, complexity, cancellation, memory, latency, and fail-closed tests |

### Executable conformance program

Implementation evidence must include:

- boundary tests for every public constructor and error class;
- property tests over arbitrary valid UTF-8 prompts and line endings;
- canonical ingress-identity properties covering same-content determinism,
  prompt versus situation/control mutation separation, explicit presence
  states, metadata-map permutation, authenticated configuration pinning, and
  absence of public caller control over identity fields;
- adversarial identity fixtures for cross-request `Q`/shared-set swaps,
  constant IDs, reuse of a prior request's valid ID with nonidentical content,
  recomputation mismatch, outer-digest collision with distinct content
  digests, and an observed
  same-complete-identity/different-canonical-bytes collision witness; all must
  fail before proposition construction, while true unobserved digest
  collisions remain covered only by the declared cryptographic assumption;
- fuzzing of framing, metadata, state decoding, and persistent input;
- model-based storage tests for snapshot publication and concurrent writers;
- metamorphic tests for input permutations, absent optional metadata, and
  irrelevant candidate additions;
- canary memories that are highly relevant but unauthorized;
- unauthorized near-neighbor additions that would crowd a global top-k;
- stale, superseded, contradictory, low-confidence, and malicious memories;
- exact values that cannot be reconstructed safely from a lossy vector;
- cross-project relevant records and same-project irrelevant records;
- focus and expectation projections with identical and deliberately mismatched
  lineage identities;
- transition conditions, horizons, statuses, provenance roots, dependency
  groups, typed reliability states, reliability schema/derivation/calibration
  domains, migration lineages, and alternative-family relations at every
  validity boundary;
- exact duplicate transitions, dependent paraphrases, saturated dependency
  groups, and one deliberately mislabeled hidden dependency;
- compatible co-outcomes in separate alternative families and mutually
  exclusive outcomes inside one family;
- known, unknown, omitted, censored, contradicted, and zero-support outcomes;
- compatible derived-zero reliability versus missing, unknown, inapplicable,
  incompatible, malformed, and explicitly migrated reliability;
- cross-state reliability migrations whose source and target variants require
  different metadata, plus rollback to the exact source variant and revision;
- a split-maxima case in which one transition passes only the coverage minimum
  and another passes only the proximity minimum;
- every expectation abstention reason and every corresponding malformed-input
  error so abstention is never used to hide corruption;
- focus-only, expectation-only where the complete-scope precondition holds,
  combined, expectation-abstaining, and empty plans;
- planning-priority tables that are total, injective, permutation-invariant,
  and demonstrably independent from serialization keys and identifier
  magnitude;
- valid integer cost boundaries plus missing-unit, unsupported-domain,
  overflow, and renderer-bound-underestimation failures;
- supported and deliberately wrong dominant expectations over identical focus;
- later observations classified exactly as support, contradiction, ambiguity,
  or a different frame without changing stored support, plus a separate
  explicit rerun case;
- missing, stale, incompatible, and corrupt derived artifacts;
- approximate-index misses and degraded search;
- empty memory with justified empty and nonempty attention;
- no situation statements and three situation statements;
- mixed, ambiguous, unsupported, and explicitly selected languages;
- answer leakage, action selection, probability inflation, fact promotion,
  raw copying, unsupported clauses, lost conditions or horizons, collapsed
  alternatives, and lost qualifications;
- budgets immediately below and at the faithful minimum;
- crashes, timeouts, resource exhaustion, and partial-output prevention;
- blocked network access and persistent-write detection; and
- cold and warm invocation on every supported platform.

These tests establish conformance only for their declared implementation and
environment.

### Empirical hypotheses

Each hypothesis is falsifiable and configuration-specific.

| ID | Hypothesis | Primary comparison |
| --- | --- | --- |
| H1 | Constrained expert attention creates meaningful headroom on context-dependent coding tasks | Expert reference versus prompt only and situation only |
| H2 | Persistent memory adds value beyond caller-supplied situation and metadata | Full memory condition versus situation only |
| H3 | Proposed candidate generation finds required memory more reliably than semantic top-k at the same candidate budget | Candidate recall and downstream outcomes |
| H4 | Proposed signal derivation adds value, and the proposed activation rule adds value over simpler rankers when derived signals are held fixed | Separate fixed-signal ranking and end-to-end derivation comparisons |
| H5a | Focus-candidate construction improves response-changing context coverage and exclusion over record-level top-k when the shared activated set is held fixed | Focus-candidate comparison over frozen activated inputs |
| H5b | The deterministic expectation baseline preserves eligible alternatives and abstains more faithfully than semantic-neighbor or dominant-outcome heuristics over identical transition evidence | Expectation-kernel comparison over one frozen shared set |
| H5c | Adding a correct qualified expectation to the same focus plan improves selected context-dependent tasks over focus-only attention without exceeding frozen anchoring and harm limits | Focus-plus-expectation versus token-matched focus-only |
| H5d | A deliberately wrong dominant expectation is detectably harmful relative to the correct and abstaining conditions | Wrong-expectation negative control versus correct and abstaining conditions |
| H5e | Combined plan selection improves mandatory closure, qualification, alternative preservation, and budget use over independent focus and expectation truncation | Combined planner over frozen branch outputs |
| H6a | A learned numerical bridge carries the required typed plan information more faithfully than a simple projection | Registered latent resampler versus MLP projection over identical frozen plans and model |
| H6b | The local renderer preserves focus and expectation meaning without material unsupported claims, probability inflation, fact promotion, exact-value errors, answer leakage, or action selection | Candidate renderer versus deterministic and expert rendering |
| H6c | The qualified release checkpoint is the smallest tested local model that passes every frozen renderer and resource gate | Model slate compared under identical plan, adapter, training, quantization, and hardware conditions |
| H7 | The complete compiler improves context-dependent task success without unacceptable harm on context-independent tasks | Candidate V1 versus prompt only and the strongest frozen non-oracle |
| H8 | V1 meets frozen local resource budgets on reference hardware | Cold/warm operational measurements |

Failure of H1 rejects the product premise before full architecture
implementation. Failure of a later hypothesis directs work to its owning
stage rather than permitting unrelated tuning.

Component comparisons reuse frozen intermediate artifacts. A ranker comparison
must not rederive signals, a focus or expectation comparison must consume the
identical shared activated set, a combined-planner comparison must not change
either branch output, and a renderer comparison must use the identical
structured plan.

### Conditions and ablations

Each sealed task is run under frozen, token-matched conditions where
applicable:

1. original prompt only;
2. prompt plus situation and metadata, without persistent memory;
3. situation-only condition plus irrelevant placebo attention of matched size;
4. token-matched raw context;
5. token-matched semantic-similarity top-k;
6. strongest frozen deterministic non-oracle baseline;
7. Nemosyne focus-only plan with the deterministic renderer;
8. Nemosyne expectation-only plan with the deterministic renderer where its
   complete-scope precondition holds;
9. Nemosyne focus-plus-expectation plan with the deterministic renderer;
10. the same Nemosyne focus-only plan with the candidate renderer;
11. the same valid Nemosyne expectation-only plan with the candidate renderer;
12. the same Nemosyne focus-plus-expectation plan with the candidate renderer;
13. the same focus plan with one deliberately wrong dominant expectation and
    otherwise identical qualification and token budget;
14. the same focus plan with a renderer-visible expectation abstention, kept
    distinct from condition `7`, whose plan has no renderer-visible expectation
    role even when validator-only abstention controls remain;
15. one frozen expert-authored focus-plus-expectation plan with the candidate
    renderer; and
16. expert/reference rendering of that identical frozen expert plan.

All conditions use the same scenario-specific `t_context`, `t_auth`, memory
revision `r`, policy revision `p`, authorized-view identity, downstream model
version, message role and placement, decoding configuration, tool access, seed
schedule, and effective budget. Every task-condition starts from the same
content-hashed repository and environment snapshot in a fresh isolated process
and model session. Mutable caches, tool state, files, and background processes
are reset or identically preseeded. Condition order is randomized only after
this carryover isolation. Only the named treatment changes.

H4 through H6 additionally use component-level swaps over frozen signals,
shared activated sets, branch outputs, plans, and renderings; the sixteen
end-to-end conditions alone do not identify those internal effects.

Signal, consolidation, adapter, and model choices also require the following
component-level ablations over immutable intermediates:

| Boundary | Frozen comparisons |
| --- | --- |
| Signal derivation | one global semantic similarity; typed direct cues; cues plus temporal and base availability; cues plus active-goal, procedural, hazard, and social channels |
| Associative activation | no graph propagation; each registered bounded hop and restart configuration over the same direct seeds |
| Focus consolidation | record-level top-k; canonical-proposition grouping without corroboration bonus; accepted support-dependency and conflict-preserving consolidation |
| Expectation derivation | no expectation; nearest transition; unbudgeted dominant outcome; dependency-budgeted deterministic baseline with alternatives and abstention; any later learned set predictor |
| Combined plan selection | independent branch truncation; score-only top-k; exhaustive canonical closure selection; any optimized implementation proven equivalent over supported limits |
| Adapter | deterministic plan labels; two-layer MLP prefix; typed latent resampler with `8`, `16`, and `32` virtual tokens |
| Training | frozen model with bridge only; identical bridge plus LoRA; quantized and unquantized inference |
| Local model | The mandatory cohort slate, deployment pairing, and capacity fallback owned by the local-renderer model-qualification specification |

No model comparison may use a different semantic plan, target set, split,
exact-slot policy, or validation rule. General benchmark scores are descriptive
metadata and cannot select the Nemosyne renderer.

These comparisons isolate:

- situation value: `2 - 1`;
- prompt-length or placebo effects: `3 - 2`;
- persistent-memory value: memory conditions against `2`;
- expectation contribution: `9 - 7` and `12 - 10`;
- expectation-only behavior where valid: `8` and `11` against `2`;
- renderer quality: `7` against `10`, `8` against `11`, `9` against `12`, and
  `15` against `16`;
- wrong-expectation sensitivity: `13` against `12` and `14`;
- complete product value: `12` against `1`, `2`, and the strongest of `4` to
  `6`; and
- remaining headroom: `16 - 12`.

Conditions `15` and `16` differ only in rendering. Conditions `10`, `12`,
`13`, and `14` share the same focus branch output. The wrong-expectation
condition is a negative control, not a candidate product configuration. The
expert plan and reference rendering obey the same source, authority, language,
placement, size, expectation, and non-answering/non-action contract. The
expert reference is not an oracle or a proven optimum.

### Metrics

#### Retrieval and selection

Let `G` be the set of required propositions. For each `g` in `G`, annotation
provides a nonempty set `support(g)` of acceptable authoritative record or
record-group bindings. Define:

\[
hit(g,C)=
\begin{cases}
1,&C\text{ contains one acceptable binding in }support(g)\\
0,&\text{otherwise}
\end{cases}
\]

\[
recall@k = \frac{\sum_{g\in G}hit(g,C)}{|G|}
\]

This recall is not applicable when `G` is empty. Empty-required cases are
evaluated through abstention and must-exclude metrics instead. Support
equivalence prevents one authoritative expression of the same required
proposition from being scored as a miss solely because another record identity
was annotated first.

Measure:

- required-memory recall at the frozen candidate budget;
- must-include proposition recall;
- must-exclude violation rate;
- precision and, when graded relevance is independently annotated, normalized
  discounted cumulative gain within budget;
- empty-attention sensitivity and specificity;
- cross-context recall;
- stale, superseded, conflict, and injection handling;
- score margins and stability under permitted perturbations; and
- worst-category and worst-repository performance.

#### Expectation and combined planning

For every explicit alternative family, measure:

- eligible-transition recall at the frozen transition budget;
- condition-, horizon-, outcome-kind-, authority-, and usage compatibility;
- typed-reliability admission by schema, derivation, calibration domain,
  missingness state, and, only for migrated values, complete migration lineage;
- exact-duplicate and dependency-group amplification under metamorphic
  additions;
- known, unknown, omitted, censored, and counterevidence mass preservation;
- mutually exclusive alternative recall and compatible co-outcome separation;
- dominant known-support stability and material-alternative preservation;
- abstention sensitivity and specificity by reason code;
- hypothesis-level and family-level effective support-group counts;
- facet coverage, coverage-qualified novelty, retrieval-completeness, and
  missingness diagnostics, including no-qualified and split-maxima cases;
- rate of invalid probability, confidence, truth, causal, goal, or action
  interpretations; and
- reconstruction of every hypothesis, control, and support summary from
  retained transition references and the pinned configuration.

For the combined planner, measure mandatory-closure recall, forbidden-item
precision, authority and relation preservation, cost-bound validity, exact
reference agreement on small fixtures, and the rate at which material
alternatives force faithful abstention rather than one-sided output.

Relative support and entropy-like dispersion are descriptive score diagnostics
only. They are not evaluated with probability calibration metrics unless a
later accepted calibration contract defines a probabilistic target and
disjoint observations.

#### Rendering

Measure:

- clause-level source traceability;
- mandatory-proposition coverage;
- unsupported-claim rate;
- qualification and conflict preservation;
- normative-authority violations;
- answer leakage;
- unsupported action or tool-selection leakage;
- expectation-to-fact, condition-to-recommendation, and evidence-share-to-
  probability promotion;
- expectation-kind, condition, horizon, alternative, counterevidence,
  uncertainty, and abstention fidelity;
- language match;
- mandatory exact-slot recall and authorized-slot precision;
- wrong-slot, duplicate-slot, and altered exact-value rates;
- exact budget compliance; and
- raw-source copying beyond explicitly permitted exact values.

#### Downstream behavior

Primary coding outcomes are:

- executable task success;
- repository-invariant preservation; and
- explicit instruction compliance.

Let the required frozen baseline set be:

\[
\mathcal{B}_{release}=\{b_{prompt},b_{strong}\}
\]

where `b_prompt` is the unchanged prompt without attention and `b_strong` is
the strongest eligible non-oracle baseline selected using development or
calibration evidence before sealed outcomes are accessible. If they are
behaviorally identical, the duplicate comparison is collapsed and recorded.
Let `Y_n(i)` and `Y_b(i)` be predeclared binary task-level success outcomes
under Nemosyne and baseline `b` for task `i`.

When generation is repeated stochastically, the protocol either defines a
binary task-level rule before evaluation or replaces these formulas with a
paired hierarchical model that estimates task-population effects and
reversals directly. Fractional within-task averages are never inserted into
the binary harm predicates, and repeated generations are never counted as
independent tasks. Let \(N_{\mathrm{task}}>0\) be the finite number of sealed
task units in the declared evaluation population. The binary task-level
estimands use frozen normalized design weights `v_i > 0`, with
`sum_i v_i = 1`. A self-weighting sample uses
\(v_i=1/N_{\mathrm{task}}\). A stratified, unequal-probability, or deliberately
balanced sample must derive and freeze its weights from the sampling design
before outcomes are observed. If no population sampling design is claimed,
uniform weights estimate only the sealed evaluation set.

Let `I_D` and `I_I` be the frozen context-dependent and context-independent
task sets. For each nonempty set, its design weights are renormalized within
that set. For each `b` in `B_release`, the context-dependent paired effect is:

\[
\Delta_b =
\sum_{i\in I_D}
\frac{v_i}{\sum_{k\in I_D}v_k}
(Y_n(i)-Y_b(i))
\]

The corresponding population harm rate is:

\[
h_{population,b} =
\sum_i v_i \mathbb{1}[Y_b(i)=1 \land Y_n(i)=0]
\]

The conditional reversal rate among tasks that the baseline solves is:

\[
h_{reversal,b} =
\frac{\sum_i v_i \mathbb{1}[Y_b(i)=1 \land Y_n(i)=0]}
     {\sum_i v_i \mathbb{1}[Y_b(i)=1]}
\]

If baseline `b` has no successes in the declared population,
`h_reversal,b` cannot be estimated. The release evaluation is inconclusive for
that mandatory comparison. A different baseline cannot substitute for either
required comparison after sealed evaluation begins. Report and gate both harm
measures for every required baseline so a low baseline success rate cannot
hide severe regressions. Critical severity classes remain separate release
blockers.

The context-independent paired effect is:

\[
\Delta_{b,independent} =
\sum_{i\in I_I}
\frac{v_i}{\sum_{k\in I_I}v_k}
(Y_n(i)-Y_b(i))
\]

For every language, task-family, and risk subgroup included in the supported
claim, compute the same non-inferiority and harm estimands with weights
renormalized inside the subgroup. Each claim-bearing subgroup must have
predeclared adequate exposure and pass its frozen gates. A failing or
underpowered subgroup prevents the broader claim; it may be excluded only by a
new, prospectively frozen narrower claim, never after inspecting the same
sealed outcomes.

Blinded human adjudication may resolve subjective instruction cases. An AI
judge is supplementary and never the sole primary endpoint.

#### Operations

Measure cold and warm:

- median and 95th-percentile latency;
- peak additional memory;
- artifact load and unload time;
- input and output size;
- timeout and crash rate;
- database-size and candidate-count scaling; and
- network and persistent-write attempts.

### Statistical protocol

Before sealed data is accessible, freeze:

- one primary endpoint, both required baseline comparisons, and their
  multiplicity treatment;
- baseline-specific minimum superiority effects `delta_min,b`;
- baseline-specific context-independent non-inferiority margins `delta_NI,b`;
- baseline-specific maximum population harm rates `h_population_max,b`;
- baseline-specific maximum conditional reversal rates `h_reversal_max,b`;
- claim-bearing language, task-family, and risk subgroups with their
  non-inferiority, harm, exposure, and power requirements;
- sampling unit, inclusion probabilities, design weights, and clustering
  hierarchy;
- sample-size and power calculation based only on development data;
- stochastic repetitions, seeds, and aggregation;
- minimum exposure and maximum one-sided rate bound for each critical failure
  class;
- timeout, crash, missing-data, exclusion, and corruption policy;
- multiplicity treatment for secondary models, languages, and subgroups; and
- the confidence-interval and hypothesis-test implementation.

Use paired cluster-aware analysis. The default independent cluster is the
connected component induced by every known shared dependence, including
semantic lineage, repository or base snapshot, base task, and generated
artifact family. A predeclared multi-level or multiway model may replace this
coarsest-unit aggregation only when it represents every known shared level. If
there is exactly one independent pair per cluster, an exact paired test is
permitted. Otherwise the primary inference uses cluster-level permutation,
aggregation to independent clusters, or a predeclared cluster-aware
hierarchical method. Confidence intervals use a compatible cluster-aware
procedure. Every estimator and interval honors the frozen design weights.
Continuous or ordinal outcomes use paired cluster resampling or a declared
hierarchical model. Generated variants and repeated model runs are not
independent samples.

A probability sample from a defined frame may support a claim about its
declared target population when the sampling design and weights are honored.
An authored, convenience, or otherwise nonprobability sealed set supports only
a claim about that benchmark unless a separate, justified generalization model
was frozen before outcomes were observed.

A release claim requires all of the following for every `b` in `B_release`:

\[
lowerCI(\Delta_b) > \delta_{min,b}
\]

\[
lowerCI(\Delta_{b,independent}) > -\delta_{NI,b}
\]

\[
upperCI(h_{population,b}) < h_{population\_max,b}
\]

\[
upperCI(h_{reversal,b}) < h_{reversal\_max,b}
\]

Because the task-level outcomes are binary, both paired effects have estimand
domain \([-1,1]\), while both harm rates have estimand domain \([0,1]\) when
defined. Every frozen threshold is finite and must satisfy
\(0<\delta_{min,b}<1\),
\(0<\delta_{NI,b}\leq1\),
\(0<h_{population\_max,b}\leq1\), and
\(0<h_{reversal\_max,b}\leq1\). A manifest containing a non-finite or
out-of-domain threshold is invalid before sealed data become accessible and
cannot support a release claim.

Every claim-bearing subgroup must additionally pass its frozen
non-inferiority and harm bounds for every required baseline. The positive
values of the margins and bounds are not selected by this specification. They
require a later decision before the sealed set is opened. Report effects and
intervals, not only p-values.

Zero observed critical failures is a release requirement but not proof of zero
risk. Each critical class must meet a predeclared minimum exposure count and a
maximum one-sided confidence bound. Any rate claim includes its sampling and
dependence assumptions.

### Sealed evaluation protocol

1. Build development and calibration evidence without sealed cases.
2. Freeze the target population, task taxonomy, sampling frame, case-selection
   procedure, inclusion probabilities, design weights, architecture revision,
   compiler configuration, parameters, renderer, downstream models, prompts,
   baselines, budgets, metrics, analysis code, thresholds, hardware, and
   resource limits.
3. Have an independent custodian sample or author sealed semantic cases from
   that frame without calibration lineages.
4. Label must-include, may-include, must-exclude, task outcome, and authority
   expectations without candidate outputs or scores.
5. Use independent annotation and documented adjudication for disputed labels.
6. Freeze and hash the manifest, artifacts, and analysis environment.
7. Randomize condition order and execute one receipt-producing evaluation.
8. Preserve failures, timeouts, exclusions, and inconclusive cases.
9. Record every sealed attempt permanently, including failed and inconclusive
   revisions.
10. If a release gate fails, create a new product revision and a newly authored
    sealed set; do not tune and rerun the exposed set as fresh evidence.
11. A first passing revision requires either a predeclared sequential-testing
    correction covering all attempts or an independent confirmatory replication
    before a supported claim.

The revision-1 activation corpus has already influenced development. It remains
regression evidence and cannot become sealed end-to-end evidence.

### Counterexample suite

The development suite must include:

- relevant memory associated with an unrelated project;
- irrelevant memory associated with the current project;
- unauthorized memory with maximal semantic similarity;
- an old preference followed by an authoritative correction;
- a high-confidence observation that conflicts with an instruction;
- malicious instructions embedded in memory, repository, or tool content;
- authorized and unauthorized copies of the same proposition;
- unresolved conflict between sources;
- duplicate memories that amplify one proposition;
- correlated evidence channels and correlated multiplicative inhibitions;
- exact duplicate transitions and many paraphrases from one dependency group;
- compatible co-outcomes incorrectly forced into one exclusive family;
- mutually exclusive outcomes incorrectly placed into separate families;
- a dominant known outcome with material unknown or omitted support;
- zero known support with large unknown support;
- one eligible transition versus many weak dependent transitions;
- one high-coverage distant transition plus one low-coverage nearby transition,
  which must not jointly pass coverage and proximity;
- a bare or unavailable reliability value defaulted to a neutral scalar;
- equal reliability numbers under incompatible schemas or calibration domains,
  including an absent or unregistered migration;
- an unavailable reliability migration that fabricates a derivation,
  calibration domain, or numeric value instead of preserving state-typed
  source metadata;
- an observed action condition rendered as a recommended action;
- relative support rendered as probability or confidence;
- a later observation from a different horizon or condition treated as direct
  contradiction;
- downstream agent output fed back as if it were an independent observation;
- an exact name, path, time, or number lost by numerical compression;
- an embedding collision or misleading nearest neighbor;
- a deleted record retained by a derived index;
- validity boundaries immediately before, at, and after a timestamp;
- empty attention as the correct result;
- a budget just below the faithful minimum;
- redundant items crowding mandatory content out of the budget;
- an approximate-index miss;
- concurrent correction during compilation;
- authorization revocation during compilation;
- unsupported or ambiguous output language;
- attention that subtly answers the user request;
- attention that promotes an expectation to fact, silently hides an
  alternative, drops a horizon, or suggests the next action;
- validator false acceptance and false rejection;
- average improvement with harm concentrated in one declared subgroup; and
- equal or tiny activation margins that flip under permitted perturbation.

### Decision and stop gates

The proof program proceeds in risk order:

| Gate | Required result | Failure action |
| --- | --- | --- |
| G0: Contract | Product, architecture, and proof documents are internally consistent | Resolve contracts before implementation |
| G1: Product headroom | Expert focus beats prompt-only and situation-only, and correctly qualified expert focus-plus-expectation beats token-matched focus-only by predeclared meaningful margins without failing harm gates | Reject, narrow, or defer the premise before architecture implementation |
| G2: Evidence harness | Formal obligations are reviewed; one versioned manifest, receipt, split, lineage, baseline, and analysis harness can represent every required condition and preserve failed or inconclusive results | Do not select implementation technologies |
| G3: Predictive semantics | The deterministic expectation baseline passes transition-schema, dependency-budget, alternative, abstention, observation-assessment, and non-probability contracts on curated and adversarial evidence | Correct or simplify predictive semantics before renderer or persistence integration |
| G4: Renderer feasibility | A deterministic renderer or registered numerical bridge plus the smallest passing local checkpoint faithfully renders frozen expert focus-and-expectation plans and exact slots within local budgets | Replace or constrain the bridge, model, or rendering contract; do not weaken a failed gate |
| G5: Memory read and snapshots | Supplied revisions, authorization views, pinned indexes, concurrent publication, and compile/management separation satisfy their contracts | Do not build persistent-memory retrieval |
| G6: Retrieval | Required-proposition and eligible-transition recall plus cross-context behavior beat frozen simple baselines | Replace or simplify retrieval |
| G7: Activation and planning | Fixed-intermediate comparisons show value for signal derivation, activation, focus construction, expectation derivation, and combined closure selection over their strongest simpler baselines | Do not calibrate or integrate a mechanism without added value |
| G8: Vertical slice | All critical invariants, offline boundaries, and resource budgets hold in one local end-to-end integration | Do not build release packaging |
| G9: Sealed evaluation | Every superiority, non-inferiority, anchoring, harm, critical, subgroup, and operational gate passes | Report failure or inconclusive evidence; do not relabel it |
| G10: Shipment | Reproducible artifacts, authenticated manifests, migrations, backup/restore, rollback, licenses, SBOM, platform CI, installation, and evidence receipts pass one release-candidate rehearsal | Do not publish or broaden the supported claim |

Stop or redirect work when:

- expert attention fails to establish product headroom;
- correct expert expectation adds no value over token-matched focus-only or the
  wrong-expectation control is not detectably harmful;
- situation-only input explains the complete benefit;
- semantic top-k or a simpler deterministic rule is non-inferior at lower
  complexity;
- selection fails while expert selection succeeds;
- rendering fails even with expert selection;
- independent annotation cannot establish sufficiently stable labels;
- any critical authorization, authority, prompt-mutation, memory-mutation,
  partial-success, or network-egress failure is observed;
- statistical power is insufficient;
- reference-hardware budgets cannot be met; or
- sealed evidence fails a frozen gate.

An underpowered or conflicting result is `Inconclusive`, not success.

### Documentation and evidence receipts

Every experiment records:

- requirement, hypothesis, and configuration identifiers;
- source and dataset revision hashes;
- transition-reliability schema, target state or missingness code,
  compatibility-policy identity, applicable target derivation and
  calibration-domain identities, and migration identity plus state-typed
  source metadata when present for each predictive case;
- semantic-lineage and split identifiers;
- sampling-frame identity, inclusion probability, and design weight per case;
- `t_context`, `t_auth`, memory revision, policy revision, and authorized-view
  identity per scenario;
- implementation commit;
- dependency, model, tokenizer, encoder, and index versions;
- prompt templates and decoding settings;
- seeds and execution order;
- hardware and operating-system identity;
- resource and attention budgets;
- raw per-case observations;
- exclusions, timeouts, and failures;
- analysis version; and
- computed metrics with confidence intervals.

Receipts are evidence artifacts, not decision records. A later decision cites
the frozen receipt when adopting a component or claim.

## Preconditions

- The V1 product contract defines the observable claim.
- Each tested component has a versioned contract and configuration.
- Reference and baseline conditions obey identical downstream placement and
  effective budgets.
- Evaluation authors can separate semantic lineages across development,
  calibration, and sealed evidence.
- The frozen target population contains adequately powered context-dependent
  and context-independent strata for every claim-bearing subgroup.
- Transition fixtures distinguish mutually exclusive alternative families
  from compatible co-outcomes, retain known dependency lineages, and bind
  every reliability state to its versioned schema and compatibility policy,
  every derived state to its derivation and calibration domain, and every
  migrated state to state-typed source metadata, its exact source revision and
  digest, and its migration lineage.
- Every focus-only, focus-plus-renderable-abstention, expectation-only,
  combined, and wrong-expectation comparison reuses its frozen shared inputs
  exactly as declared.
- Primary labels and thresholds are frozen before candidate outputs are seen.
- Private user memory is never used for training or evaluation without a
  separate consent and data-governance contract.

## Invariants

- Formal, executable, empirical, and operational evidence remain distinct.
- No development, calibration, or exposed held-out case becomes sealed
  evidence by renaming it.
- Every sealed attempt remains in the permanent claim history.
- Aggregate metrics derive from retained per-case observations.
- Component comparisons reuse frozen upstream and downstream intermediates.
- Focus and expectation comparisons consume one shared activated-set identity;
  plan and renderer comparisons consume one frozen upstream semantic result.
- A simpler baseline is not omitted because it performs well.
- Failed, timed-out, and harmed cases remain visible.
- No metric treats activation as probability, truth, or safety.
- No metric treats relative support as calibrated probability or observation
  assessment as automatic learning.
- Claims are limited to the frozen domain, languages, models, tasks, hardware,
  and configuration evaluated.
- One failed mandatory gate prevents a supported release claim.

## Edge cases

- A mathematical proof with false modeling assumptions is not product evidence.
- A property test can discover a counterexample but cannot prove semantic
  correctness for all inputs.
- An approximate retriever can satisfy no-hard-partition rules while still
  missing relevant memory.
- Complete provenance can support a false memory; support is not truth.
- A faithful renderer can express a badly selected plan.
- A well-supported expectation can still be false, and a later contradiction
  does not retroactively make its earlier support computation invalid.
- A condition recorded with an action does not establish causality or authorize
  the renderer to recommend that action.
- Separate normalization of compatible outcome families is required; forcing
  them into one denominator creates a false tradeoff.
- A useful planner can be obscured by a poor renderer.
- An average improvement can coexist with an unacceptable harm subgroup.
- Exact agreement on one floating-point platform does not imply bit-identical
  cross-platform behavior.
- Repeated generations from one task do not increase the number of independent
  task observations.
- Zero observed critical failures does not prove a zero failure rate.

## Verification

Before architecture implementation:

- review every formal obligation and its assumptions;
- implement the experiment manifest and receipt schema;
- run the G1 expert-headroom experiment;
- demonstrate that the harness can execute every baseline under equal
  placement and budget; and
- freeze the decision criteria for advancing to component implementation.

For the numerical-memory and renderer path, G2 also requires executable
fixtures that preserve:

- typed facet-space incompatibility and missing-value semantics;
- authorization before candidate generation;
- historical validity without revival of current instruction authority;
- provenance-root duplicate suppression and unresolved conflict;
- transition kind, condition, horizon, observation status, dependency budget,
  typed reliability and every source/target migration pair, exact rollback,
  alternative-family, unknown-support, jointly qualifying coverage/proximity,
  and abstention semantics;
- separate focus and expectation projections from one shared activated-set
  identity;
- exact-value slot identity and deterministic surface substitution;
- plan-role distinctions such as dominant versus secondary context,
  hypothesis versus fact, and condition versus selected action; and
- explicit separation between lexicalized attention, an answer, an action
  plan, a probability claim, and a claimed chain of thought.

Before any component is accepted:

- execute its required counterexamples;
- compare it with the strongest simpler baseline;
- retain raw observations and configuration identity;
- verify reproducibility on its declared environment; and
- record the evidence and rejected alternatives in a focused decision record.

Before `Validated` product status:

- pass the memory-read and snapshot gate;
- complete the sealed evaluation protocol;
- pass every predeclared statistical, critical, and operational gate;
- pass one reproducible G10 shipment rehearsal with rollback and recovery;
- publish an inspectable evidence receipt;
- verify product-contract boundary tests on every supported platform; and
- state every unsupported domain, language, model, and hardware class.

## Open questions

- Primary endpoint, required baseline comparisons, and minimum meaningful
  effects.
- Baseline- and subgroup-specific non-inferiority and maximum-harm margins.
- Critical failure classes, minimum exposures, and maximum rate bounds.
- Sample size, repository/task population, and clustering hierarchy.
- Sampling-frame construction, inclusion probabilities, and design weights.
- Supported languages, downstream models, and reference hardware.
- Attention and resource budgets.
- Transition schema, alternative-family registry, dependency-group policy,
  expectation materiality, coverage, effective-support, and abstention
  thresholds.
- Wrong-expectation anchoring metric and maximum tolerated harm.
- Whether expectation-only output enters the supported product claim after its
  complete-scope cases are evaluated.
- Annotation protocol and agreement threshold.
- Retrieval, renderer, and operational metric thresholds.
- Evidence-receipt serialization and long-term storage.

These values must be resolved before sealed evaluation. They must not be tuned
after the sealed outcomes are known.

## References

- [V1 product contract](v1-product-contract.md)
- [V1 reference architecture](v1-reference-architecture.md)
- [V1 delivery program](v1-delivery-program.md)
- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Activation parameter evaluation](activation-parameter-evaluation.md)
- [Curated activation evidence](curated-activation-evidence.md)
- [Cognitive memory activation and focus](cognitive-memory-activation-and-focus.md)
- [Predictive attention and expectation](predictive-attention-and-expectation.md)
- [Focus-and-expectation planning](focus-and-expectation-planning.md)
- [Vector-to-attention renderer](vector-to-attention-renderer.md)
- [Local renderer model qualification](local-renderer-model-qualification.md)
- [Superseded Decision 0011: Adopt a local read-only attention compiler for V1](../decisions/0011-adopt-local-read-only-attention-compiler-v1.md)
- [Superseded Decision 0012: Adopt numerical cognitive memory and focus compilation](../decisions/0012-adopt-numerical-cognitive-memory-and-focus-compilation.md)
- [Superseded Decision 0013: Adopt a vector-prefix local renderer qualification path](../decisions/0013-adopt-a-vector-prefix-local-renderer-qualification-path.md)
- [Decision 0014: Adopt memory-grounded predictive attention](../decisions/0014-adopt-memory-grounded-predictive-attention.md)
- [Decision 0015: Render qualified focus-and-expectation plans](../decisions/0015-render-qualified-focus-and-expectation-plans.md)
