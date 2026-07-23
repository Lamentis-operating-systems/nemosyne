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

### Formal compile model

Let:

- `P` be the retained original prompt bytes;
- `S` be zero to three situation statements;
- `X` be validated metadata containing declared contextual time `t_context`;
- `U` be the authenticated invocation context outside the request payload,
  including trusted authorization time `t_auth`;
- `ell` be the resolved declared output language;
- `B` be the attention budget resolved by configuration and policy;
- `M^r` be immutable memory revision `r` with policy revision `p`; and
- `K` be one pinned content-identified compiler configuration and immutable
  artifact set.

The proposed logical stages are:

\[
M_A^{r,p,t_{auth},U} = authorize(M^r,U,t_{auth};K)
\]

\[
Q = encode(P,S,X,t_{auth};K)
\]

\[
C = retrieve(Q,M_A^{r,p,t_{auth},U};K)
\]

\[
N = derive(Q,C;K)
\]

\[
R = rank(N;K)
\]

\[
L = plan(Q,R,C,B;K)
\]

\[
Z=(T,segments,bindings) = render(L,\ell;K)
\]

\[
T' = validate(Z,L,P,Q,\ell,B;K)
\]

\[
O = H_a \Vert T' \Vert H_p \Vert P
\]

where:

- `H_a` is the fixed `attention:\n` prefix;
- `H_p` is the fixed `\n\nuser prompt:\n` separator; and
- `||` is byte concatenation.

Each stage is partial: it returns either its complete value or an explicit
error. It does not return a plausible substitute after a failed precondition.
`segments` and `bindings` are untrusted renderer claims that cover `T` and
refer to planned proposition identities. Validation verifies their structure
and returns the exact text component `T` unchanged as `T'` or returns an error;
it is not another rendering stage.

### Formal obligations

The architecture must discharge these obligations under explicit assumptions.

#### F1: Prompt preservation

Because `P` is retained independently and appended as the final operand of
`O`, the final `|P|` bytes of every successful `O` equal `P`, and no byte
follows them. This proof depends on the serializer using the retained buffer
directly and on every adapter delivering the original bytes without
normalization.

#### F2: Authorization before relevance

Candidate generation receives only `M_A^(r,p,t_auth,U)`. Every downstream
source reference must be constructed from the candidate source set or from
the compile request.
Therefore, if constructors prevent forged references and no stage has ambient
memory access, every memory-derived source in a successful plan belongs to
`M_A^(r,p,t_auth,U)`.

Authorization also requires semantic noninterference. For two physical memory
states whose authorized projections for the call are identical, changing only
unauthorized records must not change candidate crowding, ranking, planned
meaning, rendered attention, or content-bearing diagnostics when the request,
invocation context, contextual and authorization times, configuration, and
random tape `omega` are held fixed:

\[
projection_A(M_1)=projection_A(M_2)
\Rightarrow
semanticCompile(M_1;\omega)=semanticCompile(M_2;\omega)
\]

The physical index and search procedure must therefore enforce authorization
before bounded nearest-neighbor or top-k competition, not retrieve a crowded
global top-k and filter afterward. Timing and other side channels require a
separate security model. These properties establish information-flow
eligibility and noninterference, not truth of an authorized record.

#### F3: Snapshot consistency

One immutable memory revision `r`, policy revision `p`, trusted authorization
time `t_auth`, declared context time `t_context`, invocation context `U`, and
authorized view are pinned before memory-dependent work. Authoritative records,
numerical representations, and indexes are checked against `r`; authorization
and disclosure expiry, current normative validity, and supersession use
`t_auth` and `p`; temporal relevance receives both times explicitly, but
`t_context` cannot revive historical instructions as current authority.
Downstream stages receive no ambient store handle or wall clock. One immutable
`K` pins the identities of every policy evaluator and content-identified
artifact handle for the same call. Therefore a successful call is a function
of one logical memory-policy revision, both pinned time values, and one
compiler-artifact set even when a writer or updater publishes a later revision
concurrently.

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

Every planned proposition has a nonempty essential support set. The renderer
returns an internal mapping from each output unit to existing planned
proposition identities, and validation rejects missing or unknown identities.
By construction, every structurally accepted output unit has request or
authorized-memory provenance.

This formal property does not prove that free natural language is semantically
equivalent to the mapped proposition. Semantic support, qualification
preservation, answer leakage, and validator false positives or negatives remain
executable and empirical obligations. Provenance establishes support, not
factual truth.

#### F7: Budget safety

Let `cost_K(T)` be the declared attention-cost function and `B` the available
budget. Validation accepts only when:

