# 0033: Separate source conformance from stable G0 identity

Status: Accepted
Date: 2026-07-25

## Context

Decision 0030 protected the append-only finding and conformance histories while
`DOC-CONF-22` was the active source receipt. Subsequent remediation requires a
new source receipt before the first valid DOC-00 evidence pair. The same change
also exposes two identities that earlier wording treated as one:

- the append-only `DOC-CONF-*` receipt describing one reviewed source revision;
  and
- the canonical `doc-00-g0.md` record that is replaced at one stable path over
  the lifetime of DOC-00 evidence.

Advancing both identifiers on every replacement would make historical record
validation unable to address one stable canonical record. Keeping both
identifiers fixed would let a changed reviewed archive reuse stale source
conformance. The checker previously required only that old conformance sections
remain a prefix, so a replacement archive could change without appending the
next source receipt.

## Decision

The source revision that includes this decision appends `DOC-CONF-24`.
`DOC-CONF-01..23` remain append-only history. The first valid evidence pair for
this source uses `DOC-CONF-24` as the `Record ID` at
`docs/receipts/doc-00-g0.md`.

After that first valid pair, the two sequences have distinct rules:

- `DOC-CONF-24` remains the stable canonical G0 replacement `Record ID`.
  Every later `doc-00-g0.md` replacement retains that ID and uses
  `Replaces: DOC-CONF-24 at archive digest <prior digest>`.
- Every changed reviewed archive appends exactly one next source-conformance
  section. The first later source uses `DOC-CONF-25`, the next uses
  `DOC-CONF-26`, and so on. The active structural count and section follow that
  append-only source sequence; they do not rename the stable G0 record.
- A comparison whose `docs/specifications` and `docs/decisions` trees are
  identical appends no source-conformance section. A changed reviewed archive
  with zero or more than one new section is invalid.
- Findings remain independently append-only. A source-conformance successor
  does not imply that a finding must exist, and a finding or other reviewed
  source change still requires exactly one successor for the resulting source
  revision.

The deterministic checker compares the two reviewed directory tree entries to
decide whether the archive changed. It first preserves every old finding row
and conformance section as an exact ordered prefix. For a changed archive it
then requires the successor conformance list to contain exactly one additional
contiguous identifier. Strict replacement validation applies the same rule
between the recursively validated predecessor source and the replacement
source. This rule is checked after more specific historical, identity,
replacement, and archive-integrity validation so it does not mask a stronger
failure.

The protected byte-digest boundary selected by Decision 0030 does not move.
`DOC-CONF-01..21` and `FND-001..151` remain protected by their existing
digests. `DOC-CONF-22` and every later section, plus `FND-152` and every later
finding, are protected by the trusted comparison-base append-only check before
the first evidence pair and by recursive predecessor validation after evidence
exists. No digest is rebaselined. Decision 0030 remains Accepted and is not
superseded; this decision tightens its archive-reconstruction preconditions
without changing its protected histories, trusted-comparator authority, or
historical validation model.

Every current or historical attested source commit must contain neither a
tracked root `.gitattributes` nor a tracked `.gitattributes` anywhere below
`docs/`. Root attributes can transform both reviewed directories, while a
nested attributes file can transform its reviewed subtree through
`export-ignore` or `export-subst`. The checker enumerates the source commit with
the existing hardened Git command and environment, using recursive
NUL-terminated tree output, and rejects a forbidden path before invoking
`git archive`. Attribute files below unrelated non-`docs/` directories remain
allowed because they cannot govern the two included archive paths.

`CANONICAL_G0_RECORD_ID` names the stable external record independently of
`EXPECTED_CONFORMANCE_COUNT`, which names the current source receipt. Error
messages, receipt fixtures, history validation, and replacement fixtures use
the appropriate identity rather than assuming they are always equal.

Changes to `scripts/check-v1-delivery-program.py` and
`scripts/test-v1-delivery-program-check.sh`, plus the conformance-aware fixture
construction in `scripts/test-documentation-check.sh`, are the focused
governance implementation authorized by this decision. Regression evidence
must include:

