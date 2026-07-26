# G1 evaluation envelope

Status: Experimental

## Purpose

This specification defines the versioned, pre-outcome G1 design envelope
implemented in the non-published `nemosyne-evaluation` crate. It makes the
seven-condition headroom experiment structurally complete, canonical,
content-identified, signed, and bindable to the existing EVD-01 admission
boundary.

The normative condition semantics, estimands, threshold domains, and pass rules
remain owned exclusively by `PROOF-G1-CONDITIONS-001`,
`PROOF-G1-HEADROOM-001`, and `PROOF-EXPECTATION-BRANCH-001` in the
[V1 proof program](v1-proof-program.md). This specification instantiates their
input contract and does not redefine their mathematics.

## Definitions

`G1Condition` is the closed injective map:

| Variant | Stable label |
| --- | --- |
| `Prompt` | `g1_prompt` |
| `Situation` | `g1_situation` |
| `Placebo` | `g1_placebo` |
| `Focus` | `g1_focus` |
| `Correct` | `g1_correct` |
| `Wrong` | `g1_wrong` |
| `Abstain` | `g1_abstain` |

`G1ExpectationRole::condition` fixes the four expectation roles to `Focus`,
`Correct`, `Wrong`, and `Abstain`. Callers cannot supply another role map.

`G1DesignV1` contains:

- exactly one nonzero opaque evidence identity for each closed condition;
- one shared positive effective token budget, exact attention-token count, and
  repetition count;
- one finite prospective `G1PopulationV1`;
- every proof-owned G1 threshold coordinate;
- one independent minimum-exposure and maximum one-sided rate bound for each
  closed critical failure class; and
- exactly one nonzero opaque identity for every required construction, matching,
  adjudication, statistical, safety, analysis, lineage, and custody artifact.

The common matching object is structural. Exact sameness of tokenizer,
language, placement, focus, delimiter, carrier, placebo size, and authored
blocks is represented by their required opaque identities and audit
identities. The crate does not infer those semantic facts from text.

Each `G1TaskV1` has:

- a unique `G1TaskId`;
- one independent-cluster identity;
- exactly one `ContextDependent` or `ContextIndependent` membership;
- an expectation-eligible flag permitted only in the context-dependent domain;
- one claim-bearing language, task-family, and risk intersection; and
- one positive rational design-weight numerator.

`G1PopulationV1` supplies one positive common denominator. Construction
requires the exact integer sum of all numerators to equal that denominator.
This represents normalized design weights without floating-point mass
tolerance or post-access normalization.

Exposure requirements cover:

- the complete context-dependent domain;
- the complete context-independent domain;
- the complete expectation-eligible subset; and
- every subgroup intersection represented within each applicable slice.

Every requirement has a positive minimum task count and independent-cluster
count already met by the authored population.

`G1ThresholdKey::required_keys` exposes the closed threshold coordinate set.
`G1ThresholdV1` accepts finite values strictly inside `(0, 1)`, except the
proof-owned paired maximum-difference coordinates, which accept `[0, 1)`.
Wrong-condition harm uses the closed `G1WrongControl::{Correct, Abstain}`
domain, so no unsupported role coordinate can be constructed. Signed zero is
canonicalized to positive zero.
Threshold construction validates domains only. It does not justify a selected
value or make that value non-vacuous.

`G1ArtifactKind` is the closed design-artifact inventory. It covers the
G1-specific semantic root, case and membership authorship, condition
construction, neutral carrier, placebo, tokenizer and matching, downstream
execution, sampling and clustering, endpoint and inference, multiplicity,
subgroups and power, repetitions and seeds, missingness and corruption,
expectation eligibility and semantic gates, both blinded discriminators, all
four leakage rubrics, analysis, and custody.

`SignedG1EvaluationEnvelopeV1` derives a domain-separated SHA-256 content
identity and digest from explicit canonical bytes and signs the same bytes with
Ed25519. These algorithms and their limitations retain the EVD-01 contract.

`G1ExecutionBindingV1` adds one fresh nonzero execution-instance identity and
only the execution-specific condition-order, condition-artifact audit,
hardware, operating-system, environment, and token-matching-audit identities.
Runtime, model, decoding, seed, analysis, confidence, multiplicity, custody,
and failure-policy identities remain single-sourced from the signed design.
The operation:

```rust
pub fn finalize_g1_run_manifest(
    envelope: &SignedG1EvaluationEnvelopeV1,
    execution: &G1ExecutionBindingV1,
    claims: RunManifestClaimsV1,
    signing_key_bytes: &[u8; 32],
) -> Result<SignedRunManifestV1, G1EnvelopeError>;
```

