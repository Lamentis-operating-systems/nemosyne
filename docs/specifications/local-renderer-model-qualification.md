# Local renderer model qualification

Status: Proposed

## Purpose

This specification defines how Nemosyne will qualify a small multilingual
language model for the local vector-to-attention renderer. It fixes the
candidate slate, comparison boundary, evidence categories, promotion rule, and
reproducibility requirements before model-specific results are observed.

Qualification applies only to rendering an already selected structured
`FocusExpectationPlan`. It does not evaluate memory storage, retrieval,
activation, expectation generation, planning, authorization, or the downstream
AI system as a whole.

No current model is designated the V1 winner. Published parameter counts,
language coverage, context lengths, and general benchmarks establish candidate
eligibility, not Nemosyne-specific fidelity, usefulness, latency, or memory
behavior. A later decision may select one complete renderer configuration only
after the frozen evaluation defined here has run.

Qwen3 is the first integration family because its official checkpoints are
Apache-2.0, multilingual, text-only causal language models, its model interface
accepts direct input embeddings, and MLX-LM supports both local Apple-Silicon
execution and direct embedding input. This ordering reduces integration risk;
it is not evidence that Qwen3 will win qualification.

## Definitions

### Local notation

The symbols in this specification are local to renderer qualification unless a
linked owner states otherwise.

| Symbol | Domain | Meaning |
| --- | --- | --- |
| \(c_{s,q}\) | immutable artifact tuples | One complete candidate configuration for training seed \(s\) and precision contract \(q\) |
| \(C_{\mathrm{qual}}\) | finite configuration cohorts | All paired reference- and deployment-precision configurations for one candidate under the frozen seed set |
| \(\mathcal S_{\mathrm{qual}}\) | finite nonempty seed sets | Frozen unique training-seed identities used by every qualification cohort |
| \(m_{\mathrm{qual}},r_{m_{\mathrm{qual}}},h_{m_{\mathrm{qual}}}\) | model identity, revision identity, and positive hidden dimension | One renderer-qualification candidate model and its fixed properties |
| \(N_{\mathrm{qual}}^{\mathrm{virtual}}\) | positive bounded integer | Registered virtual-embedding count used by one qualification condition |
| \(\Pi_{m_{\mathrm{qual}}}(L)\) | \(\mathbb R^{N_{\mathrm{qual}}^{\mathrm{virtual}}\times h_{m_{\mathrm{qual}}}}\) | Candidate bridge projection of plan \(L\) for model \(m_{\mathrm{qual}}\) |
| \(d_{\mathrm{qual}},\nu_{\mathrm{qual}}\) | immutable configuration identities | Complete decoding-and-stop configuration and plan/slot/structural-validation configuration |
| \(G_{\mathrm{qual}}\) | cohort predicates | Every frozen technical and empirical gate passes |
| \(D_{\mathrm{qual}}\) | cohort predicates | The selected deployment artifact is compatible with the declared installation and redistribution profile |
| \(\mathcal C_{\mathrm{qual,pass}}\) | finite cohort sets | Cohorts satisfying both qualification and deployment-compatibility predicates |

### Qualification subject

The qualification manifest defines one finite nonempty set
\(\mathcal S_{\mathrm{qual}}\) of unique training-seed identities before any
cohort is constructed. An empty set, duplicate seed identity, or seed outside
that frozen set is a manifest error.

A **candidate artifact configuration** for training seed
\(s\in\mathcal S_{\mathrm{qual}}\) and precision contract `q` is the complete
content-identified tuple:

\[
c_{s,q} =
(
m_{\mathrm{qual}},r_{m_{\mathrm{qual}}},t,r_t,e_s,r_{e_s},\phi_s,r_{\phi_s},
\Delta_s,r_{\Delta_s},s,q,\rho,d_{\mathrm{qual}},\nu_{\mathrm{qual}},
v,r_v,\tau_v
)
\]

where:

- \(m_{\mathrm{qual}}\) and \(r_{m_{\mathrm{qual}}}\) identify the base model
  and immutable model revision;
- `t` and `r_t` identify a derived tokenizer or processor artifact created
  from an immutable official revision by the deterministic, content-identified
  exact-slot augmentation;
- `e_s` and `r_e_s` identify the derived model artifact containing the
  deterministic vocabulary extension and the trained appended slot rows for
  seed `s`, while original model rows remain byte-identical to the base
  revision;
- `phi_s` and `r_phi_s` identify the trained numerical bridge and revision for
  seed `s`;
- `Delta_s` and `r_Delta_s` identify an optional LoRA adapter and revision for
  seed `s`;
