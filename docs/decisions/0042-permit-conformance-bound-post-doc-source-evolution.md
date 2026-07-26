# 0042: Permit conformance-bound post-DOC source evolution

Status: Accepted
Date: 2026-07-26

## Context

Decision 0033 requires each changed reviewed archive after DOC-00 to append
exactly one source-conformance receipt. The integrated linear checker instead
forbids every later change below `docs/specifications` or `docs/decisions`.
Because production Rust changes require a specification update, those two
rules make every implementation package require a complete replacement of the
22 DOC-00 attestations.

That replacement is not the contract selected by Decision 0033. The canonical
attestations establish DOC-00 and retain a stable G0 identity. A later
conformance receipt identifies a new reviewed source revision without
extending the earlier reviewers' claims to that revision.

The checker also freezes current conformance, specification, and decision
counts as source-code constants. Advancing any of those append-only inventories
therefore changes the governance program itself, which again forces a complete
DOC-00 replacement.

## Decision

Keep exact pull-request validation and the canonical source/evidence pair
unchanged. In explicit linear-integration mode, classify paths after the
canonical evidence commit as follows:

- `docs/receipts`, both delivery-program checker scripts, and the CI workflow
  that supplies the trusted prior-history boundary remain immutable
  DOC-00-bound paths;
- `docs/specifications` and `docs/decisions` may change only in single-parent
  commits whose finding and conformance histories preserve the direct parent
  as an exact ordered prefix; and
- every such commit appends exactly one next contiguous conformance receipt
  when the reviewed source outside the conformance history changes, and
  appends none when it does not.

Validate those conditions for every commit on the ancestry path from the
canonical evidence commit to `HEAD`. A later revert is another reviewed
archive change and therefore requires its own successor; net-zero final trees
cannot hide intermediate source history.

Define the reviewed-source change projection as every decision file, every
specification except the delivery program, and the delivery program with its
complete conformance-history appendix removed. The newly appended receipt
therefore cannot make its own prerequisite true. A receipt-only commit is an
unchanged reviewed source and is rejected.

Every post-DOC conformance section uses the closed
`post-doc-history-conformance-v1` table. Its record ID, parent, history-only
status, scope, current finding range, derived finding/conformance/specification/
decision inventory, and explicit claim boundary are exact. It does not certify
the complete source structure of an intermediate commit. Linear validation
reconstructs and validates the newly active section at each historical
source-changing commit, not only the section active at `HEAD`; the complete
structural contract is evaluated for the current checkout by the ordinary
checker path.

Replace current finding, conformance, specification, decision, and
Accepted-decision count constants with validated extensible inventories:

- the DOC-00 baseline remains at least 384 findings, 30 conformance receipts,
  12 non-template specifications, and 41 contiguous decisions;
- finding, conformance, and decision identifiers remain contiguous from one;
- finding identifiers use an unbounded decimal grammar with a minimum
  three-character zero-padded rendering, so `FND-999` is followed by
  `FND-1000`;
- conformance identifiers use an unbounded positive decimal grammar with a
  minimum two-character zero-padded rendering, so `DOC-CONF-99` is followed by
  `DOC-CONF-100`;
- the first 384 finding severities remain byte-semantically fixed while later
  P0–P3 rows are permitted;
- the ten historical Superseded decision IDs remain exact; and
- every later decision is Accepted unless a future focused decision changes
  the status policy.

The active closed conformance table states only the history and file/status
inventory counts derived from the actual validated files. Test fixtures derive
active conformance and finding identifiers from the delivery program rather
than rewriting the checker.

This decision refines Decision 0033 and supersedes only the "no later
bound-path change" part of Decisions 0040 and 0041. Their exact/rebased
source-evidence topology, canonical receipt identity, receipt-only evidence
delta, CI mode routing, and squash prohibition remain unchanged.

## Rationale

The canonical DOC-00 attestations and a later source-conformance receipt make
different claims. Keeping the former immutable while validating the latter
commit by commit preserves both boundaries. Deriving extensible inventory
sizes removes the need to edit trusted checker code for ordinary append-only
growth without weakening identifier continuity, historical status, or active
inventory reconciliation. Excluding the conformance appendix from the change
projection avoids self-attestation, while exact per-commit receipt validation
prevents a later valid successor from hiding an earlier malformed history
statement. Narrowing that statement to the reconstructed historical facts
avoids falsely claiming full structural validation of intermediate commits.

The checker used in linear mode remains the attested, unchanged governance
program. The CI workflow that supplies its trusted prior-history boundary is
protected by the same post-DOC immutability rule. A later source commit cannot
redefine either the comparator or that trust input without a complete
replacement source/evidence pair.

An exact replacement pair may descend from earlier canonical pairs already
integrated through the required rebase method. Exact mode binds the active pair
to its literal recorded source commit while validating an earlier pair as a
rebased counterpart only when its evidence commit is reachable from the
trusted prior integration head supplied by CI or the local `origin/main`
tracking ref. The counterpart must still be direct, ordered, and tree- and
receipt-equivalent. Linear integration additionally permits its explicitly
active rebased evidence commit. This does not permit a synthetic branch-local
prior identity: the trusted-history boundary, recorded source tree,
evidence-only delta, replacement chain, governance program, and canonical
receipt bytes remain mandatory.

## Alternatives

- Replace all 22 attestations for every implementation package. This is
  mechanically possible but collapses the intended distinction between stable
  G0 evidence and later source conformance.
- Exempt implementation packages from reviewed-source conformance. This would
  permit stale normative specifications and is rejected.
- Validate only the final reviewed tree. This would allow an intermediate
  rewrite and revert to disappear from the evidence chain.
- Permit later governance-program changes under the conformance rule. This
  would let the checked source redefine its own validator and is rejected.
- Infer current inventory counts without retaining baseline minima or
  historical status constraints. This would permit deletion to masquerade as
  evolution.

## Consequences

This governance change itself requires one final DOC-00 source/evidence
replacement because it changes the trusted checker and receipt contract.
After it is integrated, ordinary implementation packages can update normative
specifications and append one conformance receipt without replacing the 22
canonical attestations or changing the checker.

Regression evidence must cover a valid linear source successor with a new
finding, specification, and decision; receipt-only and missing-successor
failures; an invalid first successor followed by a valid second successor;
an invalid intermediate source repaired by a later successor without a false
historical structural claim; rewrite/revert failures; the `FND-999` to
`FND-1000` and `DOC-CONF-99` to `DOC-CONF-100` boundaries; an exact replacement
pair descending from a previously rebased canonical pair; and
continued rejection of a same-tree synthetic prior identity, later receipt
drift, or governance-program changes.
