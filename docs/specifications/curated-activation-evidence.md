# Curated activation evidence

Status: Experimental

## Purpose

This specification defines a versioned, synthetic evidence corpus for evaluating
activation parameters in a narrow coding-agent pilot domain. It binds numeric
gates, candidate signals, and expected preferences to reviewable semantic
provenance and supplies explicit reference parameter sets for the existing
activation evaluator.

The corpus does not derive signals, calibrate parameters, or establish empirical
ground truth. It is implemented in the non-published
`nemosyne-evaluation-corpus` workspace crate. `nemosyne-core` remains the sole
activation-ranking authority, and `nemosyne-evaluation` remains the sole
evaluation authority.

## Definitions

Corpus revision `1` defines five positive-evidence channels:

| Channel ID | Key | Gate meaning | Candidate-signal meaning |
| ---: | --- | --- | --- |
| 10 | `trigger_alignment` | Relevance of the explicit request to the current decision | Fit between the candidate and the explicit request |
| 20 | `observed_state_alignment` | Relevance of observed workspace, runtime, or instruction-authority state | Fit between the candidate and the observed state |
| 30 | `active_outcome_alignment` | Relevance of the currently active task outcome | Contribution of the candidate to the active task outcome |
| 40 | `capability_fit` | Relevance of current tools, permissions, and executable actions | Feasibility of using the candidate under current capabilities |
| 50 | `constraint_alignment` | Relevance of an explicit active constraint | Compatibility of the candidate with that constraint |

These channel meanings are local hypotheses of the coding-agent corpus. They
are not a general memory ontology and do not change the generic channel model
of the activation kernel.

Authored gates and signals use only this dyadic evidence grid:

| Level | Value | General ordering |
| --- | ---: | --- |
| `Absent` | `0` | No supporting applicability or fit |
| `Low` | `0.25` | Indirect or peripheral applicability or fit |
| `Medium` | `0.5` | Material but secondary applicability or fit |
| `High` | `0.75` | Direct and important applicability or fit |
| `Maximal` | `1` | Explicitly dominant applicability or exact fit |

The values are exact binary floating-point fractions. They reduce false
precision but remain authored judgments, not probabilities or calibrated
measurements. Gates express how strongly a channel matters to the current
decision:

| Gate channel | `Absent` | `Low` | `Medium` | `High` | `Maximal` |
| --- | --- | --- | --- | --- | --- |
| Trigger alignment | The request does not affect the decision | The request has peripheral bearing | The request materially frames part of the decision | The request directly frames the decision | The request is the dominant cue |
| Observed-state alignment | Observed state does not affect the decision | Old or indirect state has peripheral bearing | Material current state informs part of the decision | Current state directly shapes the decision | The exact observed state is the dominant cue |
| Active-outcome alignment | The active task outcome does not affect the decision | The active task outcome has peripheral bearing | The active task outcome materially frames part of the decision | The active task outcome directly shapes the decision | The active task outcome is the dominant cue |
| Capability fit | Capabilities do not affect the decision | Capabilities have peripheral bearing | Capabilities materially narrow some options | Capabilities directly constrain feasible options | Available or missing capability dominates the decision |
| Constraint alignment | No explicit constraint applies | A constraint has peripheral bearing | A constraint materially governs part of the decision | A constraint directly governs the decision | A hard active constraint dominates the decision |

Candidate signals express how well one candidate fits each active channel:

| Candidate-signal channel | `Absent` | `Low` | `Medium` | `High` | `Maximal` |
| --- | --- | --- | --- | --- | --- |
| Trigger alignment | Unrelated to the request | Shares only a peripheral topic | Addresses the broad task | Directly addresses the operation | Is the exact requested focus |
| Observed-state alignment | Unsupported by current observations | Related through old or indirect state | Consistent with material current state | Directly supported by current state | Identifies the exact observed state |
| Active-outcome alignment | Does not advance the outcome | Has an indirect possible contribution | Makes a material secondary contribution | Directly advances the outcome | Constitutes the active task outcome |
| Capability fit | Cannot currently be applied | Requires substantial missing capability | Is partially executable | Is executable with minor friction | Is immediately executable |
| Constraint alignment | Conflicts with the active constraint | Satisfies a peripheral part | Satisfies a material part | Directly satisfies the constraint | Exactly preserves the constraint |