- `s` identifies one seed from the frozen training-seed set;
- `q` identifies numerical precision and quantization;
- `rho` identifies the runtime, kernels, and backend;
- \(d_{\mathrm{qual}}\) identifies the complete decoding and stop
  configuration;
- \(\nu_{\mathrm{qual}}\) identifies the attention-plan schema, exact-slot
  vocabulary, deterministic formatter, substitution logic, and structural
  validator;
- `v` and `r_v` identify the independently trained semantic-verifier artifact
  and immutable revision; and
- `tau_v` identifies its calibrated threshold vector and calibration receipt.

The same qualified verifier tuple \((v,r_v,\tau_v)\) is frozen for every
renderer cohort in one protocol revision. It is not retuned per renderer,
candidate family, seed, precision, language, or observed failure. Before any
renderer cohort enters sealed evaluation, that verifier must independently
pass the architecture, split, calibration, false-acceptance, and
false-rejection contract in the vector-to-attention renderer specification.
If it does not pass, no generative renderer configuration is eligible.

A **qualification cohort** \(C_{\mathrm{qual}}\) contains exactly one paired
reference-precision and deployment-precision configuration for every
\(s\in\mathcal S_{\mathrm{qual}}\), and no configuration for another seed,
while all non-seed training choices remain identical. A missing or duplicate
pair member is a cohort-construction error. Because
\(\mathcal S_{\mathrm{qual}}\) is nonempty, every valid cohort contains at
least one deployment configuration. The manifest's development-only
\(deploy(C_{\mathrm{qual}})\) rule is total over a valid cohort and selects
exactly one of those deployment configurations before the sealed suite is
opened. The sealed quality and repeatability gates apply to the entire cohort,
not only to \(deploy(C_{\mathrm{qual}})\).

Qualification selects \(deploy(C_{\mathrm{qual}})\) from a passing cohort, not an unversioned
model name or a favorable seed. Changing any tuple member, seed set, pairing,
or development-selection rule creates a new cohort that does not inherit prior
evidence.

### Common numerical input

Every candidate receives the same immutable structured `FocusExpectationPlan`
`L` defined by the focus-and-expectation-planning specification and projected
according to the vector-to-attention renderer specification. Plan-item facets,
scalars, focus and expectation roles, ranks, masks, proposition identities,
conditions, horizons, support semantics, alternatives, counterevidence,
coverage, abstention state, exact-slot identities, binding roles, output
language, and budget are byte-identical before candidate-specific projection.
The complete validator-control collection is also byte-identical across
candidates, including exclusions, dependency groups, authority ceilings,
required qualifiers, omitted support, abstention, and no-answer/no-action
controls. It remains verifier input and is not projected into any candidate's
generative prefix.

The benchmark never serializes the numerical plan as decimal text. For model
\(m_{\mathrm{qual}}\) with hidden dimension \(h_{m_{\mathrm{qual}}}\), the
candidate bridge maps the same plan to
\(N_{\mathrm{qual}}^{\mathrm{virtual}}\) virtual input embeddings:

\[
\Pi_{m_{\mathrm{qual}}}(L)
\in
\mathbb{R}^{
N_{\mathrm{qual}}^{\mathrm{virtual}}
\times
h_{m_{\mathrm{qual}}}
}
\]

The frozen benchmark uses the same registered set of prefix lengths, bridge
architecture classes, training examples, semantic scenarios, target texts,
exact-slot tables, loss definitions, optimizer budget, random seeds, and
stopping policy for every candidate. Candidate-specific output projections
may differ only where the model hidden dimension requires it.

Each candidate tokenizes the same canonical UTF-8 control text and generation
marker with its pinned derived tokenizer. The derived artifact records its
immutable official base revision and the deterministic slot-vocabulary
augmentation required by the renderer contract. The control text contains no
user prompt, memory prose, or candidate-specific hints. Approved exact-value
surface bytes remain in the deterministic sidecar and are substituted after
generation as specified by the renderer contract.

### Candidate slate

The mandatory initial slate is:

