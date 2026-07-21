# 0005: Adopt a deterministic activation kernel

Status: Accepted
Date: 2026-07-22

## Context

Nemosyne needs a first executable algorithm without treating its future memory, situation, retrieval, or rendering architecture as settled. The available inputs at this boundary are already normalized numeric signals. The first implementation must rank candidates deterministically, expose how each input affected the result, reject incomplete data, and avoid production dependencies or unvalidated default parameters.

## Decision

Add a public activation module to `nemosyne-core`. It accepts a profile of generic numeric evidence and inhibition channels plus candidates containing complete signal sets. Channel meanings remain caller-defined.

Each evidence channel has an explicit weight and situation-dependent gate. Evidence is their normalized weighted mean:

\[
a_c = w_c g_c, \qquad
D = \sum_c a_c > 0, \qquad
E_i = \frac{\sum_c a_c e_{i,c}}{D}
\]

Each inhibition channel has an explicit strength. Inhibition multiplicatively retains evidence:

\[
R_i = \prod_j (1 - \lambda_j p_{i,j}), \qquad
A_i = E_i R_i
\]

Evaluate channels in ascending numeric identifier order. Return all candidates by descending activation and then ascending candidate identifier for exact ties. Return a complete contribution breakdown and explicit errors for invalid numeric values, duplicate identifiers, incomplete or unknown signals, and a non-positive evidence denominator.

Use private fields, validated constructors, documented getters, and no default weights. Add no production dependency. The exact behavioral contract is maintained in [`situation-conditioned-activation.md`](../specifications/situation-conditioned-activation.md).

## Rationale

The weighted evidence mean keeps differently scaled channel counts comparable while situation gates make relevance explicit. Multiplicative retention preserves a bounded score and makes each inhibition independently inspectable. Generic channels allow experiments with semantic, temporal, goal, risk, or other signals without encoding an unvalidated ontology into the core API.

Canonical evaluation and identifier-based tie-breaking remove input-order ambiguity. Complete breakdowns make the calculation testable and auditable. Requiring callers to provide parameters prevents experimental values from becoming implicit project policy.

## Alternatives

- **Accept vector inputs and compute similarities in the kernel.** Rejected because vector representation, dimensions, normalization, and similarity metrics are not yet selected. This kernel begins after feature computation.
- **Encode fixed cognitive fields.** Rejected because names such as goal, time, risk, or semantic relevance would prematurely establish an ontology. Generic channels preserve the narrower mathematical boundary.
- **Provide default weights or gates.** Rejected because no evaluated defaults exist. Callers must make every parameter explicit.
- **Use additive inhibition.** Rejected because subtraction requires an additional clipping policy and can obscure the independent retained fraction of each inhibition channel.
- **Use a logistic score.** Rejected because a bias and unconstrained coefficients would require calibration, would not reach exact interval boundaries, and would make a first implementation harder to reconstruct directly.
- **Implement an end-to-end prompt-to-attention pipeline.** Rejected because text encoding, world and situation models, storage, retrieval, selection, and language rendering remain separate unresolved boundaries.

## Consequences

Upstream code is responsible for producing normalized signals and explicit parameters. The core ranks those values but does not judge whether a channel is meaningful, truthful, or safe. A full-strength, full-signal inhibition reduces activation to zero; callers must choose such strengths deliberately.

This change does not provide text or embedding processing, vector metrics, persistence, graph activation, hard eligibility rules, top-k selection, diversity, token budgeting, an attention state, or an attention-text renderer. It makes no claim that selected parameters are learned, optimal, or suitable for safety-critical decisions.

The public contract is experimental and may evolve through the repository's specification and decision process. Deterministic evaluation order and score breakdowns support reproducible diagnosis, but this decision does not claim bit-for-bit equality across distinct floating-point environments.
