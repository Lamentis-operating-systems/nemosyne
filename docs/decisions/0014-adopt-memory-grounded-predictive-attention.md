# 0014: Adopt memory-grounded predictive attention

Status: Accepted
Date: 2026-07-24

## Context

Decision 0011 selected V1 as a local, read-only focus compiler. Decision 0012
selected a numerical memory architecture that converts activated memories
into a bounded focus plan. That focus-only semantic boundary can identify what is
relevant, but it cannot represent a second memory function that materially
changes how a downstream agent interprets a situation: several evidence-bound
expectations about an unobserved present state or a possible later state.

Focus and expectation are not synonyms. Focus describes which supported
features, constraints, conflicts, and background deserve processing priority.
Expectation describes which present or future outcomes are supported by
observed transitions under a stated scope, condition, and horizon. Neither
construct selects an action. Treating expectations as goals, facts, answers, or
instructions would silently create an action planner or inflate memory evidence
into authority.

Cognitive and neuroscience research supports memory-based prediction and
distinguishes attention from expectation, but it does not establish one
complete computational model of the brain. Predictive-processing evidence is
mixed and does not justify claims of biological equivalence, human inner
speech, or a universally correct probability model.

The repository has no observed-transition schema, expectation evaluator,
calibrated probability model, or end-to-end product evidence. A generative
model therefore cannot be made the hidden source of expectations. The first
architecture must remain deterministic, inspectable, and able to abstain.

## Decision

Extend the V1 implementation hypothesis from numerical focus compilation to
**memory-grounded predictive attention**.

After authorization, validity, candidate generation, and activation, create one
canonical eligible activated-memory set. Branch the focus planner and the new
expectation kernel from that set before final focus-budget pruning. The focus
planner may prune evidence for concise focus; that pruning must not remove a
weaker but materially distinct transition before expectation formation.

Add an immutable transition-memory representation to the authoritative memory
contract. A transition binds:

- an observed before-state;
- an observed or explicitly absent action, condition, or external event;
- an observed after-state or censored outcome;
- a horizon and observation time;
- validity, reliability, uncertainty, and contradiction status;
- provenance root and dependency group;
- authority, authorization, and allowed-use labels; and
- exact-value sidecars and versioned numerical facets.

The representation distinguishes direct observations, derivations,
predictions, contradictions, unresolved outcomes, and censoring. A predicted
outcome never becomes an observed transition without an independent
observation and a separate memory-management operation.

The expectation contract distinguishes:

1. a present-state hypothesis about an unobserved state that may explain the
   current observations;
2. a passive successor expectation about what may be observed next without
   selecting an agent action;
3. a conditional outcome expectation under an action, event, or condition
   explicitly present in the compile request or authorized memory; and
4. action selection.

The first three are eligible expectation categories. Action selection remains
outside V1. The compiler may describe conditions that qualify an expectation;
it may not choose, recommend, schedule, or execute an action.

Evaluate a deterministic, case-based expectation kernel before any learned or
generative predictor. It must use only eligible observed transitions and
qualified request evidence, preserve incompatible outcomes as alternatives,
choose representatives from valid stored propositions rather than synthesize
an averaged meaning, collapse support within one provenance dependency group,
retain counterevidence, expose coverage and disagreement, enforce finite
alternative and computation budgets, and abstain when its preconditions fail.

For each explicit mutually exclusive alternative family, the kernel may expose
normalized **relative support** across its complete known-plus-unknown group
family before output omission. A later display or token limit may omit known
surfaces, but omitted support stays in the original denominator and visible
top-\(K\) alternatives are never renormalized to one. Relative support is an
evidence share, not a probability, truth score, safety score, or expected
utility. A probability may be exposed only after a separately accepted
calibration contract passes disjoint calibration and time-later evaluation.

One expectation set represents exactly one prediction frame. A compile may
return a finite, configuration-bounded expectation bundle containing at most
one canonically ordered set per frame. Normalization, counterevidence,
alternative ranking, and abstention remain frame-local; support is never
compared across frames. One frame may abstain without suppressing qualified
expectations in another.

Produce one canonical `FocusExpectationPlan`. It keeps focus items and
expectation hypotheses as distinct roles with separate authority ceilings,
support bindings, qualifiers, horizons, uncertainty, counterevidence, and
rendering dispositions. Its validator-only controls include exclusions,
abstention reasons, evidence dependencies, and exact-value authorization. The
plan contains no answer and no executable action.

Observation-assessment semantics must be evaluated by an offline conformance
harness in V1; no prior expectation, later observation, assessment result, or
assessment endpoint is added to the product compile contract. A sealed later
observation fixture may support or contradict an immutable prior hypothesis
fixture, leave it ambiguous, or belong to a different prediction frame.
Assessment never mutates either fixture. Bayesian updating is permitted only
when a validated likelihood model applies. In the product, an independently
authorized observation can affect attention only after a separate
memory-management operation and an explicitly requested new compile from a
pinned source-evidence view. Neither renderer output nor downstream model
output mutates persistent memory.