| Candidate | Officially published facts relevant to eligibility | Qualification role |
| --- | --- | --- |
| `Qwen/Qwen3-0.6B` | 0.6B parameters, 0.44B non-embedding parameters, 28 layers, hidden size 1024, 32,768-token advertised context, Apache-2.0, Qwen3 coverage of 119 languages and dialects | Minimum-capacity and resource-floor baseline |
| `Qwen/Qwen3-1.7B` | 1.7B parameters, 1.4B non-embedding parameters, 28 layers, hidden size 2048, 32,768-token advertised context, Apache-2.0, Qwen3 coverage of 119 languages and dialects | Initial capacity and integration reference |
| `Qwen/Qwen3.5-0.8B` | 0.8B language model, hidden size 1024, 24-layer hybrid linear/full-attention layout, 262,144-token advertised context, vision encoder in the published artifact, Apache-2.0, stated coverage of 201 languages and dialects | Newer multilingual and hybrid-architecture challenger |
| `google/gemma-3-1b-it` | 1B text-only instruction-tuned model, 32K input context, a family-level claim of multilingual tokenization and support for more than 140 languages, and Gemma terms rather than Apache-2.0 | Independently licensed and independently designed control; no language is qualified by the family-level claim alone |

`Qwen/Qwen3.5-2B` is the first optional capacity fallback. Its official model
card describes a 2B language model with hidden size 2048, the same 24-layer
hybrid layout and 262,144-token context class as the 0.8B model, Apache-2.0,
and coverage of 201 languages and dialects. It enters sealed qualification
only if no mandatory smaller candidate passes every gate, or if a predeclared
diagnostic run is needed to test whether failure is capacity-related.

Qwen3.5 is evaluated in language-model-only mode. Omitting the unused vision
weights is permitted only through a pinned, reproducible conversion that
preserves every language-model tensor and records both source and converted
artifact digests.

These facts are publisher claims and configuration facts linked in
`References`. Language counts do not establish usable quality for every listed
language. Advertised context beyond the frozen benchmark input length confers
no qualification advantage. General instruction-following scores are not used
because they do not measure numerical-prefix fidelity, exclusions, answer
leakage, exact-slot use, downstream utility, or Apple-Silicon resource behavior
under this contract.

The Gemma publisher material states family-level multilingual coverage and a
multilingual tokenizer, but those statements are not per-language
qualification evidence for this task or checkpoint. The manifest therefore
grants `gemma-3-1b-it` no supported language merely from publisher prose. Each
declared language must pass the same frozen Nemosyne-specific strata before
the configuration can enter a release cohort.

### Integration order

Integration proceeds in this order:

1. wire and test direct embedding input with Qwen3-0.6B;
2. repeat the identical interface checks with Qwen3-1.7B;
3. add Qwen3.5-0.8B and verify its hybrid cache and embedding behavior;
4. add Gemma 3 1B IT under its separate access and license conditions; and
5. add Qwen3.5-2B only under the fallback rule.

The order may expose implementation defects earlier. It cannot exclude a
mandatory candidate, change that candidate's data or budget, or influence the
sealed selection rule.

### Qualification manifest

One immutable qualification manifest fixes before training or sealed
evaluation:

- dataset and semantic-scenario revisions and split membership;
- independently qualified semantic-verifier artifact, verifier-only training,
  calibration and sealed-evaluation splits, corruption generators, head schema,
  confidence procedure, thresholds, and per-stratum gates;
- numerical plan schema and exact-value sidecar schema;
- mandatory and excluded propositions and span-level support labels;
- expectation-kind, condition, horizon, alternative, counterevidence,
  coverage, abstention, and `EvidenceShareNotProbability` labels;
- supported language and script strata;
- prefix lengths and bridge architecture variants;
- bridge-only and optional bridge-plus-LoRA training contracts;
- trainable parameter sets and equal hyperparameter-search budgets;
- the finite nonempty training-seed set
  \(\mathcal S_{\mathrm{qual}}\), optimizer, schedules, early stopping, and
  maximum work;
- BF16 reference and local deployment quantization contracts;
- control text, slot vocabulary, generation marker, and output budget;
- decoding, stop, repetition, and malformed-output policy;
- runtime, dependency, kernel, and operating-system versions;
- the intended local installation and redistribution profile and each
  candidate's compatibility with it;
- minimum-supported and reference Apple-Silicon hardware profiles;
- measurement warm-up, repetition, isolation, estimators, comparison
  resolution, rounding, downstream-evaluation random tapes, and confidence
  procedures; renderer inference itself accepts no request-time randomness;
- every metric definition and hard threshold;
- downstream target-model configurations and evaluation rubric; and
- the deterministic final selection ordering.

Verifier scenario roots are disjoint from renderer training and development
roots, every renderer model-qualification sealed root, and every end-to-end
sealed root. A previously opened verifier case can never become later sealed
evidence. The verifier artifact and thresholds are frozen before renderer
sealed outputs are opened. Renderer outputs may be measured by the verifier,
but they may not update its weights, thresholds, labels, or rejection classes.

