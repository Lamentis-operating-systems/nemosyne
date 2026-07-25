# 0022: Establish content-bound DOC-00 attestations

Status: Superseded
Date: 2026-07-24
Superseded by: 0040-integrate-doc-00-through-content-equivalent-rebase.md

## Context

`DOC-00` requires consolidation and independent review evidence before
`EVD-01` exists. The prior wording called those receipts signed and immutable
while leaving encoding, signature, retention, and custody to `EVD-01`, creating
a circular prerequisite. Manual checks also left package, review, finding, and
conformance registries vulnerable to transcription drift.

## Decision

DOC-00 uses named, non-cryptographic, content-bound attestations under
`docs/receipts`. They are ordinary version-controlled Markdown evidence, not
`EVD-01` experiment receipts, signatures, immutable external records, or
certification.

Each attestation records its kind and identifier, reviewer or owner identity,
independence or ownership declaration, reviewed commit and tree, the exact
included path set, deterministic Git-archive SHA-256, method, findings,
disposition, residual limits, evidence references, replacement identity, and
date. A changed reviewed archive requires a new attestation for every affected
review. Failed and superseded attestations remain in Git history; a correction
at the canonical path uses a new reviewed digest and explicitly names the
record and digest it replaces.

The final DOC-00 merge-authorization attestation is added only after the
corrected specification-and-decision commit exists. The attested DOC-00 pair
therefore has two directly related commits: a source-freeze commit containing
the reviewed specifications, decisions, schema, and checker, followed by an
evidence commit containing only attestations and no change under the reviewed
paths. Receipt files are outside the reviewed archive, avoiding digest
self-reference. The structural checker must pass before the source freeze; the
evidence commit and final pull-request head must pass the checker's strict
receipt mode. DOC-00 is
integrated with a merge commit that preserves both exact branch commits.
Squash merge and rebase merge are invalid because either can replace the
attested source identity. Cryptographic signing, durable external custody,
access control, and experiment-receipt schemas remain owned by `EVD-01`.

Strict mode accepts the evidence commit itself before integration. Any later
validated head must descend from a two-parent merge commit whose second parent
is that exact evidence commit. Later descendants of the preserving merge do
not alter the attested pair when the reviewed paths, governance programs,
receipt schema, and canonical attestations remain tree-entry-identical to
their bound commits. Any change to one of those bound surfaces requires a
replacement source-and-evidence pair; unrelated later history does not.

The evidence record uses `Status: MergeAuthorized`; it does not claim `Merged`
or `Promoted` while still on a pull-request branch. G0 closes only after the
accountable human creates the required merge commit and the strict main-push CI
run passes against that exact ancestry. This is the sole bootstrap transition
for the package that establishes the later package-lifecycle evidence schema.
The recorded authorization is effective only after the committed evidence head
passes the strict, change-aware, and complete repository checks named in the
record; a field value alone grants no merge authority.

DOC-00 also includes a deterministic repository checker for the current
delivery-program registry. Its structural mode verifies contiguous finding and
conformance IDs, the eighteen review IDs, unique package/interface/wave
identities, declared counts, and the references required for G0. Its strict
mode additionally reconstructs the reviewed archive and validates the complete
canonical attestation set against the frozen source. Regression tests exercise
both modes, and CI requires strict mode on the final pull-request head.

## Rationale

This contract provides reproducible review evidence without claiming
cryptographic properties that cannot exist before `EVD-01`. Binding reviews to
the specification-and-decision archive makes source drift visible. Mechanical
registry checks remove error-prone manual counting while leaving semantic
review to named reviewers.

## Alternatives

- Build the complete experiment receipt system before DOC-00. This inverts the
  dependency because its contract is part of DOC-00.
- Call ad hoc Markdown receipts signed and immutable. That would be false.
- Keep all registry checks manual. Repeated edits have already made count and
  ownership drift a material risk.

## Consequences

DOC-00 acceptance and G0 wording use “content-bound attestation,” not “signed
immutable receipt.” The repository gains governance tooling and tests but no
runtime dependency or product behavior. Its merge-method exception must be
visible in the pull-request handoff. Later `EVD-01` evidence does not
retroactively turn these attestations into experiment receipts.