No active judgment level is an unspecified or neutral placeholder. Every
anchored value also exposes whether it is `Applicable` or `Inactive`. Gates are
always applicable judgments. When a gate is `Absent`, every corresponding
candidate signal is canonically `Inactive` with numeric level `Absent`; that
zero exists only to satisfy the evaluator's complete channel schema and is not
a candidate-fit judgment. A nonzero signal under an absent gate is invalid.

A scenario contains:

- a globally unique `ScenarioId`;
- a `SemanticCaseId` shared by its paired contrast;
- a broad `ScenarioCategoryId`;
- a declared `Calibration` or `HeldOut` split;
- a concise title and situation description;
- constructed scenario facts with scenario-local `FactId` values;
- one anchored gate value per channel, each with its own cited facts and
  channel-specific rationale;
- at least two candidate focus items, each with one anchored signal per channel,
  each with its own cited facts and channel-specific rationale; and
- one or more strict expected preferences, each with cited facts and a
  rationale authored independently of evaluator output.

All scenario provenance is explicitly `Constructed`. Human-readable evidence is
offline audit metadata and is never passed to the activation kernel.

`ScenarioCategoryId` represents broad coverage and may occur in both splits.
`SemanticCaseId` represents one semantic source and must occur in exactly one
split. Revision `1` contains four categories and eight semantic cases. Each case
contains two paired-contrast scenarios with the same candidate identifiers and
descriptions and an exactly reversed strict preference set:

| Scenarios | Split | Category | Semantic case |
| --- | --- | --- | --- |
| 1001, 1002 | Calibration | Active constraints | Activate or deactivate a preservation constraint |
| 1101, 1102 | Calibration | Observed state | Select the currently implicated diagnostic source |
| 1201, 1202 | Calibration | Requested outcome | Follow the exact request focus |
| 1301, 1302 | Calibration | Operational feasibility | Follow the active task outcome |
| 2001, 2002 | Held-out | Operational feasibility | Prefer the currently executable capability |
| 2101, 2102 | Held-out | Requested outcome | Implement and test or perform read-only review |
| 2201, 2202 | Held-out | Active constraints | Apply documentation governance only when required |
| 2301, 2302 | Held-out | Observed state | Follow the current authoritative correction |

Paired contrasts expose a preference reversal under two related authored
situations. They do not establish a controlled intervention or causal effect.
A held-out split is a declared usage boundary, not access control or a
statistically blind test set. Revision `1` reference results were visible
during implementation and are frozen as regression observations. The held-out
partition therefore exercises the separation workflow but cannot support an
independent performance or generalization claim.

Semantic-source separation alone is insufficient when two expected
preferences have the same algebraic structure. Let `q` map the five levels to
the integers `0` through `4`. For every expected preference \(p \succ o\),
revision `1` computes the integer coefficient vector

\[
z_c=q(g_c)\left(q(e_{p,c})-q(e_{o,c})\right).
\]

For mixed-sign vectors, the vector is divided by the greatest common divisor of
its nonzero absolute coefficients. For a vector with only positive or only
negative coefficients, each nonzero coefficient is reduced to its sign. Zero
coordinates and coefficient signs are retained; an all-zero vector has its own
canonical form. No canonical algebraic preference signature may occur in both
partitions. Under idealized nonnegative real arithmetic without inhibition, a
one-sign inequality depends only on signed support, while a mixed-sign
inequality is preserved by positive scaling. The check is therefore a
conservative structural leakage guard. It may reject two preferences whose
exact `f64` evaluator scores or outcomes differ because floating-point
normalization and rounding are not part of the signature. Signature equality
does not imply equal score bits or equal `Satisfied`, `Tied`, or `Violated`
outcomes. The guard is not a complete test for distributional leakage and does
not establish statistical independence.

Merged corpus revisions are immutable. Any observable change to channels,
rubrics, facts, judgments, preferences, splits, or reference parameters creates
a new revision while preserving revision `1`. Once held-out results have
influenced parameters, channels, rubrics, or authoring, later held-out claims
require previously unauthored semantic cases; revising observed cases does not
restore independence.

