# 0008: Adopt deterministic activation parameter evaluation

Status: Accepted
Date: 2026-07-22

## Context

The activation kernel ranks candidates for one explicit profile, but the project has no reproducible way to measure whether one fixed parameter set satisfies expected preferences across multiple situations. Selecting or tuning weights without that measurement boundary would produce unverifiable numbers and would mix evaluation with optimization.

The evaluator must preserve the kernel's narrow numeric contract. It must not establish how gates or candidate signals are derived, introduce default parameters, or treat a hand-authored suite as evidence of real-world validity.

## Decision

Add a non-published `nemosyne-evaluation` workspace crate with a public activation-evaluation API and only a path dependency on `nemosyne-core`.

Represent fixed evidence weights and inhibition strengths separately from scenarios. Each scenario supplies situation-dependent evidence gates, activation candidates, and an acyclic set of expected strict pairwise preferences. Require complete, explicit channel correspondence and reject invalid structure rather than filling missing values.

For each scenario, construct one `ActivationProfile` and call `rank_activations` exactly once. Compare the returned activation scores using exact `f64` ordering. Classify every declared preference as satisfied, tied, or violated; a kernel identifier tie-break does not satisfy a preference whose scores are equal.

Return the complete scenario rankings, per-preference scores and outcomes, outcome counts, micro-accuracy, macro-accuracy, and the proportion of scenarios whose preferences are all satisfied. Count every explicitly declared edge once, including a declared transitive edge. Do not return a composite fitness score, a recommended parameter set, or an implicit baseline.

Canonicalize all identifier-bearing collections. Preserve any activation-kernel error as the source of an evaluation error annotated with its scenario identifier. Abort without a partial report when any scenario fails.

The exact contract is maintained in [`activation-parameter-evaluation.md`](../specifications/activation-parameter-evaluation.md).

## Rationale

Separating fixed parameters from situation-specific gates allows the same parameter set to be tested across a suite and later compared by an independent calibrator. Pairwise preferences express partial orders without inventing target score magnitudes. Reporting ties separately prevents deterministic presentation order from being mistaken for evidence that a strict expectation was met.

Micro-accuracy weights every declared preference equally. Macro-accuracy weights every scenario equally. Scenario pass rate exposes whether complete scenario expectations hold. Keeping these metrics separate avoids hiding tradeoffs in an unvalidated aggregate objective.

Calling the kernel once per scenario makes it the sole scoring authority and ensures all preference comparisons share one canonical result. A separate crate keeps offline evaluation concerns outside the runtime kernel.

## Alternatives

- **Add evaluation to `nemosyne-core`.** Rejected because dataset-level measurement is an offline concern and would broaden the runtime kernel.
- **Store a complete activation profile in every scenario.** Rejected because weights and strengths could vary silently between situations, preventing meaningful comparison of one parameter set.
- **Require a total candidate order.** Rejected because many scenarios justify only specific pairwise expectations.
- **Use target activation values or score margins.** Rejected because no calibrated score scale or meaningful universal margin has been established.
- **Treat ties according to kernel ranking order or an epsilon.** Rejected because the expectation is strict score preference and no tolerance policy has been validated.
- **Provide equal-weight, semantic-only, or other built-in baselines.** Rejected because baselines must be explicit inputs rather than hidden policy.
- **Combine the metrics into one fitness score.** Rejected because the relative value of preference-level accuracy and whole-scenario success is not established.
- **Implement calibration in the same change.** Rejected because optimization requires an independently testable evaluator and curated evaluation evidence first.
- **Add serialization, storage, or a CLI.** Rejected because the first contract is an in-memory library boundary and no durable dataset format is selected.

## Consequences

Callers must construct explicit parameter sets and validated numeric scenarios. They remain responsible for the meaning, provenance, and quality of gates, signals, and expected preferences. Metrics describe agreement with one finite suite; they do not establish optimality, statistical significance, generalization, probability calibration, or safety.

The evaluator adds no text, prompt, embedding, vector, persistence, dataset-generation, parallel-processing, or parameter-learning capability. It does not change `nemosyne-core`, publish a crate, add an external dependency, or alter release behavior.

A curated evaluation suite and an offline parameter calibrator remain separate future decisions. Any calibrator must consume this evaluator as a measurement boundary rather than duplicate the activation formula.
