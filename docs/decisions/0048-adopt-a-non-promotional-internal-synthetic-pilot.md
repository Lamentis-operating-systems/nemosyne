# 0048: Adopt a non-promotional internal synthetic pilot

Status: Accepted
Date: 2026-07-26

## Context

The project wants a cheap next evaluation step for whether generated focus
context helps an agent preserve relevant prior constraints relative to no
context and same-size placebo context. The inputs will be AI-generated.

Self-generated tasks, labels, and condition material do not satisfy the
independent authorship, adjudication, threshold, custody, or controlled
execution requirements of formal G1. Reusing the formal evidence dispositions
would create a material risk that a structural or synthetic result is later
misrepresented as product evidence.

No exact generation model/version, cost ceiling, or privacy disposition has
yet been approved.

## Decision

Add a separate `nemosyne_evaluation::synthetic_pilot` module and experimental
specification. Use only the fixed classification
`InternalSyntheticPilot/NonPromotional` and the terminal dispositions
`Completed`, `Invalid`, and `Aborted`.

Require a complete model/version/cost/privacy disclosure before generated
material can enter a frozen pilot. Bind exact generation-prompt and runtime
configuration, retain every generation attempt, require one-to-one generated
task provenance, and mirror the seven structural variants only under
`pilot_*` labels.

Freeze and content-identify the generation log, corpus, exact condition
artifacts, deterministic scoring procedure, controlled-runner manifest, and
their complete join before accepting observations. Make the frozen package
non-cloneable and consume it during receipt construction. Encode
outcome-driven regeneration as forbidden.

Report only per-condition descriptive counts and constraint-following rates.
Provide no model client, generated production corpus, statistical threshold,
formal evidence conversion, product authorization, or release integration.

## Rationale

A separate type and identity domain makes accidental promotion harder than a
warning field on the formal evidence path. Exact roots and consuming
finalization make mutation and cross-bound composition observable inside the
offline crate. Complete attempt retention exposes selective regeneration or
discarding rather than hiding it.

Mandatory model, cost, and privacy disclosure keeps the still-unselected
external dependency visible. Omitting a model client prevents repository code
from silently choosing or invoking a proprietary service.

## Alternatives

- Use the formal G1 envelope with synthetic identities. Rejected because a
  structurally complete object could be mistaken for independent evidence.
- Generate an unversioned local JSON corpus. Rejected because generation
  attempts, provenance, frozen procedures, and post-outcome mutation would not
  be bound.
- Select and call an external model in this change. Rejected because no exact
  model/version, cost, privacy, retention, or data-destination choice has been
  approved.
- Report a binary product result from the pilot. Rejected because a generated,
  internally scored corpus supports only descriptive pipeline learning.

## Consequences

The repository can validate a complete proposed pilot package and summarize a
controlled run without entering the formal evidence system. An actual run is
still blocked on the explicit model disclosure, generated corpus, scorer, and
runner choices.

The content roots are not signatures, trusted timestamps, or durable external
custody evidence. The API cannot prove that an operator had no outcome access
before constructing a root. Those limitations must remain explicit in every
pilot report.

Because this change advances reviewed documentation source, the same package
appends only the required history-and-inventory successor `DOC-CONF-37`; it
does not alter any earlier conformance receipt or expand that receipt into
implementation or empirical evidence.

Formal G1 remains unchanged and still requires a fresh lineage-disjoint,
independently authored and adjudicated evaluation with prospectively justified
statistics, independent custody, and controlled execution. Pilot tasks,
labels, observations, thresholds, and roots are ineligible for reuse or
promotion.
