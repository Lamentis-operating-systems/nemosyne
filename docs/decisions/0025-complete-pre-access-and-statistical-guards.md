# 0025: Complete pre-access and statistical guards

Status: Accepted
Date: 2026-07-24

## Context

The proof program required a custodian guard witness for rejected attempts but
did not bind it explicitly to successful outcome-access admission. G9 gated
overall harm while context-independent non-inferiority could hide concentrated
reversals. The wrong-expectation control was required to differ from both
correct and abstaining conditions, but only correct-versus-wrong
differentiation was gated. Delivery also placed renderer-selection
multiplicity inside the pre-renderer G1 package.

## Decision

Every successful `ValidForOutcomeAccess` record binds one authenticated
`PreAccessGuardWitnessV1` with a closed canonical field set and a
manifest-bound `ValidatedRun` subject for the same attempt, exact signed run
manifest, sealed source, validation window, closed validation and analysis
principal sets, capability-issuance state, and access and analysis ledger
boundaries. The sole checked admission constructor accepts only that subject
and compares those joins field by field. Early rejection uses the disjoint
`RejectedAttempt` subject bound to the rejection receipt's exact attempted
kind, attempt identity, complete-input or consumed-prefix commitment, and
absent-or-established sealed-source state; it never requires or fabricates a
run-manifest identity and represents an unestablished sealed source explicitly
as absent. Fixed field precedence makes multi-mismatch errors deterministic.

Pre-access validation has three disjoint terminal results: a valid signed
design artifact or `ValidForOutcomeAccess`; a `PreAccessRejectionReceipt`
backed by a structurally valid, exactly matching `RejectedAttempt` witness; or
a validator-authenticated `PreAccessCustodyFailureRecord` when required guard
evidence is missing, invalid, wrong-subject, or mismatched. The custody-failure
record retains only the opaque attempt, attempted kind, non-retaining input
commitment, already established allowlisted identities, closed guard-error
class and field, validation stage, validator and
validation-implementation identities, trusted time, and validator signature.
It contains no raw attempted or witness bytes, no custodian signature, no
admission, and no outcome field. It is evidence only that validation stopped;
it makes no no-access or custody claim.

A missing, invalid, wrong-subject, or mismatched witness therefore prevents
outcome access and returns no partial rejection receipt, admission value, or
capability. Both valid witness subjects share the same custody-window,
principal, capability, and ledger semantics; neither is unconditional proof
about systems outside the declared custody boundary.

For every mandatory G9 baseline, the context-independent set \(I_I\) has
separate population-harm and conditional-reversal estimands and prospectively
frozen maxima. Their weights are normalized within \(I_I\). Empty,
underexposed, missing, or zero-baseline-success cases are `Inconclusive` before
the affected division or claim, and cannot borrow the whole-population result.

The expectation negative control has two blinded semantic-discrimination
gates: correct versus wrong and abstaining versus wrong. Both belong to the
same predeclared envelope-local multiplicity family and have prospectively
frozen minimum rates. Harm, anchoring, and leakage for the wrong condition
remain upper-bounded only; no positive harm is required.

Multiplicity ownership is domain-local. G1 covers only its frozen headroom and
expectation-branch comparisons, cohorts, subgroups, and repetitions. Renderer
checkpoint, bridge, latent-query, precision, seed-aggregation, gate, and
fallback selection belongs exclusively to renderer qualification. G9 owns its
separate sealed product family. No package may pull a future artifact family
into an earlier envelope.

## Rationale

Successful admission needs the same custody evidence as rejection to support a
conditional no-prior-access claim. Separate \(I_I\) harm gates prevent
population dilution. Two semantic discriminators implement the accepted
negative-control meaning without treating user harm as useful signal.
Domain-local multiplicity families remain freezeable before their own outcomes
without inventing future renderer configurations during G1.

## Alternatives

- Infer a guard witness from a structurally valid run manifest. Structural
  validity does not attest capability or ledger state.
- Rely on overall G9 harm. Context-dependent gains can dilute concentrated
  context-independent regressions.
- Use wrong-condition harm as the abstention discriminator. This rewards
  unsafe behavior.
- Freeze one universal multiplicity family in G1. Future artifact candidates
  are neither known nor part of the G1 estimand domain.

## Consequences

Evidence schemas, protocol manifests, analysis code, receipts, and delivery
packages must carry the new witness, terminal custody-failure, and statistical
fields. G1, renderer qualification, and G9 retain separate correction
identities and cannot reuse outcomes or alpha allocation. Threshold values
remain unresolved until their own prospectively frozen evaluation packages.
