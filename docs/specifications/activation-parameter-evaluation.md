# Activation parameter evaluation

Status: Experimental

## Purpose

This specification defines deterministic offline evaluation of fixed activation parameters across a finite suite of numeric scenarios. It measures whether the existing activation kernel produces explicitly expected pairwise candidate preferences. It does not select, optimize, or learn parameters.

The evaluator is implemented in the non-published `nemosyne-evaluation` workspace crate. The activation formula and candidate ranking remain owned by `nemosyne-core`.

## Definitions

`ScenarioId` is an opaque numeric identifier. `ChannelId`, `CandidateId`, `ActivationCandidate`, `RankedActivation`, and `UnitInterval` retain their definitions from the situation-conditioned activation specification.

An activation parameter set

\[
\theta = (\{w_c\}_{c\in C}, \{\lambda_j\}_{j\in J})
\]

contains:

- a finite set `C` of evidence parameters, each with a channel identifier and fixed weight `w_c` in `[0, 1]`; and
- a finite set `J` of inhibition parameters, each with a channel identifier and fixed strength `lambda_j` in `[0, 1]`.

Evidence and inhibition parameters share one channel-identifier namespace. At least one evidence weight must be positive. Parameter sets contain no gates and provide no defaults.

An evaluation scenario `s` contains:

- one scenario identifier;
- exactly one situation-dependent gate `g_(s,c)` in `[0, 1]` for every evidence channel `c` and no gate for an inhibition channel;
- at least two activation candidates with complete normalized signals; and
- a nonempty set `P_s` of expected directed preferences.

An expected preference `(a, b)` means only that candidate `a` is expected to receive a strictly greater activation score than candidate `b`. Preferences may define a partial order; candidates need not occur in any preference.

Every declared preference is one independently counted observation. Explicitly declaring a transitive preference therefore gives that relation the same weight as every other declared edge.

An evaluation suite is a nonempty set of scenarios. Parameters, scenarios, gates, candidates, candidate signals, and preferences are stored in ascending numeric identifier order. Preferences are ordered first by preferred candidate and then by other candidate.

The public operation is:

```rust
pub fn evaluate_parameters(
    parameters: &ActivationParameters,
    suite: &EvaluationSuite,
) -> Result<EvaluationReport, EvaluationError>;
```

The public input model comprises `ActivationParameter`, `EvidenceParameter`, `InhibitionParameter`, `ActivationParameters`, `EvidenceGate`, `ExpectedPreference`, `EvaluationScenario`, and `EvaluationSuite`. The public output model comprises `PreferenceOutcome`, per-preference and per-scenario results, and `EvaluationReport`. `EvaluationError` represents invalid evaluator input or a scenario-scoped kernel failure.

The principal validating constructors are:

```rust
ActivationParameters::new(Vec<ActivationParameter>)
EvaluationScenario::new(
    ScenarioId,
    Vec<EvidenceGate>,
    Vec<ActivationCandidate>,
    Vec<ExpectedPreference>,
)
EvaluationSuite::new(Vec<EvaluationScenario>)
```

All public model fields are private. Construction validates and canonicalizes owned input; documented getters expose immutable views and scalar values.

For every scenario, the evaluator creates one activation profile from the fixed weights and strengths plus that scenario's gates. It invokes `rank_activations` exactly once. For candidate `i`, the existing kernel computes:

\[
A_{s,i} =
\frac{\sum_{c\in C} w_c g_{s,c} e_{s,i,c}}
     {\sum_{c\in C} w_c g_{s,c}}
\prod_{j\in J}(1-\lambda_j p_{s,i,j})
\]

The evaluator does not reproduce or modify this calculation.

For an expected preference `(a, b)`, its outcome is:

\[
O_s(a,b)=
\begin{cases}
\mathrm{Satisfied}, & A_{s,a}>A_{s,b}\\
\mathrm{Tied}, & A_{s,a}=A_{s,b}\\
\mathrm{Violated}, & A_{s,a}<A_{s,b}
\end{cases}
\]

Comparisons use the computed `f64` values exactly. They use neither an epsilon nor the kernel's candidate-identifier tie-break.

For scenario `s`, let `N_s = |P_s|` and let `S_s` be its satisfied-preference count. Scenario accuracy is:

\[
Q_s=\frac{S_s}{N_s}
\]

For `K` scenarios, the report metrics are:

\[
Q_{micro}=\frac{\sum_s S_s}{\sum_s N_s}
\]

\[
Q_{macro}=\frac{1}{K}\sum_s Q_s
\]

