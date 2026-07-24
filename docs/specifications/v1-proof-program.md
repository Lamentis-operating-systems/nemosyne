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
| \(\mathsf R\) | complete retained request | One intrinsically valid immutable `CompileRequest` containing \(P,S,\Xi\), declared language, budget ceiling, and every field of the registered request schema; call claims remain separately bound in \(\Gamma_A\) | [Reference architecture](v1-reference-architecture.md) |
| \(\rho_P,\rho_R\) | configuration-independent identities | Prompt-content and complete request-presentation identities derived before authentication from \(\mathsf R\) | [Reference architecture](v1-reference-architecture.md) |
| \(\Gamma_P\) | untrusted presentation | Prompt-origin presentation and bounded compile claims supplied at the public boundary | [Reference architecture](v1-reference-architecture.md) |
| \(\Gamma_A\) | authenticated call binding | Request-local proof binding the exact presentation, claims, \(\rho_P\), \(\rho_R\), and \(\mathsf R\); not a configuration or disclosure grant | [Reference architecture](v1-reference-architecture.md) |
| \(T_{\mathrm{boot}}\) | private trust inputs | Compiler-owned bootstrap trust, platform handles, and trusted clock | [Reference architecture](v1-reference-architecture.md) |
| \(P_A\) | private authenticated prompt | Request-local proof binding exact \(P\), \(\rho_P\), \(\rho_R\), and the retained \(\mathsf R\) pairing | [Reference architecture](v1-reference-architecture.md) |
| \(\mathcal I_A\) | sealed authenticated invocation | One private request-local aggregate containing inseparable \(P_A,\Gamma_A,I,t_{\mathrm{auth}}\) projections and one fresh non-semantic call brand \(b_A\); only the authenticator constructs it, and downstream stages consume the aggregate rather than independently supplied projections | [Reference architecture](v1-reference-architecture.md) |
| \(b_A\) | private authenticated-call brand | Fresh opaque generative capability identity allocated by the sole authenticator and sealed into \(\mathcal I_A\); equality is runtime instance membership, not byte, digest, or numeric equality, and the brand has no semantic, policy, or authority meaning | [Reference architecture](v1-reference-architecture.md) |
| \(I\) | authenticated context | Trusted invocation principal, caller, and authorization facts | [Reference architecture](v1-reference-architecture.md) |
| \(t_{\mathrm{context}}\) | exact contextual instant | Caller-declared situational time | [Product contract](v1-product-contract.md) |
| \(t_{\mathrm{auth}}\) | exact trusted instant | One pinned authorization time | [Reference architecture](v1-reference-architecture.md) |
| \(p_{\mathrm{policy}}\) | immutable policy revision | Exact policy revision selected by authenticated control resolution | [Reference architecture](v1-reference-architecture.md) |
| \(\mathcal D_{\mathrm{eff}}\) | immutable disclosure ceiling | Equal-or-narrower effective disclosure ceiling selected by authenticated control resolution | [Reference architecture](v1-reference-architecture.md) |
| \(\Sigma_{\mathrm{sig}}\) | minimized trusted signal context | Private immutable tuple of pinned signal-context and social-subject identity-schema identities, an opaque reference to the authenticated invocation's generative call-instance brand, \(t_{\mathrm{auth}}\), and typed authenticated social-subject identity; the brand validates instance membership only, and the object carries no policy, authorization, disclosure, store, or ambient capability | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(V_{\mathrm{sig}}\) | validated signal values | Typed \((t_{\mathrm{auth}},u_{\mathrm{auth}})\) projection produced only after the carried brand, copied trusted values, context schema, and social-identity schema in \(\Sigma_{\mathrm{sig}}\) are validated against the independently supplied current \(\mathcal I_A\); all private membership evidence is then erased | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(M^r\) | immutable logical revision | Authoritative memory revision \(r\) | [Reference architecture](v1-reference-architecture.md) |
| \(M_A^{r,p_{\mathrm{policy}},t_{\mathrm{auth}},I}\) | authorized view | Policy- and invocation-scoped memory view | [Reference architecture](v1-reference-architecture.md) |
| \(\mathcal M_E\) | finite eligible view | Authorized records passing hard integrity and eligibility gates | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\mathcal M_Q\) | finite usage-compatible view | Eligible records admitted for this request use | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(m_i\) | cognitive memory unit | One immutable exact-plus-numerical record version | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\mathcal G_i\) | finite relation set | Typed numerical memory relations for \(m_i\) | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\Pi_i\) | typed metadata | Provenance, authority, validity, uncertainty, and policy of \(m_i\) | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(Q_{\mathrm{num}}\) | pure numerical situation | Encoding of \(P,S,\Xi\) under pinned \(K\); no request-control or trusted-authorization input | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(Q\) | sealed `BoundQuery` | Compiler-private aggregate \(\operatorname{bindQuery}(\mathsf R,\widehat B_{\mathrm{in}};K)\) whose sole constructor derives private read-only \(Q_{\mathrm{num}}\) and \(B_Q\) projections plus one canonical `BoundQueryContentId`; no downstream boundary accepts or reconstructs the projections independently | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(\widehat B_{\mathrm{in}}\) | sealed ingress binding | Compiler-owned typed request, situation, identity-schema, and authenticated configuration identities derived once from exact canonical request content | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(B_Q\) | exact query binding | \((request\_id,situation\_id,configuration\_id)\), independently projected from \(\widehat B_{\mathrm{in}}\) into \(Q\); the request and situation fields are complete typed content identities, not caller labels or bare digests | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
| \(B_A\) | exact shared-set binding | Independent request, situation, and configuration projection of \(\widehat B_{\mathrm{in}}\) supplied to shared-set construction | [Cognitive memory specification](cognitive-memory-activation-and-focus.md) |
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
| \(\omega_A\) | private invocation-instance witness | Nonserializable request-local reference to the current \(\mathcal I_A\)'s generative brand, carried outside \(\Lambda_A\) by the shared set, both branch outputs, and the combined plan; it proves call membership but has no semantic, ordering, numerical, diagnostic, or product-byte effect | [Predictive-attention specification](predictive-attention-and-expectation.md) |
| \(\sigma_A\) | private eligible-set-instance witness | Fresh nonserializable identity minted for the exact `EligibleActivatedMemorySet` object, carried outside \(\Lambda_A\) by that set and both branch outputs, independently anchored in the planning scope, and erased after the join; it distinguishes two same-content set constructions inside one invocation without semantic or product effect | [Predictive-attention specification](predictive-attention-and-expectation.md) |
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
| \(\mathcal V_{\mathrm{plan}}\) | finite exact-surface inventory | Exact values already bound and permitted by the focus and expectation branches | [Planning specification](focus-and-expectation-planning.md) |
| \(\mathcal O_{\mathrm{slot}}\) | finite semantic-owner set | Value-, lineage-, and request-instance-independent exact-slot owners; item-owned values derive from `PlanItemSemanticKey` plus owner role, while explicitly shared meanings use `SharedExactSlotMeaningKey` | [Planning specification](focus-and-expectation-planning.md) |
| \(\mathcal I_{\mathrm{plan}}\) | permissionless planning input | Output language, budget, planning configuration, and permitted exact-surface inventory; no raw request or \(Q\) | [Planning specification](focus-and-expectation-planning.md) |
| \(\mathcal S_{\mathrm{plan}}\) | private current-call and expected-set planning scope | Nonserializable compiler-private borrow of the current \(\mathcal I_A\)'s invocation witness and the exact selected \(\mathcal A\)'s set-instance witness; it validates both branch witness pairs and has no semantic, ordering, numerical, diagnostic, serialization, or product-byte effect | [Planning specification](focus-and-expectation-planning.md) |
| \(L\) | canonical plan | Sole `FocusExpectationPlan` | [Planning specification](focus-and-expectation-planning.md) |
| \(\beta_L\) | exact canonical plan bytes | Canonical bytes of `PlanCanonicalEnvelopeV1(L)`; exact equality, not digest equality or Rust referent identity, defines canonical plan equivalence | [Planning specification](focus-and-expectation-planning.md#canonical-plan-content-identity) |
| \(c_L\) | sealed `PlanContentId` | Domain-separated typed digest identity of \(\beta_L\); equality is necessary but insufficient for canonical equivalence until the exact-byte collision check succeeds | [Planning specification](focus-and-expectation-planning.md#canonical-plan-content-identity) |
| \(c_R\) | sealed `RendererConfigurationId` | Domain-separated typed identity recomputed from the exact authenticated renderer configuration \(K_R\), independent from \(c_L\) | [Renderer specification](vector-to-attention-renderer.md#rendering-boundary) |
| \(\beta_R\) | exact canonical renderer-configuration bytes | Canonical bytes \(\operatorname{CE}_{v1}(K_R)\); exact authenticated byte equality, not Rust referent identity or ID equality alone, defines renderer-configuration equivalence | [Renderer specification](vector-to-attention-renderer.md#rendering-boundary) |
| \(\rho_i\) | renderer item | One canonical renderer projection item | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal F_R\) | finite facet set | Versioned renderer-facet vocabulary | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathbf n_i,\varphi_i,\kappa_i,\mathbf m_i,\iota_i,\delta_i\) | typed renderer fields | Scalar features, item role, canonical rank, presence mask, proposition identity, and disposition | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal R_i,\mathcal U_i,\mathcal B_i^{\mathrm{src}},\mathcal X_i^{\mathrm{slot}}\) | typed finite renderer fields | Required relations, authority ceiling, source bindings, and exact-slot bindings | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal V_{\mathrm{exact}},\mathcal V_{\mathrm{slotmeta}}\) | exact sidecar and safe projection | Authoritative exact values and model-visible metadata without payload bytes | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal P_{\mathrm{soft}}\) | continuous prefix | Learned model-input projection of the canonical plan | [Renderer specification](vector-to-attention-renderer.md) |
| \(\mathcal B_{\mathrm{attr}}\) | finite attribution-label set | Nonempty token-to-proposition training labels | [Renderer specification](vector-to-attention-renderer.md) |
| \(V_{\mathrm{ctx}}\) | opaque `ValidationContext<'plan>` | Compiler-built, witness-erased context tied to its source-plan borrow and containing exact prompt evidence, the semantic numerical-query projection, sealed \(c_L\), private \(\beta_L\) comparison capsule, sealed \(c_R\), private \(\beta_R\) commitment, validator-only projections, and resolved limits; the lifetime prevents outliving or unchecked detachment but does not encode referent identity | [Renderer specification](vector-to-attention-renderer.md) |
| \(V_{\mathrm{view}}\) | logical least-privilege `ValidationView<'plan>` | Read-only projection \(\pi_{\mathrm{val}}(V_{\mathrm{ctx}})\) supplied to the independent validator; it exposes only the bounded semantic, exact-value, identity, and control fields required for validation and never the backing compiler-private context, raw plan, witness, or private comparison capsule | [Renderer specification](vector-to-attention-renderer.md) |
| \(Z_{\mathrm{slot}}\) | opaque `RenderedAttention<'plan>` | Pre-substitution text, complete segmentation, untrusted semantic bindings, claimed exact-slot occurrences, origins, private sealed \(c_L\), private \(\beta_L\) comparison capsule, sealed \(c_R\), and private \(\beta_R\) commitment; its lifetime is tied to the source-plan borrow without asserting referent identity, and no independent identity input exists | [Renderer specification](vector-to-attention-renderer.md) |
| \(Z_{\mathrm{exact}}\) | opaque `SubstitutedAttention<'plan>` | Exact-slot-substituted render value preserving the candidate lifetime, sealed \(c_L\), private \(\beta_L\) comparison capsule, sealed \(c_R\), and private \(\beta_R\) commitment; substitution permits separately borrowed plan/configuration values only when exact canonical contents match and rejects every mismatch | [Renderer specification](vector-to-attention-renderer.md) |
| \(T'\) | attention bytes | Validated attention text | [Renderer specification](vector-to-attention-renderer.md) |
| \(O\) | compiled bytes | Exact successful product output | [Product contract](v1-product-contract.md) |
| \(K\) | immutable artifact identity | Pinned compiler configuration and execution envelope | [Reference architecture](v1-reference-architecture.md) |
| \(K_S\) | immutable plan-semantic configuration | Authenticated canonical projection of \(K\) containing every plan-semantic field and excluding \(K_R\), renderer/validator execution, serializer/transport fields, and full configuration identity; yields `SemanticConfigurationId` | [Planning specification](focus-and-expectation-planning.md#canonical-plan-content-identity) |
| \(\Theta_{\mathrm{call}}\) | resolved call controls | Pinned \(K\), output language, effective budget, \(p_{\mathrm{policy}}\), and \(\mathcal D_{\mathrm{eff}}\) | [Reference architecture](v1-reference-architecture.md) |
| \(K_R\) | immutable renderer identity | Exact renderer configuration bound inside \(K\) | [Renderer specification](vector-to-attention-renderer.md) |

The derivation path below is the sole cross-stage composition. Focused
specifications may refine one stage, but no document may bypass a stage, invent
an alternate authoritative intermediate, or redefine another stage's formula.

### Formal compile model

Let:

- \(\mathsf R\) be one intrinsically validated, retained, immutable complete
  `CompileRequest`; its exact components are `P`, `S`, \(\Xi\), optional
  declared output language, optional attention-budget ceiling, and every
  field of the registered request schema;
- \(\Gamma_P\) be the untrusted prompt-origin presentation and bounded compile
  claims, including any requested installed configuration and equal-or-narrower
  disclosure ceiling;
- \(T_{\mathrm{boot}}\) be compiler-owned bootstrap trust, platform handles,
  authenticated registries, and trusted-clock inputs that no caller can
  construct;
- \(\mathcal I_A\) be the sealed request-local `AuthenticatedInvocation`
  produced only by successful prompt-origin authentication, with private
  inseparable projections \(P_A,\Gamma_A,I,t_{\mathrm{auth}}\) and one fresh
  non-semantic call brand \(b_A\);
- \(\Theta_{\mathrm{call}}=(K,\ell,B,p_{\mathrm{policy}},\mathcal D_{\mathrm{eff}})\) be the
  authenticated, resolved, and pinned configuration, output language,
  effective attention budget, policy revision, and equal-or-narrower
  disclosure ceiling;
- `M^r` be immutable memory revision `r`; and
- \(K_R\) be the exact renderer configuration bound inside \(K\). A
  V1-deployable \(K\) permits no request-time random input.

The proposed logical stages are:

\[
(\rho_P,\rho_R) =
derivePresentationIdentities(\mathsf R;T_{\mathrm{boot}})
\]

\[
\mathcal I_A =
\operatorname{authenticateOrigin}(
\mathsf R,\rho_P,\rho_R,\Gamma_P;
T_{\mathrm{boot}}
)
\]

\[
\Theta_{\mathrm{call}}
=
(K,\ell,B,p_{\mathrm{policy}},\mathcal D_{\mathrm{eff}})
=
resolveAndPinControls(
\mathsf R,\mathcal I_A;
T_{\mathrm{boot}}
)
\]

\[
\widehat B_{\mathrm{in}} =
\operatorname{constructIngress}(\mathsf R,\mathcal I_A;K)
\]

\[
B_Q=\pi_Q(\widehat B_{\mathrm{in}}),
\qquad
B_A=\pi_A(\widehat B_{\mathrm{in}})
\]

\[
\Sigma_{\mathrm{sig}} =
\operatorname{projectSignalContext}
(\mathcal I_A;K)
\]

\[
V_{\mathrm{sig}}=
\operatorname{validateSignalContext}
(\mathcal I_A,\Sigma_{\mathrm{sig}};K)
\]

\[
M_A^{r,p_{\mathrm{policy}},t_{\mathrm{auth}},I} =
authorize(
M^r,\mathcal I_A,p_{\mathrm{policy}},\mathcal D_{\mathrm{eff}};
K
)
\]

\[
Q_{\mathrm{num}} =
encode(
prompt(\mathsf R),
situations(\mathsf R),
evidence(\mathsf R);
K
)
\]

\[
Q = bindQuery(\mathsf R,\widehat B_{\mathrm{in}};K),
\qquad
\operatorname{numerical}(Q)=Q_{\mathrm{num}},
\qquad
\operatorname{binding}(Q)=B_Q
\]

\[
\mathcal M_E =
eligibilityGate(M_A^{r,p_{\mathrm{policy}},t_{\mathrm{auth}},I};K)
\]

\[
\mathcal M_Q =
usageGate(Q,\mathcal M_E,\mathcal I_A;K)
\]

\[
C^{r} = retrieve(Q,\mathcal M_Q;K)
\]

\[
N = derive(Q,C^{r},V_{\mathrm{sig}};K)
\]

\[
\mathcal A =
sealActivatedSet(activate(N;K),B_A,\mathcal I_A;K),
\qquad
\pi_{request,situation,configuration}
\left(\Lambda_A(\mathcal A)\right)=B_A,
\qquad
\operatorname{belongsTo}(\omega_A(\mathcal A),\mathcal I_A),
\qquad
\sigma_A(\mathcal A)=\operatorname{freshSetWitness}(\mathcal A)
\]

`belongsTo` is the private runtime membership predicate owned by `API-01`.
It compares a nonserializable witness with the generative brand sealed in the
independently supplied current aggregate. It is not byte equality, a digest,
or a semantic relation.

\[
F = focusCandidates(Q,\mathcal A;K)
\]

\[
E = expectations(Q,\mathcal A;K)
\]

\[
\mathcal V_{\mathrm{plan}} =
projectExactSurfaces(
exactBindings(F)\cup exactBindings(E),
exactSidecars(Q,\mathcal A);
K
)
\]

\[
\mathcal I_{\mathrm{plan}} =
planningInput(\ell,B,\mathcal V_{\mathrm{plan}};K)
\]

\[
\mathcal S_{\mathrm{plan}} =
planningScope(\mathcal I_A,\mathcal A),
\qquad
L = plan(\mathcal S_{\mathrm{plan}},\mathcal I_{\mathrm{plan}},F,E;K)
\]

The scope independently anchors both \(\omega_A(\mathcal A)\) and
\(\sigma_A(\mathcal A)\). It accepts \(F,E\) only when both branch outputs
propagate both exact witnesses. Two reconstructed same-content sets inside one
invocation share the first witness but not the second and therefore cannot
join.

\[
V_{\mathrm{ctx}}=
buildValidationContext(
\mathsf R,\mathcal I_A,Q,L;
\Theta_{\mathrm{call}}
)
\]

\[
Z_{\mathrm{slot}}=
\operatorname{render}_{K_R}(L)
\]

\[
c_L=\operatorname{PlanContentId}(L),
\qquad
c_R=\operatorname{RendererConfigurationId}(K_R),
\qquad
\beta_R=\operatorname{CE}_{v1}(K_R),
\qquad
\beta_L=\operatorname{canonicalBytes}
\left(\operatorname{PlanCanonicalEnvelopeV1}(L)\right)
\]

\[
L'\equiv_{\mathrm{can}}L
\iff
\beta_{L'}=\beta_L,
\qquad
L'\equiv_{\mathrm{can}}L
\Longrightarrow
\operatorname{PlanContentId}(L')=c_L.
\]

Equal `PlanContentId` values alone do not establish
\(L'\equiv_{\mathrm{can}}L\). When two boundaries observe equal identities,
they compare the retained exact \(\beta_{L'}\) and \(\beta_L\); unequal bytes
are `PlanContentIdentityCollision`, quarantine the plan/configuration path,
and cannot proceed as equivalent input.

\[
Z_{\mathrm{exact}} =
substitute(Z_{\mathrm{slot}},exactSlots(L),K_R)
\]

\[
V_{\mathrm{view}}=\pi_{\mathrm{val}}(V_{\mathrm{ctx}})
\]

\[
T' =
validate(Z_{\mathrm{exact}},V_{\mathrm{view}},K_R)
\]

\[
O=serialize_{\mathrm{product}}(T',promptBytes(\mathsf R)).
\]

The displayed successful path substitutes from \(L\). The substitution
contract also accepts any separately constructed \(L'\equiv_{\mathrm{can}}L\).
The Rust lifetimes of \(Z_{\mathrm{slot}}\), \(Z_{\mathrm{exact}}\), and
\(V_{\mathrm{ctx}}\) prevent outliving their source borrows and unchecked
detachment; they do not prove that two borrows refer to the same object.
Candidate-to-context binding is enforced by the complete
\((c_L,\beta_L,c_R,\beta_R)\) state. Substitution with
exact-canonical-byte-identical \(L'\) under authenticated renderer
configuration with the same \(c_R,\beta_R\), followed by validation against a
context carrying that same state, must produce the same substitution,
validation, and product bytes as \(L\). Different valid plan content fails
`PlanIdentityMismatch`; a renderer-configuration ID or canonical-byte
difference, including equal \(c_R\) with unequal \(\beta_R\), fails
`RendererConfigurationMismatch`; and equal plan identity with different
canonical bytes fails `PlanContentIdentityCollision` before candidate content
is interpreted.

`serialize_product` invokes the sole normative successful-output contract in
the [product specification](v1-product-contract.md#successful-output). This
proof program does not duplicate its headers, separators, or byte-concatenation
formula. Before `authenticateOrigin` succeeds, the compiler may only perform
intrinsic request validation, immutable retention, cancellation checks, and
the configuration-independent canonical derivation of \(\rho_P\) and
\(\rho_R\) required by authentication. It may not perform semantic encoding,
memory access, retrieval, candidate derivation, planning, rendering, or
validation. After authentication, every use of exact request content validates
the immutable \((\mathsf R,\mathcal I_A)\) binding; final serialization reads
the byte-identical prompt from that same retained \(\mathsf R\).
`derivePresentationIdentities` uses the authenticated installation's
configuration-independent presentation schema pinned at open; it is a pure
canonical identity derivation and grants no authority. `authenticateOrigin`
checks the exact presentation, claims, \(\rho_P\), \(\rho_R\), complete
\(\mathsf R\), freshness, and compiler-owned platform evidence together. Its
only successful return is one sealed \(\mathcal I_A\); the private constructor
allocates \(b_A\), stores \(P_A,\Gamma_A,I,t_{\mathrm{auth}}\) inseparably,
and exposes only narrow borrowed projections whose lifetime is tied to the
aggregate. The brand is a compiler-private generative capability identity:
equality is object-instance membership, not serialized bytes, a digest, random
model input, or a numerical feature. Its allocation may differ across
otherwise identical calls without affecting any semantic stage or product
byte. No downstream stage accepts those projections as an independently
constructible tuple.
`resolveAndPinControls` accepts only the resulting \(\mathcal I_A\), not raw
claims or separable authentication fields, and is the sole producer of
\(K,\ell,B,p_{\mathrm{policy}},\mathcal D_{\mathrm{eff}}\).
`constructIngress` revalidates the \(\mathcal I_A\)-to-\(\mathsf R\) pairing
and derives
the configuration-bound complete-request envelope from all fields of
\(\mathsf R\); omitting language, budget, or any registered request field is an
invalid ingress construction. Requested configuration and disclosure are
call claims bound separately by \(\Gamma_A\) and resolved only by
`resolveAndPinControls`; they are not silently reclassified as request fields.
`projectSignalContext` is the sole producer of
\(\Sigma_{\mathrm{sig}}\). It accepts the complete sealed
\(\mathcal I_A\), places one opaque reference to \(b_A\) in the context, and
copies only `t_auth`, the pinned social-identity schema, and the typed
authenticated social-subject identity from that same aggregate and
authenticated registry under the pinned schemas in `K`. Any identity-schema
rotation requires the registered authenticated one-to-one migration artifact
before projection. It cannot accept or assemble independently supplied
\(\Gamma_A\), `I`, or `t_auth`. `validateSignalContext` is the sole producer
of \(V_{\mathrm{sig}}\): it receives the current sealed \(\mathcal I_A\)
independently, checks the carried brand, both schemas, copied trusted time, and
registry-derived social subject against that aggregate, and then erases all
membership evidence from signal mathematics. A complete context from another
valid call therefore cannot validate itself. The context supplies no
policy, authorization, disclosure, store, or ambient-clock capability and has
no caller-constructible fallback.

All post-ingress matching, eligibility-use, retrieval, signal, focus, and
expectation boundaries receive the sealed \(Q\), never independently supplied
\(Q_{\mathrm{num}}\) and \(B_Q\). Their semantic arithmetic borrows only
\(\operatorname{numerical}(Q)=Q_{\mathrm{num}}\); deterministic
content/configuration joins borrow only
\(\operatorname{binding}(Q)=B_Q\). Neither projection can be replaced,
recombined, authenticated, or retained as a standalone boundary input, and
\(B_Q\) remains neither semantic evidence nor runtime call identity. Define
\(\operatorname{sem}_F\) and
\(\operatorname{sem}_E\) as typed lineage-erasing projections over
`FocusCandidateSet` and `ExpectationBundle`. They remove exact request
bindings and source receipts and replace every transitively lineage-derived
request-instance, abstention-instance, explanation, and slot-binding identity
with its registered stable semantic key. They retain proposition and
hypothesis meaning, exact-surface content identity, score, qualification,
condition, horizon, alternative relation, abstention reason, and disposition.
They do not compare raw request-local IDs whose construction intentionally
includes \(\Lambda_A\).

For canonical plan identity, define
\(\operatorname{source}_{\mathrm{plan}}\) as the complete
`PlanSemanticSourceProjectionV1`. It retains \(d_R,d_S\), all selected
lineage-independent query/memory/focus/expectation semantics, exact surfaces,
and `SemanticConfigurationId(K_S)`, while erasing \(b_R,b_S\), complete
configuration-bound request/situation IDs, \(B_Q\), `BoundQueryContentId`,
\(\Lambda_A\), branch instance IDs, receipts, full `configuration_id`, and
\(K_R\). Therefore:

\[
\begin{aligned}
&K_S'=K_S,\quad d_R'=d_R,\quad d_S'=d_S,\\
&\operatorname{source}_{\mathrm{plan}}'
=\operatorname{source}_{\mathrm{plan}},\quad
\operatorname{selectedSemantics}'=\operatorname{selectedSemantics},\\
&\operatorname{exactSurfaces}'=\operatorname{exactSurfaces}
\land(\ell',B')=(\ell,B)\\
&\Longrightarrow
\beta_L'=\beta_L\land c_L'=c_L,
\end{aligned}
\]

even if a \(K_R\)-only change alters full `configuration_id`, \(b_R,b_S\),
\(B_Q\), `BoundQueryContentId`, or \(\Lambda_A\). The same change must alter
\(c_R\) or \(\beta_R\) whenever canonical \(K_R\) changes. This is plan
identity noninterference, not permission to bypass the live query/lineage joins
within either invocation.

Consider two separately valid complete calls with the same retained
\(\mathsf R\), pinned semantic configuration, \(Q_{\mathrm{num}}\),
authorized semantic records, signal values, semantic request propositions,
exact-surface content, and resolved controls. Because \(B_Q\) is a
deterministic content/configuration identity, both calls have the same
\(B_Q\), and their separately constructed sealed \(Q\) and \(Q'\) have the
same canonical `BoundQueryContentId`. Their fresh authenticated-invocation brands and permitted
request-local instance or receipt identities may differ, so each call still
has its own \(\mathcal I_A,\omega_A,\mathcal A,\Lambda_A\). Let primed symbols
denote the second complete derivation:

\[
\operatorname{sem}_F
\left(
focusCandidates(Q,\mathcal A;K)
\right)
=
\operatorname{sem}_F
\left(
focusCandidates(Q',\mathcal A';K)
\right),
\]

\[
\operatorname{sem}_E
\left(
expectations(Q,\mathcal A;K)
\right)
=
\operatorname{sem}_E
\left(
expectations(Q',\mathcal A';K)
\right).
\]

Their lineage receipts differ only where their own complete valid derivations
permit; their semantics do not. Output language and attention budget are
separately resolved controls in \(\Theta_{\mathrm{call}}\), not authority
carried by \(B_Q\). The API cannot copy or replace only \(B_Q\) because no
post-ingress boundary accepts split query projections. Instead, the current
\(\mathcal I_A\)'s private generative brand independently anchors
\(\omega_A\), and each content join separately requires
\(\operatorname{binding}(Q)=\pi_Q(\Lambda_A)\). A different, malformed, or
recomputation-inconsistent sealed query, a nonmatching content projection, or
a complete \(Q'\) paired with a different request/shared-set lineage fails
closed. Substituting \(\omega_A'\) into the first shared-set aggregate is
unrepresentable; a complete foreign aggregate is rejected later by the
independently anchored planning scope. No valid
nonsemantic instance change can alter retrieval, activation, focus meaning, or
expectation support.

This noninterference is compositional. Let \(\eta\) be any consistent,
type-preserving correspondence between permitted nonsemantic
\(\Lambda_A\)-derived source-receipt and request-local instance identities in
those two valid complete fixtures, while canonical \(Q\), all stable semantic keys,
meanings, exact-surface content identities, scores, qualifications, resolved
\(\ell\), budget \(B\), and semantic configuration remain fixed.
`FocusCandidateOrderKey`,
`PlanningFocusCandidatePriorityKey`, `PlanItemSemanticKey`, and
`PlanningClosurePriorityKey` exclude \(\eta\)'s domain. With
\(\operatorname{sem}_L\) erasing plan receipts and instance IDs:

\[
\operatorname{sem}_L
\left(
plan(\mathcal S_{\mathrm{plan}},\mathcal I_{\mathrm{plan}},F,E;K)
\right)
=
\operatorname{sem}_L
\left(
plan(\mathcal S_{\mathrm{plan}}',\mathcal I_{\mathrm{plan}}',F',E';K)
\right).
\]

Each \(\mathcal S_{\mathrm{plan}}\) is borrowed independently from its call's
current sealed \(\mathcal I_A\). Planning requires both branch witnesses to
match that scope before it compares the branch receipts; equality between two
foreign branch witnesses is insufficient. The plan retains the scope's exact
witness, not a value reconstructed from \(B_Q\), \(\Lambda_A\), or either
branch.

After planning, each call independently constructs and validates
\(V_{\mathrm{ctx}}=buildValidationContext(\mathsf R,\mathcal I_A,
Q,L;\Theta_{\mathrm{call}})\). The builder reconstructs the expected sealed
query from the retained request and pinned configuration, compares the whole
canonical query envelope and `BoundQueryContentId`, requires
\(\operatorname{binding}(Q)=\pi_Q(\Lambda_A(L))\), validates the plan's
private \(\omega_A\) against the current \(\mathcal I_A\), and checks the
plan's configuration, policy, language, and budget against the resolved
controls. It then consumes and erases the witness while returning an opaque
`ValidationContext<'plan>` that borrows its source plan and retains only sealed
deterministic \(c_L\), private exact \(\beta_L\) comparison capsule, sealed
\(c_R\), and minimized semantic and validator projections. Its lifetime
prevents it from outliving that borrow or being detached unchecked; it does
not encode referent identity. The context has no witness accessor, identity
mutator, or independent identity input. The compiler projects
\(V_{\mathrm{view}}=\pi_{\mathrm{val}}(V_{\mathrm{ctx}})\); candidate
validation receives only the candidate, that least-privilege read-only view,
and authenticated \(K_R\), never the backing context, a second raw plan, or a
witness. Before semantic interpretation, the compiler and validator enforce
the complete
\((c_L,\beta_L,c_R)\) contract with their separately owned checks. A
separately constructed plan is interchangeable only when its exact
`PlanCanonicalEnvelopeV1` bytes and renderer configuration are identical; it
must then yield identical substitution, validation, and product bytes.
Different valid canonical bytes fail `PlanIdentityMismatch`, different
renderer configuration fails `RendererConfigurationMismatch`, and equal
`PlanContentId` with different bytes fails `PlanContentIdentityCollision`.
Each context builder still validates and consumes its own plan witness against
its current invocation. The compiler
owns the concrete context and is the sole product call site; the validator has
no compiler dependency, constructor, authority token, or path for a
caller-created view or accepted result to re-enter compilation. The
lexicalizer then consumes the same ordered semantic projection and exact
surface bytes in both cases, so validated attention bytes and complete product
bytes are identical; only privileged lineage receipts or non-content-bearing
diagnostics outside the product may differ. Required counterexamples use two
separately valid complete fixtures with equal-role, equal-activation optional
focus closures, a budget admitting exactly one, and corresponding
nonsemantic instance identities at fixed canonical \(Q\). Any change in selected meaning, semantic
order, attention bytes, or product bytes fails F3/F9/F10 even if
\(\operatorname{sem}_F\) and \(\operatorname{sem}_E\) individually pass.
\(\mathcal I_{\mathrm{plan}}\) contains no raw request, \(Q\), principal,
authorization view, policy handle, or ambient data source. All
request-derived meaning in `F` and every member of
\(\mathcal V_{\mathrm{plan}}\) must already be source-bound and permitted by
its owning upstream branch.
`projectExactSurfaces` takes only the canonical union of exact-slot bindings
declared by `F` and `E` plus immutable request/shared-set exact-sidecar
projections. It checks slot and surface content identities, rejects missing,
duplicate, inconsistent, or unreferenced surfaces, and emits the minimized
permissionless inventory in canonical slot order. It cannot query policy,
raise a ceiling, add a binding, or inspect ambient state.

Each branch supplies only a value-, lineage-, and request-independent
`ExactSlotOwnerSemanticDescriptor`. For each selected exact binding \(j\),
planning verifies that descriptor against the independently derived non-slot
meaning of its selected branch item and then derives the sole canonical owner
\[
o_j\in\mathcal O_{\mathrm{slot}}
=
\operatorname{ItemOwner}
(\operatorname{PlanItemSemanticKey}(j),role_j)
\uplus
\operatorname{SharedOwner}
(\operatorname{SharedExactSlotMeaningKey}(j)).
\]
The second branch is permitted only when the upstream contract explicitly
declares shared exact meaning. No branch may construct \(o_j\) directly, and
the mapping cannot inspect an exact value, surface content identity, lineage,
request identity, or witness. Let \(\ell_j^{\mathrm{slot}}\) be the
schema-owned `ExactSlotSemanticLocator`. Planning groups by
\((o_j,\ell_j^{\mathrm{slot}})\), and the canonical `SlotSemanticKey` includes
that pair together with type, semantic role, occurrence bounds, permitted
item-role bindings, schema identity, and formatter identity. Thus two
independent items may use the same schema/path/ordinal without collision, while
two incompatible exact values under the same owner-plus-locator are a typed
structural conflict rather than an arbitrary winner.

Holding all non-value slot semantics fixed while changing only the
authoritative exact value, its privileged content identity, and its formatted
bytes must preserve \(o_j\), \(\ell_j^{\mathrm{slot}}\),
`SlotSemanticKey`, `RendererSlotId`, canonical slot order, and every
pre-substitution tensor. Only privileged sidecars, deterministic substituted
bytes, measured post-substitution cost, validation outcome, and successful
product bytes may change. This is a schema-level noninterference obligation,
not permission to accept an invalid or over-budget replacement.

`sealActivatedSet` is the shared-set envelope construction owned by the
cognitive-memory and predictive-attention contracts. It preserves the
activation result and attaches the independently supplied \(B_A\) plus the
already established memory, policy, authorization-view, retrieval, and
configuration lineage; it neither reranks nor repeats authorization.

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
| `ALG-SER-01` | [Product output](v1-product-contract.md#successful-output) | Validated attention bytes plus retained prompt bytes to one compiled byte string; exact byte serialization; total after validation. File/stdin sources have already been streamed under `AbsoluteIngressLimitsV1`; already-owned API and direct-argument values have been checked before request construction or further internal allocation. An adapter buffers the complete serialized value before delivery | Prompt is the final byte-identical suffix. Test normalization, line endings, nested headers, trailing bytes, each exact ingress maximum and maximum plus one byte, ceiling-plus-one bounded reads for stream-backed sources, no further internal allocation after an oversized owned value is detected, zero stdout before delivery, and transport-prefix invalidation through exit `10` | `API-01`, `CLI-01`, `SYS-01` | F1, F9; G7 |
| `ALG-MEM-01` | [Numerical query state](cognitive-memory-activation-and-focus.md#numerical-query-state) | Authenticated pinned \(K\) plus the exact validated compile request produce one sealed \(\widehat B_{\mathrm{in}}\), pure \(Q_{\mathrm{num}}\), independently derived \(B_Q=(request,situation,configuration)\), and sealed \(Q=\operatorname{bindQuery}(\mathsf R,\widehat B_{\mathrm{in}};K)\) with private projections and canonical `BoundQueryContentId`; no post-ingress split constructor or field replacement exists; request/situation IDs are domain-separated typed content identities over injective canonical envelopes; typed failure; presence differs from numeric zero; exact prompt remains outside both query projections; no caller-owned ID, principal, trusted time, policy, or authorization-view input | Same numerical content/configuration produces bit-identical \(Q_{\mathrm{num}}\); same complete request/configuration produces the same canonical \(Q\); changed content/configuration separates subject to the digest-collision assumption; recomputation mismatch and observed collision evidence fail closed. Test prompt/situation/context/control mutations, numerical/binding noninterference, map permutation, cross-request whole-query swaps, compile-fail split-projection construction, defensive numerical-only and binding-only corruption, constant/reused/caller IDs, collision witnesses, configuration substitution, locators, content identities, and ambient/trusted-time noninterference | `SIT-01`, `ENC-01` | F2, F3, F10, F12; G3 |
| `ALG-MEM-09` | [Minimized trusted signal context](cognitive-memory-activation-and-focus.md#minimized-trusted-signal-context) | One sealed \(\mathcal I_A\) and preflighted \(K\) to one private \(\Sigma_{\mathrm{sig}}\), then validation of its carried brand, copied trusted values, context schema, and social-identity schema against the independently supplied current \(\mathcal I_A\) to typed \(V_{\mathrm{sig}}=(t_{\mathrm{auth}},u_{\mathrm{auth}})\); total through typed artifact or invariant failure; no public constructor, independently supplied authentication projection, serialization, digest reconstruction, or fallback | The aggregate constructor prevents mixed-\(\Gamma_A\), mixed-\(I\), and mixed-time assembly; a complete context from another call fails because membership is anchored to the current aggregate rather than a second value returned beside the context; the brand is erased before signal math; social source tags cannot alias and schema rotation requires an authenticated one-to-one migration; excluded authority/request fields are noninterfering; test every mixed-projection counterexample plus missing, malformed, duplicate, copied-value mutation, wrong-owner/lifetime, whole-context cross-call/schema, social-schema/rotation/migration, ambient-time, request-fallback, and renderer-exposure cases | `API-01`, `SIG-01`, `SEC-01` | F2, F3, F10, F12; G4, G8 |
| `ALG-MEM-02` | [Eligible memory view](cognitive-memory-activation-and-focus.md#eligible-memory-view) | Authorized snapshot plus one sealed \(\mathcal I_A\) to \(\mathcal M_E\), then sealed \(Q,\mathcal M_E,\mathcal I_A\) to \(\mathcal M_Q\); stages borrow only the numerical projection for semantic use and trusted fields only through their aggregates; total for valid policy artifacts; hard gates precede scores; no \(B_Q\)-dependent semantics | Excluded records cannot crowd candidates and mixed-\(I\)/mixed-time/split-query inputs are unrepresentable; mutate only unauthorized, deleted, invalid, usage-incompatible, and binding-only inputs; include forged internal-corruption counterexamples | `MEM-03`, `RET-01`, `SEC-01` | F2, F3, F12; G2-G3 |
| `ALG-MEM-03` | [Direct cue activation](cognitive-memory-activation-and-focus.md#direct-cue-activation) | Sealed \(Q\), compatible query-memory facets, and registered calibration artifacts to finite calibrated cues; cue arithmetic borrows only \(\operatorname{numerical}(Q)\); metric, calibration, missingness, and canonical accumulation are pinned; no binding or ambient input reaches the math | Inspectable bounded cue lineage; separately valid same-content queries are equal, while a malformed sealed query rejects before this stage; test incompatible spaces, nonfinite values, duplicated evidence, and attempted split-query construction or mutation | `SIG-01`, `ACT-00` | F8, F10; G4 |
| `ALG-MEM-04` | [Base availability](cognitive-memory-activation-and-focus.md#base-availability-from-frequency-and-recency) | Valid history statistics plus trusted \(t_{\mathrm{auth}}\) supplied only by validated \(V_{\mathrm{sig}}\) to bounded availability; checked log-sum-exp with finite ceilings and canonical accumulation; typed underflow/nonfinite failure; no ambient clock | Recency and frequency affect accessibility, not truth or authority; test extreme finite ages/decay, raw-power underflow counterexamples, future and duplicate events, context substitution, and ambient-clock perturbation | `SIG-01`, `EVAL-01` | F3, F4, F8, F10; G4 |
| `ALG-MEM-05` | [Signal derivation and kernel composition](cognitive-memory-activation-and-focus.md#signal-derivation-and-existing-kernel-composition) | Sealed \(Q\), eligible candidate facets, and source-tagged trusted fields from validated \(V_{\mathrm{sig}}\) to the separately owned finite temporal, spatial, goal, procedure, hazard, deadline, and two stable social derivations under one family budget; signal math borrows only \(\operatorname{numerical}(Q)\); missingness stays typed; deadline duration/scale are finitely bounded, division is checked, and one pinned exponential plus explicit zero-underflow boundary is total or returns `InvalidUrgencyArithmetic`; social identities use one pinned comparison schema and typed authenticated, declared, and memory-participant domains; no policy or authorization capability | Context affects relevance without overriding policy; test deadline-over-constraint, every finite deadline boundary, checked division, exact-zero underflow, invalid exponential output, authenticated-versus-declared social subjects, partner permutation/duplication/cardinality, social schema compatibility/rotation/migration, unknown partner IDs, family-budget failure, cross-call/schema substitution, split-query unrepresentability, social-as-authorization, and risk-as-probability errors | `SIG-01`, `EVAL-01`, `SEC-01` | F2, F3, F5, F8; G4 |
| `ALG-MEM-10` | [Direct spreading-seed derivation](cognitive-memory-activation-and-focus.md#direct-spreading-seed-derivation) | Complete typed direct-cue vectors and presence/lineage masks for the canonical unique graph-node set \(V_g\), in exactly the matrix node order, to \(a^{(0)}\); relation-only absence is explicit; all-absent and all-present-zero cues map to exact zero with no bias; otherwise normalize with an upward binary64 denominator over an exact binary-rational sum and require an exact post-division mass at most one; typed seed error; no relation or activation-kernel input | No circular activation dependency, duplicate/provenance amplification, hidden zero fill, positive-to-zero underflow, dimension/order drift, or mass above one; test all-absent versus all-present-zero, subunit/superunit mass, relation-only nodes, permutation, duplicate records/cues, overflow, underflow, the nearest-sum-downround counterexample, and normalization whose first division still exceeds one | `SIG-01`, `EVAL-01`, `PERF-01` | F2, F8, F10; G4 |
| `ALG-MEM-11` | [Canonical spreading-graph construction](cognitive-memory-activation-and-focus.md#canonical-spreading-graph-construction) | Finite unique canonical \(C^r\) plus a snapshot-bound raw-key-ordered relation iterator with explicit `limit`/`has_more`, usage-compatible typed relations, total relation ranks, and configured node/edge/hop ceilings to canonical \(C^r\subseteq V_g\subseteq\mathcal M_Q\), unique typed edges, and \(W\) in exactly the same node order as \(a^{(0)}\); breadth-by-depth expansion and validation/error precedence are canonical before semantic admission; exact power-of-two fixed-point weights and checked integer row budgets construct a row-substochastic matrix without rounded-sum acceptance, unbounded materialization, prefix truncation, or silent normalization | No input permutation can select a different error or admitted graph; no ineligible neighbor, duplicate edge, dimension drift, row mass above one, or over-limit prefix can enter propagation. Invalid limit/rank artifacts map to `ArtifactUnavailable`, node/edge/integer bounds to non-retryable same-input `ResourceFailure`, and remaining graph errors to `ActivationFailure`, retaining the typed source. Test empty/isolated/relation-only graphs, raw and accepted order, multi-invalid permutations, iterator materialization bounds and `has_more`, orientation, duplicate/mixed-version edges, node/edge/hop boundaries, fixed-point conversion, integer overflow, row mass at/either side of one, unauthorized neighbors, exact routing, and zero-size complexity domains | `SIG-01`, `CORE-02`, `PERF-01` | F2, F8, F10; G4 |
| `ALG-MEM-06` | [Bounded spreading activation](cognitive-memory-activation-and-focus.md#bounded-spreading-activation) | \(a^{(0)}\) and \(W\) from `ALG-MEM-10/11` to bounded propagation; finite depth; canonical sparse `f64` order; checked arithmetic; exact binary-rational component-mass checks; configuration-derived forward-error bound; only exact mass in \((1,1+\varepsilon_{\mathrm{mass}}]\) is divided by an upward binary64 denominator and then exactly rechecked, otherwise typed `InvalidSpreadingArithmetic` | Every accepted iterate is finite, nonnegative, and has exact stored-component mass at most one without general clamping; no ineligible graph crossing; test the nearest-sum-downround fixture, normalization-still-above-one fixture, bound edges, nonfinite arithmetic, cycles, signed zero, and post-correction failure | `CORE-01`, `CORE-02`, `PERF-01` | F2, F8; G4 |
| `ALG-MEM-07` | [Kernel composition](cognitive-memory-activation-and-focus.md#signal-derivation-and-existing-kernel-composition) | Derived channels and gates to activation candidates; exact channel identities, no defaults, typed errors | Preserve kernel bounds and explainability; test hidden zero fill and reused channel lineage | `ACT-00`, `ACT-01`, existing kernel tests | F8, F10; G4 |
| `ALG-MEM-08` | [Proposition consolidation](cognitive-memory-activation-and-focus.md#request-local-proposition-consolidation) | One sealed \(Q\), one complete \(\mathcal A=\texttt{EligibleActivatedMemorySet<'call>}\) carrying inseparable \(\Lambda_A\) plus private \(\omega_A,\sigma_A\), and pinned \(K\) to `deriveRequestPropositions(&BoundQuery, &EligibleActivatedMemorySet<'call>; K)`; the implementation borrows \(Q_{\mathrm{num}}\), \(B_Q\), \(\Lambda_A\), \(\omega_A\), and \(\sigma_A\) only internally, requires \(B_Q=\pi_Q(\Lambda_A)\), and returns one complete bounded `FocusCandidateSet` preserving both witnesses; registered equivalence and conflict only; total through the named typed request-proposition or join error; \(B_Q\) is not semantic evidence and a valid zero-record \(\mathcal A\) still admits request-only focus | Deduplicate without losing conflict, authority, or provenance; preserve semantics under permitted nonsemantic instance correspondence. Split query or lineage/witness assembly is unrepresentable at this API; `PLAN-01` preserves but cannot classify either witness because it has no independent planning scope. Test whole-query join mismatch, both aggregate witnesses, same-call set reconstruction, compile-fail mixed-projection construction, empty-memory request-only focus, embedding-only equivalence, cloned sources, and complete cross-request aggregate swaps; current-call and exact-set rejection belong to `PLAN-02` | `PLAN-01`, `EVAL-01` | F5, F6, F12; G4 |
| `ALG-ACT-01` | [Activation mathematics](situation-conditioned-activation.md#mathematics) | Valid profile and candidates to complete ranking; implemented `f64`, canonical channels, exact score tie then `CandidateId` | \(E_i,R_i,A_i\in[0,1]\), monotonicity, deterministic reconstruction | Existing `nemosyne-core` tests, `ACT-00` | F8; G4 |
| `ALG-EXP-00` | [Situation and expectation query](predictive-attention-and-expectation.md#situation-and-expectation-query) | One sealed \(Q\), the complete immutable `&EligibleActivatedMemorySet<'call>` instance also supplied to `ALG-MEM-08`, carrying inseparable \(\Lambda_A\) plus \(\omega_A,\sigma_A\), and pinned \(K\) to exact \(\operatorname{binding}(Q)=\pi_Q(\Lambda_A)\) validation and a canonical finite set of at most \(F_{\max}\) frame queries. Any expectation-private view is derived only inside this call; no branch projection, reconstructed set, or independently supplied lineage is accepted. The stage is total through the closed `ExpectationQueryBindingError`, artifact error, resource error, or valid zero-frame result | \(B_Q\) is borrowed only as a deterministic content/configuration join and never becomes semantic evidence or call identity; focus and expectation consume one complete eligible activated set and preserve both witnesses, while frame-query meaning and order remain invariant across separately valid same-content calls and corresponding nonsemantic instance lineage. Test exact-instance routing through fresh set branding, same-call reconstruction rejection at planning, compile-fail projection/reconstruction overloads, every binding-error mapping, valid same-content repetition, defensive whole-query corruption, paired-valid instance correspondence, permutation, zero frame, frame ceiling, and complete cross-request aggregate swap | `EXP-01`, `EXP-02`, `API-01` | F2, F3, F10, F12, F15; G4, G8 |
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
| `ALG-PLAN-01` | [Structural projection](focus-and-expectation-planning.md#structural-plan-and-renderable-projection) | One compiler-private current-call and expected-set `PlanningInvocationScope`, immutable branch-owned `PlanningSourceProjection` values with focus/expectation semantic keys and matching private invocation/set witnesses, and a minimized permissionless exact-surface inventory to same-call/exact-set validation, closure selection, lowering-only ceiling meets, exact-slot joins keyed by value-independent owner plus locator, exact-value-independent `RendererSlotId` assignment, tagged \(G(X)\), and \(V(X)\); no principal, policy, authorization/disclosure view, or live grant | Two mutually agreeing foreign branches or two branches from a reconstructed same-call set cannot validate without the independently anchored scope; renderer and validator truths cannot cross; planning cannot raise authority, allowed-use, or surface ceilings, use instance lineage or exact content as meaning/order, collapse independent same-schema slot owners, or turn inventory presence into permission. Test missing/foreign/mixed/expired invocation and set witnesses, same-call reconstructed sets, projection/key mismatch, semantic collision, paired-valid same-content correspondence, ambient-authority noninterference, same-locator distinct owners, same-owner conflicting values, exact-value-independent slot keys/order/tensors, inventory minimization, exact-payload substitution, and exclusion leakage | `PLAN-02`, `VAL-01` | F5-F7, F12, F17; G4 |
| `ALG-PLAN-02` | [Mandatory closure](focus-and-expectation-planning.md#mandatory-closure) | Candidates to atomic semantic closures; registered relations; overlapping tagged members counted once | Preserve qualifiers, conflicts, and complete material families; test dropped horizons and alternatives | `PLAN-02`, `EVAL-01` | F5, F6, F15; G4 |
| `ALG-PLAN-03` | [Cost contract](focus-and-expectation-planning.md#cost-contract) | Renderable projection and rendered bytes to checked integer bound and measured cost; one unit, canonical accumulation, no saturation | Accepted bound is conservative and zero iff \(G=\varnothing\); test slot expansion, overflow, and tokenizer mismatch | `PLAN-02`, `REN-04`, `PERF-01` | F7; G4, G6 |
| `ALG-PLAN-04` | [Feasible subsets](focus-and-expectation-planning.md#feasible-subsets) | Finite closures to structural and budget-feasible sets; explicit \(\mathcal J\) and cardinality ceilings | Budget cannot create false empty success; test control-only and all-nonempty-over-budget cases | `PLAN-02`, `EVAL-01` | F7, F9; G4 |
| `ALG-PLAN-05` | [Unified selection](focus-and-expectation-planning.md#canonical-unified-selection) | Finite feasible closure subsets to \(X^*\); complete unified bit objective over branch-tagged `PropositionSemanticKey`/`ExpectationItemSemanticKey`, `RelationSemanticKey`, and slot keys containing value-independent owner-plus-locator semantics; no epsilon, cost utility, or request-instance tie-break | Unique deterministic cross-branch result without cross-scale score comparison and equivariant across separately valid same-content calls; permute every input, vary only permitted nonsemantic \(\Lambda_A\)-derived instance identities and private invocation witnesses, mutate exact payloads without changing owner/key/order, test semantic-key collisions, distinct same-locator owners, same-owner value conflict, the equal-role/equal-activation one-of-two budget counterexample, and repairable dependencies | `PLAN-02` exhaustive oracle and equivalence tests | F3, F9, F10, F15; G4 |
| `ALG-VAL-00` | [Rendering boundary](vector-to-attention-renderer.md#rendering-boundary) | After planning, compiler-owned `buildValidationContext` maps one retained request, sealed invocation, sealed \(Q\), selected \(L\), authenticated \(K_R\), and call controls to opaque \(V_{\mathrm{ctx}}\), then projects only \(V_{\mathrm{view}}=\pi_{\mathrm{val}}(V_{\mathrm{ctx}})\) to the independent validator. The builder revalidates the query/lineage/call/control joins, erases the witness, recomputes `PlanCanonicalEnvelopeV1`, and retains minimized validation data plus \(c_L,\beta_L,c_R,\beta_R\). Candidate construction, substitution, context construction, and validation independently derive \(c_R,\beta_R\) from authenticated \(K_R\); no caller supplies either. Substitution owns the closed eleven-variant error including configuration mismatch, plan collision, and final renderer cost; validation owns only plan/configuration mismatch through the view. The compiler compares equal-ID plan-byte capsules before the independent validator. The boundary is total through the named closed errors and returns no partial semantic value | Malformed query or foreign-call plan rejects in the builder. Canonically different plan content is the owning substitution/validator `PlanIdentityMismatch`; equal plan ID with unequal \(\beta_L\) is `PlanContentIdentityCollision`; any renderer ID or canonical-content disagreement, including equal \(c_R\) with unequal \(\beta_R\), is `RendererConfigurationMismatch` and quarantines. Separately authenticated byte-identical \(K_R\) values and canonical-content-identical plans are interchangeable and produce identical bytes. Test every included/excluded plan and renderer field, \(K_R\)-only plan-ID noninterference, query corruption, foreign witness, lifetime/detachment, no-context-import and view-only access, all eleven substitution variants and precedence, both validator mismatches, plan/configuration same-ID/different-byte cases, successful equal-content swaps, and no partial semantic result | `API-01`, `VAL-01`, `REN-06` | F2, F3, F6, F9, F12; G4, G8 |
| `ALG-REND-01` | [Renderer projection](vector-to-attention-renderer.md#renderer-projection-view) | \(G(L)\), safe slot metadata, and language to typed tensors; fixed facets, masks, and canonical rank. Each exact slot exposes only `RendererSlotId`, content-independent `ExactSlotOwnerSemanticKey`, `ExactSlotSemanticLocator`, `SlotSemanticKey`, type, role, bounds, permissions, schema, and formatter; authoritative values, surface content identities/bytes, binding instances, \(B_Q\), and \(\Lambda_A\) remain privileged | Preserve selected semantics without exact payload access. Changing only an exact value plus its content identity/formatted bytes preserves owner keys, slot keys, IDs, order, and every pre-substitution tensor while deterministic substituted/product bytes may change; independent same-locator owners remain distinct and one owner-plus-locator cannot carry conflicting values; alter masked and validator-only fields and reject model-visible payload or lineage leakage | `REN-01`, `REN-02`, `VAL-01` | F5, F6, F17; G4 |
| `ALG-REND-02` | [Latent resampler](vector-to-attention-renderer.md#typed-latent-resampler) | Bounded item tensors to bounded continuous prefix; fixed dimensions and deterministic runtime identity | Canonical-order determinism and no hidden state; test rank permutation, missing facets, and precision drift | `REN-02`, `REN-03`, `PERF-01` | F10, F17; G4, G6 |
| `ALG-REND-03` | [Generation](vector-to-attention-renderer.md#generation) | Nonempty prefix to slot-bearing claim, then substitution and validation to \(T'\) or error; frozen decoding and checked slots | No answer, action, unsupported claim, or exact-value invention; test injection, smuggling, language, and cost violations | `REN-04`, `VAL-01`, `SEC-01`, `SYS-01` | F7, F17; G4, G7 |
| `ALG-REND-04` | [Training objective](vector-to-attention-renderer.md#combined-objective) | Frozen examples to candidate parameters; research-only; manifest-bound weights, seeds, splits, masks, and numeric policy | Training does not itself prove faithfulness; test leakage, attribution gaming, and nonfinite runs | `ML-01`-`ML-03`, `REN-05`, `REN-06` | F17 empirical support only; G4, G6 |

The predictive specification keeps finer derivation identities because several
proof obligations share one executable stage. Their aggregation is complete
and fixed as follows:

| Cross-stage identity | Predictive-local derivation identities |
| --- | --- |
| `ALG-EXP-00` | `EXP-QRY-001`, `EXP-LIN-001` |
| `ALG-EXP-01` | `EXP-ELIG-001` |
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

Because \(\mathsf R\) retains `P` independently and the authenticated
\((\mathsf R,\mathcal I_A)\) pairing is checked before compilation, the final
`|P|` bytes of every successful `O` equal `P`, and no byte follows them. This
proof depends on the serializer using `promptBytes(\mathsf R)` directly and on
every adapter delivering the original bytes without normalization.

#### F2: Authorization before relevance

Candidate generation receives only the request-compatible \(\mathcal M_Q\),
which is a subset of \(\mathcal M_E\), itself a subset of
\(M_A^{r,p_{\mathrm{policy}},t_{\mathrm{auth}},I}\). Every downstream source reference must be
constructed from the candidate source set or from the compile request.
Therefore, if constructors prevent forged references and no stage has ambient
memory access, every memory-derived source in a successful plan belongs to
\(M_A^{r,p_{\mathrm{policy}},t_{\mathrm{auth}},I}\).

Authorization also requires semantic noninterference. For two physical memory
states whose authorized projections for the call are identical, changing only
unauthorized records must not change candidate crowding, ranking, planned
meaning, rendered attention, or content-bearing diagnostics when the request,
invocation context, contextual and authorization times, and configuration are
held fixed:

\[
projection_A(M_1)=projection_A(M_2)
\Rightarrow
semanticCompile(
M_1;\mathsf R,\mathcal I_A,
\Sigma_{\mathrm{sig}},V_{\mathrm{sig}},\Theta_{\mathrm{call}}
)
=
semanticCompile(
M_2;\mathsf R,\mathcal I_A,
\Sigma_{\mathrm{sig}},V_{\mathrm{sig}},\Theta_{\mathrm{call}}
)
\]

The physical index and search procedure must therefore enforce authorization
before bounded nearest-neighbor or top-k competition, not retrieve a crowded
global top-k and filter afterward. Timing and other side channels require a
separate security model. These properties establish information-flow
eligibility and noninterference, not truth of an authorized record.

#### F3: Snapshot consistency

One immutable memory revision `r`, policy revision \(p_{\mathrm{policy}}\), trusted authorization
time `t_auth`, declared context time `t_context`, invocation context `I`, and
authorized view are pinned before memory-dependent work. Authoritative records,
numerical representations, and indexes are checked against `r`; authorization
and disclosure expiry, current normative validity, and supersession use
`t_auth` and \(p_{\mathrm{policy}}\); temporal relevance receives both times explicitly, but
`t_context` cannot revive historical instructions as current authority.
\(Q_{\mathrm{num}}\) carries only caller-supplied `t_context`; `t_auth`
reaches authorization, validity, and request-usage compatibility directly;
signal and availability derivations receive it only through
\(V_{\mathrm{sig}}\) after exact context validation. It is never an input to `encode`, and no signal
stage reads an ambient clock or reconstructs the trusted subject.
Downstream stages receive no ambient store handle or wall clock. One immutable
`K` pins the identities of every policy evaluator and content-identified
artifact handle, execution runtime, precision policy, supported platform
class, and deterministic inference policy for the same call. Stochastic
encoding, retrieval, planning, renderer decoding, or validation is not
V1-deployable. Therefore, for fixed \(\mathsf R\), sealed
\(\mathcal I_A\), \(\Sigma_{\mathrm{sig}}\),
\(V_{\mathrm{sig}}\), and \(\Theta_{\mathrm{call}}\), a successful call is a function of one logical
memory-policy revision, both pinned time values, and one compiler-artifact and
execution set even when a writer or updater publishes a later revision
concurrently.

After exact request authentication, the compiler resolves and pins `K`;
`SIT-01` then constructs \(\widehat B_{\mathrm{in}}\) from the same complete
\(\mathsf R\). Under the injectivity of the registered canonical encoding and
collision resistance of its authenticated digest algorithm:

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

Let `State` be all persistent compiler state, including memory, derived
representations, indexes, caches, logs, telemetry, and installed artifacts.
Compilation has the transition:

\[
(\mathit{State},request) \rightarrow (\mathit{State},result)
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

Under `ALG-PLAN-03` and `ALG-PLAN-04`, planning uses the same checked unit and
finite budget domain later enforced on exact output. Checked substitution
measures the complete post-expansion attention value and returns
`RendererCostBoundViolation` before constructing `SubstitutedAttention` when
that value exceeds the bound. The validator therefore receives only an
already measured in-budget candidate; it does not own or repeat the deciding
cost check. Planning, substitution, rendering, and validation never truncate
semantic content. Mandatory content that cannot fit fails. When any otherwise
justified nonempty faithful plan exists but none fits, the call also fails
rather than returning budget-driven empty attention. One over-budget optional
closure remains skippable when another faithful nonempty result fits. Invalid
units, incomplete domains, unsupported configurations, or overflow fail
before selection. `ALG-REND-03` maps the substitution-owned bound violation to
public `FaithfulnessFailure` and invalidates the exact renderer qualification.

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
An adapter buffers the complete serialized value before delivery and emits no
stdout bytes before delivery begins. A transport failure may expose a prefix
after delivery has begun; exit `10` invalidates that prefix and the call
remains unsuccessful. The contract does not claim rollback of bytes already
accepted by an operating-system transport.

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

Focus and expectation each receive an immutable borrow of the exact same
complete sealed `EligibleActivatedMemorySet<'call>` object
\(\mathcal A\). The compiler performs no projection, filtering, copying,
reconstruction, or separately materialized equivalent before either call.
Each aggregate-taking branch may derive a private view only inside its own
call, and that view never replaces \(\mathcal A\). Each output carries the same
request, situation, memory, policy, authorization-view, retrieval, and
configuration identities, the invocation-instance witness \(\omega_A\), and
the fresh set-instance witness \(\sigma_A\). A compiler-private
`PlanningInvocationScope` independently borrows both expected witnesses from
the current invocation and selected set rather than from either branch. The
combined planner validates the identities and both witnesses before selection:

\[
\begin{aligned}
lineage(L)&=lineage(F)=lineage(E)=lineage(\mathcal A),\\
\omega(L)&=\omega(F)=\omega(E)=\omega(\mathcal I_A),\\
\sigma(F)&=\sigma(E)=\sigma(\mathcal A).
\end{aligned}
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
construction. Bound `Q` receives \(B_Q\) from the first projection, while
\(\Lambda_A\)'s request, situation, and configuration projection receives the
same binding directly from ingress. It is forbidden to initialize
\(\Lambda_A\) from bound `Q` or to initialize either query form from
authorization state. The
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
distinct focus and expectation semantics. Two same-content sets reconstructed
inside the same authenticated invocation share \(\omega_A\) but receive
different \(\sigma_A\); mutually agreeing branch outputs from the reconstructed
set therefore fail against the independently anchored planning scope.

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
| `V1-R01` | Exact prompt/request origin authentication, zero to three situations, caller-supplied contextual time, optional declared location, explicit request metadata, compiler-owned content identities, and separate private trusted caller and authorization time that do not enter either query projection | Invocation context, authenticated prompt, compiler ingress, and situation encoding | Exact prompt/request presentation freshness, substitution, cross-pair, and replay; origin, count, canonical-envelope, contextual-time/location/metadata presence; deterministic \(Q_{\mathrm{num}}\); independently derived \(B_Q\) sealed together as one canonical \(Q\); numerical/binding semantic noninterference; split-construction impossibility; same-content identity, content/control/configuration mutation, map-permutation, whole-query cross-request swap, defensive projection corruption, constant/reused/caller-ID, recomputation/collision-witness, locator/content-identity, invalid-input, trusted-time noninterference, and forged-time authorization-isolation tests |
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
- compile-fail API fixtures proving that \(Q_{\mathrm{num}}\) and \(B_Q\)
  cannot be independently supplied, replaced, or recombined after ingress,
  plus adversarial internal-corruption fixtures for numerical-only,
  binding-only, whole-\(Q\), and shared-set swaps,
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
- aggregate-only focus and expectation APIs, witness preservation before an
  independent current-call scope exists, and rejection of complete foreign
  branches only at `PLAN-02` or post-plan context construction;
- exact slots with identical schema/path/ordinal but distinct item-owner keys,
  explicit shared-owner slots, same-owner-plus-locator conflicting values,
  and exact-value mutations that preserve every semantic key, slot ID, order,
  and pre-substitution tensor;
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
- validation-context witness consumption and nonexposure; lifetime checks
  proving that contexts and candidates cannot outlive their source-plan borrows
  or be detached unchecked, without treating a lifetime as referent identity;
  private construction and preservation of `PlanContentId`, the exact
  `PlanCanonicalEnvelopeV1` byte-comparison capsule, and
  `RendererConfigurationId` for both render candidate types; successful swaps
  only between separately constructed plans with byte-identical
  `PlanCanonicalEnvelopeV1` values under the same renderer configuration, with
  identical substitution, validation, and product bytes;
  `PlanCanonicalEnvelopeV1` included/excluded-field
  metamorphisms; observed same-ID/different-byte collision quarantine before
  the independent validator; all eleven `RendererSubstitutionError` variants
  with fixed precedence and total public mappings, including
  `RendererConfigurationMismatch` and substitution-owned
  `RendererCostBoundViolation`; rejection of canonical-content-different
  candidate/plan swaps as substitution-owned `PlanIdentityMismatch`; and
  rejection of plan- or renderer-configuration-different substituted
  candidate/validation-view pairs as the corresponding validator-owned
  mismatch;
- budgets immediately below and at the faithful minimum;
- pre-access sealed-evaluation fixtures with a designed empty or underexposed
  `I_D` or `I_I`, proving `RejectedPreAccess`, no sealed-outcome access, and no
  normalization or arithmetic; plus separate post-access fixtures in which a
  manifest that passed those checks loses required realized outcome cells
  under its frozen failure policy, proving that the affected estimand and
  interval remain absent, no zero denominator is evaluated, the complete
  result is `Inconclusive`, and the release claim is blocked;
- G1 pre-architecture fixtures that instantiate all six `g1_*` conditions,
  verify equal focus and token-matched expert-attention inputs across the four
  expectation roles, reconstruct every G1 expectation estimand, and prove that
  no candidate artifact or G9 lineage enters the receipt;
- envelope-parameterized expectation-branch fixtures with a designed empty or
  underexposed \(I_E^{(e)}\), incomplete or noninjective condition maps, or
  missing required pre-access fields, proving `RejectedPreAccess` and no
  outcome access; plus separate valid-envelope fixtures with realized
  condition loss, zero focus-only successes, and G9 candidate abstention on a
  prospectively expectation-eligible case, proving that none
  can be excluded, relabeled, or converted into a zero-valued estimand after
  outputs are visible;
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
| H5c | Adding a correct qualified expectation to the same focus plan improves the applicable manifest's prospectively frozen expectation-eligible tasks over focus-only attention without exceeding frozen anchoring, leakage, and harm limits | Focus-plus-expectation versus token-matched focus-only |
| H5d | A deliberately wrong dominant expectation is detectably harmful and more anchoring on the same prospectively frozen tasks than the correct and abstaining conditions without gaining answer or action authority | Wrong-expectation negative control versus correct and abstaining conditions |
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

#### G1 pre-architecture conditions

G1 tests product headroom before a candidate architecture exists. Its manifest
therefore uses the following closed condition labels, which are disjoint from
the numbered G9 labels:

| Label | Frozen condition |
| --- | --- |
| `g1_prompt` | Original prompt only |
| `g1_situation` | Prompt plus the same situation and metadata, without persistent memory |
| `g1_focus` | Expert-authored focus-only attention |
| `g1_correct` | The same expert-authored focus plus one correctly qualified expectation |
| `g1_wrong` | The same expert-authored focus plus one deliberately wrong dominant expectation with otherwise matched qualification |
| `g1_abstain` | The same expert-authored focus plus an explicit renderer-visible expectation abstention |

The four expert-attention conditions use the same independently authored focus,
language, placement, effective budget, and attention-token count. Only the
expectation intervention changes. The prompt-only and situation-only conditions
retain the same downstream model, placement, decoding, environment, and
effective budget and establish the separate product-headroom comparisons. Every
expert attention block preserves the no-answer, no-action, no-fact-promotion,
and no-probability-promotion boundaries. The `g1_wrong` intervention alone is
deliberately incorrect or unsupported and is never a product configuration; all
other expert conditions obey the complete evidence-support contract. G1 case
semantics, expectation eligibility, labels, condition construction,
token-matching procedure, and design weights are frozen before any G1 outcome
is visible. No candidate artifact, candidate renderer, or
implementation-derived plan participates in these conditions.

#### G9 implementation conditions

Each G9 sealed task is run under frozen, token-matched conditions where
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

All G1 and G9 conditions use the same scenario-specific `t_context`, `t_auth`,
memory revision `r`, policy revision \(p_{\mathrm{policy}}\), authorized-view
identity, downstream model version, message role and placement, decoding
configuration, tool access, seed schedule, and effective budget. Every
task-condition starts from the same content-hashed repository and environment
snapshot in a fresh isolated process and model session. Mutable caches, tool
state, files, and background processes are reset or identically preseeded.
Condition order is randomized only after this carryover isolation. Only the
named treatment changes.

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

- G1 expert-focus headroom: `g1_focus` against `g1_prompt` and
  `g1_situation`;
- G1 expert expectation contribution: `g1_correct - g1_focus`;
- G1 wrong-expectation sensitivity: `g1_wrong` against `g1_correct` and
  `g1_abstain`;
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

#### G1 headroom estimands

G1 owns a closed condition domain
\(\mathcal C_{\mathrm{G1}}=\{P,S,F,+,W,A\}\), mapped injectively to
`g1_prompt`, `g1_situation`, `g1_focus`, `g1_correct`, `g1_wrong`, and
`g1_abstain`. Its headroom baseline set is exactly
\(\mathcal B_{\mathrm{G1}}=\{P,S\}\); no G9 condition, candidate output, or
release baseline may replace either member.

Before any G1 outcome is accessible, the signed G1 envelope freezes a finite
claim-bearing headroom population
\(\mathcal T_H^{(\mathrm{G1})}\), two disjoint membership sets
\(I_D^{(\mathrm{G1})}\) and \(I_I^{(\mathrm{G1})}\) whose union is
\(\mathcal T_H^{(\mathrm{G1})}\), and one positive normalized design weight
\(v_i^{(\mathrm{G1})}\) for every task:

\[
0<\left|\mathcal T_H^{(\mathrm{G1})}\right|<\infty,
\qquad
I_D^{(\mathrm{G1})}\cap I_I^{(\mathrm{G1})}=\varnothing,
\qquad
I_D^{(\mathrm{G1})}\cup I_I^{(\mathrm{G1})}
=\mathcal T_H^{(\mathrm{G1})},
\qquad
\sum_{i\in\mathcal T_H^{(\mathrm{G1})}}v_i^{(\mathrm{G1})}=1.
\]

Membership is authored from case semantics before condition outputs are
generated. \(I_D^{(\mathrm{G1})}\) contains cases whose successful resolution
requires the frozen context premise; \(I_I^{(\mathrm{G1})}\) contains
claim-bearing negative-control cases that do not. No outcome, model behavior,
attention text, or post-outcome judgment may change membership. Each set and
each claim-bearing language, task-family, and risk-subgroup intersection has a
positive frozen minimum task count and independent-cluster count.
\(I_E^{(\mathrm{G1})}\), defined below, is a prospectively selected subset of
\(I_D^{(\mathrm{G1})}\), never an alias for it.

Before any G1 outcome access, structural validation requires the finite,
nonempty population; complete and unique case membership; the exact disjoint
partition above; one finite \(v_i^{(\mathrm{G1})}>0\) for every and only every
population member; exact normalized weight mass; and a positive finite-integer
task-count and independent-cluster-count minimum for both domains and every
claim-bearing subgroup, with the frozen pre-outcome memberships already
meeting each such minimum. A missing case, duplicate membership, partition gap
or overlap, unknown member, missing/non-finite/nonpositive weight, weight-mass
failure, absent or nonpositive exposure minimum, designed membership below a
minimum, or incomplete subgroup exposure contract prevents construction and
signature of a valid `IF-G1-ENVELOPE`. `EVD-02` emits only a
`PreAccessRejectionReceipt` and must reject the attempt before opening
outcomes, normalizing weights, computing an estimand, or constructing an
interval.

Let \(Y_x^{(\mathrm{G1})}(i)\in\{0,1\}\) be the frozen binary task outcome
under condition \(x\in\mathcal C_{\mathrm{G1}}\). For
\(d\in\{D,I\}\), write \(I_d^{(\mathrm{G1})}\) for the corresponding set and,
only after its exposure preconditions pass, define:

\[
v_i^{(\mathrm{G1},d)}
=
\frac{v_i^{(\mathrm{G1})}}
{\sum_{k\in I_d^{(\mathrm{G1})}}v_k^{(\mathrm{G1})}}.
\]

For each \(b\in\mathcal B_{\mathrm{G1}}\), the focus headroom effect in domain
\(d\), population harm rate, and conditional reversal rate are:

\[
\Delta_{F,b,d}^{(\mathrm{G1})}
=
\sum_{i\in I_d^{(\mathrm{G1})}}v_i^{(\mathrm{G1},d)}
\left(Y_F^{(\mathrm{G1})}(i)-Y_b^{(\mathrm{G1})}(i)\right),
\]

\[
h_{F,b,d}^{(\mathrm{G1})}
=
\sum_{i\in I_d^{(\mathrm{G1})}}v_i^{(\mathrm{G1},d)}
\mathbb{1}
\left[
Y_b^{(\mathrm{G1})}(i)=1
\land
Y_F^{(\mathrm{G1})}(i)=0
\right],
\]

\[
h_{F,b,d\mid b}^{(\mathrm{G1})}
=
\frac{
\sum_{i\in I_d^{(\mathrm{G1})}}v_i^{(\mathrm{G1},d)}
\mathbb{1}
\left[
Y_b^{(\mathrm{G1})}(i)=1
\land
Y_F^{(\mathrm{G1})}(i)=0
\right]
}{
\sum_{i\in I_d^{(\mathrm{G1})}}v_i^{(\mathrm{G1},d)}
\mathbb{1}\left[Y_b^{(\mathrm{G1})}(i)=1\right]
}.
\]

The envelope freezes, separately for \(b=P\) and \(b=S\), positive
context-dependent superiority margins
\(\delta_{F,b,D}^{min,(\mathrm{G1})}\), context-independent
non-inferiority margins
\(\delta_{F,b,I}^{NI,(\mathrm{G1})}\), maximum population-harm bounds
\(h_{F,b,d}^{max,(\mathrm{G1})}\), maximum conditional-reversal bounds
\(h_{F,b,d\mid b}^{max,(\mathrm{G1})}\), confidence procedures, multiplicity
treatment, and subgroup gates. Every such threshold is finite and, for every
\(b\in\{P,S\}\) and \(d\in\{D,I\}\), must satisfy:

\[
0<\delta_{F,b,D}^{min,(\mathrm{G1})}<1,
\qquad
0<\delta_{F,b,I}^{NI,(\mathrm{G1})}<1,
\]

\[
0<h_{F,b,d}^{max,(\mathrm{G1})}<1,
\qquad
0<h_{F,b,d\mid b}^{max,(\mathrm{G1})}<1.
\]

A missing, non-finite, zero, negative, or otherwise out-of-domain threshold
prevents construction and signature of a valid `IF-G1-ENVELOPE` before any
outcome becomes accessible and yields only a `PreAccessRejectionReceipt`.
`EVD-02` must not open that envelope, inspect outcomes, normalize weights,
compute an estimand, or construct an interval; this is an invalid manifest,
not an observed `Inconclusive` result.

G1 headroom passes only if every simultaneous or multiplicity-adjusted
comparison satisfies:

\[
lowerCI\left(\Delta_{F,b,D}^{(\mathrm{G1})}\right)
>
\delta_{F,b,D}^{min,(\mathrm{G1})},
\qquad
lowerCI\left(\Delta_{F,b,I}^{(\mathrm{G1})}\right)
>
-\delta_{F,b,I}^{NI,(\mathrm{G1})},
\]

\[
upperCI\left(h_{F,b,d}^{(\mathrm{G1})}\right)
<
h_{F,b,d}^{max,(\mathrm{G1})},
\qquad
upperCI\left(h_{F,b,d\mid b}^{(\mathrm{G1})}\right)
<
h_{F,b,d\mid b}^{max,(\mathrm{G1})}
\quad
\text{for each }d\in\{D,I\}.
\]

After a structurally valid envelope has been admitted and outcomes are
accessible only through the frozen execution policy, a required realized cell
that is missing, failed, empty, or below its frozen exposure minimum produces
no affected weight normalization, estimate, interval, or division, and the
complete G1 receipt is `Inconclusive`. If a required baseline has no realized
successes in a domain, its conditional reversal rate is undefined and the same
rule applies. Neither a zero value nor the other baseline or domain may
substitute. These realized-evidence dispositions do not repair or relabel a
structurally invalid envelope. G1 cases, condition variants, labels, outcomes,
weights, thresholds, and estimands are lineage-disjoint from G9.

#### G9 complete-product estimands

The following complete-product symbols are local to G9. They do not define or
reuse the G1 headroom domains, conditions, weights, outcomes, or thresholds.

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
task units in the finite claim-bearing G9 population:

\[
\mathcal T_H^{(\mathrm{G9})}
=\{i_1,\ldots,i_{N_{\mathrm{task}}}\},
\qquad
N_{\mathrm{task}}
=\left|\mathcal T_H^{(\mathrm{G9})}\right|>0.
\]

Before candidate selection, candidate bytes, condition outputs, or sealed
outcomes are accessible, the candidate-independent protocol freezes
\(\mathcal T_H^{(\mathrm{G9})}\), the context-dependent and
context-independent memberships
\(I_D:=I_D^{(\mathrm{G9})}\) and
\(I_I:=I_I^{(\mathrm{G9})}\), and requires the exact disjoint partition:

\[
I_D\cap I_I=\varnothing,
\qquad
I_D\cup I_I=\mathcal T_H^{(\mathrm{G9})},
\qquad
\mathcal T_H^{(\mathrm{G9})}=I_D\uplus I_I.
\]

Membership is authored only from frozen case semantics: \(I_D\) contains
cases whose successful resolution requires the context premise, while \(I_I\)
contains claim-bearing negative controls that do not. Candidate behavior,
attention text, abstention, success, failure, and any post-outcome judgment
cannot create, remove, or reclassify a member. A gap, overlap, duplicate,
unknown member, or member outside \(\mathcal T_H^{(\mathrm{G9})}\) invalidates
the attempted protocol or run-manifest data, prevents construction and
signature of a valid protocol or run manifest, and produces only a
`PreAccessRejectionReceipt` before any weight normalization, estimand,
interval, or division is attempted.

Let \(v_i:=v_i^{(\mathrm{G9})}>0\) be the frozen G9 design weight for every
\(i\in\mathcal T_H^{(\mathrm{G9})}\), with:

\[
\sum_{i\in\mathcal T_H^{(\mathrm{G9})}}v_i=1.
\]

A self-weighting sample uses \(v_i=1/N_{\mathrm{task}}\). A stratified,
unequal-probability, or deliberately balanced sample must derive and freeze
its weights from the sampling design before outcomes are observed. If no
population sampling design is claimed, uniform weights estimate only the
sealed evaluation set.

Before sealed outcomes are accessible, the candidate-independent protocol
also freezes a positive minimum task count and a positive minimum
independent-cluster count for each partition member set. A release-claim-
bearing design requires both sets to be nonempty and to meet both frozen
exposure minima. A designed empty or underexposed set invalidates the protocol
or attempted run-manifest data, prevents construction and signature of a valid
protocol or run manifest, and produces only a `PreAccessRejectionReceipt`;
sealed outcomes remain inaccessible, and no normalization, estimand, interval,
or division is attempted. Because every \(v_i>0\), a design that passes those
preconditions makes each within-set weight denominator strictly positive.
None of this population, partition, its memberships, cases, weights, or
outcomes may be sourced from G1.

After a valid manifest has opened the sealed phase, a missing or failed
required outcome cell may reduce the realized analyzable exposure below its
frozen minimum under the predeclared failure policy. Only that realized
post-access insufficiency makes the affected paired effect and confidence
interval absent, suppresses the formulas below, and yields `Inconclusive`,
blocking the release claim. Zero, a synthetic effect, the other set, or a
post-outcome reclassification cannot replace the missing estimand.

After those preconditions pass, design weights are renormalized within each
set. For each `b` in `B_release`, the context-dependent paired effect is:

\[
\Delta_b =
\sum_{i\in I_D}
\frac{v_i}{\sum_{k\in I_D}v_k}
(Y_n(i)-Y_b(i))
\]

The corresponding population harm rate is:

\[
h_{population,b} =
\sum_{i\in\mathcal T_H^{(\mathrm{G9})}}
v_i \mathbb{1}[Y_b(i)=1 \land Y_n(i)=0]
\]

The conditional reversal rate among tasks that the baseline solves is:

\[
h_{reversal,b} =
\frac{
\sum_{i\in\mathcal T_H^{(\mathrm{G9})}}
v_i \mathbb{1}[Y_b(i)=1 \land Y_n(i)=0]
}{
\sum_{i\in\mathcal T_H^{(\mathrm{G9})}}
v_i \mathbb{1}[Y_b(i)=1]
}
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

#### Expectation-branch estimands

Let \(e\in\{\mathrm{G1},\mathrm{G9}\}\) identify one evaluation envelope.
Each envelope owns its own prospectively frozen expectation-eligible task set
\(I_E^{(e)}\subseteq I_D^{(e)}\), sampling design, thresholds, exposure
requirements, condition artifacts, and semantic root. The G1 and G9 roots and
all of their semantic lineages are disjoint. Neither envelope may reuse cases,
variants, labels, outcomes, weights, or estimands from the other.
Let \(v_i^{(e)}>0\) be the envelope's prospectively frozen normalized design
weight for task \(i\), with
\(\sum_{i\in\mathcal T_H^{(e)}}v_i^{(e)}=1\). Uniform weights support only the
corresponding sealed or authored benchmark unless the frozen sampling design
justifies a broader population claim.

Each envelope manifest defines one injective condition-label map
\(\kappa_e\) over the closed experimental roles
\(\mathcal R_E=\{F,+,W,A\}\):

| Role | Meaning | G1 condition label | G9 condition label |
| --- | --- | --- | --- |
| \(F\) | Focus-only comparator | `g1_focus` | `10` |
| \(+\) | Correct qualified focus-plus-expectation treatment | `g1_correct` | `12` |
| \(W\) | Deliberately wrong dominant-expectation control | `g1_wrong` | `13` |
| \(A\) | Explicit renderer-visible expectation-abstention control | `g1_abstain` | `14` |

For G1, all four conditions are independently expert-authored before
architecture selection. For G9, they are the frozen candidate and intervention
conditions defined above. Within an envelope, the four conditions use identical
frozen input, focus meaning, attention-realization method, placement, and
effective budget; only the expectation role changes. The realization method is
the frozen expert-authoring protocol for G1 and the exact candidate renderer for
G9. \(W\) and \(A\) are experimental interventions, not supported product
outputs for a fixture that supports a positive expectation.

Membership in \(I_E^{(e)}\) is determined from independently authored case
semantics before any condition output or score is accessible and, for G9,
before candidate artifacts are selected. A case belongs to \(I_E^{(e)}\) only
when its fixture prospectively permits all four mapped conditions. Membership
must not depend on whether a candidate emits an expectation, abstains,
succeeds, or fails.

Each envelope freezes positive minimum task and independent-cluster exposure
for \(I_E^{(e)}\). A designed empty or underexposed \(I_E^{(e)}\), a missing
required mapped condition, or a malformed exposure field invalidates that
envelope before any condition output or outcome is accessible and is
`RejectedPreAccess`, not `Inconclusive`. Every included case then requires
paired outcomes for all four roles under the frozen missing-data and failure
policy. In G9, candidate abstention where the fixture requires a positive
expectation is retained and scored under the predeclared \(+\)-condition
failure rule; it is not treated as missing. An absent condition, invalid
condition, timeout, or other failed run after a valid envelope opens follows
the frozen failure policy. No case is removed or reassigned after outputs are
visible. If those realized failures make a required comparison lose its frozen
exposure minimum, all affected expectation-branch estimands and intervals for
that envelope are absent and its complete evaluation is `Inconclusive`.

Every language, task-family, and risk subgroup included in the applicable
predictive claim must likewise have prospectively frozen positive task and
independent-cluster exposure in its \(I_E^{(e)}\) intersection and pass the
same expectation-branch gates with subgroup-renormalized design weights. An
underexposed or failing subgroup prevents the broader predictive claim and may
be removed only through a newly frozen narrower claim.

For \(i\in I_E^{(e)}\), define normalized design weights:

\[
v_i^{E,e} =
\frac{v_i^{(e)}}{\sum_{k\in I_E^{(e)}}v_k^{(e)}}
\]

Let \(Y_r^{(e)}(i)\in\{0,1\}\) be the primary task outcome under the condition
mapped from role \(r\in\mathcal R_E\). Let
\(A_r^{(e)}(i)\in\{0,1\}\) indicate a frozen anchoring failure: the downstream
agent treats an expectation as established fact or lets it displace required
investigation, counterevidence, or an explicit task invariant. Let
\(\mathcal C_L=\{\mathrm{answer},\mathrm{action},\mathrm{fact},
\mathrm{probability}\}\) be the closed leakage-class domain. For every
\(c\in\mathcal C_L\), let \(L_{r,c}^{(e)}(i)\in\{0,1\}\) indicate the
separately frozen class-specific leakage failure in the attention text or
downstream behavior. `answer` means answering the user's task rather than
framing attention; `action` includes a recommendation, next-step selection,
tool selection, or other action choice; `fact` means promoting an expectation
or unsupported content to established fact; and `probability` means inventing
or promoting an unsupported probability or confidence. The class indicators
may overlap and are never collapsed for gating. Executable checks take
precedence; any remaining classification uses blinded adjudication under
class-specific rubrics frozen before outputs are accessible.

The correct expectation contribution over the identical focus is:

\[
\Delta_{E,+}^{(e)} =
\sum_{i\in I_E^{(e)}}v_i^{E,e}
\left(Y_+^{(e)}(i)-Y_F^{(e)}(i)\right)
\]

Its population harm and conditional reversal rates are:

\[
h_{E,+}^{(e)} =
\sum_{i\in I_E^{(e)}}v_i^{E,e}
\mathbb{1}[Y_F^{(e)}(i)=1\land Y_+^{(e)}(i)=0]
\]

\[
h_{E,+\mid focus}^{(e)} =
\frac{
\sum_{i\in I_E^{(e)}}v_i^{E,e}
\mathbb{1}[Y_F^{(e)}(i)=1\land Y_+^{(e)}(i)=0]
}{
\sum_{i\in I_E^{(e)}}v_i^{E,e}\mathbb{1}[Y_F^{(e)}(i)=1]
}
\]

If the mapped \(F\) condition has no successes,
\(h_{E,+\mid focus}^{(e)}\) is undefined and the mandatory
expectation-contribution comparison is `Inconclusive`; the population harm rate
cannot substitute for it.

For \(r\in\{+,A\}\), wrong-expectation sensitivity is measured against the
correct and abstaining conditions by:

\[
\Delta_{E,wrong,r}^{(e)} =
\sum_{i\in I_E^{(e)}}v_i^{E,e}
\left(Y_r^{(e)}(i)-Y_W^{(e)}(i)\right)
\]

\[
h_{E,wrong,r}^{(e)} =
\sum_{i\in I_E^{(e)}}v_i^{E,e}
\mathbb{1}[Y_r^{(e)}(i)=1\land Y_W^{(e)}(i)=0]
\]

The second quantity is a negative-control manipulation check, not an acceptable
product harm rate.

For every \(r\in\mathcal R_E\), define \(a_r^{(e)}\), and for every
\((r,c)\in\mathcal R_E\times\mathcal C_L\), define
\(\ell_{r,c}^{(e)}\):

\[
a_r^{(e)}=\sum_{i\in I_E^{(e)}}v_i^{E,e}A_r^{(e)}(i),
\qquad
\ell_{r,c}^{(e)}
=\sum_{i\in I_E^{(e)}}v_i^{E,e}L_{r,c}^{(e)}(i)
\]

and the paired expectation-induced differences:

\[
\Delta a_{+,focus}^{(e)}=a_+^{(e)}-a_F^{(e)},
\qquad
\Delta \ell_{+,focus,c}^{(e)}
=\ell_{+,c}^{(e)}-\ell_{F,c}^{(e)}.
\]

For diagnostics only, the protocol may additionally report:

\[
L_{r,\cup}^{(e)}(i)
=
\mathbb{1}
\left[
\exists c\in\mathcal C_L:
L_{r,c}^{(e)}(i)=1
\right],
\qquad
\ell_{r,\cup}^{(e)}
=
\sum_{i\in I_E^{(e)}}v_i^{E,e}L_{r,\cup}^{(e)}(i).
\]

Neither this union rate nor any average, maximum, or other composite across
classes can satisfy, replace, or weaken a class-specific leakage gate.

The wrong-expectation anchoring manipulation checks are:

\[
\Delta a_{wrong,r}^{(e)}=a_W^{(e)}-a_r^{(e)},
\qquad r\in\{+,A\}
\]

These estimands do not authorize the mapped \(W\) condition as a product
configuration. The mapped \(+\) condition must also pass the independently
frozen semantic correctness, qualification, alternative, faithfulness, and
exact-value gates. A statistically useful but unsupported expectation cannot
pass G1 or G9.

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

One signed manifest selects exactly one applicable envelope
\(e\in\{\mathrm{G1},\mathrm{G9}\}\). Before any outcome for that envelope is
accessible, it freezes the following envelope-local state. Freezing or
evaluating \(e\) neither requires the other envelope to be present nor permits
either envelope to borrow the other's cases, memberships, weights, thresholds,
outcomes, or estimands:

- one primary endpoint, the comparator set required by \(e\), and its
  multiplicity treatment;
- for \(e=\mathrm{G1}\), every
  \(\delta_{F,b,D}^{min,(\mathrm{G1})}\),
  \(\delta_{F,b,I}^{NI,(\mathrm{G1})}\),
  \(h_{F,b,d}^{max,(\mathrm{G1})}\), and
  \(h_{F,b,d\mid b}^{max,(\mathrm{G1})}\) threshold defined above; for
  \(e=\mathrm{G9}\), every baseline-specific `delta_min,b`, `delta_NI,b`,
  `h_population_max,b`, and `h_reversal_max,b` threshold;
- positive minimum task and independent-cluster exposure for
  \(I_D^{(e)}\) and \(I_I^{(e)}\), with envelope-local cases, memberships,
  weights, outcomes, thresholds, and estimands;
- the envelope-local condition map \(\kappa_e\), candidate-independent
  membership rule, and positive minimum task and independent-cluster exposure
  for \(I_E^{(e)}\);
- the expectation-contribution superiority margin
  `delta_E_plus_min`, maximum population and conditional-reversal bounds
  `h_E_plus_max` and `h_E_plus_focus_max`, maximum absolute and incremental
  anchoring bounds `a_E_plus_max` and `delta_a_E_plus_max`, and, for every
  \(r\in\mathcal R_E\) and \(c\in\mathcal C_L\), class-specific maximum
  absolute leakage bounds `l_E_r_c_max` plus class-specific incremental bounds
  `delta_l_E_plus_c_max`, each scoped to the applicable envelope; the optional
  union leakage rate is diagnostic only and has no gate;
- for both correct and abstaining controls, the minimum wrong-expectation task
  effect `delta_E_wrong_min,r`, minimum wrong-expectation harm
  `h_E_wrong_min,r`, and minimum wrong-expectation anchoring effect
  `delta_a_E_wrong_min,r`, each scoped to the applicable envelope;
- one envelope-local multiplicity family and correction or predeclared
  hierarchical testing order covering every expectation-branch superiority,
  harm, anchoring, and every class-specific leakage claim separately;
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

Only a G9 evaluation in which both \(I_D^{(\mathrm{G9})}\) and
\(I_I^{(\mathrm{G9})}\) passed their frozen nonempty and exposure
preconditions may evaluate the release inequalities. Using the G9-local
aliases above, it then requires all of the following for every `b` in
`B_release`:

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

The G1 receipt applies the headroom estimands over
\(I_D^{(\mathrm{G1})}\) and \(I_I^{(\mathrm{G1})}\), then applies the
expectation-branch estimands with \(e=\mathrm{G1}\), the exact six-label
`g1_*` map, its own frozen thresholds and multiplicity treatment, and
\(I_E^{(\mathrm{G1})}\). It records the two focus-superiority, two
focus-non-inferiority, population-harm, conditional-reversal,
correct-contribution, wrong-versus-correct, and wrong-versus-abstention
comparisons separately. Missing, invalid, empty, underexposed, or
zero-denominator required outcome cells encountered under an already valid
envelope are `Inconclusive` before affected arithmetic. Invalid attempted
envelope data or a threshold prevents construction and signature of a valid
envelope, produces only a `PreAccessRejectionReceipt` before outcome access,
and is not relabeled as an observed `Inconclusive` result. A G1 case, weight,
outcome, threshold, estimand, or result cannot satisfy or seed a G9 gate.

For either \(e\in\{\mathrm{G1},\mathrm{G9}\}\), a positive expectation claim
requires that envelope's \(I_E^{(e)}\), mapped conditions, realized cells,
task and independent-cluster exposure, required-subgroup exposure, and
zero-denominator preconditions to pass first. G1 evaluates the following
system as part of G1 headroom; an expectation-enabled release evaluates the
same system independently with \(e=\mathrm{G9}\). Every threshold below is
owned by, frozen in, and identified with that one envelope. The complete
simultaneous or multiplicity-adjusted expectation gate is:

\[
lowerCI(\Delta_{E,+}^{(e)}) > \delta_{E,+}^{min,(e)}
\]

\[
upperCI(h_{E,+}^{(e)}) < h_{E,+}^{max,(e)}
\]

\[
upperCI(h_{E,+\mid focus}^{(e)}) < h_{E,+\mid focus}^{max,(e)}
\]

\[
upperCI(a_+^{(e)}) < a_{E,+}^{max,(e)}
\]

\[
upperCI(\Delta a_{+,focus}^{(e)}) < \Delta a_{E,+}^{max,(e)}
\]

\[
upperCI(\ell_{r,c}^{(e)}) < \ell_{E,r,c}^{max,(e)},
\qquad
r\in\mathcal R_E,\quad c\in\mathcal C_L
\]

\[
upperCI(\Delta \ell_{+,focus,c}^{(e)})
< \Delta \ell_{E,+,c}^{max,(e)},
\qquad c\in\mathcal C_L.
\]

Against both the correct \(r=+\) and abstaining \(r=A\) controls, the
deliberately wrong expectation must also remain a detectable negative control
within the same envelope:

\[
lowerCI(\Delta_{E,wrong,r}^{(e)})
> \delta_{E,wrong,r}^{min,(e)}
\]

\[
lowerCI(h_{E,wrong,r}^{(e)}) > h_{E,wrong,r}^{min,(e)}
\]

\[
lowerCI(\Delta a_{wrong,r}^{(e)})
> \Delta a_{E,wrong,r}^{min,(e)},
\qquad r\in\{+,A\}.
\]

Because the task-level outcomes are binary, the baseline paired effects have
estimand domain \([-1,1]\), while the baseline harm rates have estimand domain
\([0,1]\) when defined. The expectation-branch task effects and rate
differences also have domain \([-1,1]\); the branch's absolute rates have domain
\([0,1]\). When \(e=\mathrm{G9}\), every frozen complete-product baseline
threshold is finite and must satisfy
\(0<\delta_{min,b}<1\),
\(0<\delta_{NI,b}<1\),
\(0<h_{population\_max,b}<1\), and
\(0<h_{reversal\_max,b}<1\). The separately named G1 headroom thresholds obey
their earlier domains instead and are not required by a G9 manifest; the G9
baseline thresholds are not required by a G1 manifest. For the selected \(e\),
every minimum expectation-branch effect or negative-control
bound lies strictly inside \((0,1)\), every absolute maximum rate, including
each \(\ell_{E,r,c}^{max,(e)}\), lies strictly inside \((0,1)\), and every
maximum paired rate difference, including each
\(\Delta\ell_{E,+,c}^{max,(e)}\), lies in \([0,1)\). All thresholds owned by
the selected envelope are finite. A manifest containing a missing, non-finite,
vacuous, or out-of-domain required threshold is invalid before its sealed data
become accessible; no affected normalization, estimate, interval, or release
arithmetic may begin, and the manifest cannot support a claim.

Every claim-bearing subgroup must additionally pass its frozen
non-inferiority and harm bounds for every required baseline. The positive
values of the margins and bounds are not selected by this specification. They
require a later decision before the sealed set is opened. Domain validity
alone is insufficient: the manifest must justify each value against the
operational consequence, target population, minimum useful effect, exposure,
and planned precision. A threshold selected merely to make failure impossible
or a harm ceiling selected near one is invalid even when it lies inside the
formal domain. Report effects and intervals, not only p-values.

Zero observed critical failures is a release requirement but not proof of zero
risk. Each critical class must meet a predeclared minimum exposure count and a
maximum one-sided confidence bound. Any rate claim includes its sampling and
dependence assumptions.

### Sealed evaluation protocol

1. Build development and calibration evidence without sealed cases.
2. Before selecting a release candidate, freeze and independently sign the
   candidate-independent G9 protocol: target population, task taxonomy,
   sampling frame, case-selection procedure, inclusion probabilities, design
   weights, prompt corpus/template/placement rules, condition map, baselines,
   budgets, metrics, \(I_D\)/\(I_I\)/\(I_E^{(\mathrm{G9})}\) membership rules
   and exposure minima, analysis code, thresholds, multiplicity treatment,
   subgroup and critical-class rules, adjudication, custody, required
   runtime/hardware/resource fields, and one deterministic post-verification
   run-manifest finalization rule.
3. Under that signed protocol, have an independent custodian sample or author
   sealed semantic cases whose roots are disjoint from development,
   calibration, and G1 lineages; label must-include, may-include, must-exclude,
   task outcome, authority,
   expectation-eligibility, anchoring, and leakage rubrics, and complete
   independent annotation and documented adjudication. Candidate artifacts,
   outputs, and scores cannot influence case authorship, membership, labels, or
   adjudication. Only the sealed-root digest and custody metadata, not case or
   label content, are exposed for later manifest finalization.
4. Reproducibly freeze one release candidate under `REL-01` without sealed-case,
   label, or outcome access.
5. Verify the exact packaged bytes without rebuilding or retuning under
   `RCV-01`, producing one immutable `IF-RCV-RECEIPT`. A failed verification
   rejects that candidate identity.
6. After successful verification and before any sealed case, label, or outcome
   becomes accessible to candidate-development, run-finalization, or
   evaluation-execution principals, apply the signed protocol's deterministic
   finalization rule. Bind the protocol digest, exact `IF-RCV-RECEIPT`,
   candidate architecture, artifacts and configuration, downstream model and
   runtime versions, prompt-corpus/template/placement identity, decoding and
   seeds, baselines, metrics, analysis, multiplicity, thresholds, target
   hardware, resource limits and budgets, all frozen
   \(I_D^{(\mathrm{G9})}\)/\(I_I^{(\mathrm{G9})}\)/
   \(I_E^{(\mathrm{G9})}\) gates, and the G1-disjoint sealed
   root into one complete signed immutable `IF-G9-RUN-MANIFEST`.
7. Reject and permanently retire the attempt on an incomplete or mismatched
   join, lineage overlap, signature failure, manifest mutation, or prohibited
   access before signature. No partial manifest or reconstructed substitute may
   proceed.
8. Only after the run manifest is signed, permit the independent evaluation
   executor to access the sealed material. Randomize condition order through
   the frozen seed schedule and execute that exact manifest once, unchanged,
   without reconstruction, substitution, tuning, threshold changes, or
   post-outcome reassignment.
9. Bind every output, realized exposure, resource observation, failure, timeout,
   exclusion, and inconclusive result to the unchanged run-manifest digest.
10. Record every sealed attempt permanently, including failed and inconclusive
    revisions.
11. If a release gate fails, create a new product revision, candidate identity,
    protocol/run-manifest identity, and newly authored sealed set; do not tune
    and rerun the exposed set as fresh evidence.
12. A first passing revision requires either a predeclared sequential-testing
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
| G1: Product headroom | The exact `g1_prompt`, `g1_situation`, `g1_focus`, `g1_correct`, `g1_wrong`, and `g1_abstain` conditions pass their matched-construction audit; `g1_focus` passes both frozen superiority gates over \(I_D^{(\mathrm{G1})}\), both non-inferiority gates over \(I_I^{(\mathrm{G1})}\), and every population-harm and conditional-reversal bound; on separate positive-exposure \(I_E^{(\mathrm{G1})}\), `g1_correct` passes contribution gates while `g1_wrong` is detectably worse and more anchoring than `g1_correct` and `g1_abstain`; all task/cluster/required-subgroup, multiplicity, answer/action/fact/probability-leakage, and no-reuse gates pass. An invalid attempted envelope or threshold prevents valid signature and emits only a `PreAccessRejectionReceipt` before outcome access. Under an already valid envelope, any missing, invalid, empty, underexposed, or zero-denominator required outcome cell is `Inconclusive` before arithmetic | Reject, narrow, or defer the premise before architecture implementation |
| G2: Evidence harness | Formal obligations are reviewed; one versioned manifest, receipt, split, lineage, baseline, and analysis harness can represent every required condition and preserve failed or inconclusive results | Do not select implementation technologies |
| G3: Predictive semantics | The deterministic expectation baseline passes transition-schema, dependency-budget, alternative, abstention, observation-assessment, and non-probability contracts on curated and adversarial evidence | Correct or simplify predictive semantics before renderer or persistence integration |
| G4: Renderer feasibility | A deterministic renderer or registered numerical bridge plus the smallest passing local checkpoint faithfully renders frozen expert focus-and-expectation plans and exact slots within local budgets | Replace or constrain the bridge, model, or rendering contract; do not weaken a failed gate |
| G5: Memory read and snapshots | Supplied revisions, authorization views, pinned indexes, concurrent publication, and compile/management separation satisfy their contracts | Do not build persistent-memory retrieval |
| G6: Retrieval | Required-proposition and eligible-transition recall plus cross-context behavior beat frozen simple baselines | Replace or simplify retrieval |
| G7: Activation and planning | Fixed-intermediate comparisons show value for signal derivation, activation, focus construction, expectation derivation, and combined closure selection over their strongest simpler baselines | Do not calibrate or integrate a mechanism without added value |
| G8: Vertical slice | All critical invariants, offline boundaries, and resource budgets hold in one local end-to-end integration | Do not build release packaging |
| G9: Sealed evaluation | Under the candidate-independent protocol, exact verified candidate, pre-access signed run manifest, and unchanged-once execution, every complete-product superiority, non-inferiority, paired \(I_E^{(\mathrm{G9})}\) correct-contribution, wrong-expectation negative-control, multiplicity, separate answer/action/fact/probability leakage, population harm, conditional reversal, anchoring, positive task/independent-cluster/required-subgroup exposure, subgroup, critical-rate, operational, resource, and exact \(\mathcal T_H^{(\mathrm{G9})}=I_D^{(\mathrm{G9})}\uplus I_I^{(\mathrm{G9})}\) membership/weight gate passes. An invalid attempted population partition, run manifest, or threshold prevents valid manifest signature and emits only a `PreAccessRejectionReceipt` before sealed outcome access. Under an already valid manifest, any missing, invalid, empty, underexposed, or zero-denominator required outcome cell is `Inconclusive` before normalization or arithmetic, has no estimate or interval, and blocks release | Report failure or inconclusive evidence; do not relabel it |
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

Every attempted G1-envelope or G9-run-manifest validation first produces
exactly one minimal `PreAccessRejectionReceipt` or `ValidForOutcomeAccess`
record. Every attempted G9-protocol validation instead produces exactly one
minimal `PreAccessRejectionReceipt` or one valid signed `IF-G9-PROTOCOL`.
A valid G9 protocol is candidate-independent: it neither grants outcome access
nor creates a `ValidExperimentReceipt`; only the later post-RCV
`IF-G9-RUN-MANIFEST` validation can produce `ValidForOutcomeAccess`.
A `PreAccessRejectionReceipt` records only:

- the attempted protocol, envelope, or run-manifest kind;
- an opaque attempt identity;
- the exact attempted bytes and their digest when those bytes are available,
  or the available prefix plus the stage and field at which bounded parsing
  failed;
- every configuration, source, dataset, implementation, and custody identity
  successfully established before rejection;
- one closed typed rejection reason and the validation stage that produced it;
- verifier, validation-implementation, hardware, operating-system, and time
  identity; and
- an auditable proof that sealed condition outputs and outcomes remained
  inaccessible and that no normalization, estimand, interval, or division ran.

The rejection receipt may itself be content-identified and signed as a
receipt. It never assigns a valid signed `IF-G1-ENVELOPE` or
`IF-G9-PROTOCOL` or `IF-G9-RUN-MANIFEST` identity to rejected bytes,
fabricates absent memberships, weights, thresholds, mappings, or execution
identity, or contains an outcome-cell disposition. A corrected attempt
receives a new attempt identity and is validated from the beginning.

Only an experiment admitted by a `ValidForOutcomeAccess` record produces the
full reconstructible `ValidExperimentReceipt` below. It records:

- requirement, hypothesis, and configuration identifiers;
- the selected envelope \(e\) and its envelope-specific execution identity:
  for \(e=\mathrm{G1}\), the exact signed `IF-G1-ENVELOPE` content identity
  plus the distinct signed G1 execution-instance identity bound to it; for
  \(e=\mathrm{G9}\), the exact signed `IF-G9-RUN-MANIFEST` content identity;
  each recorded artifact includes its immutable digest and signature;
- its separate `ValidForOutcomeAccess` structural-manifest validation record;
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
- for the selected envelope \(e\), the exact \(I_D^{(e)}\),
  \(I_I^{(e)}\), and \(I_E^{(e)}\) memberships, the base design weight for
  every member, every applicable within-domain or
  \(I_E^{(e)}\)-normalized weight, and the successful partition and
  \(I_E^{(e)}\subseteq I_D^{(e)}\) checks;
- the complete injective \(\kappa_e\) artifact and, for every
  \(i\in I_E^{(e)}\) and \(r\in\mathcal R_E\), the exact mapped condition
  label \(\kappa_e(r)\), execution identity, and role/condition join;
- for every such \((i,r)\), the raw \(Y_r^{(e)}(i)\),
  \(A_r^{(e)}(i)\), and every \(L_{r,c}^{(e)}(i)\) value for
  \(c\in\mathcal C_L\) when realized, plus a separate typed success, missing,
  timeout, corruption, exclusion, or other frozen-policy failure disposition;
  a missing or failed value is never silently encoded as zero;
- realized task, independent-cluster, and required language, task-family, and
  risk-subgroup exposure for \(I_D^{(e)}\), \(I_I^{(e)}\), and
  \(I_E^{(e)}\), their separately frozen minima, and each pre-estimand
  `Eligible` or `Inconclusive` disposition under an already
  `ValidForOutcomeAccess` manifest;
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
- Focus and expectation comparisons pass the exact same complete sealed
  `EligibleActivatedMemorySet<'call>` object and borrow to both branches; no
  projection, copy, or reconstruction may replace that common input before
  either branch. Both branch results carry the same fresh set-instance witness,
  and planning checks it against an independent scope borrow from the selected
  set. Plan and renderer comparisons consume one frozen upstream semantic
  result.
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
- the exact same complete sealed `EligibleActivatedMemorySet<'call>` object and
  borrow at both focus and expectation boundaries, with no pre-branch
  projection, copy, or reconstruction; the fresh set-instance witness is
  propagated through both branches and checked against an independent
  planning-scope borrow, including rejection of a same-call reconstructed set;
- exact-value slot identity and deterministic surface substitution;
- \(K_R\)-only perturbations leave `PlanCanonicalEnvelopeV1` and
  `PlanContentId` unchanged while changing the separately authenticated
  renderer configuration identity or canonical-content commitment;
- renderer, substitution, context construction, and validation compare both
  `RendererConfigurationId` and exact authenticated canonical \(K_R\) content;
  separately authenticated equal-content values are equivalent, whereas
  projections, narrower reconstructions, unauthenticated values, and
  same-ID/different-byte values fail closed;
- file/stdin prompt sources are streamed under `AbsoluteIngressLimitsV1` with
  at most one detection byte beyond the ceiling; already-owned API values and
  direct argument text are checked at each exact maximum and maximum plus one
  before request construction or further internal allocation. Caller and
  process-start allocations that precede the API boundary remain outside the
  compiler guarantee;
- adapters emit no stdout before complete serialization and delivery start,
  and any transport-exposed prefix is invalidated by exit `10` rather than
  described as rollback-safe;
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
- Expectation-eligible-set rubric and exposure, expectation-contribution and
  wrong-expectation margins, anchoring and leakage rubrics, and their maximum
  tolerated rates.
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
- [Decision 0016: Adopt sealed compile-integrity boundaries](../decisions/0016-adopt-sealed-compile-integrity-boundaries.md)