- a changed reviewed archive with exactly one next conformance section;
- changed archives with no successor and with more than one successor;
- an unchanged reviewed archive with no successor;
- a strict initial evidence pair;
- a strict valid replacement that advances only source conformance while
  retaining G0 `Record ID: DOC-CONF-24`; and
- strict replacement failure when changed reviewed content reuses the previous
  active conformance section;
- strict failure for a tracked root `.gitattributes` using `export-ignore`; and
- strict failure for a tracked `docs/specifications/.gitattributes` using
  `export-subst`; and
- general documentation-policy fixtures that append exactly one synthetic
  successor when their reviewed specification or decision tree changes, so a
  missing successor cannot mask the behavior each fixture is intended to
  exercise.

## Rationale

Source conformance describes a source revision and therefore must advance with
that revision. The canonical G0 file describes one replaceable evidence role
and therefore needs a stable record identity. Separating them preserves both
append-only source history and unambiguous replacement ancestry.

Comparing Git tree entries avoids treating timestamps, working-tree state, or
receipt-only changes as reviewed-source changes. Requiring exactly one
successor makes one pull-request source freeze correspond to one current
conformance statement and prevents both stale reuse and artificial sequence
skips. Retaining the existing protected digest avoids an unnecessary
governance rebaseline while the trusted-prefix mechanisms cover all later
history.

Disabling system, global, and `.git/info/attributes` inputs does not disable
attributes committed in the source tree. Because those tracked files can
change the bytes or membership produced by `git archive`, allowing them would
make the nominal archive command depend on a second transformation program
inside the attested input. Rejecting only the root and `docs/**` attribute
paths closes that ambiguity without imposing an unrelated repository-wide
ban. NUL-delimited tree enumeration preserves exact path boundaries and avoids
newline-based path confusion.

## Alternatives

- Advance the G0 `Record ID` with every source receipt. Historical canonical
  paths would no longer have one stable replacement identity.
- Keep source conformance and G0 identity fixed. Changed reviewed archives
  could retain a stale current-source statement.
- Allow any positive number of appended conformance sections. One source
  revision could skip or fabricate intermediate source states.
- Infer archive change from the archive digest recorded by the new receipt.
  That value is supplied by the evidence being validated; Git tree comparison
  derives the condition from the actual predecessor and successor sources.
- Permit tracked archive attributes and treat their effects as part of the
  source semantics. Reviewers would have to evaluate a second archive
  transformation language, and a later rule could silently omit or substitute
  reviewed bytes while retaining the same recorded command.
- Rebaseline the protected conformance digest through `DOC-CONF-23`. Existing
  trusted-prefix validation already protects those bytes, so a rebaseline adds
  authority and migration cost without closing another gap.

## Consequences

The first valid DOC-00 evidence set uses `DOC-CONF-24` at the canonical G0 path.
Future source work updates the active conformance count and appends one section
but does not change the external G0 `Record ID`. Fixture builders must model
both identities explicitly.

Every changed reviewed archive incurs a small documentation obligation even
when package counts and findings do not change. That cost is intentional: the
new section states which exact source revision was reconciled and why no other
structural value changed. Replacement validation performs two bounded Git tree
lookups and parses the two delivery histories after the archive and predecessor
bindings have otherwise passed.

Documentation-policy fixtures use a test-only, idempotent successor helper:
the first specification or decision mutation after a fixture baseline appends
one next synthetic conformance section, while later mutations in the same
prospective commit append none. Initial fixture baselines and non-reviewed
changes remain untouched. This preserves the production rule inside tests
without weakening the separate negative fixtures for missing, multiple, or
noncontiguous successors.

Repositories using root attributes must remove or relocate them before a
DOC-00 source freeze. Attributes outside `docs/` remain permitted when they are
not at the repository root. Every historical source encountered during
replacement validation is checked under the same rule, so a newly hardened
head cannot legitimize an older archive whose tracked attributes could have
altered reviewed bytes.

This decision refines Decisions 0022 and 0030. It changes neither the
non-cryptographic nature of DOC-00 attestations nor the merge-commit
requirement, evidence schema, protected digests, reviewed path set, or
accountable-human boundary.
