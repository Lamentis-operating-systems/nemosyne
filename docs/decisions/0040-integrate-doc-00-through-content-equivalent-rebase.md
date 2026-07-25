# 0040: Integrate DOC-00 through content-equivalent rebase

Status: Superseded
Date: 2026-07-26
Superseded by: 0041-route-doc-00-validation-by-attestation-delta.md

## Context

Decision 0022 required a merge commit so the reviewed source and its direct
attestation child would retain their pull-request commit identities. The
repository's active `main` ruleset instead requires linear history. GitHub's
Rebase and merge method preserves commit order and trees but always creates new
commit identities. The two requirements cannot both hold.

Weakening the receipts to unbound prose or allowing squash would discard the
reviewed two-commit boundary. Disabling linear history would change repository
policy for one bootstrap package.

## Decision

DOC-00 uses GitHub Rebase and merge. Squash remains invalid. The pull-request
head must first pass strict receipt validation with the recorded source commit
as the exact direct parent of the evidence commit.

Main-push validation uses an explicit linear-integration mode. It accepts the
rewritten pair only when all of these conditions hold:

- one single-parent evidence commit directly follows one source commit;
- the evidence commit is the common last modification of all 22 canonical
  attestations and changes exactly those files;
- the integrated source tree equals every record's `Source tree`;
- every canonical receipt entry, including the reviewed `Archive SHA-256`
  binding proved before integration, remains byte-identical;
- the source preserves the preceding canonical attestation entries and the
  evidence commit contains the exact current entries;
- every recursively validated predecessor pair satisfies the same
  tree-, archive-, ordering-, schema-, and replacement-binding rules; and
- every later first-parent commit is linear and leaves all DOC-00-bound paths
  unchanged.

`Source commit` remains the exact pre-integration review anchor. Pull-request
validation resolves it and proves the direct source/evidence relationship.
After GitHub rewrites commit identities, main-push validation uses the
integrated direct parent as the source counterpart and re-proves its exact tree
and receipt bindings. It does not reconstruct the Tar archive from the
rewritten commit because Git archive headers include commit-sensitive metadata.
It never treats the recorded commit identity and the integrated identity as
equal.

The CI workflow selects exact mode for pull requests and linear-integration
mode only for pushes to `main`. The modes are explicit command-line contracts;
the checker does not infer trust from branch names or environment variables.

This decision supersedes Decision 0022. It changes only integration topology.
The non-cryptographic receipt model, stable G0 identity, protected histories,
trusted-comparator rules, source/evidence separation, replacement semantics,
and accountable-human boundary selected by Decisions 0030 and 0033 remain in
force.

## Rationale

Tree identities and receipt blobs are invariant under GitHub's commit rewrite,
while commit identities and a reconstructed Tar archive are not. Requiring
exact commit and archive validation before integration, then exact tree,
receipt, and evidence-delta equivalence afterward, preserves review
accountability without relaxing linear history.

An explicit post-integration mode prevents a pull-request head from silently
substituting content equivalence for the stronger pre-integration ancestry
check. The direct two-commit shape and exact evidence delta distinguish rebase
integration from squash.

## Alternatives

- Remove required linear history. Rejected because repository policy remains
  selected.
- Use squash merge. Rejected because it destroys the source/evidence boundary.
- Accept any commit with the same final tree. Rejected because it does not
  preserve pair order, evidence-only delta, or recursive replacement history.
- Require the original commit objects after integration. Rejected because
  GitHub rebase creates new identities and deleted pull-request refs are not a
  durable object-retention contract.

## Consequences

DOC-00 gains one explicit main-push validation mode and focused rebase,
reordering, squash, drift, and descendant regressions. The source and evidence
commits remain separately reviewable before integration and separately visible
after integration, but their post-integration commit identities differ.

The final pull-request head must pass exact strict validation. G0 closes only
after the rebased main head passes linear-integration validation and every
required CI job.