verifies the envelope, requires `G1RunManifest` claims, and places the exact
signed envelope plus execution bindings in the reconstructible EVD-01 run
payload. It does not grant outcome access. A matching independently
authenticated guard witness must still pass the sole
`ValidForOutcomeAccess` transition.

## Preconditions

Construction rejects:

- an all-zero opaque identity;
- missing or duplicate conditions, design artifacts, run artifacts,
  thresholds, exposures, tasks, or critical failure classes;
- zero, inverted, or inconsistent token and repetition bounds;
- an empty population, empty context domain, or empty expectation subset;
- duplicate task identities;
- zero task weight, integer weight-mass mismatch, partition gaps, or
  expectation eligibility outside the dependent domain;
- absent, zero, extraneous, or unmet task/cluster exposure requirements;
- non-finite or out-of-domain thresholds and critical rate bounds; and
- a non-G1 run-manifest kind during finalization.

Inputs are bounded to 4,096 tasks. Envelope signing reserves 4,096 bytes of the
EVD-01 run-payload ceiling for the domain framing, signature material, and
execution binding; oversize canonical designs fail before receiving an
envelope identity.

## Invariants

- Conditions, tasks, exposures, thresholds, critical classes, design
  artifacts, and run artifacts are stored in canonical order.
- Permuting input collections does not change envelope bytes, content
  identity, digest, or signature.
- Changing any frozen design value changes the content identity, subject to the
  cryptographic collision assumption.
- The signed envelope has a G1-specific cryptographic domain, and no G9
  condition or outcome type exists in the G1 design API. Future cross-envelope
  semantic-lineage disjointness remains an explicit validation obligation.
- The population partition and normalized rational weight mass are complete
  before signature.
- The four expectation-role mappings are fixed and injective.
- Wrong-expectation harm, anchoring, and leakage have maxima only; the API has
  no positive wrong-harm requirement.
- No outcome, candidate adapter, renderer, model checkpoint, release threshold,
  or product authorization is an envelope input.
- A signed design envelope is not `ValidForOutcomeAccess`; a signed run
  manifest is not `ValidForOutcomeAccess`; only the existing exact
  manifest/witness join creates that value.
- Construction is conditional on the supplied authored identities and
  adjudications. A valid signature proves integrity relative to supplied keys;
  it does not prove that a placebo is irrelevant, a carrier is neutral, a
  threshold is justified, a custodian is independent, or outcomes were
  inaccessible outside the controlled system.

## Edge cases

- A one-task domain is structurally valid only when its positive task and
  cluster minima do not exceed one.
- Multiple tasks may share one independent cluster; exposure counts distinct
  cluster identities rather than tasks.
- Zero is valid only for the paired maximum-difference threshold coordinates.
- Zero observed critical failures is an outcome and is not represented in this
  pre-outcome envelope.
- An invalid design receives no valid G1 envelope identity. EVD-02 must route
  the attempted input through EVD-01's guarded rejection or custody-unavailable
  result rather than relabeling it `Inconclusive`.
- Missing or underexposed realized cells after valid admission are owned by
  EVD-02 and are not repaired by this design layer.

## Verification

`crates/nemosyne-evaluation/tests/g1_evaluation_envelope.rs` verifies:

- the complete seven-condition, 52-threshold, 49-design-artifact contract;
- canonical identity and signature under input permutation;
- identity change after a frozen-value mutation;
- missing closed fields;
- partition, exact weight-mass, and exposure failures;
- strict threshold domains;
- complete run-artifact binding; and
- the full signed-envelope to signed-run-manifest to matching-guard-witness
  admission path.

This is structural and cryptographic implementation evidence only. Empirical
headroom, threshold justification, independent authorship/adjudication,
statistical power, G1 pass/fail, and product value remain unvalidated.

## Open questions

Concrete G1 tasks, authored condition bytes, threshold values, analysis
procedures, independent reviewers, and custody identities are frozen by the
actual TGT-00 evidence artifact before EVD-02. This implementation deliberately
does not select them. G1 does not require a G9 envelope to exist. A later G9
protocol must compare its prospective semantic root against retained G1
lineage evidence and reject reuse; a G1 constructor cannot prove future
disjointness in isolation.

## References

- [V1 proof program](v1-proof-program.md)
- [V1 delivery program](v1-delivery-program.md)
- [Decision 0017](../decisions/0017-control-evaluation-interventions-and-pre-access-evidence.md)
- [Decision 0025](../decisions/0025-complete-pre-access-and-statistical-guards.md)
- [Decision 0044](../decisions/0044-adopt-authenticated-evd-01-evidence-envelope.md)
- [Decision 0045](../decisions/0045-adopt-a-typed-g1-evaluation-envelope.md)
