# G1 execution admission harness

Status: Experimental

## Purpose

This specification defines the narrow in-process boundary between EVD-01
outcome admission and a later EVD-02 G1 analyzer. It prevents an admitted run
from being evaluated against a different signed G1 design or execution
binding.

It does not execute G1, interpret outcomes, select thresholds, or create a
pass, fail, or `Inconclusive` experiment receipt.

## Definitions

The public constructor is:

```rust
AdmittedG1RunV1::bind(
    admission: ValidForOutcomeAccess,
    envelope: &SignedG1EvaluationEnvelopeV1,
    execution: &G1ExecutionBindingV1,
) -> Result<AdmittedG1RunV1, G1EnvelopeError>
```

The constructor verifies the signed envelope, reconstructs the canonical G1
run payload from that envelope and execution binding, and requires byte
equality with the payload inside the admitted EVD-01 manifest.

`AdmittedG1RunV1` retains:

- the complete `ValidForOutcomeAccess`;
- the complete verified signed G1 envelope; and
- the complete execution binding.

It contains no outcome or evaluation disposition.

## Preconditions

A caller must already possess:

- one valid signed G1 envelope;
- one complete G1 execution binding;
- one signed G1 run manifest built from those exact values; and
- one matching independently authenticated guard witness joined as
  `ValidForOutcomeAccess`.

Empirical EVD-02 execution additionally requires externally supplied evidence
that is not present in this repository:

- concrete independently authored tasks, memberships, subgroups, clusters,
  weights, prompts, situation descriptions, and metadata;
- exact seven-condition artifacts for every task and repetition, including the
  neutral carrier, placebo, tokenizer, token-count, and matching audits;
- prospectively justified threshold values and executable confidence,
  hypothesis-test, multiplicity, subgroup, power, missingness, and failure
  procedures;
- frozen endpoint, anchoring, leakage, and both blinded-discriminator rubrics;
- named independent authors, adjudicators, reviewers, analysis principals,
  custodians, keys, ledgers, and custody records;
- sealed downstream-model, runtime, decoding, seed, hardware, operating-system,
  and environment artifacts; and
- a controlled runner that isolates task-condition cells, applies the frozen
  order and seed schedule, captures all failures, and writes the sealed outcome
  source.

Opaque identities or synthetic test fixtures do not substitute for these
artifacts.

## Invariants

- EVD-01 remains the sole outcome-access boundary.
- The bound signed envelope and execution instance are exact, not caller
  descriptions or post-access selections.
- A payload mismatch returns `G1EnvelopeError::RunBindingMismatch`; no partial
  value is returned.
- Binding makes no claim that authored artifacts are semantically correct,
  independent, adequately powered, or in controlled custody.
- No pass, fail, or `Inconclusive` API exists until a later implementation can
  reconstruct and validate every proof-owned G1 result.
- Test fixtures demonstrate structural behavior only and cannot be cited as
  empirical G1 evidence.
- G1 inputs, outcomes, thresholds, and evidence remain unavailable for tuning
  and remain disjoint from G9.

## Edge cases

- A different execution identity fails even when every other run artifact is
  equal.
- A differently signed envelope fails even when its design has the same
  content identity.
- An invalid envelope fails signature or content verification before payload
  comparison.
- Missing external empirical inputs block execution before any G1 claim. Their
  absence in the current repository is not a post-admission `Inconclusive`
  receipt.

## Verification

`crates/nemosyne-evaluation/tests/g1_evaluation_envelope.rs` verifies:

- successful binding of the exact admitted envelope and execution;
- rejection of a different execution binding; and
- rejection of a differently signed envelope.

The tests use synthetic values solely to exercise the public contract. No
controlled empirical G1 run, G1 disposition, threshold justification, or
product-headroom evidence is produced by this package.

## Open questions

The exact external artifact transport, sealed outcome format, proof-owned
analysis implementation, and reconstructible G1 receipt schema remain
unselected. They must be fixed prospectively with the independent inputs above
before EVD-02 accesses outcomes.

## References

- [G1 evaluation envelope](g1-evaluation-envelope.md)
- [V1 proof program](v1-proof-program.md)
- [V1 delivery program](v1-delivery-program.md)
- [Decision 0017](../decisions/0017-control-evaluation-interventions-and-pre-access-evidence.md)
- [Decision 0025](../decisions/0025-complete-pre-access-and-statistical-guards.md)
- [Decision 0044](../decisions/0044-adopt-authenticated-evd-01-evidence-envelope.md)
- [Decision 0045](../decisions/0045-adopt-a-typed-g1-evaluation-envelope.md)
- [Decision 0046](../decisions/0046-bind-evd-02-to-the-exact-admitted-g1-run.md)
