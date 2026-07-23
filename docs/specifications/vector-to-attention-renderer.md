# Vector-to-attention renderer

Status: Proposed

## Purpose

This specification proposes one concrete V1 candidate for rendering a bounded
structured numerical attention plan as natural-language attention text. The
candidate uses typed numerical projectors, a small latent resampler, dynamic
soft-prefix embeddings, and a local Qwen-family causal language model in
non-thinking mode.

The renderer is a surface-realization component. It does not retrieve memory,
rank candidates, select propositions, resolve conflicts, change authority, or
answer the original user prompt. It receives only the authoritative plan
envelope selected by the upstream compiler and a compatible pinned rendering
configuration. Language and budget are fields of that envelope, not separate
inputs.

Decision 0013 accepts this vector-prefix qualification path as the intended V1
implementation direction. The specification remains `Proposed` because no
implementation or qualifying evidence exists. In particular, the initial
dimensions, release checkpoint, quantization, production runtime, and decoding
configuration remain subject to the verification and ablation gates defined
below. They are not biological claims and are not asserted to reproduce human
thought.

## Definitions

### Rendering boundary

Let `L` be the immutable structured attention-plan envelope defined by the
cognitive-memory specification and `K_R` the content-identified rendering
configuration. Let `ell_L` and `B_L` denote the resolved output language and
post-substitution budget stored in `L`. Let `J` be a read-only validation
context containing the retained original prompt, prompt-derived intent labels,
and trusted request policy facts. `J` is never renderer-model input. The
candidate renderer computes:

\[
Z =
\operatorname{render}_{K_R}(L)
\]

where `Z` is an internal `RenderedAttention` value containing:

- generated attention text before exact-value substitution;
- a complete segmentation of that text;
- untrusted bindings from output units to planned proposition identities; and
- the exact-value slot occurrences claimed by the renderer.

Deterministic slot substitution produces:

\[
Z_{\mathrm{exact}} =
\operatorname{substitute}(Z,\operatorname{slots}(L),K_R)
\]

Faithfulness and policy validation then either returns the exact substituted
text unchanged or returns an explicit error:

\[
T =
\operatorname{validate}(Z_{\mathrm{exact}},L,J,K_R)
\]

The successful product result remains the compiled text defined by the V1
product contract. The numerical prefix, model tokens, segment bindings, and
slot table are internal artifacts and are not additional product results.

### Canonical plan items

The renderer does not accept one undifferentiated memory vector. It consumes
the one authoritative `L` envelope; it does not construct a second renderer
plan. The selected inclusion items in that envelope form the finite,
canonically ordered sequence:

\[
\operatorname{items}(L)=(u_1,\ldots,u_n)
\]

Each item is:

\[
u_i =
\left(
\{v_{i,f}\}_{f\in F},
x_i,
\rho_i,
r_i,
m_i,
\pi_i,
\Gamma_i,
\mathcal A_i,
P_i,
S_i,
d_i
\right)
\]

where:

- `F` is the versioned set of numerical facets;
- \(v_{i,f}\in\mathbb{R}^{d_f}\) is the item value in facet space `f`;
- \(x_i\in\mathbb{R}^{q}\) contains validated scalar features;
- \(\rho_i\) is the plan-item role;
- \(r_i\) is its canonical plan rank;
- \(m_i\) is the facet-presence mask;
- \(\pi_i\) is the stable planned-proposition identity; and
- \(\Gamma_i\) contains typed dominant, secondary, conflict, and qualification
  relations;
- \(\mathcal A_i\) is the authority ceiling and
  current-versus-historical usage class;
- \(P_i\) contains essential request or authorized-memory support and
  provenance identities;
- \(S_i\) is the set of typed exact-value slot bindings permitted for the
  item; and
- \(d_i\) is its mandatory or optional disposition.

The envelope additionally carries the exact-value sidecar, global typed
relations, mandatory and optional proposition sets, the control-only exclusion
records `X_L`, `ell_L`, and `B_L`. Every upstream plan field is either
represented in the item tensor view or retained losslessly for control,
substitution, and validation. No lossy `FocusPlan`-to-`RendererPlan` projection
exists.

`X_L` is never projected into the generative prefix and none of its exact
surfaces is a substitution slot. It is passed only to deterministic and
semantic validators. This prevents an exclusion from becoming a negatively
marked but still model-visible prompt while ensuring that no exclusion can be
silently optimized away.

Facet spaces are typed and versioned. A semantic embedding, temporal encoding,
activation score, authority feature, and uncertainty feature are not
interchangeable merely because all are numerical.

The role vocabulary, scalar schema, facet dimensions, normalization rules,
maximum item count, and canonical ordering belong to the pinned attention-plan
schema. An implementation must reject an incompatible schema rather than
silently projecting it.

### Exact-value sidecar

Loss-sensitive values are not reconstructed from numerical representations.
The plan carries an authorized exact-value sidecar:

\[
V =
\{(s_j,a_j,b_j,\tau_j,\Omega_j,n_j^{\min},n_j^{\max})\}_{j=1}^{m}
\]

where:

- \(s_j\) is a fixed renderer slot identity;
- \(a_j\) is the authoritative exact typed value;
- \(b_j\) is the exact approved UTF-8 surface byte sequence produced from
  \(a_j\) by the pinned deterministic formatter;
- \(\tau_j\) is its value type;
- \(\Omega_j\) is the set of permitted proposition-identity and semantic-role
  pairs for the slot; and
- \(n_j^{\min}\) and \(n_j^{\max}\) are finite integers satisfying
  \(0\leq n_j^{\min}\leq n_j^{\max}\), with `n_max` bounded by the plan schema
  and rendering budget.