The manifest uses immutable artifact revisions and cryptographic content
digests and is authenticated by the installation trust root defined by the V1
architecture. Content identity without manifest authenticity is insufficient.
Mutable repository names, branches such as `main`, floating package versions,
and model aliases are insufficient.

### Required training conditions

Each mandatory candidate is evaluated first with the base model frozen and
only the registered bridge components trainable. This is the **bridge-only**
condition.

If bridge-only misses a frozen development gate, the candidate may enter the
registered **bridge-plus-LoRA** condition. Sealed held-out results cannot
trigger or shape further training. The base checkpoint remains frozen; only
the bridge and the predeclared low-rank updates are trainable. LoRA searches
receive the same rank set, target-module policy, training examples, optimizer
budget, and selection rule across candidates, subject only to published
module-name differences recorded in the manifest.

A candidate that passes bridge-only is not required to add LoRA. A LoRA
configuration cannot replace, erase, or retrospectively reinterpret its
bridge-only result.

### Reference and deployment precision

For every seed, the candidate is evaluated in the frozen unquantized reference
precision and in the target local deployment quantization. Only the deployment
configuration is selectable. It must pass all absolute gates and the
predeclared maximum regression from its paired reference configuration for the
same seed. A passing reference configuration cannot rescue a failing
deployment configuration.

Evidence does not transfer between quantizations, runtimes, kernels, or model
conversions. Quantized artifact size alone is not a quality result.

## Preconditions

- The V1 attention-plan and exact-slot contracts are frozen for the
  qualification revision.
- The benchmark contains no private user memory unless a later explicit data
  governance decision permits it.
- Variants of one semantic scenario occur in exactly one dataset split.
- The held-out split and its hard thresholds are sealed before any candidate
  is evaluated on it.
- Every mandatory candidate has the same eligible training, development, and
  held-out semantic scenarios.
- The authenticated manifest defines a finite nonempty
  \(\mathcal S_{\mathrm{qual}}\), and every candidate cohort contains exactly
  one reference/deployment pair for every seed in that set.
- Candidate-specific tokenization uses the same canonical control bytes and
  conveys no additional semantic information.
- The prefix-length set and all model-independent bridge parameters are the
  same across candidates.
- Candidate-specific bridge parameters exist only where hidden dimensions or
  published module names require them and are recorded explicitly.
- All inputs fit within the smallest advertised context and the frozen local
  runtime limit. Larger context windows receive no extra examples.
- The runtime exposes direct input embeddings with correct masks, positions,
  and cache behavior. A text-only wrapper that accepts only token IDs is not a
  conforming substitute.
- Thinking or reasoning output is disabled explicitly where the model
  supports it. Qwen3 uses its hard `enable_thinking=False` switch. Qwen3.5 is
  pinned to non-thinking behavior even though the 0.8B checkpoint defaults to
  non-thinking.
- The renderer has no network capability during training evaluation, local
  resource measurement, or V1-style inference.
- The qualification manifest's authenticity and intended policy scope have
  been verified against the pinned trust root before any contained digest is
  trusted.
- Model, tokenizer, runtime, and redistribution terms have been reviewed
  before an artifact is downloaded, trained, packaged, or distributed. A
  selectable candidate has an explicit recorded determination that the
  intended installation and distribution profile is compatible with those
  terms.
- Resource measurements run on an otherwise declared and controlled host
  state with thermal, power, and competing-workload conditions recorded.

Failure of a precondition makes the comparison incomplete. It does not count
as a passing or losing quality result.

## Invariants

### No unsupported winner

No candidate may be selected from model cards, general benchmarks, parameter
count, release date, anecdotal prompts, or one language alone. First
integration, easiest runtime support, or author preference does not confer a
qualification advantage.

All mandatory candidates must complete the frozen technical protocol. A
candidate that may be evaluated under its terms but is incompatible with the
intended installation or redistribution profile remains a measured,
non-selectable control. If a mandatory candidate cannot legally or technically
be evaluated at all, the qualification remains incomplete unless a later
decision changes the slate before sealed results are inspected.

### Identical semantic work

Every candidate receives the same numerical inclusion-item view, exact-slot
identities, language, budget, target meaning, and downstream cases. The shared
verifier receives the same control-only exclusions for every candidate; the
generator receives none of them. The projector may adapt tensor width; it may
not add candidate-specific facts, prompts, examples, or retrieval.

The original user prompt is never supplied to the renderer. The evaluation
harness retains that prompt and the separately held labels outside the renderer
so it can detect whether generated text nevertheless begins answering the
request.

### Non-thinking surface realization

