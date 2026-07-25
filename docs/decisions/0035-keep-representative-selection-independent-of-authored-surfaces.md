# 0035: Keep representative selection independent of authored surfaces

Status: Accepted
Date: 2026-07-25

## Context

The predictive contract requires every positive-support outcome group to bind
one stored `TransitionRecordVersionId`. Planning retains that identity, and its
record owns the only exact sidecars that may accompany the expectation.

The weighted-medoid section nevertheless allowed a canonical authored
proposition surface to bypass representative selection. A surface string does
not identify a stored transition or its sidecars. For a group containing
multiple eligible transitions, that shortcut left the representative,
canonical order, and exact-value ownership undefined even though downstream
contracts required all three.

## Decision

Select exactly one stored representative for every nonempty positive-support
outcome group before choosing any lexical surface.

When the configured outcome dissimilarity is available and valid, use the
weighted medoid and its existing smallest-`TransitionRecordVersionId`
tie-break. When that distance is legitimately unavailable, use the existing
total lexicographic fallback. Malformed configured distance remains an error
and cannot silently select the fallback.

An authored proposition surface is a lexical realization only. Its presence,
absence, or bytes cannot change:

- the selected `TransitionRecordVersionId`;
- the representative's exact-sidecar ownership;
- the medoid or fallback inputs;
- the tie-break or canonical hypothesis order; or
- support, uncertainty, authority, or provenance.

The selected stored representative exclusively supplies the representative
identity and its own exact sidecars. A renderer may prefer a registered
authored surface only after that binding exists. It cannot use the surface to
invent, replace, combine, or suppress sidecars.

Executable evidence must include otherwise identical outcome groups with and
without an authored surface and show identical representative identity,
sidecar ownership, tie-break, and order under both the distance and fallback
paths. Performance evidence includes the bounded representative-selection
work even when an authored surface exists. An optimized implementation may
avoid materializing pairwise work only if it proves the same representative
as the reference algorithm throughout the supported domain.

## Rationale

Identity, provenance, and exact-value custody must remain grounded in one
stored transition. Lexical convenience cannot satisfy that role. Keeping
surface choice downstream also prevents wording availability from changing
semantic order or numerical results.

The rule makes representative selection total without introducing a synthetic
record, new sidecar merge, or another identifier domain. It preserves the
existing medoid and fallback mathematics and only removes an invalid
short-circuit.

## Alternatives

- **Let an authored surface replace the representative.** Rejected because a
  surface has no stored transition identity or exclusive sidecar owner.
- **Bind the authored surface to a separately chosen transition.** Rejected
  because that merely hides representative selection in another registry and
  can diverge from the numerical reference.
- **Synthesize a representative from the group.** Rejected because it would
  create unsupported identity and exact values.
- **Drop representative identity from planning.** Rejected because
  provenance, canonical ordering, reconstruction, and exact-sidecar custody
  require it.
- **Skip the medoid whenever an authored surface exists and use the fallback.**
  Rejected because wording availability would still change the selected
  transition and invalidate the configured distance semantics.

## Consequences

The predictive specification, planning wireframe, proof registry, and tests
must agree that representative selection always precedes surface choice.
Authored surfaces no longer reduce the reference algorithm's worst-case
representative-selection work. Any later optimization must preserve the exact
selected identity and sidecars or be governed as an explicit approximation.

Because this accepted decision changes reviewed source and the governed
finding and decision inventories, the same revision appends `DOC-CONF-25` and
advances only the corresponding checker constants and regression fixtures.
That mechanical governance update implements Decision 0033; it does not
rebaseline protected history, rename the stable `DOC-CONF-24` G0 record, or
weaken receipt validation.

This decision refines Decision 0014 and does not change the product output,
support mathematics, architecture-neutral focus-adapter selection, or the
separation of relevance, truth, authorization, and action.