Each binding in \(S_i\) is a pair \((s_j,\kappa_{i,j})\), where
\(\kappa_{i,j}\) identifies the value's semantic role within proposition `i`.
For example, two timestamp slots may play different start-time and deadline
roles. Slot identity, value type, and binding role are numerical renderer
inputs; the authoritative value and surface bytes are not.

If \(n_j\) is the count of emitted token identity \(s_j\), substitution
requires:

\[
n_j^{\min}\leq n_j\leq n_j^{\max}
\]

A slot is mandatory exactly when \(n_j^{\min}>0\). Every occurrence must lie
inside an output unit bound to one pair in \(\Omega_j\). Counts, permissions,
and roles are checked on token identities before text decoding.

\[
b_j =
\operatorname{surface}_{K_R}(a_j,\tau_j,\ell_L)
\]

The initial candidate reserves a fixed vocabulary of 64 slot tokens, such as
`<NV_00>` through `<NV_63>`. Sixty-four is an initial registered capacity for
evaluation, not a validated optimum. The final maximum must be no greater than
the plan schema and rendering budget can validate exhaustively.

Formatting an exact timestamp, number, location, path, or identifier into an
authorized surface form occurs deterministically before model generation.
Every value type defines length, character, quoting, escaping, and permitted
placement rules. Arbitrary raw memory text is not an exact-value slot. The
model chooses only whether and where an allowed slot token appears. It does
not choose or transform the slot bytes.

### Per-facet item projection

Each numerical facet has its own learned projector:

\[
W_f:\mathbb{R}^{d_f}\rightarrow\mathbb{R}^{d_a}
\]

The initial adapter width is:

\[
d_a=512
\]

For item `i`, the projected representation is:

\[
h_i =
\operatorname{LayerNorm}
\left(
\sum_{f\in F}m_{i,f}W_fv_{i,f}
+
\operatorname{MLP}_{num}(x_i)
+
E_{role}(\rho_i)
+
E_{rank}(r_i)
+
E_{relations}(\Gamma_i)
+
E_{authority}(\mathcal A_i)
+
E_{disposition}(d_i)
+
E_{slots}(S_i,V)
\right)
\]

The slot-binding term is:

\[
E_{slots}(S_i,V)
=
\sum_{(s_j,\kappa_{i,j})\in S_i}
\left(
E_{slot}(s_j)
+
E_{type}(\tau_j)
+
E_{binding}(\kappa_{i,j})
\right)
\]

The numerical MLP accepts only the finite, range-validated scalar schema. Role
and rank embeddings encode plan structure, not source authority. The renderer
cannot infer a higher authority from a role or a large activation value.
Slot embeddings tell the renderer which fixed output slot is associated with
which typed proposition role without exposing the slot's exact payload.

`E_relations` uses the closed relation vocabulary and the canonical rank of
each relation target; it does not embed arbitrary record identifiers.
`E_authority` and `E_disposition` encode closed categorical contracts, not a
learned authority ordering. Essential support and provenance identities `P_i`
remain available to the attribution head and validator but are not projected
as semantic features: arbitrary identifiers must not change prose. Thus every
plan field is retained, while only fields with an authored surface-realization
role enter `h_i`.

The item matrix is:

\[
H=(h_1,\ldots,h_n)\in\mathbb{R}^{n\times512}
\]

Padding items are masked. Missing facets contribute nothing through their
presence mask. Unknown facets, roles, scalar fields, slot identities, value
types, binding roles, or rank encodings are errors.

### Typed latent resampler

The initial candidate uses 32 learned latent queries:

\[
Q^{(0)}\in\mathbb{R}^{32\times512}
\]

Two resampler blocks map the variable-length item matrix to a fixed-size
prefix. For block \(\ell\):

\[
\widetilde Q^{(\ell)}
=
Q^{(\ell)}
+
\operatorname{MHA}_{cross}
\left(
\operatorname{LN}(Q^{(\ell)}),
\operatorname{LN}(H),
\operatorname{LN}(H)
\right)
\]

\[
\widehat Q^{(\ell)}
=
\widetilde Q^{(\ell)}
+
\operatorname{MHA}_{self}
\left(
\operatorname{LN}(\widetilde Q^{(\ell)})
\right)
\]

\[
Q^{(\ell+1)}
=
\widehat Q^{(\ell)}
+
\operatorname{FFN}
\left(
\operatorname{LN}(\widehat Q^{(\ell)})
\right)
\]

Cross-attention uses the item-presence mask. The initial candidate uses eight
attention heads and two blocks. Query count `32`, adapter width `512`, eight
heads, and two blocks are registered starting values selected for a bounded
first experiment. They are not claims about human working-memory capacity and
must be compared with smaller and larger alternatives.

The resampler is an information bottleneck. Upstream plan items remain
available to deterministic validation; the resampled prefix never becomes the
authoritative plan.

### Soft-prefix projection

Let \(d_{\mathrm{LM}}\) be the pinned renderer model's hidden dimension. A
learned projection maps the resampler output into the model input-embedding
space:

\[
P =
\operatorname{RMSNorm}
\left(
Q^{(2)}W_P
\right)
\in\mathbb{R}^{32\times d_{\mathrm{LM}}}
\]

For the initial Qwen3-1.7B candidate:

\[
d_{\mathrm{LM}}=2048
\]

The causal language-model input embedding sequence is:

\[
E_{in}
=
\left[
E_{token}(C_{\ell_L,K_R});
P;
E_{token}(R_{K_R})
\right]
\]

