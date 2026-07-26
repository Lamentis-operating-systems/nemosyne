# 0045: Adopt a typed G1 evaluation envelope

Status: Accepted
Date: 2026-07-26

## Context

EVD-01 authenticates generic run manifests and guard witnesses but treats the
run payload as opaque bytes. TGT-00 needs a narrower pre-outcome contract that
cannot omit a G1 condition, population partition, exposure rule, mandatory
threshold, negative control, or analysis identity while still reusing the
existing evidence entrance.

The G1 proof anchors own the experimental semantics and mathematics. This
decision selects only their first in-process typed representation.

## Decision

Implement the G1 envelope inside `nemosyne-evaluation::evidence` with:

- one closed seven-condition type and fixed expectation-role mapping;
- exact rational task weights and a complete prospective domain/subgroup
  exposure contract;
- a closed typed threshold-key inventory with coordinate-specific numeric
  domains;
- closed design- and run-artifact inventories represented by nonzero opaque
  evidence identities;
- a G1-specific semantic-root binding and domain-separated envelope identity;
- canonical ordering, domain-separated SHA-256 identities, and Ed25519
  signatures; and
- one finalizer that embeds the verified signed envelope and exact execution
  bindings into the existing `SignedRunManifestV1`.

Continue to require the existing independently authenticated guard-witness
join before outcome access. Keep all outcomes, estimators, candidate adapters,
candidate renderers and checkpoints, release gates, persistence formats, and
product authority out of this module. The frozen downstream comparison-model
identity remains a required G1 design artifact.

Run finalization adds only execution-specific identities. It reuses design
identities from the signed envelope rather than accepting a second,
potentially conflicting runtime, model, decoding, seed, analysis,
multiplicity, custody, or failure-policy value.

## Rationale

Closed typed inventories make omission observable before signing and keep the
G1 envelope cryptographically domain-separated from a future G9 envelope.
Cross-envelope semantic reuse still requires an explicit later lineage check.
Rational weights make the required normalized mass exact.
Reusing EVD-01 preserves one admission boundary and avoids a second custody or
signature system. Opaque identities bind externally reviewed artifacts without
asking this structural layer to infer semantic neutrality, irrelevance, or
independence from bytes.

## Alternatives

- Keep the G1 payload opaque. Rejected because required proof fields could be
  omitted without a typed construction error.
- Duplicate all G1 formulas in Rust. Rejected because the proof anchors remain
  the sole normative owners and EVD-02, not TGT-00, performs outcome arithmetic.
- Store floating-point task weights with a tolerance. Rejected because exact
  rational mass is deterministic and adequate for the bounded authored design.
- Infer carrier neutrality and placebo irrelevance from their text. Rejected
  because those are blinded authorship/adjudication claims, not reliable
  structural properties.
- Introduce a persistent JSON schema now. Rejected because Decision 0044 leaves
  transport and storage compatibility open.
- Let a signed envelope grant outcome access. Rejected because only the complete
  run-manifest and guard-witness join satisfies EVD-01.

## Consequences

TGT-00 can construct and authenticate a complete G1 design and bind one exact
execution identity without implementing experiment execution. The type surface
is intentionally verbose because closed omissions are failures.

The result remains conditional evidence. A valid signature does not prove
semantic correctness, independent adjudication, justified thresholds, adequate
power, or absence of external access. Those require concrete authored
artifacts, review, custody evidence, and EVD-02.
