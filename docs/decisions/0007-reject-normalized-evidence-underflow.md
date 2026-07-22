# 0007: Reject normalized evidence underflow

Status: Accepted
Date: 2026-07-22

## Context

Decision 0006 rejects positive evidence parameters when their product underflows to zero. A positive effective weight can nevertheless become zero when divided by a larger evidence denominator. The affected channel would then be accepted but could never contribute to a candidate score.

For example, effective weights containing the smallest positive `f64` together with two unit weights produce a denominator of two. Dividing the smallest weight by that denominator yields zero.

## Decision

Reject an activation profile when a positive effective evidence weight produces a zero normalized weight. Return a distinct `NormalizedWeightUnderflow` error containing the affected channel identifier.

Keep explicit zero weights and gates valid. They intentionally disable a channel and remain distinct from an unrepresentable positive value.

## Rationale

Silently converting configured positive evidence into an inactive channel would make the profile disagree with its numeric inputs and with the explicit underflow policy. A dedicated error preserves deterministic behavior and lets callers rescale or remove the affected channel deliberately.

## Alternatives

- **Accept the zero normalized weight.** Rejected because it silently disables positive evidence.
- **Use higher-precision arithmetic.** Rejected because the kernel currently defines `f64` inputs, outputs, and evaluation, and no additional numeric dependency is justified.
- **Clamp the normalized weight to the smallest positive value.** Rejected because that would invent a value that does not follow from the documented formula.
- **Drop the affected channel automatically.** Rejected because missing or unusable information is never interpreted implicitly.

## Consequences

An additional valid-unit-interval profile can now fail construction when its configured scales exceed representable normalized `f64` precision. Callers must rescale the profile while preserving its intended ratios or choose a different channel set.

The scoring formula, canonical evaluation order, and behavior of explicitly inactive channels remain unchanged.