where `C` is a fixed, versioned control prefix carrying the resolved language
and rendering rules, and `R` is the fixed generation marker. The control
prefix contains no memory text and no original user prompt. All continuous
prefix embeddings precede generated output and are visible through ordinary
causal attention.

This is an input-dependent soft-prefix adapter. It is distinct from classical
prefix tuning, in which a task-specific prefix is directly optimized and
reused across inputs.

### Candidate language model

The primary initial candidate is the post-trained dense
`Qwen/Qwen3-1.7B` model:

- local execution only;
- non-thinking mode only;
- no tool use;
- no network access;
- no model-selected retrieval;
- a pinned tokenizer and chat or control template;
- a finite output-token limit derived from the attention budget; and
- a frozen base checkpoint during the first training phase.

Qwen3-0.6B is the required latency and capacity-floor comparison. A larger
candidate may be introduced only when the smaller registered candidates fail a
named renderer gate. Qwen3.5 and non-Qwen models remain eligible comparison
candidates, but require their own runtime-compatibility and multilingual
evaluation.

No model is designated "best" before evaluation on the Nemosyne renderer
contract. Qwen3 is accepted as the first integration family; the release
checkpoint and parameter count become an accepted V1 choice only through a
later decision record supported by the proof program and model-qualification
specification.

### Generation

Let \(\theta\) be the frozen base model parameters and
\(\Delta\theta\) an optional LoRA update. The generation distribution is:

\[
p(y_{1:T}\mid L,K_R)
=
\prod_{t=1}^{T}
p_{\theta+\Delta\theta}
\left(
y_t
\mid
E_{in}(L,K_R),
y_{<t}
\right)
\]

The model target is a compact focus description. It is not a hidden-reasoning
transcript, a simulated human chain of thought, an answer, or an action plan.
The renderer must express only the selected plan within its qualifications and
budget.

Decoding parameters, stop tokens, maximum output length, and randomness are
part of `K_R`. The deterministic reference condition uses greedy decoding.
Greedy decoding does not by itself establish bit-identical output across
different kernels, quantizations, devices, or runtimes.

### Slot substitution

The model emits fixed slot-token identities rather than exact slot bytes.
Substitution is a total deterministic operation only when:

- every emitted slot exists in the pinned sidecar;
- the slot's proposition-and-role pair belongs to \(\Omega_j\) for every
  output unit containing it;
- every sidecar entry has a valid schema-bounded occurrence contract;
- no forbidden or unknown slot is emitted;
- every observed count satisfies
  \(n_j^{\min}\leq n_j\leq n_j^{\max}\); and
- substitution preserves valid UTF-8 and the attention budget.

The substitution implementation scans token identities before decoding them
to ordinary text. It must not identify slots through string matching after a
tokenizer has normalized or split them. Slot expansion inherits the slot
token's proposition binding, and segmentation offsets are deterministically
recomputed over the substituted byte sequence.

Approved surface bytes are inserted without model rewriting. If the
substituted text violates grammar, type-specific placement policy, or budget,
the result is rejected; the substitution stage does not ask the model to
repair it.

### Segmentation and proposition bindings

The renderer returns a complete, nonoverlapping segmentation and untrusted
bindings from each output unit to planned proposition identities. A trainable
attribution head may calculate token-to-proposition support:

\[
\widehat a_{t,i}
=
\sigma
\left(
g(d_t)^\top k(h_i)
\right)
\]

where \(d_t\) is the decoder hidden state for output token `t`. These scores
help form segment bindings and support auxiliary training losses. They do not
prove that the output semantically follows the proposition.

The independent validator receives the original plan, exact-value table,
resolved language, output text, segmentation, and claimed bindings. It does
not accept a binding merely because the renderer produced a high score.

### Fail-closed faithfulness validation

Validation is a decision procedure over the post-substitution output. It
returns the exact supplied text unchanged or one typed error; it never rewrites
or repairs generation. Its configuration and thresholds are immutable
artifacts in `K_R`.

The deterministic structural layer verifies:

- renderer, plan, tokenizer, slot, formatter, and validator schema identity;
- complete token-origin information retained before text decoding;
- exact-slot authorization, multiplicity, mandatory presence, substitution,
  and post-expansion budget;
- one complete, nonoverlapping byte segmentation;
- a known support binding for every assertion-bearing unit;
- membership of unbound units in the closed surface-only class;
- complete mandatory proposition coverage and absence of excluded proposition
  bindings;
- output-language and framing constraints; and
- absence of malformed, repeated, truncated, or post-stop content.

The closed surface-only class contains only Unicode whitespace, Unicode
punctuation, and fixed structural delimiters enumerated by `K_R`. A connective,
relation, modifier, noun, verb, slot, or other semantic token cannot use it.

Before ordinary tokens are accepted as text, a deterministic literal guard:

1. rejects a reserved slot marker produced through any non-slot tokenization;
2. rejects an ordinary-token span that byte-matches or
   formatter-normalization-matches any output-authorized exact surface in
   `V_L`;
3. rejects any span that byte-matches or formatter-normalization-matches a
   forbidden exact surface in `X_L`; and
4. rejects ordinary-token spans matching the closed `exact_only` grammars in
   `K_R`, initially URLs, filesystem paths, structured identifiers, and
   locale-specific numbers, quantities, dates, and times.

The guard proves that planned exact values and registered exact-only types enter
accepted output only through slots. Natural-language named entities cannot all
be recognized by a deterministic lexical grammar. An invented name or location
that is not an authorized slot is therefore an unsupported semantic claim,
measured and rejected by the semantic layer where detected; this architecture
does not falsely claim structural prevention of every possible invented
literal.

The independent semantic layer receives each assertion-bearing output unit,
its claimed plan bindings, the full relation and qualification view of `L`,
and `J`. A separately trained and calibrated verifier must decide:

- whether the unit is supported by every claimed proposition;
- whether it introduces an unbound proposition or causal relation;
- whether it preserves negation, uncertainty, temporal scope, conflict,
  dominant-versus-secondary relations, and authority ceiling;
- whether it realizes excluded content or raw source-like text;
- whether it begins answering the retained user prompt; and
- whether global composition omits mandatory meaning or changes relationships
  between individually supported units.

Renderer attribution scores are features, not acceptance evidence. The
verifier has an independent artifact identity, frozen training split,
calibration set, thresholds, and false-acceptance and false-rejection report.
It cannot generate replacement text, retrieve memory, access the network, or
write persistent state.

### Independent semantic-verifier candidate

The intended V1 verifier is a separately parameterized dual-branch
classifier, not a second causal renderer:

1. a pinned multilingual text encoder maps each assertion-bearing output unit
   and the retained prompt view in `J` to token and pooled representations;
2. verifier-only per-facet projectors map the bound plan propositions,
   relations, qualifications, authority features, mandatory-set summary, and
   `X_L` exclusion controls into a typed latent sequence;
3. bidirectional cross-attention compares text units with the claimed support
   and the global control sequence; and
4. calibrated multi-label heads estimate unsupported meaning, unbound meaning,
   relation or scope change, authority strengthening, excluded-content
   realization, answer leakage, and mandatory-content omission.

The verifier shares no learned tensor, optimizer state, output head, or
continuous prefix with the renderer. It receives no raw memory prose and does
not convert plan vectors to natural-language premises. Renderer-provided
bindings select comparisons but never supply labels or acceptance decisions.
The exact multilingual encoder, latent width, and classifier depth are chosen
by a verifier qualification manifest; changing any of them creates a new
unqualified artifact.

Verifier training, threshold calibration, and sealed evaluation use disjoint
semantic-scenario roots. None of those roots may occur in renderer training or
renderer development selection. They also remain disjoint from every renderer
model-qualification sealed suite and every end-to-end sealed suite; a
previously opened verifier case can never become later sealed evidence.
Controlled corruptions cover each rejection class individually and in
composition. Human-reviewed or independently adjudicated labels, generator
identities, split membership, and provenance are content-identified.

For each violation head `h`,
\(\widehat p_h\in[0,1]\), and acceptance requires
\(\widehat p_h < \tau_h\). Before sealed evaluation, calibration selects the
maximum member of a pinned, finite, ascending set of distinct candidate
thresholds in `[0,1]` whose pinned one-sided confidence procedure keeps the
upper bound on bad-output acceptance below the manifest-declared
\(\alpha_h\), where \(0<\alpha_h<1\). An empty feasible threshold set fails
qualification. The manifest fixes a confidence level \(\gamma_h\) with
\(0<\gamma_h<1\), a positive integer minimum case count per language and
violation stratum, comparison precision, rounding, and the candidate-threshold
construction before calibration results are opened. A value outside these
domains is a configuration error. Global acceptance is the conjunction of
every head decision and all deterministic checks.

Qualification reports false acceptance and false rejection overall and for
every required language, violation class, plan role, exact-value type, plan
size, and composed-adversarial stratum. An underpowered stratum, missing score,
non-finite value, or failed hard gate disqualifies the artifact. If no verifier
artifact passes, the generative renderer is unavailable; renderer quality
cannot compensate for an unqualified validator.

Any missing artifact, incompatible schema, non-finite score, ambiguous
decision, threshold failure, or validator error rejects the generative result.
It returns an explicit error and must not expose the rejected text, silently
drop plan content, or rerun the request through another renderer. A
deterministic renderer is a separately selectable, authenticated configuration
chosen before request execution and follows the same substitution and
validation stages; it is not a post-failure fallback. A model-generated
renderer cannot be released until the validator's worst-stratum
false-acceptance and false-rejection gates pass on disjoint adversarial and
ordinary cases. The exact verifier encoder and classifier remain a
qualification choice, but this input, decision, failure, and evidence contract
is fixed.

## Training

### Training example contract

Each training example contains:

- one versioned structured attention plan;
- one target attention text;
- target segmentation;
- span-to-proposition support labels;
- mandatory and optional proposition labels;
- explicit exclusions;
- exact-value slots and expected occurrences;
- generator, schema, teacher, reviewer, and dataset-version provenance.

The resolved language and maximum post-substitution budget are read from the
plan envelope and must not be duplicated as independently editable labels.
Explicit exclusions belong to the training harness and validator view of the
plan. They are never included in the generator item sequence or continuous
prefix.

Exact values in target text are replaced with their fixed slot tokens before
training. Variants derived from one semantic scenario remain in one dataset
split to prevent train-test leakage.

The training corpus must include empty attention, one-item plans, multi-item
plans, redundancy, conflict, uncertainty, temporal qualification, exclusions,
mixed-language source evidence, short and long exact values, and deliberately
corrupted negative targets.

Empty-plan records verify the deterministic bypass and contribute no
generative loss. Every example entering the language-model objective has at
least one unmasked target-output token.

### Phase A: bridge-only alignment

Before Phase A, a deterministic vocabulary-extension step appends the 64
registered slot strings in ascending slot-identity order to the immutable
official tokenizer revision. It rejects any string that is not one unique
atomic token or that aliases an existing token. Each new input row is
initialized to the exact arithmetic mean of the pinned model's existing
special-token input rows using the manifest's dtype and reduction order. Where
the model ties input and output embeddings, the new output row is the same
storage; otherwise it is initialized from the corresponding input row. The
resulting tokenizer, expanded embedding/output tensors, token-ID map, and
initialization receipt form one content-identified derived model artifact.