The model produces only the candidate attention surface text and internal
bindings required by the renderer contract. A `<think>` block, separate
reasoning field, hidden-reasoning transcript, simulated human chain of thought,
tool call, or answer to the original request is a qualification failure.

The intended target is concise source-bound focus-and-expectation text. Fluent
prose does not compensate for unsupported meaning, lost qualifications,
hypothesis-to-fact promotion, probability inflation, or action selection.

### Artifact isolation

No model, tokenizer, bridge, LoRA, slot table, runtime, or decoding setting may
change between measured cases in one candidate configuration. No generated
prefix, KV cache, exact sidecar, or request-local state is reused across
independent cases.

Cold measurements begin without resident candidate weights or generated cache;
the process-start and file-system page-cache policy is pinned in the manifest.
Warm measurements begin from the exact resident state defined there. Unload
measurements include weights, bridge, LoRA, prefix buffers, KV cache, and
request-local bindings.

### Frozen gates

Hard thresholds are fixed before sealed evaluation. A threshold cannot be
weakened, a metric removed, a language dropped, or a case relabeled after
candidate results are known. A later protocol revision must preserve the
original report and rerun every candidate.

An overall average cannot hide a failed mandatory language, script,
proposition role, expectation kind, horizon class, abstention class,
exact-value type, conflict class, or resource gate. Missing measurements and
validator errors fail the applicable gate.

### Local qualification

Resource claims apply only to the pinned hardware, operating system, runtime,
quantization, input sizes, and lifecycle. No result establishes performance on
another Mac or makes the renderer "imperceptible" without a separately defined
and passed user-facing threshold.

## Metrics and gates

The vector-to-attention renderer specification owns metric semantics,
instrument inputs, and violation classes. This qualification specification
owns the mandatory cohort strata, aggregation, threshold-freezing procedure,
promotion rule, and deployment artifact. The list below names required views;
it does not create a parallel metric definition.

### Semantic fidelity

The report measures at least:

- mandatory-proposition precision and recall;
- preservation of negation, uncertainty, temporal qualification, conflict,
  source authority, and dominant-versus-secondary relationships;
- preservation of expectation kind, condition, horizon, competing
  alternatives, counterevidence, coverage qualification, and abstention;
- hypothesis-to-fact promotion rate;
- probability- or confidence-inflation rate for relative support;
- unsupported-action rate;
- unsupported-claim rate;
- prohibited causal or normative strengthening;
- must-exclude violation rate;
- raw-source copying beyond approved exact slots;
- clause-to-proposition support coverage; and
- malformed, repeated, empty-for-nonempty, and over-budget output rates.

Metrics are reported overall and by semantic scenario, plan size, proposition
role, expectation kind, horizon class, abstention state, qualification type,
and worst mandatory stratum.

The report separates raw renderer quality from post-validation system
behavior. It includes validator false acceptance, false rejection, abstention,
and error rates for every required language and violation stratum. Rejecting
every output cannot pass the maximum false-rejection or minimum successful
compilation gates, while fluent raw output cannot excuse a false-acceptance
failure.

### Exact-value behavior

The report measures:

- exact-slot precision and recall;
- missing, duplicate, swapped, unknown, and unauthorized slots;
- byte-identical substitution of names, paths, URLs, identifiers, numbers,
  dates, times, locations, and non-ASCII values;
- grammatical placement after deterministic substitution; and
- final budget compliance after slot expansion.

Exact-value correctness comes from deterministic slots, not from asking the
model to reconstruct literal values from vectors.

### Language behavior

For every declared language and script stratum, measure:

- output-language match;
- unintended language or script switching;
- preservation of intentional language-independent exact slots;
- semantic-fidelity metrics within that language;
- exact-slot behavior within that language; and
- budget and malformed-output rates.

Publisher language counts are reported only as metadata. Qualification support
is limited to the frozen languages whose individual gates pass.

### Leakage and authority

The report measures:

- answer leakage;
- excluded-proposition leakage;
- raw-memory or raw-source reconstruction;
- injection-like content changing the control behavior;
- output that strengthens data into an instruction, goal, preference, or
  certainty without plan support;
- output that turns an expectation into an observed fact or evidence share into
  probability;
- output that selects a downstream action or validation tool;
- reasoning-trace or tool-call emission; and
- cross-case state or cache leakage.

A leak in a mandatory adversarial class is not averaged away by benign cases.

### Downstream effect

For each frozen downstream target-model configuration, compare:

1. original prompt without attention;
2. prompt with deterministic reference attention; and
3. prompt with candidate-rendered attention.

