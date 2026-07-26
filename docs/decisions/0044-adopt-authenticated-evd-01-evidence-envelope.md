# 0044: Adopt the authenticated EVD-01 evidence envelope

Status: Accepted
Date: 2026-07-26

## Context

`EVD-01` requires an offline boundary that records whether pre-access guards
admitted an experiment result. The result must remain distinguishable from a
guarded rejection or unavailable custody, and later verification must not rely
on mutable process state.

Decisions 0017 and 0025 define the semantic guard contract. This decision
selects the first in-process implementation and its dependency boundary. It
does not select a product algorithm, persistent wire format, or release claim.

## Decision

Implement `EVD-01` in the non-published `nemosyne-evaluation` crate as the
public `evidence` module.

Use:

- closed versioned Rust types with private fields and validating constructors;
- explicit canonical bytes and domain-separated SHA-256 identities;
- Ed25519 signatures for run manifests, guard witnesses, and experiment
  receipts;
- an independently supplied `GuardAuthorityV1` for witness verification;
- disjoint valid, guarded-rejection, and custody-unavailable results;
- consuming, non-cloneable successful admission before receipt construction;
  and
- digest-and-length commitments that do not retain rejected raw input.

Canonical bytes are internal evidence identities. They are not a selected
storage or transport format. `sha2` and `ed25519-dalek` are dependencies only
of the offline evaluation crate; `nemosyne-core` remains unchanged.

## Rationale

Canonical commitments make evidence reproducible and permutation-invariant.
Independent witness authentication prevents an untrusted caller from
constructing successful guard evidence directly. Consuming admission keeps the
validated manifest and witness joined through receipt construction. Keeping
the implementation outside `nemosyne-core` preserves the runtime boundary.

## Alternatives

- Store unsigned structs. Rejected because later verification could not
  distinguish authentic evidence from caller-authored values.
- Retain rejected input. Rejected because evidence needs only bounded
  commitments and rejection facts.
- Select JSON or another persistent encoding now. Rejected because EVD-01
  needs stable internal identity, not a compatibility commitment.
- Implement the boundary in `nemosyne-core`. Rejected because EVD-01 is
  offline evaluation infrastructure.

## Consequences

The module can prove integrity only relative to supplied keys, identities,
time, and custody facts. It does not prove external key protection, absence of
pre-access computation, experimental validity, product utility, or release
readiness. Those claims remain assigned to later evidence and release gates.