\[
Q_{pass}=\frac{|\{s\mid S_s=N_s\}|}{K}
\]

A tie is not satisfied. A scenario passes only when all its preferences are satisfied.

The report contains:

- every scenario in ascending `ScenarioId` order;
- the complete kernel ranking for each scenario;
- each expected preference, both candidate scores, and its outcome;
- satisfied, tied, violated, and total counts per scenario and globally; and
- micro-accuracy, macro-accuracy, and scenario pass rate as finite values in `[0, 1]`.

The report exposes no composite fitness value and does not identify a preferred parameter set.

## Preconditions

Validated constructors and evaluation enforce:

- parameter channel identifiers are unique across evidence and inhibition kinds;
- at least one evidence parameter has a positive weight;
- a suite contains at least one scenario and scenario identifiers are unique;
- every scenario contains at least two candidates with unique identifiers;
- every scenario contains at least one expected preference;
- a preference references two existing, distinct candidates;
- each directed preference occurs once and the preference graph is acyclic;
- every scenario supplies exactly one gate for each evidence parameter and no other gate; and
- every candidate supplies exactly the signal schema required by the generated activation profile.

Duplicate items, missing or unknown gates, gates targeting inhibition channels, invalid preferences, and activation-kernel failures are explicit errors. No missing input is interpreted as zero.

## Invariants

Canonical ordering makes the complete report independent of input collection order. Evaluation order is ascending `ScenarioId`; preference order is ascending `(preferred CandidateId, other CandidateId)`.

Canonical evaluation supports repeatable results on one floating-point environment. Like the activation kernel, the evaluator does not claim bit-identical results across distinct floating-point environments.

Each scenario invokes the activation kernel once, and every preference score is read from that one result. The evaluator neither recalculates scores nor calls the kernel per preference.

For every successful report:

\[
N_{satisfied}+N_{tied}+N_{violated}=N_{total}
\]

globally and within every scenario. All three aggregate metrics are finite and in `[0, 1]`.

The report contains all ranked candidates, including candidates absent from the partial expected order. Preference evaluation does not filter or reorder the kernel ranking.

An error aborts the complete operation. No partial report is returned. An `ActivationError` is preserved as `EvaluationError::Activation { scenario_id, source }` together with the affected `ScenarioId`.

## Edge cases

- An empty suite is invalid.
- A scenario with fewer than two candidates is invalid.
- A scenario without preferences is invalid.
- A self-preference, duplicate preference, unknown candidate reference, or cyclic preference graph is invalid.
- Both opposing preferences between two candidates form a cycle and are invalid.
- A transitive but acyclic preference is valid.
- Candidates not referenced by a preference remain valid and appear in the ranking.
- A scenario in which every positive evidence weight has gate zero fails through the activation kernel; it is not assigned a zero score or skipped.
- Equal activation scores produce `Tied` even when the kernel orders the candidates by identifier.
- One invalid scenario prevents evaluation of every report metric.

## Operational boundary

This evaluator accepts only already normalized numeric gates and candidate signals. It does not derive those inputs from text, prompts, embeddings, vectors, world state, or memory.

It provides no parameter selection, optimization, learning, grid search, random search, gradient method, default parameters, automatic baseline, durable dataset format, serialization, database, CLI, scenario generator, dataset split, statistical significance test, release threshold, safety guarantee, parallel execution, or performance benchmark.

The suite is an in-memory measurement input. Its results establish only agreement between one explicit parameter set and that finite suite.

## Verification

Public-boundary tests must cover a hand-calculated multi-scenario suite; satisfied, tied, and violated preferences; micro-, macro-, and scenario-pass calculations; and the global and per-scenario count invariant.

Tests must also cover exact ties despite candidate-identifier ordering, partial orders, all structural input failures, missing and unknown gates, gates targeting inhibition channels, preference cycles, activation errors with scenario context, and identical reports for permuted inputs.

At least one test must evaluate two explicitly constructed parameter sets against the same suite without describing either set as optimal. Repository verification follows the documentation, formatting, Clippy, Rustdoc, and test checks required by `AGENTS.md`.

## Open questions

None within this evaluator. Scenario authoring, dataset persistence, baseline selection, parameter calibration, statistical interpretation, and release thresholds remain outside this specification.

## References

- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Decision 0008: Adopt deterministic activation parameter evaluation](../decisions/0008-adopt-deterministic-activation-parameter-evaluation.md)
- [`nemosyne-core`](../../crates/nemosyne-core/)
- [`nemosyne-evaluation`](../../crates/nemosyne-evaluation/)