\[
cost_K(T) \leq B
\]

Planning and rendering never truncate semantic content after validation. If
mandatory qualified content cannot fit, the call fails with insufficient
budget.

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

### Requirement traceability

| ID | Product requirement | Owning boundary | Required evidence |
| --- | --- | --- | --- |
| `V1-R01` | Authentic prompt, zero to three situations, resolved contextual time, explicit request metadata, and separate trusted caller and authorization time | Invocation context and ingress | Origin, count, contextual-time, metadata, absence, invalid-input, and forged-time authorization-isolation tests |
| `V1-R02` | One complete result or explicit error with separate transport failure | Orchestrator and adapters | F9, failure injection, and adapter delivery tests |
| `V1-R03` | Byte-identical original prompt and exact framing | Ingress and serializer | F1, golden tests, and arbitrary UTF-8 property tests |
| `V1-R04` | Read-only one-revision compilation | Snapshot and compile capability graph | F3, F4, concurrency, configuration-pinning, and write-detection tests |
| `V1-R05` | Authorization before unified cross-context relevance | Policy gate and candidate generation | F2, canary exclusions, cross-context recall, and revocation policy tests |
| `V1-R06` | Source support, qualification, and no authority promotion | Plan and validation | F5, F6, adversarial provenance, and semantic-fidelity cases |
| `V1-R07` | Focus description, not answer, unsupported claim, or raw dump | Planner, renderer, and validation | Proposition labels, leakage, support, and raw-copy metrics |
| `V1-R08` | Declared language, finite budget, faithful empty attention, and budget error | Planner, renderer, and validation | Per-language evaluation and exact budget-boundary tests |
| `V1-R09` | Local memory and no compile network access or disclosure | Runtime and packaging | Network-denied integration, capability audit, and storage-location tests |
| `V1-R10` | No discovery, downstream AI invocation, or automatic learning | Compile dependency boundary | Capability tests and prohibited-call detection |
| `V1-R11` | Numerical relevance after ingress with retained exact evidence | Encoding through planning | Schema, reconstruction-limit, provenance, and perturbation tests |
| `V1-R12` | Coding agents are the first supported domain and claims remain bounded | End-to-end harness and release process | Sealed coding-task outcomes and frozen evidence receipts |
| `V1-R13` | Memory management remains separate from compile | Compile capability boundary | Absence of management dependencies, persistent-write detection, and explicit rejection of management requests |

### Executable conformance program

Implementation evidence must include:

- boundary tests for every public constructor and error class;
- property tests over arbitrary valid UTF-8 prompts and line endings;
- fuzzing of framing, metadata, state decoding, and persistent input;
- model-based storage tests for snapshot publication and concurrent writers;
- metamorphic tests for input permutations, absent optional metadata, and
  irrelevant candidate additions;
- canary memories that are highly relevant but unauthorized;
- unauthorized near-neighbor additions that would crowd a global top-k;
- stale, superseded, contradictory, low-confidence, and malicious memories;
- exact values that cannot be reconstructed safely from a lossy vector;
- cross-project relevant records and same-project irrelevant records;
- missing, stale, incompatible, and corrupt derived artifacts;
- approximate-index misses and degraded search;
- empty memory with justified empty and nonempty attention;
- no situation statements and three situation statements;
- mixed, ambiguous, unsupported, and explicitly selected languages;
- answer leakage, raw copying, unsupported clauses, and lost qualifications;
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
| H5 | Proposed planning improves coverage and exclusion when ranking, budget, and renderer are held fixed | Planner-only comparison over frozen ranked inputs |
| H6 | The renderer preserves planned meaning without material unsupported claims or answer leakage | Candidate renderer versus deterministic and expert rendering |
| H7 | The complete compiler improves context-dependent task success without unacceptable harm on context-independent tasks | Candidate V1 versus prompt only and the strongest frozen non-oracle |
| H8 | V1 meets frozen local resource budgets on reference hardware | Cold/warm operational measurements |

Failure of H1 rejects the product premise before full architecture
implementation. Failure of a later hypothesis directs work to its owning
stage rather than permitting unrelated tuning.

Component comparisons reuse frozen intermediate artifacts. A ranker comparison
must not rederive signals, a planner comparison must not rerank candidates, and
a renderer comparison must use the identical structured plan.

### Conditions and ablations

Each sealed task is run under frozen, token-matched conditions where
applicable:

1. original prompt only;
2. prompt plus situation and metadata, without persistent memory;
3. situation-only condition plus irrelevant placebo attention of matched size;
4. token-matched raw context;
5. token-matched semantic-similarity top-k;
6. strongest frozen deterministic non-oracle baseline;
7. Nemosyne selection with a deterministic renderer;
8. Nemosyne selection with the candidate renderer;
9. one frozen expert-authored attention plan with the candidate renderer; and
10. expert/reference rendering of that identical frozen expert plan.

All conditions use the same scenario-specific `t_context`, `t_auth`, memory
revision `r`, policy revision `p`, authorized-view identity, downstream model
version, message role and placement, decoding configuration, tool access, seed
schedule, and effective budget. Every task-condition starts from the same
content-hashed repository and environment snapshot in a fresh isolated process
and model session. Mutable caches, tool state, files, and background processes
are reset or identically preseeded. Condition order is randomized only after
this carryover isolation. Only the named treatment changes.

H4 and H5 additionally use component-level swaps over frozen signals, rankings,
and plans; the ten end-to-end conditions alone do not identify those internal
effects.

These comparisons isolate:

- situation value: `2 - 1`;
- prompt-length or placebo effects: `3 - 2`;
- persistent-memory value: memory conditions against `2`;
- selection quality: `8` against `9`;
- renderer quality: `7` against `8`, and `9` against `10`;
- complete product value: `8` against `1`, `2`, and the strongest of `4` to
  `6`; and
- remaining headroom: `10 - 8`.

Conditions `9` and `10` differ only in rendering. The expert plan and reference
rendering obey the same source, authority, language, placement, size, and
non-answering contract. The expert reference is not an oracle or a proven
optimum.

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

#### Rendering

Measure:

- clause-level source traceability;
- mandatory-proposition coverage;
- unsupported-claim rate;
- qualification and conflict preservation;
- normative-authority violations;
- answer leakage;
- language match;
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
independent tasks. The binary task-level estimands use frozen normalized design
weights `v_i > 0`, with `sum_i v_i = 1`. A self-weighting sample uses
`v_i = 1/N`. A stratified, unequal-probability, or deliberately balanced sample
must derive and freeze its weights from the sampling design before outcomes
are observed. If no population sampling design is claimed, uniform weights
estimate only the sealed evaluation set.

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
- validator false acceptance and false rejection;
- average improvement with harm concentrated in one declared subgroup; and
- equal or tiny activation margins that flip under permitted perturbation.

### Decision and stop gates

The proof program proceeds in risk order:

| Gate | Required result | Failure action |
| --- | --- | --- |
| G0: Contract | Product, architecture, and proof documents are internally consistent | Resolve contracts before implementation |
| G1: Headroom | Expert reference beats prompt-only and situation-only by the predeclared meaningful margin | Reject or narrow the V1 premise |
| G2: Boundary model | Formal obligations are reviewed and executable harnesses can represent every boundary | Do not select implementation technologies |
| G3: Renderer feasibility | The candidate renderer faithfully renders frozen expert plans within local budgets | Replace or constrain rendering before retrieval integration |
| G4: Memory read and snapshots | Supplied revisions, authorization views, pinned indexes, concurrent publication, and compile/management separation satisfy their contracts | Do not build persistent-memory retrieval |
| G5: Retrieval | Required-proposition recall and cross-context behavior beat frozen simple baselines | Replace or simplify retrieval |
| G6: Ranking and planning | Fixed-intermediate comparisons show value over semantic and rule baselines | Do not calibrate a mechanism without added value |
| G7: Vertical slice | All critical invariants and resource budgets hold in an offline local integration | Do not build release packaging |
| G8: Sealed evaluation | Every superiority, non-inferiority, harm, critical, and operational gate passes | Report failure or inconclusive evidence; do not relabel it |

Stop or redirect work when:

- expert attention fails to establish product headroom;
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
- A simpler baseline is not omitted because it performs well.
- Failed, timed-out, and harmed cases remain visible.
- No metric treats activation as probability, truth, or safety.
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
- Annotation protocol and agreement threshold.
- Retrieval, renderer, and operational metric thresholds.
- Evidence-receipt serialization and long-term storage.

These values must be resolved before sealed evaluation. They must not be tuned
after the sealed outcomes are known.

## References

- [V1 product contract](v1-product-contract.md)
- [V1 reference architecture](v1-reference-architecture.md)
- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Activation parameter evaluation](activation-parameter-evaluation.md)
- [Curated activation evidence](curated-activation-evidence.md)
- [Decision 0011: Adopt a local read-only attention compiler for V1](../decisions/0011-adopt-local-read-only-attention-compiler-v1.md)