Evaluation uses the same prompts, random tapes, target-model revisions, and
predeclared rubric. Report helpful, neutral, and harmful changes overall and by
scenario class. A candidate must satisfy both the minimum utility gate and the
maximum harm gate. Renderer fluency or agreement with a teacher is not a
substitute for downstream evidence.

### Local resource behavior

On each declared Mac profile, measure:

- candidate-exclusive installed artifact bytes;
- cold model-and-bridge load time;
- cold end-to-end rendering latency;
- warm prefix-processing latency;
- warm generation latency and total latency;
- p50, p95, and worst observed latency under the manifest procedure;
- peak unified-memory delta;
- resident memory while warm;
- residual memory after the declared unload boundary;
- generated token count and decode throughput; and
- quantized-versus-reference regressions.

The minimum-supported hardware profile is the selection host. Reference
hardware results are diagnostic and cannot rescue a failure on the minimum
profile.

**Candidate-exclusive installed artifact bytes** are the sum of the
uncompressed logical byte lengths of unique, content-identified regular files
added to a frozen clean runtime baseline and reachable from the deployed
candidate manifest. They include installed model or converted language-model
tensors, the derived tokenizer and slot vocabulary, bridge, optional LoRA,
formatter and validator assets, and candidate-only runtime dependencies.
Byte-identical files in the common baseline are excluded; caches, logs,
temporary conversion output, and training checkpoints not used at inference
are excluded. A content digest is counted once even if referenced through
multiple paths. Removing unused Qwen3.5 vision weights affects this measure
only through the pinned conversion already required by this specification.
The one verifier tuple shared by every candidate belongs to the frozen common
baseline. Its installed bytes, load time, latency, and memory remain separately
reported as fixed end-to-end system overhead; they are not attributed
differently to renderer candidates.

For latency, the manifest freezes the p95 estimator and a deterministic
one-sided 95-percent confidence-bound procedure, including repetition count,
resampling seed, and integer comparison unit. The selection key is the upper
confidence bound rounded upward to that unit. Values with the same resulting
integer key tie. Peak-memory comparison uses the maximum observed delta rounded
upward to the frozen integer unit. These rules are fixed before sealed
measurement.

### Repeatability

Every stochastic training condition uses all frozen seeds. Renderer generation
uses the frozen deterministic decoding policy and no request-time random tape.
Downstream target-model experiments may use frozen random tapes that belong to
the evaluation manifest, not the renderer contract. Every seed has one paired
reference and deployment artifact, and the development-only
\(deploy(C_{\mathrm{qual}})\) rule
is fixed before held-out evaluation. The report includes:

- variation across training seeds;
- output and metric variation across repeated inference runs;
- platform or kernel divergence;
- non-finite values and runtime failures; and
- complete per-case provenance sufficient to reproduce any aggregate.

One favorable seed cannot qualify a cohort. Every frozen seed pair must pass
the absolute and paired-regression gates, and the cohort-level repeatability
gate must pass.

## Selection rule

Let \(G_{\mathrm{qual}}(C_{\mathrm{qual}})\) mean that every seed's deployment
configuration in qualification cohort \(C_{\mathrm{qual}}\) passes every
frozen semantic, exact-value, language, leakage,
downstream, quantization, paired-reference-regression, repeatability, and
local-resource gate. Let \(D_{\mathrm{qual}}(C_{\mathrm{qual}})\) mean that
\(deploy(C_{\mathrm{qual}})\) is compatible with the manifest's intended
installation and redistribution profile. Let
\(\mathcal C_{\mathrm{qual,pass}}\) denote:

\[
\mathcal C_{\mathrm{qual,pass}}=
\left\{
C_{\mathrm{qual}}
\mid
G_{\mathrm{qual}}(C_{\mathrm{qual}})
\land
D_{\mathrm{qual}}(C_{\mathrm{qual}})
\right\}.
\]

The universal gate in \(G_{\mathrm{qual}}\) ranges over the nonempty
\(\mathcal S_{\mathrm{qual}}\); it cannot pass vacuously. The total
development rule above makes \(deploy(C_{\mathrm{qual}})\) defined for every
valid cohort before \(D_{\mathrm{qual}}\) is evaluated.

Reference-precision configurations and legally incompatible controls are never
members of \(\mathcal C_{\mathrm{qual,pass}}\), even when their technical
measurements pass.

If \(\mathcal C_{\mathrm{qual,pass}}\) is empty after all mandatory candidates
are complete, run the
Qwen3.5-2B fallback under the identical sealed protocol. If the fallback also
fails, no model is selected. The project retains deterministic rendering,
narrows the output contract, revises the renderer hypothesis, or gathers new
training evidence in a new protocol revision.