Revision `1` contains authored contrasts involving all five channels. Those
contrasts also vary other channel judgments and therefore do not demonstrate
independent channel discrimination. Cases 1001 and 1002 provide semantic
constraint coverage, but trigger, observed-state, and outcome evidence already
support their expected preference when constraint weight is zero.

One fixed local sensitivity observation is executable for cases 2201 and 2202:

| Cases | Baseline weights for channels 10, 20, 30, 40, 50 | Ablation weights | Baseline outcomes | Ablated outcomes |
| --- | --- | --- | --- | --- |
| 2201, 2202 | `1, 1, 1, 1, 1` | `1, 1, 1, 1, 0` | `Satisfied`, `Satisfied` | `Violated`, `Satisfied` |

At those exact authored inputs and parameter points, removing constraint
evidence changes case 2201 but not case 2202. This observation does not
establish a controlled intervention, causal effect, channel independence,
identifiability, or generalization.

Revision `1` defines two reference parameter sets:

| Reference ID | Key | Weights for channels 10, 20, 30, 40, 50 |
| ---: | --- | --- |
| 10 | `trigger_only` | `1, 0, 0, 0, 0` |
| 20 | `uniform_evidence` | `1, 1, 1, 1, 1` |

Neither reference contains inhibition. They are explicit measurement points,
not defaults, recommendations, calibrated parameters, or quality thresholds.

The principal public operation is:

```rust
pub fn coding_agent_v1() -> Result<ActivationEvidenceCorpus, CorpusError>;
```

`ActivationEvidenceCorpus` exposes its revision, channel and category
definitions, ordered reference parameter sets, and `Calibration` and `HeldOut`
partitions. Each partition exposes semantic scenario evidence and the derived
existing `EvaluationSuite`. Numeric evaluator input is derived from the
annotated evidence; it is not accepted as a second independent truth.

The artifact also exposes:

```rust
pub fn regression_fingerprint(&self) -> u64;
```

This deterministic, non-cryptographic value covers every publicly observable
corpus field, including the derived evaluator suites. It is a revision
regression tripwire and diagnostic only; equality does not prove identity,
authenticity, or integrity. Revision `1` uses 64-bit FNV-1a with the protocol
prefix `nemosyne.activation-evidence-corpus.regression-fingerprint.v1`.
Canonical sequences include their length, strings use a length-prefixed UTF-8
encoding, integers use little-endian bytes, floating-point values use their
exact bit pattern, and level, applicability, split, and provenance variants use
fixed tags. Protocol version `1` defines explicit encodings for its supported
evidence and inhibition parameter kinds. Encountering a future unsupported
parameter kind fails loudly rather than silently assigning a shared fallback
encoding.

## Preconditions

Corpus construction enforces:

- nonempty, already-trimmed human-readable metadata;
- unique channel, category, reference, scenario, fact, and candidate identifiers
  in their documented scopes;
- unique, byte-exact, case-sensitive channel, category, and reference stable
  keys within their namespaces;
- exactly the five revision-1 evidence channels and no inhibition parameter;
- one gate and one candidate signal per channel, using only `EvidenceLevel`;
- fact references that resolve within the containing scenario;
- inactive candidate signals that use only the canonical zero encoding;
- preference evidence that corresponds exactly to the derived evaluator
  preference;
- nonempty calibration and held-out partitions;
- corpus-wide unique scenario identifiers;
- semantic cases that never cross split or category boundaries;
- no canonical algebraic preference signature shared by calibration and
  held-out evidence;
- exactly two scenarios per semantic case, with matching candidate identities
  and reversed expected preferences;
- every category occurring in both splits;
- reference channel schemas matching the corpus channels exactly; and
- a positive trigger-alignment gate in every revision-1 scenario, so the
  trigger-only reference always has effective evidence.

Existing core and evaluator constructors validate candidate, gate, preference,
and suite structure. Corpus construction preserves their errors with corpus
context and evaluates every reference parameter set against both partitions to
verify complete numeric compatibility. A tied or violated reference preference
is valid corpus evidence and does not fail construction.

## Invariants

Channels, categories, references, scenarios, facts, gates, candidates,
candidate signals, and preference evidence are exposed in ascending numeric
identifier order. Rebuilding revision `1` produces equal public artifacts,
equal regression fingerprints, and equal evaluator reports on one
floating-point environment.

The corpus maintains one derivation path:

