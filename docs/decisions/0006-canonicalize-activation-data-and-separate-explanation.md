# 0006: Canonicalize activation data and separate explanation

Status: Accepted
Date: 2026-07-22

## Context

The first activation kernel represented evidence and inhibition as separate collections in profiles, candidates, and results. Candidates therefore repeated channel-kind information already owned by the profile. Ranking also allocated a complete contribution breakdown for every candidate, although callers ordinarily need detailed evidence only for selected results. Profile-invariant effective and normalized weights were recomputed for every candidate.

The API remains experimental and has not been merged into `main`, so correcting these boundaries now is preferable to preserving avoidable compatibility.

## Decision

Represent each profile as one globally canonicalized sequence of typed `ActivationChannel` values and each candidate as one globally canonicalized sequence of untyped channel signals. The profile is the sole authority for channel kind. Reject duplicate, missing, and unknown identifiers explicitly.

Prepare effective and normalized evidence weights during profile construction. Reject a positive weight and positive gate whose `f64` product underflows to zero, rather than treating the channel as explicitly inactive.

Make ranking and explanation separate operations. `rank_activations` returns compact aggregate values for every candidate. `explain_activation` evaluates one candidate and returns the same aggregate values with complete per-channel contributions. Both operations use one statically dispatched evaluator; the compact path records no contribution collections.

Store immutable canonical collections as boxed slices. Keep evidence and inhibition configuration and contribution types distinct. Organize the public facade, model, errors, and evaluation in focused activation submodules without exposing their physical layout as separate public modules.

## Rationale

One signal sequence removes impossible category mismatches and duplicate schema truth. A prepared profile avoids repeated invariant arithmetic. Separating compact ranking from targeted explanation changes contribution storage from proportional to every candidate-channel pair to proportional only to the explained candidate's channels.

Distinct public types retain domain clarity, while the private trace abstraction shares arithmetic without runtime dispatch. Boxed slices express that canonical inputs and explanations do not change length after construction.

## Alternatives

- **Keep separate evidence and inhibition signal collections.** Rejected because the candidate would continue to encode channel kind redundantly and permit category mismatches.
- **Keep complete breakdowns in every ranked result.** Rejected because it imposes allocations and memory proportional to all candidate-channel pairs even when no explanation is consumed.
- **Add a flag controlling breakdown generation.** Rejected because one return type would then have conditional completeness and a less precise contract.
- **Use dynamic trace dispatch.** Rejected because the two trace modes are known internally and static dispatch keeps the compact path direct.
- **Use a generic channel or contribution type.** Rejected because it would reduce source repetition at the cost of obscuring the different mathematical roles.
- **Introduce hash maps for candidate evaluation.** Rejected because canonical ordered slices provide deterministic linear validation and evaluation for the intended post-retrieval scale.
- **Add parallelism, SIMD, compensated summation, streaming, or top-k selection now.** Rejected until workload benchmarks demonstrate a need and establish a performance baseline.

## Consequences

This is an intentional breaking change to the unmerged experimental API. Callers construct one `Vec<ActivationChannel>` and one signal collection per candidate. Callers request explanations explicitly after ranking when needed.

The kernel remains an all-candidate deterministic sorter intended for bounded post-retrieval sets, currently assumed to contain roughly 10 to 500 candidates. The scoring formula, canonical floating-point order, tie-breaking, dependency boundary, and non-scope selected by decision 0005 remain unchanged.

Adding evidence channels changes the shared normalization denominator. Correlated inhibition channels compound multiplicatively. This decision documents those model sensitivities but does not change the formula or claim calibrated, independent, learned, or safety-valid inputs.