The fallback's training and development selection are completed and its
artifacts are frozen before the held-out suite is opened. Mandatory sealed
results may trigger the already frozen fallback evaluation; they cannot be
used to tune it.

Within \(\mathcal C_{\mathrm{qual,pass}}\), select
\(deploy(C_{\mathrm{qual}})\) lexicographically by:

1. lowest candidate-exclusive installed artifact bytes;
2. lowest frozen comparison key for p95 cold end-to-end latency on the
   minimum-supported Mac;
3. lowest frozen comparison key for p95 warm total latency on that Mac;
4. lowest frozen comparison key for peak unified-memory delta on that Mac; and
5. stable deployed artifact identity for an exact tie.

This makes "smallest and fastest" deterministic. Quality metrics above their
hard gates do not permit a larger or slower candidate to win unless a separate
predeclared quality ordering is added before sealed evaluation.

The selected configuration remains experimental until the complete
end-to-end V1 proof program passes. Model qualification alone does not validate
the product.

## Edge cases

### Empty attention plan

An empty plan follows the deterministic empty-attention path and does not
invoke a candidate model. It remains in the suite as a contract and resource
baseline but does not contribute invented-text quality credit.

### Minimal and saturated plans

The suite includes one-item plans, maximum-item plans, plans at every registered
prefix length, plans with no exact values, and plans at exact-slot and output
budget limits. Exceeding a structural limit is an explicit pre-render failure,
not a model-quality sample.

### Focus and expectation dispositions

The suite separately covers focus-only, focus-plus-renderable-abstention,
valid expectation-only, combined, single-hypothesis, and
competing-hypothesis plans. Focus-only fixtures contain no renderer-visible
expectation role; validator-only abstention controls may remain.
Deliberately corrupted targets promote hypotheses to facts, remove conditions
or horizons, hide counterevidence, collapse alternatives, claim probability,
and prescribe unsupported actions. A model cannot pass by learning one fixed
attention-text shape.

### Non-Latin and mixed-language evidence

The suite includes mandatory non-Latin scripts, right-to-left text, combining
characters, and mixed-language source evidence. The resolved output language,
not source-vector language or exact slot bytes, controls the narrative.

### Quantization divergence

A deployment quantization that passes absolute quality but exceeds its frozen
regression ceiling fails. A different quantization is a new configuration and
must rerun the protocol.

### Runtime lacks direct embedding input

The candidate fails integration qualification. Numerical vectors are not
serialized as text to bypass the missing interface.

### Reasoning or answer output

Any reasoning trace, tool call, or answer-like text is counted under its
leakage metric and rejected by the runtime validator where possible. Removing
the visible trace after generation does not turn the run into a passing
non-thinking configuration.

### Access or license incompatibility

An unavailable mandatory artifact leaves the comparison incomplete. A
technically measurable candidate whose terms are incompatible with the
declared installation or redistribution profile remains in the report as a
non-selectable control and cannot enter
\(\mathcal C_{\mathrm{qual,pass}}\). In particular, Gemma access
requires acceptance of Google usage terms; any packaging or redistribution
plan must satisfy those terms independently of technical results.

### New model or revision

A newly released model does not replace a mandatory candidate automatically.
It may be added to a new qualification-manifest revision. Existing sealed
results remain unchanged and the same protocol runs for every candidate in the
new slate.

## Verification

### Interface conformance

Before training, executable tests prove for each candidate runtime:

- direct continuous embedding input without decimal serialization;
- exact prefix shape, dtype, mask, positions, and ordering;
- fixed control-token and generation-marker placement;
- non-thinking configuration and absence of reasoning segments;
- exact-slot token atomicity and substitution behavior;
- output-token and stop enforcement;
- cache isolation across calls;
- BF16 and deployment-quantization loading;
- complete artifact-manifest verification; and
- network-denied local operation.

### Controlled comparison

The development evaluation compares on identical plans:

1. deterministic template rendering;
2. the registered simple MLP bridge;
3. the registered latent-resampler bridge with frozen base model; and
4. latent resampler plus LoRA only where bridge-only missed a gate.

Every condition is evaluated on the same focus-only,
focus-plus-renderable-abstention, valid expectation-only, combined, and
deliberately corrupted semantic scenario roots.

The registered prefix-length set is `8`, `16`, and `32` virtual tokens. The
vector-to-attention renderer specification may evaluate additional ablations,
but final model comparison uses the same frozen set for every candidate.