The first phase freezes every original base language-model parameter and every
original input- and output-vocabulary row. A gradient mask permits updates only
to the appended slot input rows and, when the output matrix is untied, their
separately appended output rows. For a tied model these are the same storage.
Optimizer state is created only for those appended rows and the bridge
parameters. Trainable parameters are:

- per-facet projectors;
- scalar MLP;
- role, rank, relation, authority, and disposition embeddings;
- latent queries and two resampler blocks;
- soft-prefix projection and normalization;
- appended exact-slot input rows and their tied or separately appended output
  rows; and
- proposition-attribution head.

This phase tests whether the plan representation can be translated into the
existing model space. Its result must be evaluated independently. Training
LoRA from the beginning would make it unclear whether the bridge learned the
mapping or the language model memorized renderer examples.

### Phase B: bridge plus LoRA

Only when the frozen bridge-only condition misses a predeclared development
gate may the second phase jointly train the retained bridge and low-rank model
updates. Sealed held-out results cannot trigger Phase B or change its
configuration. The registered starting configuration is:

- LoRA rank `16`;
- LoRA alpha `32`;
- zero base-model weight updates; and
- LoRA over the attention and feed-forward linear projections.

These values are initial experiment parameters, not accepted defaults or
optimality claims. A bridge-plus-LoRA model must demonstrate improvement over
the frozen bridge on held-out semantic scenarios, languages, exact-value
patterns, and counterexamples. Otherwise the additional model adaptation is
rejected.

Quantized LoRA training is an optional resource optimization, not a semantic
change. A model trained or evaluated with one quantization cannot inherit
evidence from another without an explicit equivalence evaluation.

### Token likelihood

For target output \(y^\*\), teacher-forced language-model loss is:

\[
\mathcal L_{LM}
=
-\frac{1}{|Y|}
\sum_{t\in Y}
\log p(y_t^\*\mid L,y_{<t}^\*,K_R)
\]

`Y` is the nonempty set of unmasked target-output positions. Loss is computed
only over target output, not the fixed control prefix or continuous plan
prefix. The normalization prevents a longer target from receiving greater
weight solely because it contains more tokens.

### Proposition attribution

Let \(a_{t,i}^\*\in\{0,1\}\) label whether plan proposition `i` supports target
token or span `t`. Let `A` be the nonempty set of unmasked
token-proposition labels. Multi-label attribution loss is:

\[
\mathcal L_{attr}
=
\frac{1}{|\mathcal A|}
\sum_{(t,i)\in\mathcal A}
\operatorname{BCE}
\left(
\widehat a_{t,i},
a_{t,i}^\*
\right)
\]

Attribution labels include one closed surface-only class. It may label only
whitespace, punctuation, and structural delimiters enumerated by `K_R`.
Connectives, relations, modifiers, nouns, verbs, exact values, and every other
assertion-bearing unit require one or more planned-proposition labels. Padding
and unavailable labels are masked out of `A`; they are not negative examples.

### Mandatory coverage

Predicted soft coverage of proposition `i` is:

\[
\widehat c_i =
\max_{t\in Y}\widehat a_{t,i}
\]

Let `I_cov` be the set of plan-item propositions required in the target and
let \(c_i^\*=1\) for each of them. Optional propositions are masked
from this loss. Control-only exclusions are not generator items and therefore
cannot be attribution-head classes; they are enforced by validation and
measured with controlled negative outputs. Coverage loss is:

\[
\mathcal L_{cov}
=
\frac{1}{|\mathcal I_{\mathrm{cov}}|}
\sum_{i\in\mathcal I_{\mathrm{cov}}}
\operatorname{BCE}
\left(
\widehat c_i,
c_i^\*
\right)
\]

When `I_cov` is empty, `L_cov` is defined as zero. This auxiliary loss does not
replace exact mandatory-inclusion or exclusion validation.

### Exact-slot loss

Let `J_slot` be the target positions containing exact-value slot tokens:

\[
\mathcal L_{slot}
=
-\frac{1}{|J_{\mathrm{slot}}|}
\sum_{t\in J_{\mathrm{slot}}}
\log p(y_t^\*\mid L,y_{<t}^\*,K_R)
\]

The separate term permits slot errors to receive greater weight than ordinary
surface-token errors. The final weight is selected through development
evidence and frozen before sealed evaluation. When `J_slot` is empty,
`L_slot` is defined as zero.

### Contrastive corruption loss

Each positive target may be paired with controlled corruptions that:

- omit a mandatory proposition or qualification;
- include a control-only excluded proposition known to the training harness;
- strengthen uncertainty or authority;
- swap an exact-value slot;
- introduce an unsupported causal relation;
- copy raw source text; or
- begin answering the original prompt.

For any nonempty unmasked output-position set \(Y_y\), define the
length-normalized teacher-forced log-likelihood:

\[
s(L,y)
=
\frac{1}{|Y_y|}
\sum_{t\in Y_y}
\log p(y_t\mid L,y_{<t},K_R)
\]

Higher `s` is better. Training rejects a non-finite sequence score. For finite
margin \(\mu\geq0\):

\[
\mathcal L_{contrast}
=
\max
\left(
0,
\mu-s(L,y^+)+s(L,y^-)
\right)
\]

Negative examples are generated from the same semantic scenario and carry the
exact violated rule. They are not treated as alternative acceptable outputs.
When an example has multiple registered corruptions, `L_contrast` is their
arithmetic mean in canonical corruption-identity order rather than an
unnormalized sum.

### Combined objective

The candidate training objective is:

\[
\mathcal L
=
\mathcal L_{LM}
+
\lambda_{attr}\mathcal L_{attr}
+
\lambda_{cov}\mathcal L_{cov}
+
\lambda_{slot}\mathcal L_{slot}
+
\lambda_{contrast}\mathcal L_{contrast}
\]

Every component is therefore normalized per example before the weighted sum.
The coefficients
\(\lambda_{attr},\lambda_{cov},\lambda_{slot},\lambda_{contrast}\) and margin
\(\mu\) are finite and nonnegative. Their values are not set by this
specification. Every training run records their exact values in a
content-identified experiment manifest.
No training loss establishes semantic faithfulness without independent
held-out evaluation.

## Runtime and artifact contract

### Reference runtime

The first Apple-Silicon research and fine-tuning integration uses MLX-LM with a
small custom training and generation layer because its Qwen implementations
accept external input embeddings and its training stack supports local LoRA.
This is not a selection of the production Rust runtime.

Hugging Face Transformers is the cross-platform reference because the Qwen3
model contract accepts `inputs_embeds`. Every implementation must pass exactly
one of token IDs or input embeddings to each model invocation, provide the
attention mask and positions covering the complete prefix, and verify cache
behavior for continuous first-step inputs.

High-level text-generation servers that accept only tokenized text do not
implement this renderer contract. Serializing vectors as decimal strings is
not a conforming substitute.

A local Rust integration may use a runtime that accepts external token
embeddings, such as the low-level `llama.cpp` batch interface, only after
conformance tests demonstrate:

- correct mixed sequencing of fixed control tokens, continuous prefix
  embeddings, and generated tokens;
- identical plan-prefix placement;
- correct position and cache updates;
- supported LoRA and quantization behavior; and
- no network or cross-request state leakage.

### Artifact identity

`K_R` binds at least:

- attention-plan schema and facet vocabulary;
- every facet-projector identity;
- scalar normalization and numerical MLP;
- role, rank, relation, authority, and disposition vocabularies;
- slot identity, value-type, and binding-role vocabularies;
- query count, adapter width, head count, and resampler depth;
- soft-prefix projector and normalization;
- base model checkpoint;
- derived model checkpoint containing the deterministic vocabulary extension,
  original-row freeze mask, and appended slot rows;
- LoRA checkpoint, when present;
- official tokenizer base revision, derived tokenizer revision, deterministic
  augmentation receipt, and fixed control template;
- slot-token vocabulary and deterministic substitution implementation;
- attribution head;
- decoding and stop configuration;
- output language policy;
- structural validator, literal-grammar, semantic verifier, calibration,
  threshold, and validation-corpus identities;
- runtime and quantization identity; and
- numerical dtype and execution backend.

An authenticated manifest anchored in the installation trust root authorizes
the artifact identities in `K_R`; content digests then verify the immutable
opened bytes. Artifacts are opened immutably before rendering. A renderer may
not download, update, train, publish a cache, or replace an artifact during
compilation.

Numerical representations produced for another plan schema, encoder, or
normalization version are incompatible even when their tensor shapes match.

### Local resource behavior

Cold-load time, warm latency, prefix-processing latency, generation latency,
peak memory, persistent resident memory, and energy use are empirical release
metrics. The model may remain resident under a separately defined local
lifecycle policy, but no generated KV state or plan prefix may be reused
across logically distinct calls.

Model unloading must release model weights, LoRA state, adapter weights,
continuous-prefix buffers, KV cache, exact-value sidecars, and request-local
bindings according to the adopted privacy and resource policy.

## Preconditions

Rendering begins only when:

- `L` passed structural, authority, provenance, and budget planning checks;
- every plan item uses the pinned, supported schema;
- control-only exclusions are canonical, complete, within `N_exclude`, and
  absent from the generative item sequence and substitution sidecar;
- all tensors and scalars are finite and correctly normalized;
- canonical item ordering and unique proposition identities are established;
- exact-value slots are unique, authorized, type-valid, and within capacity;
- every slot binding references a known slot, value type, and semantic role;
- every reserved slot is one unique atomic tokenizer token and its textual
  marker cannot pass validation through an alternate tokenization;
- `ell_L` is one supported resolved output language;
- `B_L > 0` is finite and sufficient for mandatory post-substitution content;
- `J` is retained outside the renderer model, bound to the same request and
  prompt bytes, and available only to the validator;
- all renderer, tokenizer, adapter, validator, and runtime artifacts are
  present, compatible, authorized by the authenticated manifest, immutable,
  integrity-checked, and qualified for their declared role;
- the compile dependency boundary exposes no network capability;
- no prior request state is visible to the renderer.

## Invariants

- The authoritative structured plan remains unchanged by rendering.
- Only the planner selects or excludes propositions.
- The renderer sees neither the whole memory universe nor the raw user prompt.
- The model cannot promote source authority, confidence, validity, or
  normative force.
- Exact values are never reconstructed from lossy vectors.
- Every planned exact value and every accepted value in a registered
  `exact_only` lexical class comes only from an authorized sidecar slot,
  deterministic formatting, and deterministic substitution.
- Unknown invented named entities remain an empirical semantic-validation risk,
  not a structurally eliminated class.
- Arbitrary raw memory or source text cannot enter the output through an
  exact-value slot.
- Slot substitution performs no semantic rewriting.
- Every assertion-bearing output unit has a complete claimed binding; only the
  closed non-assertive surface class may remain unbound.
- Claimed neural bindings remain untrusted until independent validation.
- Unsupported, excluded, answer-like, over-budget, wrong-language, or
  unsegmentable output is an error.
- Validation accepts the exact substituted text unchanged or returns an error;
  it does not silently repair generation.
