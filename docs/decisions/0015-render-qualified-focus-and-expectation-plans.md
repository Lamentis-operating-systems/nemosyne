# 0015: Render qualified focus-and-expectation plans

Status: Accepted
Date: 2026-07-24

## Context

Decision 0013 selected a local vector-prefix renderer for a numerical
focus-only `AttentionPlan`. Decision 0014 replaces that terminal plan with a
`FocusExpectationPlan` containing separately qualified focus items and
competing expectation hypotheses.

The existing bridge principles remain useful: direct numerical input,
per-facet projection, bounded latent resampling, exact-value slots, local
execution, independent faithfulness validation, and qualification of the
smallest model that passes task-specific gates. The prior training and
validation contract is nevertheless incomplete for expectations. It does not
require horizon, hypothesis category, conditional scope, relative-support
semantics, counterevidence, abstention, or the explicit separation between an
expectation and an action.

Allowing the language model to fill those gaps would transfer semantic
selection from the deterministic plan to the renderer and make polished prose
look more certain or actionable than its evidence.

## Decision

Retain the input-dependent vector-prefix renderer as the first generative
lexicalization hypothesis, but make the canonical
`FocusExpectationPlan` its sole semantic input.

The plan owns every renderable proposition and relation. Focus items and
expectation hypotheses use distinct roles and type embeddings. An expectation
binding includes its category, scope, condition, horizon, representative
proposition, support and counterevidence references, uncertainty
qualification, and disposition. Relative support is renderer-visible only
with a non-probability label. Abstention reasons and excluded claims are
validator-only unless the plan explicitly selects a supported abstention
statement for rendering.

The renderer may:

- order and lexically connect selected plan items within the plan's relations;
- realize the requested supported language;
- use authorized exact-slot placeholders; and
- compress wording within the finite budget without changing meaning.

The renderer may not:

- create, remove, merge, rank, or reclassify a hypothesis;
- convert relative support into probability or certainty;
- omit a mandatory condition, horizon, conflict, uncertainty, or abstention
  qualifier;
- present a hypothesis as observed fact;
- infer a goal or recommend an action from an expectation;
- answer the original prompt;
- retrieve memory or inspect the full memory database; or
- persist any output.

Every V1-deployable renderer configuration uses deterministic inference with
no request-time random tape. A neural candidate uses greedy or a separately
specified deterministic constrained decoder. Its runtime, numerical precision,
target platform class, decoding rules, and tie behavior are content-identified
qualification inputs. Stochastic decoding remains research-only and requires a
later decision that adds its random source to the callable contract, lineage,
and receipts before it can enter a product release.

The bridge, tokenizer extension, exact-slot resolver, and independent semantic
verifier remain separately versioned artifacts. Training data must annotate
focus and expectation spans, their source proposition identities, hypothesis
categories, conditions, horizons, qualifiers, mandatory relations, exact
slots, and forbidden action or answer leakage. Corruptions must include
probability inflation, fact promotion, horizon loss, condition loss,
alternative collapse, unsupported action language, and suppressed abstention.

Qualification compares a deterministic renderer, a simple projection, and the
registered latent-resampler bridge over identical frozen plans. A release
renderer must pass focus fidelity, expectation fidelity, exact-value,
authority, exclusion, language, budget, leakage, local-resource, and
repeatability gates. Model selection remains evidence-based; no checkpoint or
size is made a permanent winner by this decision.

Generation remains non-thinking surface realization. Reasoning traces,
chain-of-thought text, planning tokens, tool calls, and hidden candidate
generation are not accepted renderer outputs.

Maintain the tensor, training, exact-slot, verifier, failure, and evaluation
contracts in
[`vector-to-attention-renderer.md`](../specifications/vector-to-attention-renderer.md)
and the model promotion process in
[`local-renderer-model-qualification.md`](../specifications/local-renderer-model-qualification.md).

This decision supersedes Decision 0013. It retains its bridge and
qualification direction but replaces the focus-only source schema and
associated fidelity contract.

## Rationale

One combined plan lets the bridge learn relations between focus and expectation
without letting the language model decide those relations. Separate roles make
loss of an expectation qualifier observable and trainable.

Exact slots continue to protect loss-sensitive values. Independent semantic
validation remains necessary because exact substitution does not prove that
ordinary generated language preserves category, condition, horizon, or
uncertainty.

Keeping the renderer small and semantically subordinate avoids a second
prediction model call. The expectation kernel performs the predictive work;
the local language model performs one bounded lexicalization pass.

## Alternatives

- **Use a second language model for expectations.** Rejected because it adds
  latency, duplicates semantic work, and creates an ungrounded predictor.
- **Render focus and expectation in separate model calls.** Rejected because
  the calls can disagree, duplicate content, and violate the single-plan
  budget.
- **Serialize the combined plan as decimal JSON.** Rejected for the same
  geometry and token-efficiency reasons recorded in Decision 0013.
- **Use deterministic templates only.** Retained as the mandatory baseline and
  a possible qualified release renderer, but not selected as the sole
  architecture before multilingual and compression evidence exists.
- **Allow the renderer to improve weak hypotheses.** Rejected because language
  fluency is not additional evidence.
- **Hide uncertainty to keep attention concise.** Rejected because omitted
  qualification changes the semantic claim.
- **Include a suggested next action with every expectation.** Rejected because
  action selection belongs to the downstream agent.

## Consequences

Renderer schemas, fixtures, training data, metrics, and validators must be
extended before end-to-end integration. Existing focus-only examples remain
valid only when the selected plan has no renderer-visible expectation role.
Focus plus a selected renderable abstention is a distinct plan shape;
validator-only abstention may remain in a focus-only plan without producing
expectation prose.

The qualified renderer is still a fallible local model. A complete
documentation contract and passing component tests do not prove semantic
fidelity or downstream usefulness.

If no generative candidate passes all gates, the release path may select a
separately qualified deterministic renderer or stop. It may not weaken
faithfulness, authority, uncertainty, action-leakage, or resource gates.