A later learned predictor may consume the numerical situation and an unordered
set of activated memories and emit a bounded set of parallel hypothesis slots.
It is eligible for adoption only after the transition-data contract,
deterministic baseline, independent evaluation corpus, abstention contract, and
time-later evidence exist. It remains separate from the renderer and must be
compared with the deterministic baseline.

An optional language-model hypothesis proposer is outside the V1 release path.
If researched later, its candidates are speculative, receive no
self-declared probability or authority, and cannot become memory truth without
independent observation.

Maintain the normative terminology, transition contract, derivation,
mathematics, proofs, counterexamples, and examples in
[`predictive-attention-and-expectation.md`](../specifications/predictive-attention-and-expectation.md).

This decision supersedes Decisions 0011 and 0012. It retains Decision 0011's
local, read-only, single-result compiler boundary and Decision 0012's
authoritative exact plane, typed numerical facets, authorization-before-
relevance rule, activation boundary, request-local consolidation, and
non-biological claim discipline. It replaces their focus-only semantic
boundary and terminal plan with the two-branch focus-and-expectation
architecture.

## Rationale

Branching from one eligible activated set avoids duplicate retrieval and policy
work while preventing a tight focus budget from deleting alternative outcomes.
Keeping planners distinct makes the meaning of a focus score and an
expectation-support score independently testable.

Observed transitions provide a falsifiable, provenance-bound source for
expectations. Dependency-aware aggregation prevents duplicated imports from
masquerading as independent evidence. Medoid or stored-proposition
representatives preserve an actually supported meaning and avoid the
unvalidated assumption that ordinary embedding arithmetic preserves
transitions.

Per-frame sets prevent short- and long-horizon evidence from competing in one
denominator. A bounded canonical bundle supports several explicit horizons
without implying that its serialization order is a global relevance or
probability ranking.

A deterministic baseline establishes data, error, attribution, and evaluation
contracts before a learned predictor can obscure them. Explicit abstention is
preferable to emitting polished but unsupported expectations.

The combined plan lets the final attention text orient the downstream agent
toward relevant evidence and bounded expectations while leaving investigation,
decision, and action with that agent.

## Alternatives

- **Keep V1 focus-only.** Rejected because it omits the refined product
  hypothesis that memory can support competing present or future
  expectations. The public output remains unchanged, so the extension can be
  evaluated without adding a second product operation.
- **Merge focus and expectation into one score or prose paragraph.** Rejected
  because relevance, predictive support, horizon, contradiction, and
  abstention have different semantics and evidence requirements.
- **Pool several prediction frames into one expectation set or global
  ranking.** Rejected because condition and horizon define the comparison
  frame. Cross-frame normalization would turn incomparable support into a
  misleading order and would prevent one frame from abstaining independently.
- **Form expectations after final focus pruning.** Rejected because the focus
  budget can erase lower-ranked but incompatible outcomes before predictive
  grouping.
- **Use embedding addition, subtraction, or arithmetic means as transitions.**
  Rejected for ordinary embedding spaces because no current contract proves
  that those operations preserve state-transition meaning.
- **Treat relative support as probability.** Rejected until independent
  calibration and time-later evaluation justify that interpretation.
- **Ask the local renderer to imagine likely outcomes.** Rejected because it
  would make a generative language model the semantic and authority boundary.
- **Select the most supported action.** Rejected because support for an outcome
  is not expected utility, permission, safety, or an instruction to act.
- **Adopt a learned set predictor immediately.** Rejected because the repository
  lacks the transition data, deterministic baseline, split discipline, and
  calibration evidence required to evaluate one.
- **Persist prediction errors automatically.** Rejected because compilation is
  read-only and model output is not independent evidence.

## Consequences

The product, architecture, proof, renderer, evaluation, persistence, and
delivery specifications must distinguish focus from expectation and map both
to the unchanged one-text product result.

Transition-memory creation, observation capture, correction, and persistent
learning remain management-plane work. They require explicit authority,
provenance, migration, and consent contracts before implementation.

The first repository work after this decision is the evidence manifest,
receipt, split, and product-headroom harness required by the proof program.
Only after its entrance gates pass does predictive-semantic implementation
proceed in order: transition schema and validators, deterministic expectation
baseline, expectation evaluation corpus, uncertainty and abstention, combined
plan, renderer adaptation, then end-to-end integration. Learned prediction
cannot enter the critical path before sufficient governed real transition
evidence exists.

The architecture is an accepted implementation direction, not validated
product behavior. No release may claim probability, prediction quality,
biological equivalence, action quality, or generalization from this decision
alone.
