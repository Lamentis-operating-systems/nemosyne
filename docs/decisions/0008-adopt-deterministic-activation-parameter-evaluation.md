# 0008: Adopt deterministic activation parameter evaluation

Status: Accepted
Date: 2026-07-22

## Context

The activation kernel ranks candidates for one explicit profile, but the project has no reproducible way to measure whether one fixed parameter set satisfies expected preferences across multiple situations. Selecting or tuning weights without that measurement boundary would produce unverifiable numbers and mix evaluation with optimization.

The evaluator must preserve the kernel's narrow numeric contract. It must not establish how gates or candidate signals are derived, introduce default parameters, or treat a hand-authored suite as evidence of real-world validity. Its preference representation must also prevent logically equivalent partial orders from receiving different metric weights merely because one declaration includes redundant transitive edges.

Finite `f64` arithmetic creates a second boundary: evidence-weight tuples that are algebraically proportional over real numbers can produce different representable scores and strict outcomes. The evaluator must expose this behavior rather than hide it behind normalization or an unstated tolerance.

## Decision

Add a non-published `nemosyne-evaluation` workspace crate with a public activation-evaluation API and only a path dependency on `nemosyne-core`.

Represent fixed evidence weights and inhibition strengths separately from scenarios. Each scenario supplies situation-dependent evidence gates, activation candidates, and an acyclic transitive reduction of expected strict pairwise preferences. Reject a declared preference when another directed path already connects the same preferred candidate to the same other candidate. Partial orders and candidates absent from the preference graph remain valid.

For each scenario, construct one `ActivationProfile` and call `rank_activations` exactly once. Preserve the caller's exact finite `f64` parameter values and compare returned activation scores exactly. Do not normalize, quantize, rescale, or use an epsilon. The kernel's candidate-identifier tie-break does not satisfy a preference whose scores are equal.

Return the complete kernel ranking and both scores and the strict outcome for every preference. Derive each scenario's counts and accuracy solely from its preference results. Derive global counts, micro-accuracy, macro-accuracy, and scenario pass rate solely from the ordered scenario reports. Do not accept caller-supplied aggregates or maintain a second calculation path.

Canonicalize identifier-bearing collections, preserve kernel errors with their scenario identifiers, and abort without a partial report when any scenario fails. Do not return a composite fitness score, a recommended parameter set, or an implicit baseline.

Strict outcomes and accuracy describe agreement for one exact parameter tuple and one finite suite. They are not margin, robustness, numerical-stability, generalization, or safety results. Downstream parameter-selection claims require semantically grounded preference provenance, explicit baselines, and disjoint calibration and held-out suites. Those evidence artifacts remain caller responsibilities.

The exact contract is maintained in [`activation-parameter-evaluation.md`](../specifications/activation-parameter-evaluation.md).

## Rationale

Separating fixed parameters from situation-specific gates allows the same parameter set to be tested across a suite and later compared by an independent calibrator. Pairwise preferences express partial orders without inventing target score magnitudes. Requiring their transitive reduction prevents redundant graph syntax from silently reweighting micro-accuracy.

Calling the kernel once per scenario makes it the sole scoring authority and ensures all preference comparisons share one canonical result. Deriving aggregates from preference outcomes makes those observations the report's source of truth. Retaining both candidate scores permits separate robustness analysis without weakening the strict evaluation contract.

Preserving exact parameter tuples keeps the evaluator faithful to the kernel and avoids hidden numeric policy. Explicitly documenting the finite-precision boundary prevents algebraic equivalence from being mistaken for guaranteed operational equivalence.

A separate crate keeps offline evaluation concerns outside the runtime kernel. Micro-accuracy weights every accepted preference edge equally, macro-accuracy weights every scenario equally, and scenario pass rate exposes complete scenario agreement without combining them into an unvalidated objective.

## Alternatives

- **Add evaluation to `nemosyne-core`.** Rejected because dataset-level measurement is an offline concern and would broaden the runtime kernel.
- **Store a complete activation profile in every scenario.** Rejected because weights and strengths could vary silently between situations, preventing comparison of one parameter set.
- **Require a total candidate order.** Rejected because many scenarios justify only specific pairwise expectations.
- **Count every explicitly declared transitive edge.** Rejected because equivalent partial orders could receive different metric weights based only on redundant representation.
- **Automatically remove transitive edges.** Rejected because silently repairing a suite would hide an authoring error and change the caller's declared observations.
- **Expand every graph to its transitive closure.** Rejected because it would invent observations and disproportionately weight long ordered chains.
- **Normalize proportional evidence weights.** Rejected because it would mutate exact caller input, add rounding policy, and obscure kernel behavior.
- **Quantize parameters before evaluation.** Rejected because no precision grid has been selected and such a grid belongs to a future parameter-search contract.
- **Use target scores, margins, or an epsilon.** Rejected because no calibrated score scale or universal tolerance has been established.
- **Treat ties according to kernel ranking order.** Rejected because the expectation is a strict score preference.
- **Accept externally supplied counts or aggregate metrics.** Rejected because derived state could disagree with the underlying preference results.
- **Provide built-in baselines or a composite fitness score.** Rejected because both would introduce unvalidated policy.
- **Implement calibration in the same change.** Rejected because optimization requires an independently testable evaluator and curated evidence first.
- **Make the evaluator own provenance, dataset splits, serialization, storage, or a CLI.** Rejected because those concerns require separate contracts.

## Consequences

Callers must construct explicit parameter sets and validated numeric scenarios whose preference graphs contain no cycles or redundant transitive edges. They remain responsible for the meaning, provenance, and quality of gates, signals, and expected preferences.

Two proportional parameter tuples remain distinct inputs and can produce different strict results through finite `f64` rounding. Callers that search parameter space must define any quantization or canonical representative outside this evaluator.

The report has one derivation path from preference outcomes through scenario reports to global aggregates. Score pairs remain available for independent robustness analysis, but no robustness metric or threshold is selected.

The evaluator adds no text, prompt, embedding, vector, persistence, dataset generation, parallel processing, or parameter learning. It does not change `nemosyne-core`, publish a crate, add an external dependency, or alter release behavior. A curated calibration suite, a disjoint held-out suite, and any later parameter calibrator require separate decisions and evidence.