Development data selects bridge dimensions, one training condition per model
family, and \(deploy(C_{\mathrm{qual}})\) under the frozen rule. Every seed artifact in the
resulting cohort remains in sealed evaluation. The sealed held-out suite is
evaluated once after all mandatory and fallback choices, cohorts, artifacts,
and thresholds are frozen. Test results never feed further training or
candidate-specific prompt changes.

Manifest and cohort-construction tests reject an empty seed set, duplicate
seed identities, a seed outside \(\mathcal S_{\mathrm{qual}}\), a missing or
duplicate reference/deployment pair, and a development-selection rule that
does not return exactly one deployment configuration from the valid cohort.

### Report

The qualification report contains:

- the complete authenticated manifest, trust-root identity, and content
  digests;
- every candidate and configuration attempted;
- precondition, integration, training, and evaluation failures;
- aggregate and per-case metrics;
- results by every mandatory stratum and worst group;
- all bridge-only, LoRA, reference-precision, and quantized results;
- hardware and resource traces;
- final gate outcomes without hidden exclusions;
- the mechanical selection-order inputs; and
- either one selected experimental configuration or an explicit no-selection
  result.

Reproduction from the manifest must yield the same semantic inputs and the
same mechanical selection decision from measurements that satisfy the frozen
repeatability tolerances. If an independent run changes a comparison key or
winner outside those tolerances, qualification is not reproducible.
Bit-identical generated text across different Apple kernels is not assumed
unless separately demonstrated.

## Open questions

- The frozen language and script coverage required for the first coding-agent
  release claim.
- The minimum-supported Mac and exact power and thermal measurement protocol.
- The target deployment quantization and acceptable BF16-to-quantized
  regression.
- The final training corpus, loss weights, search budget, and threshold
  values.
- The downstream target models, judge design, and acceptable helpful,
  neutral, and harmful rates.
- The exact intended installation and redistribution profile used for the
  first qualification manifest.
- The production Rust runtime. MLX-LM is the first research integration, not a
  selected product runtime.

## References

- [V1 product contract](v1-product-contract.md).
- [V1 reference architecture](v1-reference-architecture.md).
- [V1 proof program](v1-proof-program.md).
- [V1 delivery program](v1-delivery-program.md).
- [Predictive attention and expectation](predictive-attention-and-expectation.md).
- [Focus and expectation planning](focus-and-expectation-planning.md).
- [Vector-to-attention renderer](vector-to-attention-renderer.md).
- [Decision 0013 (superseded)](../decisions/0013-adopt-a-vector-prefix-local-renderer-qualification-path.md).
- [Decision 0015](../decisions/0015-render-qualified-focus-and-expectation-plans.md).
- Qwen Team, [Qwen3 Technical
  Report](https://arxiv.org/abs/2505.09388), 2025.
- Qwen, [`Qwen3-0.6B` model
  card](https://huggingface.co/Qwen/Qwen3-0.6B) and
  [`config.json`](https://huggingface.co/Qwen/Qwen3-0.6B/blob/main/config.json).
- Qwen, [`Qwen3-1.7B` model
  card](https://huggingface.co/Qwen/Qwen3-1.7B) and
  [`config.json`](https://huggingface.co/Qwen/Qwen3-1.7B/blob/main/config.json).
- Qwen, [`Qwen3.5-0.8B` model
  card](https://huggingface.co/Qwen/Qwen3.5-0.8B).
- Qwen, [`Qwen3.5-2B` model
  card](https://huggingface.co/Qwen/Qwen3.5-2B).
- Google DeepMind, [Gemma 3 model
  card](https://ai.google.dev/gemma/docs/core/model_card_3) and
  [`gemma-3-1b-it` artifact](https://huggingface.co/google/gemma-3-1b-it).
- Google, [Gemma Terms of Use](https://ai.google.dev/gemma/terms).
- Hugging Face Transformers, [Qwen3 model
  interface](https://huggingface.co/docs/transformers/model_doc/qwen3).
- MLX-LM, [Apple-Silicon generation and fine-tuning
  support](https://github.com/ml-explore/mlx-lm),
  [Qwen3 direct-embedding
  implementation](https://github.com/ml-explore/mlx-lm/blob/main/mlx_lm/models/qwen3.py),
  [Qwen3.5 direct-embedding
  implementation](https://github.com/ml-explore/mlx-lm/blob/main/mlx_lm/models/qwen3_5.py),
  and [LoRA and QLoRA
  guide](https://github.com/ml-explore/mlx-lm/blob/main/mlx_lm/LORA.md).
- MLX, [Unified memory
  documentation](https://ml-explore.github.io/mlx/build/html/usage/unified_memory.html).
