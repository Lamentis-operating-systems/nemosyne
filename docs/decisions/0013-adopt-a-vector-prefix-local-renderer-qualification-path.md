# 0013: Adopt a vector-prefix local renderer qualification path

Status: Accepted
Date: 2026-07-23

## Context

Decision 0012 makes a bounded numerical `AttentionPlan` the source of meaning
for rendering. A standard text-only language-model API cannot consume that
state without first serializing floats and identifiers as text. Such
serialization discards vector geometry, spends tokens on decimal syntax, and
would invite the language model to rediscover roles already decided by the
planner.

The renderer must express a short multilingual focus narrative without
retrieving new memories, changing priorities, answering the original prompt,
or guessing names, paths, times, and numbers. It must run locally on the user's
machine and remain small enough for one-call use. General model benchmarks do
not measure this task, and no official evidence provides an apples-to-apples
Apple-Silicon winner.

Continuous-prefix research, learned latent resamplers, modality-to-language
bridges, and low-rank adaptation establish technical precedents for projecting
non-text representations into a frozen language model. They do not establish
that Nemosyne's vectors are already decodable or that one prefix length or
model is optimal.

## Decision

Adopt an input-dependent soft-prefix bridge as the first generative V1 renderer
hypothesis. Do not serialize Nemosyne vectors as decimal text.

For every canonical plan item, use separate projectors for each vector space,
a numeric-feature encoder, plan-role, rank, relation, authority, and
disposition embeddings, and presence masks to produce typed adapter states.
Retain source and provenance identities alongside those states for attribution
and validation without treating arbitrary identifiers as semantic features.
Feed the variable-size state set through a small learned-query latent resampler
and project its fixed-size output directly into the selected language model's
input-embedding space.

Register the first experiment with:

- adapter width `512`;
- `32` learned latent queries;
- two cross-attention/resampler blocks;
- eight attention heads; and
- one projected virtual token per latent query.

These values are a reproducible starting configuration, not biological
quantities or accepted optima. Benchmark `8`, `16`, and `32` virtual tokens and
the required simpler MLP baseline before selecting a release configuration.

Exact names, identifiers, paths, URLs, numbers, dates, times, and localized
surface forms remain outside the lossy prefix. The renderer may emit only
predeclared exact-slot tokens. A deterministic postprocessor verifies slot
authorization, identity, cardinality, and placement, substitutes the approved
byte-preserving value, and then performs plan-faithfulness validation. Missing,
unknown, duplicate, unauthorized, or invented slots fail the call.

The authoritative plan envelope owns output language, budget, inclusion and
exclusion sets, relations, exact slots, and validation qualifications. The
renderer has no independently editable copy. Exclusions are mandatory
control-only plan records: they cannot be optimized away, but they are not
renderable items or generative-prefix inputs. A deterministic literal guard
rejects ordinary-token reproduction of planned or forbidden exact surfaces and
registered exact-only lexical classes. Other invented named entities remain an
empirical semantic-validation risk and must pass a separate fail-closed
verifier; exact slots are not claimed to make arbitrary language generation
structurally hallucination-free.

Use a separately parameterized dual-branch semantic verifier: one multilingual
encoder represents rendered text and retained prompt context, verifier-only
projectors represent plan support and exclusion controls, and calibrated heads
decide support, scope, authority, exclusion, answer leakage, and mandatory
coverage. It shares no learned tensors with the renderer. A verifier is frozen
and independently qualified before any generative renderer can pass sealed
evaluation.

Use Qwen3 as the first integration family because its official artifacts are
Apache-2.0, multilingual, text-only, locally supported, and expose direct input
embeddings. Use Qwen3-0.6B for the smallest compatibility and resource
baseline and Qwen3-1.7B as the initial capacity and feasibility reference.
Thinking is disabled; a reasoning trace is not a renderer output.

Do not declare a production model winner from general benchmarks. Qualify at
least:

1. Qwen3-0.6B;
2. Qwen3-1.7B;
3. Qwen3.5-0.8B; and
4. Gemma 3 1B IT as an independently licensed architecture control.

Use Qwen3.5-2B as the first capacity fallback if every smaller candidate misses
a mandatory quality gate. Select the smallest and fastest candidate that
passes every frozen fidelity, exclusion, leakage, multilingual, downstream
utility, quantization, and local-resource gate. A failing renderer is replaced
or constrained; its gate is not weakened.

Train in two separately evaluated phases:

1. freeze the language model and train only facet projectors, numeric encoders,
   structural embeddings, latent resampler, output projection, appended
   exact-slot rows, and attribution heads; then
2. only if bridge-only fails, retain the trained bridge and add a narrow LoRA
   adaptation to a still-frozen base model.