- An empty plan renders empty attention without invoking the model unless a
  later accepted decision proves a different path necessary.
- No renderer step mutates persistent memory, access history, model weights,
  training data, or compiler configuration.
- No network operation occurs during compilation.
- Changing any bound artifact identity creates a different renderer
  configuration and requires separate evidence.

## Edge cases

### Empty plan

An empty attention plan produces empty attention deterministically. It does not
ask the model to invent a generic focus statement.

### No exact values

The sidecar and slot occurrences are empty. Any emitted slot token is an error.

### Slot-capacity overflow

A plan requiring more exact values than the pinned slot vocabulary supports is
rejected before model invocation. Values are not merged into one ambiguous
slot.

### Unknown or repeated slots

Unknown slots, unauthorized slots, prohibited repetitions, missing mandatory
slots, slots bound to the wrong proposition, or a reserved marker emitted
through ordinary non-slot token identities cause rendering failure.

### Long exact values

Substitution is checked against the final byte and token budget. A model output
that fit before substitution but exceeds the budget afterward is rejected.

### Missing facets

Declared optional facets use presence masks. Missing required facets or an
unknown facet schema are representation failures, not zero vectors.

### Conflicts and uncertainty

Conflicting propositions and uncertainty arrive as distinct qualified plan
items. The renderer must preserve the planner's qualifications. It may not
collapse a conflict into one asserted fact.

### Mixed-language evidence

Source-vector language does not select the output language. The complete
attention text must use `ell_L`, except for exact sidecar values whose authorized
surface form is intentionally language-independent.

### Injection-like source content

Raw source text is not part of the renderer input. A plan item that
semantically resembles an instruction remains data governed by its plan role
and authority. It cannot modify the fixed renderer control contract.
Exact-value types cannot carry arbitrary source passages. Their deterministic
surface formatter applies the pinned quoting and escaping rules before
substitution.

### Degenerate generation

Repetition, empty generation for a nonempty mandatory plan, stop-token failure,
malformed segmentation, non-finite logits, or a runtime error returns explicit
renderer failure. The compiler does not expose a partial attention text.

### Quantization drift

Changing quantization, kernel, or runtime can change generation from the same
prefix. Evidence for one execution identity does not validate another.

## Failure modes

The candidate must be evaluated explicitly for:

- information loss through the 32-query bottleneck;
- confusion between focus, background, constraints, exclusions, and
  uncertainty;
- continuous prefixes outside the language model's useful embedding region;
- omission of low-activation but mandatory content;
- unsupported causal or normative connections;
- answer leakage;
- raw source reconstruction;
- wrong-language output and multilingual interference;
- slot omission, duplication, swapping, or grammatical misuse;
- incorrect proposition bindings;
- overfitting to plan order, schema version, templates, or teacher style;
- LoRA memorization hiding a weak bridge;
- representation drift after re-encoding memory;
- quantization-specific prefix sensitivity;
- cache contamination between calls;
- platform-dependent output changes; and
- latency or memory use that exceeds the local V1 budget.

None of these failure modes is ruled out by the architecture alone.

## Verification

### Required baselines and ablations

Every renderer evaluation uses the identical frozen structured plan. At
minimum, compare:

1. deterministic template rendering;
2. direct deterministic serialization of plan labels;
3. two-layer MLP projection with the frozen registered resource-floor model;
4. two-layer MLP projection with the frozen registered capacity reference;
5. 32-query latent resampler with the frozen resource-floor model;
6. 32-query latent resampler with the frozen capacity reference;
7. registered latent resampler plus LoRA on the capacity reference, only after
   its bridge-only development-gate failure; and
8. expert reference rendering of the identical plan.

The local-renderer model-qualification specification owns the checkpoint names,
seed cohorts, precision pairing, and promotion rule used by these roles.

The latent-resampler study additionally varies:

- query count: `8`, `16`, `32`, and `64`;
- adapter width: `256`, `512`, and `1024`;
- resampler depth: `1`, `2`, and `4`; and
- bridge-only versus bridge-plus-LoRA training.

Model, adapter, and dimension selection uses development evidence. Final
thresholds and one candidate configuration are frozen before sealed
evaluation.

### Contract tests

Executable tests cover:

- canonical item-order stability;
- facet masks and incompatible schemas;
- finite-value and dimension validation;
- exact soft-prefix shape, mask, position, and dtype;
- empty-plan fast path;
- every slot-substitution success and failure case;
- preservation of approved Unicode, path, timestamp, and number surface bytes;
- type-specific exact-value quoting, escaping, and placement;
- alternate tokenizations of reserved slot-marker text;
- ordinary-vocabulary attempts to reproduce exact surfaces or `exact_only`
  lexical classes;
- output-budget checks after substitution;
- complete nonoverlapping segmentation;
- known and unknown proposition bindings and the closed surface-only grammar;
- language and stop-token enforcement;
- semantic-verifier fail-closed behavior, threshold boundaries, unavailable
  artifacts, and independent false-acceptance and false-rejection fixtures;
- no prompt or raw-memory access at the renderer boundary;
- no persistent write or network operation;
- runtime failure without partial output;
- cache isolation across calls; and
- artifact-manifest mismatch.

### Renderer metrics

Held-out evaluation measures:

- mandatory-proposition coverage;
- must-exclude violation rate;
- clause-level source traceability;
- unsupported-claim rate;
- qualification, conflict, and authority preservation;
- answer leakage;
- exact-slot precision and recall;
- wrong-slot and altered-value rate;
- output-language match;
- raw-source copying beyond approved slots;
- repetition and malformed-output rate;
- budget compliance;
- cold and warm latency;
- prefix-prefill and generation latency;
- peak and resident memory; and
- downstream utility against prompt-only and deterministic-renderer
  conditions.

