# Situation-conditioned activation

Status: Experimental

## Purpose

This specification defines the first deterministic activation-ranking contract in `nemosyne-core`. It combines already normalized numeric evidence and inhibition signals into bounded, explainable candidate scores. It does not define how an upstream system obtains those signals.

## Definitions

The unit interval is `U = [0, 1]`. Every weight, gate, strength, signal, intermediate retention factor, and reported score is represented as a finite `f64` in `U`.

`ChannelId` and `CandidateId` are numeric identifiers. Channel identifiers share one namespace across evidence and inhibition channels.

An activation profile defines one globally ordered channel set containing:

- a finite set `C` of evidence channels, each with a weight `w_c` in `U` and a situation-dependent gate `g_c` in `U`; and
- a finite set `J` of inhibition channels, each with a strength `lambda_j` in `U`.

A candidate `i` has one signal for every profile channel and no other signals. The profile, rather than the candidate, determines whether each signal is evidence `e_(i,c)` or inhibition `p_(i,j)`. Profiles and candidates canonicalize their channels into ascending `ChannelId` order.

The public operations are:

```rust
pub fn rank_activations(
    profile: &ActivationProfile,
    candidates: &[ActivationCandidate],
) -> Result<Vec<RankedActivation>, ActivationError>;

pub fn explain_activation(
    profile: &ActivationProfile,
    candidate: &ActivationCandidate,
) -> Result<ActivationExplanation, ActivationError>;
```

## Mathematics

For each evidence channel, its effective weight is:

\[
a_c = w_c g_c
\]

The evidence denominator must be positive:

\[
D = \sum_{c \in C} a_c > 0
\]

The candidate's evidence score is:

\[
E_i = \frac{\sum_{c \in C} a_c e_{i,c}}{D}
\]

The contribution reported for one evidence channel is:

\[
q_{i,c} = \frac{a_c e_{i,c}}{D}
\]

For `f64` evaluation, the implementation first computes the normalized weight

\[
n_c = \frac{a_c}{D}
\]

and then computes `q_(i,c) = n_c e_(i,c)`. This division-first evaluation avoids losing a representable normalized contribution when `a_c` is subnormal. A positive `a_c` whose normalized weight is not representable as a positive `f64` is rejected explicitly.

For each inhibition channel, the retention factor is:

\[
r_{i,j} = 1 - \lambda_j p_{i,j}
\]

Total retention and final activation are:

\[
R_i = \prod_{j \in J} r_{i,j}
\]

\[
A_i = E_i R_i
\]

The empty product is one, so `R_i = 1` when `J` is empty.

Each ranked result contains only its candidate identifier, `E_i`, `R_i`, and `A_i`. Ranking does not construct per-channel contribution collections.

An explanation contains the same aggregate result plus a complete breakdown for one requested candidate. An evidence entry reports its channel identifier, weight, gate, signal, effective weight, normalized weight, and contribution. An inhibition entry reports its channel identifier, strength, signal, and retention factor. Ranking and explanation use the same evaluator, so their aggregate values for the same profile and candidate are bit-identical.

Profile construction computes and stores each effective and normalized evidence weight once. Candidate evaluation therefore requires no repeated weight-gate multiplication or denominator division.

## Preconditions

The API validates the complete input before returning ranked results:

- every supplied `f64` must be finite and in `U`;
- each channel identifier must occur exactly once in the profile, including across channel kinds;
- an evidence channel with positive `w_c` and `g_c` must have a representable nonzero `f64` product; otherwise construction reports effective-weight underflow rather than silently disabling the channel;
- every positive effective evidence weight must remain representable and positive after division by `D`; otherwise construction reports normalized-weight underflow;
- at least one evidence channel must have `w_c * g_c > 0`, so `D > 0`;
- every candidate identifier must be unique in the input;
- every candidate must contain exactly one signal for each profile channel; and
- a candidate must not contain an unknown or duplicate signal.

A violation returns an explicit `ActivationError`. Missing or unknown information is never interpreted as zero.

## Invariants

Evidence channels and inhibition channels are each evaluated in ascending `ChannelId` order using `f64`, even though both kinds share one canonical profile layout. Input ordering therefore does not change the calculated result.

The mathematical construction preserves:

\[
0 \leq E_i, R_i, A_i \leq 1
\]

Computed `E_i`, `R_i`, and `A_i` are clamped to `U` only to contain floating-point rounding outside the interval. The implementation does not otherwise clip, rescale, or normalize results.

For an explanation, the ordered sum of the reported evidence contributions produces `E_i` before the documented interval clamp. The reported inhibition factors multiply to `R_i`, and `E_i * R_i` reconstructs `A_i`, subject to floating-point rounding and that clamp. Both breakdown lists are ordered by ascending `ChannelId`.

