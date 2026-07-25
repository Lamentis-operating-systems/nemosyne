# 0039: Validate documentation history per commit

Status: Accepted
Date: 2026-07-25

## Context

The change-aware documentation check compared the pull-request merge base
directly with its final head. That correctly rejected a decision introduced in
`Superseded` state, but it also rejected an honest two-commit sequence in which
an accepted decision was introduced and only later superseded with its
replacement. The same collapsed comparison required one conformance successor
for the whole range even when each reviewed-source commit had appended exactly
one sequential successor.

DOC-00 contains both kinds of history. Decisions 0019 and 0023 have real
accepted-to-superseded transitions, while Decision 0028 must be corrected so
that its accepted state also precedes its exact supersession in Git history.
Accepting the final tree alone would erase that distinction and admit
fabricated history.

## Decision

Change-aware validation applies the existing decision, specification, and
reviewed-source conformance transition rules to each consecutive commit on the
pull request's first-parent path from its merge base to its head.

Each transition remains subject to the existing rules:

- a decision or specification cannot first appear as `Superseded`;
- an accepted decision may become `Superseded` only through the exact
  metadata-only transition and an accepted replacement added in that same
  commit;
- historical decisions remain immutable after supersession;
- each changed reviewed source appends exactly one next conformance receipt;
  once Decision 0030 has selected that governance, with the selecting commit
  validated by its own checker; and
- protected-history digests remain checked across the complete pull-request
  range.

The first-parent sequence is the canonical state history because every merge's
resulting tree appears as one transition from its first parent. A merge cannot
hide a direct addition or rewrite: its resulting change is validated at that
transition. A head not connected to the merge base through first parents is
rejected.

This decision authorizes only the history-aware orchestration of existing
validators and its regression fixtures. It does not relax any transition,
replacement, protected-history, conformance, pull-request declaration, or
receipt requirement. A commit before Decision 0030 exists cannot be governed
retroactively by its append-only receipt rule. Once the delivery program names
that governance, a missing Decision 0030 record does not disable validation;
the selected program checker continues to fail closed.

## Rationale

Commit-by-commit validation distinguishes an actual transition from a final
tree that merely claims one. Reusing the existing validators preserves their
single-transition semantics and avoids a second implementation of decision or
conformance rules.

## Alternatives

- **Continue validating only merge base to final head.** Rejected because it
  discards valid intermediate states and makes honest multi-commit history
  indistinguishable from fabricated metadata.
- **Permit newly added superseded records in DOC-00.** Rejected because it
  would weaken the historical record solely to accept the current diff.
- **Add record-specific exceptions.** Rejected because exceptions would not
  express a reusable conformance rule and could conceal future fabricated
  history.
- **Validate only the final tree and trust commit messages.** Rejected because
  commit messages are not normative transition evidence.

## Consequences

The documentation regression suite must retain both a positive two-commit
accepted-to-superseded fixture and the negative direct-superseded-addition
fixture. DOC-00 must preserve an actual accepted state for Decision 0028 before
its exact supersession; the checker contains no exception for that record.

This governance correction resolves `FND-382`. The same source revision
resolves the attribution-boundary defect `FND-381`, appends `DOC-CONF-28`, and
advances only the governed finding, conformance, decision, and test
inventories.
