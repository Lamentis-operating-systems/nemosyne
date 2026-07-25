# 0041: Route DOC-00 validation by attestation delta

Status: Accepted
Date: 2026-07-26

## Context

Decision 0040 selected exact validation for pull requests and linear validation
for pushes to `main`. After the first rebase integration, however, every later
pull request inherits rewritten source and evidence commits. Exact validation
would reject that valid inherited history before considering the pull request's
own changes.

The pull-request checkout is also normally a synthetic merge commit. Such a
commit is unsuitable evidence for a branch that must integrate linearly.

## Decision

The documentation job checks out the pull request's exact head commit with full
history. It selects the receipt mode from the trusted base-to-head delta over
the 22 canonical attestation paths:

- a pull request that changes any canonical attestation uses exact mode;
- a pull request that leaves all canonical attestations unchanged uses
  linear-integration mode; and
- a push to `main` uses linear-integration mode.

The checker modes remain explicit. The workflow supplies full base and head
object IDs; branch names, pull-request numbers, actors, and untrusted
environment choices do not select the mode.

Exact mode continues to require the recorded source commit and receipt-only
evidence child. Linear mode continues to require the directly ordered rebased
counterparts and rejects every later commit that changes a DOC-00-bound path,
including a change later reverted to the same final tree. Therefore a
receipt-changing pull request cannot obtain the weaker inherited-history path,
and a net-zero bound-path rewrite cannot evade the history check.

This decision supersedes Decision 0040 only for CI mode routing and pull-request
checkout. It preserves Decision 0040's rebase topology, exact and linear checker
contracts, archive-metadata boundary, and squash rejection.

## Rationale

The attestation delta distinguishes a new DOC-00 source/evidence pair from a
normal descendant without special-casing this pull request. Checking the actual
head avoids treating GitHub's merge preview as repository history.

## Alternatives

- Use exact mode for every pull request. Rejected because every ordinary
  descendant of the first rebase integration would fail.
- Use linear mode for every pull request. Rejected because a new attestation
  set requires the stronger exact pre-integration check.
- Special-case one branch or pull-request number. Rejected because it is not a
  reusable content-based policy.
- Accept the synthetic merge preview in linear mode. Rejected because that
  would weaken the selected linear-history invariant.

## Consequences

The workflow gains one content-based mode-selection branch. Regression evidence
must show that a rebased unchanged descendant fails exact mode and passes linear
mode, while receipt-changing heads continue to use the already tested exact
contract.