Holding every other input fixed, increasing an evidence signal with positive effective weight cannot decrease activation. Increasing an inhibition signal whose strength is positive cannot increase activation.

The output contains every input candidate exactly once. Results are ordered by descending `A_i`, then by ascending `CandidateId` when the computed activations compare exactly equal. No epsilon comparison or implicit score threshold is applied.

## Edge cases

- A valid profile and an empty candidate list return an empty result list.
- A candidate whose evidence signals are all zero has `E_i = A_i = 0`.
- An evidence channel with weight zero or gate zero has effective weight and contribution zero.
- Positive weight and gate values whose product underflows to zero are rejected explicitly.
- A positive effective weight whose normalized weight underflows to zero is rejected explicitly.
- With no inhibition channels, `R_i = 1` and `A_i = E_i`.
- If any inhibition channel has strength one and signal one, its retention factor is zero and `A_i = 0`.
- `NaN`, positive or negative infinity, negative values, and values above one are invalid.
- Negative zero is valid and is stored and reported as positive zero.
- Duplicate profile channels, duplicate candidates, missing signals, duplicate signals, unknown signals, effective- or normalized-weight underflow, and a non-positive evidence denominator are errors.

## Operational boundary

The kernel is intended for a bounded post-retrieval candidate set. No supported
candidate-count range is claimed without a frozen configuration and retained
benchmark receipt on declared hardware. It deliberately returns all candidates
and performs a full deterministic sort; it is not a database-scale retrieval
or top-k interface.

The evidence denominator depends on the complete active evidence-channel set. Adding or removing a channel can therefore change every normalized evidence contribution even when existing channel parameters are unchanged. Multiplicative inhibition also treats each inhibition channel as an independent retention factor; correlated inhibition channels can compound one underlying concern. Callers own channel construction and calibration. This kernel does not claim statistical independence, calibrated probabilities, or safety semantics.

## Computational complexity

Let \(n_{\mathrm{act}}\) be the activation-candidate count,
\(c_{\mathrm{act}}=|C|+|J|\) the total activation-profile-channel count, and
\(s\) the number of signals supplied to one candidate constructor. The
implemented public operations have these worst-case bounds:

| Operation | Time | Additional space, excluding caller-owned input |
| --- | --- | --- |
| `ActivationProfile::new` | \(O(c_{\mathrm{act}}\log c_{\mathrm{act}})\) for canonical sorting plus \(O(c_{\mathrm{act}})\) preparation | \(O(c_{\mathrm{act}})\) owned canonical profile |
| `ActivationCandidate::new` | \(O(s\log s)\) for canonical signal sorting | \(O(s)\) owned canonical candidate |
| `rank_activations` | \(O(n_{\mathrm{act}}c_{\mathrm{act}}+n_{\mathrm{act}}\log n_{\mathrm{act}})\) | \(O(n_{\mathrm{act}})\) candidate-reference and result vectors |
| `explain_activation` | \(O(c_{\mathrm{act}})\) | \(O(c_{\mathrm{act}})\) returned contribution breakdown |

Ranking first canonicalizes candidate references, validates and evaluates each
candidate in canonical channel order, then performs the required full result
sort. The bound does not assume a constant channel count. The complete
breakdown returned by explanation is output, not hidden workspace. Any future
optimization must remain extensionally identical for every accepted input,
including exact tie behavior and error precedence, or establish a new
contract.

## Verification

Public-boundary tests must cover valid interval boundaries, every invalid numeric class, every structural error, an empty candidate list, absent and complete inhibition, inactive evidence channels, deterministic tie-breaking, and input-order invariance.

Tests must include a hand-calculated case with `E = 0.6`, `R = 0.8`, and `A = 0.48`; reconstruct scores from an explanation; verify bounded finite outputs; verify evidence and inhibition monotonicity; verify effective- and normalized-weight underflow handling; verify both public operations' structural validation; and verify bit-identical aggregates from ranking and explanation.

A numeric red-light-versus-appointment example must demonstrate that caller-selected signals and gates can rank an immediate candidate above a secondary candidate. This example is algorithm evidence only and does not establish a safety policy or safety guarantee.

Repository verification is performed by the documentation checks, formatting, Clippy with warnings denied, Rustdoc with warnings denied, and the workspace test suite required by `AGENTS.md`.

## Open questions

None within this kernel. Signal derivation, semantic channel selection, calibration, and downstream selection remain outside this specification.

## References

- [Decision 0005: Adopt a deterministic activation kernel](../decisions/0005-adopt-deterministic-activation-kernel.md)
- [Decision 0006: Canonicalize activation data and separate explanation](../decisions/0006-canonicalize-activation-data-and-separate-explanation.md)
- [Decision 0007: Reject normalized evidence underflow](../decisions/0007-reject-normalized-evidence-underflow.md)
- [`nemosyne-core`](../../crates/nemosyne-core/)