```text
constructed facts and anchored judgments
    -> ActivationCandidate and EvaluationScenario
    -> EvaluationSuite
    -> evaluate_parameters
```

It does not reproduce ranking mathematics, preference outcomes, or aggregate
metrics.

Construction never mutates facts, judgments, or preferences in response to a
reference result.

The authoring and review protocol, rather than the constructor, provides the
semantic boundary: situation facts may describe only the request, observed
state, active outcome, capability, or constraint. They must not contain channel
levels, scores, parameter references, ranking conclusions, or evaluator
results. Preference rationales derive only from cited facts. Every channel
judgment is authored separately with its own cited facts and explanation of why
those facts support the chosen rubric level. Construction does not synthesize
per-channel provenance from vector-wide metadata.

A future corpus used for an independent selection claim must freeze its facts,
preferences, rationales, and annotations before any held-out report is
inspected. Revision `1` does not claim that stronger process property.

## Edge cases

- Empty or whitespace-padded metadata is invalid rather than normalized.
- Missing, duplicate, or unknown fact references are errors that identify the
  exact gate, candidate signal, or preference containing the reference.
- Duplicate channel, category, or reference stable keys are invalid even when
  their numeric identifiers differ.
- A nonzero candidate signal under an absent gate is an error.
- A semantic case crossing splits is an error even when scenario identifiers
  differ.
- A category absent from one split is an error.
- A paired contrast with different candidate identities or descriptions, or
  preferences that do not reverse exactly, is an error.
- Unassessed candidates and other unevaluated aspects may share coarse numeric
  representations across splits, but canonical algebraic preference signatures
  may not.
- Candidates absent from the expected partial order remain valid and appear in
  the complete evaluator ranking.
- Reference ties and violations remain reportable and do not invalidate the
  corpus.

## Verification

Tests must verify the complete revision-1 artifact through its public API,
including channel definitions, evidence-grid conversion, canonical order,
partition sizes, split disjointness, case pairing, category coverage,
per-channel provenance resolution, conservative algebraic preference-signature
isolation, the exact-`f64` non-equivalence boundary of that guard, partial-order
distractors, absence of inhibition, the fixed constraint-ablation observation,
adversarial permutation of every reorderable authoring collection, and the
complete revision fingerprint.

Both reference parameter sets must evaluate both partitions through
`evaluate_parameters`. Tests freeze every scenario's ordered candidate ranking,
exact final-score bits, expected preference, and observed preference outcome.
Evidence scores must equal final scores and retention must equal one because
revision `1` contains no inhibition. Aggregate counts are reconstructed from
those scenario fixtures before they are compared with the evaluator report.
Reference success, ties, or violations are observations, not corpus validity
rules, quality thresholds, or proof of non-circular authoring.

Revision `1` records these derived observations:

| Reference | Split | Satisfied | Tied | Violated |
| --- | --- | ---: | ---: | ---: |
| `trigger_only` | Calibration | 4 | 4 | 0 |
| `trigger_only` | Held-out | 5 | 2 | 1 |
| `uniform_evidence` | Calibration | 8 | 0 | 0 |
| `uniform_evidence` | Held-out | 8 | 0 | 0 |

`CorpusError` distinguishes invalid metadata, unresolved fact references,
inactive-signal encoding, split, category, semantic-case, paired-contrast,
cross-split algebraic preference-signature, stable-key, and reference-parameter
violations, plus wrapped activation or evaluation failures. Fact-reference
failures retain the exact evidence location. Wrapped failures retain the
affected split, scenario, or reference identifier as applicable and preserve
their underlying source through `Error::source`.

Repository verification follows the documentation, formatting, Clippy, Rustdoc,
and test checks required by `AGENTS.md`.

## Open questions

Signal and gate derivation from observable inputs, empirical scenario
collection, corpus expansion, parameter calibration, margin or robustness
analysis, inhibition semantics, statistical interpretation, and release
thresholds remain outside this specification.

## References

- [Activation parameter evaluation](activation-parameter-evaluation.md)
- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Decision 0010: Adopt a versioned curated activation evidence corpus](../decisions/0010-adopt-versioned-curated-activation-evidence.md)
- [`nemosyne-evaluation-corpus`](../../crates/nemosyne-evaluation-corpus/)
