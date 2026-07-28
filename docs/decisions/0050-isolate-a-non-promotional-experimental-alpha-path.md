# 0050: Isolate a non-promotional Experimental Alpha path

Status: Accepted
Date: 2026-07-28

## Context

The formal V1 delivery program intentionally blocks `CORE-01` on a passing G1
receipt. Internal synthetic work nevertheless found a narrow mechanism worth
exploring: structured applicability controls, terminal abstention and conflict,
typed focus readiness, and a deterministic non-LLM baseline.

Starting the canonical package early would bypass the G1 entrance condition.
Waiting for G1 before any implementation exploration would prevent a small
disposable prototype from testing whether the proposed type boundary is
workable. Synthetic feasibility results do not resolve that tension by
becoming formal evidence.

## Decision

Nemosyne may add one non-published `nemosyne-experimental-alpha` crate under
the [Experimental Alpha path](../specifications/experimental-alpha-path.md).
The crate is a disposable research consumer of already available canonical
primitives. No canonical V1, G1/G3, release, or published crate may depend on
it, re-export it, or consume its artifacts.

Alpha owns only alpha-namespaced applicability controls, closed applicability
and readiness outcomes, typed prose-free focus structure, deterministic
baseline realization, and local structural fixtures. It exposes no canonical
`*V1` API and creates no formal envelope, receipt, disposition, eligibility,
promotion, or release artifact.

The formal delivery registry remains exactly 54 packages. Its dependencies,
gates, `CORE-01` eligibility, G1/G3 requirements, and release rules are
unchanged. Alpha is not a delivery package and cannot satisfy, unblock, or
provide entrance evidence for any one of them.

## Rationale

One-way dependency and separate types make accidental promotion mechanically
detectable. A non-published crate permits implementation learning without
turning its synthetic fixtures into product evidence or creating a second V1
path.

## Alternatives

- **Start `CORE-01` before G1.** Rejected because it changes the accepted
  entrance condition and would blur exploration with the canonical V1 path.
- **Put Alpha code in `nemosyne-core`.** Rejected because canonical consumers
  could then acquire experimental behavior without an explicit dependency.
- **Treat the internal synthetic study as G1.** Rejected because it lacks the
  independent inputs, custody, controlled execution, and formal dispositions
  required by G1.
- **Define persistence, a product API, or a model path now.** Rejected because
  none is needed to test the narrow applicability/readiness boundary.

## Consequences

A later implementation pull request must satisfy the machine-checkable
acceptance criteria in the Alpha specification. Failure may revise or delete
Alpha without changing V1. Adoption into V1 remains a separate decision and
must follow the unchanged package order and formal gates; Alpha code, types,
fixtures, and results are not promoted in place.