The training target is a source-bound focus narrative, not a freely invented
inner monologue. Supervision includes required and excluded propositions,
dominant and secondary relationships, uncertainty, social perspective,
exact-slot use, output language, maximum size, and span-to-plan support
annotations. Token loss is supplemented by versioned attribution, coverage,
slot, and counterfactual corruption losses. Private user memory is not training
data without a separate explicit consent and data-governance decision.

Every encoder, vector schema, normalization, adapter, base checkpoint, LoRA,
tokenizer, slot vocabulary, quantization, runtime, and decoding configuration
is content-identified as one compatible renderer artifact set. Compilation
performs no download, update, training, or network access.

The slot vocabulary is a deterministic, content-identified extension of an
immutable official tokenizer revision. Original model rows remain frozen during
bridge-only training; only appended slot rows and registered bridge components
may update. Artifact digests are accepted only through an authenticated
manifest anchored outside the mutable renderer bundle.

Use MLX-LM as the first Apple-Silicon research and fine-tuning integration. A
production Rust runtime is not selected by this decision; it must demonstrate
the same external-embedding, masking, position, cache-isolation, quantization,
and deterministic-slot contracts before adoption.

Maintain the exact adapter and training contract in
[`vector-to-attention-renderer.md`](../specifications/vector-to-attention-renderer.md)
and the model benchmark and promotion rules in
[`local-renderer-model-qualification.md`](../specifications/local-renderer-model-qualification.md).

## Rationale

Per-facet projectors preserve the fact that semantic, temporal, spatial, goal,
and risk vectors occupy different spaces. A learned-query resampler handles a
variable number of plan items while presenting a bounded input to a small
language model. Direct embeddings preserve the geometry that decimal
serialization would destroy.

A frozen-model phase tests whether the bridge carries the required information.
Adding LoRA only afterward prevents adaptation from concealing a nonfunctional
bridge through target memorization. The same frozen plans and targets permit
component-level attribution.

Deterministic exact slots make the model choose only whether an approved value
belongs in a sentence, never what that value is. This removes one important
class of hallucinated dates, paths, names, and numbers and keeps potentially
instruction-like raw values out of generative selection.

Qwen3 provides a low-risk first integration path, but Nemosyne-specific
qualification is the only valid way to choose between model sizes and families.
The explicit control model and capacity fallback prevent the first convenient
checkpoint from becoming permanent architecture by inertia.

## Alternatives

- **Serialize vectors as JSON or decimal text.** Rejected because it loses
  geometry, wastes tokens, and trains number formatting rather than a
  vector-to-language interface.
- **Use one linear projection of an averaged state.** Retained only as a simple
  benchmark because it cannot reliably preserve variable cardinality, roles,
  conflict, and exact support.
- **Use deterministic templates only.** Retained as the mandatory safety and
  complexity baseline. It is not selected as the sole renderer because
  multilingual composition and natural compression may require generative
  realization. A deterministic renderer may be selected before a request; it
  is not an automatic retry after another renderer fails.
- **Fine-tune the complete language model immediately.** Rejected because it is
  more expensive and makes bridge efficacy difficult to identify.
- **Train bridge and LoRA in one unseparated phase.** Rejected because the
  language model may memorize output patterns while ignoring the numerical
  prefix.
- **Ask the model to reconstruct exact values from embeddings.** Rejected
  because embedding inversion is model-specific, probabilistic, lossy, and a
  privacy risk rather than an exact-value contract.
- **Use Qwen3-0.6B as the permanent winner now.** Rejected because size and
  multilingual claims do not establish Nemosyne-specific fidelity.
- **Use Qwen3-1.7B as the permanent winner now.** Rejected because extra
  capacity does not prove the smallest viable local model or resource fit.
- **Adopt Qwen3.5 solely because it is newer.** Rejected because its hybrid
  architecture and embedding-input runtime behavior require the same frozen
  qualification as every other candidate.
- **Imitate a human chain of thought.** Rejected because neither human inner
  experience nor model-generated reasoning traces are a faithful universal
  target for this product.
- **Use a cloud renderer.** Rejected because V1 compilation is local and
  network-free.

## Consequences

The first renderer implementation requires a custom training and inference
path that can pass external embeddings; ordinary text-only chat APIs are
insufficient. Model and runtime updates are compatibility changes, not
transparent replacements.

The experiment program must compare templates, a simple MLP prefix, the latent
resampler with a frozen model, and the resampler plus LoRA on identical plans.
It must measure required-proposition coverage, excluded-content leakage,
unsupported claims, answer leakage, exact-slot correctness, language match,
budget compliance, downstream benefit and harm, cold and warm latency, peak
memory, unload behavior, and quantization regressions on declared Mac hardware.

No untested parameter count, language, quantization, latency, RAM use, or
user-imperceptibility claim follows from this decision. The accepted path is a
qualified experimental architecture, not a validated renderer.

If every small model or prefix configuration fails, the project must simplify
the output contract, use the deterministic baseline, test the declared capacity
fallback, or reject the renderer hypothesis before integrating retrieval.