Metrics are reported overall, by language, plan size, proposition role,
conflict class, exact-value type, and worst supported category.

### Training diagnostics

Bridge-only and bridge-plus-LoRA runs report:

- all component losses separately;
- exact trainable parameter sets;
- gradient and activation norms;
- virtual-prefix norm and distribution drift;
- results by semantic scenario rather than augmented row;
- performance on unseen plan permutations and schema-compatible
  perturbations;
- negative-corruption discrimination;
- quantized and unquantized results; and
- multiple training seeds.

Training loss, language fluency, and correct slot copying do not establish
semantic renderer quality. Promotion requires the independent renderer gates
in the V1 proof program.

### Stop conditions

The candidate is replaced or narrowed when:

- deterministic rendering meets the same utility threshold with lower risk;
- no tested bridge beats direct MLP projection materially;
- mandatory coverage or exclusion remains below the frozen threshold;
- unsupported claims or answer leakage exceed the frozen ceiling;
- exact slots cannot be used reliably across supported languages;
- LoRA gains do not generalize to unseen semantic scenarios;
- the smallest passing model exceeds local resource budgets; or
- no runtime can execute continuous prefixes locally under the required trust
  and artifact contract.

## Open questions

- The final attention-plan roles, facet spaces, scalar schema, and item limit.
- Whether role-specific query banks improve binding enough to justify their
  added assumptions.
- The accepted number of exact-value slots and permitted multiplicity rules.
- The supported exact-value types and locale formatter repertoire used by
  planning and deterministic substitution.
- The final Qwen checkpoint or alternative local renderer model.
- Whether post-trained or base-model initialization generalizes better.
- The final LoRA target modules, rank, alpha, and loss weights.
- Whether greedy decoding satisfies quality requirements or constrained
  decoding is needed.
- The semantic verifier's exact multilingual encoder, latent dimensions,
  manifest confidence targets, calibrated thresholds, and accepted
  false-decision rates under the fixed dual-branch contract.
- The production local runtime and Rust integration boundary.
- The accepted quantization and unload lifecycle.
- Renderer release thresholds by language and worst-case category.

## References

- Li and Liang, [Prefix-Tuning: Optimizing Continuous Prompts for
  Generation](https://aclanthology.org/2021.acl-long.353/), 2021.
- Lester, Al-Rfou, and Constant, [The Power of Scale for Parameter-Efficient
  Prompt Tuning](https://aclanthology.org/2021.emnlp-main.243/), 2021.
- Tsimpoukelli et al., [Multimodal Few-Shot Learning with Frozen Language
  Models](https://arxiv.org/abs/2106.13884), 2021.
- Jaegle et al., [Perceiver IO: A General Architecture for Structured Inputs
  and Outputs](https://arxiv.org/abs/2107.14795), 2021.
- Alayrac et al., [Flamingo: a Visual Language Model for Few-Shot
  Learning](https://arxiv.org/abs/2204.14198), 2022.
- Li et al., [BLIP-2: Bootstrapping Language-Image Pre-training with Frozen
  Image Encoders and Large Language Models](https://arxiv.org/abs/2301.12597),
  2023.
- Liu et al., [Visual Instruction
  Tuning](https://arxiv.org/abs/2304.08485), 2023.
- Liu et al., [Improved Baselines with Visual Instruction
  Tuning](https://arxiv.org/abs/2310.03744), 2023.
- Hu et al., [LoRA: Low-Rank Adaptation of Large Language
  Models](https://arxiv.org/abs/2106.09685), 2021.
- Dettmers et al., [QLoRA: Efficient Finetuning of Quantized
  LLMs](https://arxiv.org/abs/2305.14314), 2023.
- Gu et al., [Incorporating Copying Mechanism in Sequence-to-Sequence
  Learning](https://aclanthology.org/P16-1154/), 2016.
- See, Liu, and Manning, [Get To The Point: Summarization with
  Pointer-Generator Networks](https://aclanthology.org/P17-1099/), 2017.
- Moryossef, Goldberg, and Dagan, [Step-by-Step: Separating Planning from
  Realization in Neural Data-to-Text
  Generation](https://aclanthology.org/N19-1236/), 2019.
- Shen et al., [Select and Attend: Towards Controllable Content Selection in
  Text Generation](https://aclanthology.org/D19-1054/), 2019.
- Su et al., [Plan-then-Generate: Controlled Data-to-Text Generation via
  Planning](https://aclanthology.org/2021.findings-emnlp.76/), 2021.
- Song et al., [Structural Information Preserving for Graph-to-Text
  Generation](https://aclanthology.org/2020.acl-main.712/), 2020.
- Qwen Team, [Qwen3 Technical
  Report](https://arxiv.org/abs/2505.09388), 2025.
- Apple ML Research,
  [MLX-LM](https://github.com/ml-explore/mlx-lm).
- Hugging Face Transformers,
  [Qwen3 implementation](https://github.com/huggingface/transformers/blob/main/src/transformers/models/qwen3/modeling_qwen3.py).
- ggml-org,
  [`llama.cpp` input-batch interface](https://github.com/ggml-org/llama.cpp/blob/master/include/llama.h).
- [V1 product contract](v1-product-contract.md).
- [V1 reference architecture](v1-reference-architecture.md).
- [V1 proof program](v1-proof-program.md).
- [Local renderer model qualification](local-renderer-model-qualification.md).
- [Decision 0013: Adopt a vector-prefix local renderer qualification
  path](../decisions/0013-adopt-a-vector-prefix-local-renderer-qualification-path.md).
