# DOC-00 attestations

This directory contains the non-cryptographic, content-bound attestations
defined by Decision 0022. They are version-controlled review evidence, not
experiment receipts, signatures, external certification, or product evidence.

## Canonical paths

- `doc-00-g0.md`: the current DOC-00 merge-authorization attestation;
- `consolidations/consol-01.md` through `consol-03.md`: the three current
  consolidation attestations; and
- `reviews/rev-01.md` through `rev-18.md`: the eighteen current independent
  review attestations.

Every canonical file uses one two-column `Field | Value` table with these
fields in this exact order:

1. `Schema`
2. `Record ID`
3. `Kind`
4. `Status`
5. `Actor`
6. `Declaration`
7. `Completed at`
8. `Source commit`
9. `Source tree`
10. `Included paths`
11. `Archive algorithm`
12. `Archive SHA-256`
13. `Method`
14. `Findings`
15. `Disposition`
16. `Residual limits`
17. `Evidence references`
18. `Replaces`

`Schema` is `doc00-attestation-v1`. `Included paths` is exactly
`docs/specifications, docs/decisions`. `Archive algorithm` is exactly
`git-archive-tar-sha256-v1`. That algorithm hashes the raw stdout bytes from:

```text
GIT_NO_REPLACE_OBJECTS=1 GIT_ATTR_NOSYSTEM=1 git --no-replace-objects -c core.attributesFile=<OS null device> -c tar.umask=0002 archive --format=tar <source-commit> -- docs/specifications docs/decisions
```

The checker resolves `<OS null device>` through the standard-library null
device for the current platform; it is `/dev/null` on this source-freeze
environment. No bytes are read from that path.

The repository must have no nonempty `.git/info/attributes` file while
reconstructing the archive. An attested source commit must contain neither a
tracked root `.gitattributes` nor a tracked `.gitattributes` anywhere below
`docs/`; either could transform or omit reviewed archive bytes. Tracked
attribute files below unrelated non-`docs/` directories remain permitted.
Reviews and consolidations use
`Status: Pass` and `Disposition: Pass`; the G0 record uses
`Status: MergeAuthorized` and `Disposition: Pass`. It does not claim the pull
request is already merged or promoted.

The canonical `Kind` values are `Review`, `Consolidation`, and
`MergeAuthorization`. Every review uses the exact declaration
`Independent reviewer; did not author or remediate the reviewed source.`;
every consolidation uses
`Integration owner for the named consolidation pass.`; and the G0 record uses
`Principal integrator for DOC-00 merge authorization; not the accountable
human or an independent reviewer.`.

All current records bind the same source commit, source tree, included paths,
and archive digest. The source commit contains the reviewed specifications and
decisions. Receipt files are excluded from the reviewed archive so the digest
does not depend on itself.

The attested DOC-00 source/evidence pair consists of two direct commits. The
first freezes the reviewed source and passes the structural checker. Its direct
child adds only the attestations, leaves the included paths unchanged, and
passes:

```text
./scripts/check-v1-delivery-program.py --require-receipts
```

The DOC-00 pull request is merged with **Rebase and merge**. Pull-request
validation requires the recorded source commit to be the exact direct parent
of the evidence commit. GitHub rewrites both commit identities during
integration, so main-push validation instead requires their direct, ordered,
content-equivalent counterparts: the source tree, canonical receipt entries,
their original reviewed-archive binding, and the evidence-only 22-path delta
remain exact. The archive digest is reconstructed against the exact source
commit before integration; it is not reconstructed against a rewritten commit
because Git archive metadata is commit-sensitive. Squash merge is invalid. G0
closes only after the strict main-push CI run passes in explicit
linear-integration mode.

Before integration, strict validation accepts only the exact evidence commit.
After integration, strict linear validation resolves the common last-modified
evidence counterpart and its direct source parent, then reconstructs every
content and replacement binding from those integrated commits. Later linear
descendants remain valid only when the governance programs, this schema, and
all canonical attestations stay tree-entry-identical. Reviewed specifications
and decisions may evolve only through the commit-local successor-conformance
rule below. A change to a canonical attestation, governance program, or this
schema requires a replacement source-and-evidence pair.

The recorded merge authorization becomes effective only after the committed
evidence head passes the strict checker, the change-aware documentation check,
and every repository check named by the G0 record. A failure requires replacing
the evidence commit before review; the field alone grants no merge authority.

Review actors are unique and declare that they did not author or remediate the
reviewed source. Consolidation actors declare their ownership or integration
role. A passing record uses `Findings: None`; a material P0, P1, or P2 finding
blocks a passing disposition. `Evidence references` names the evidence used by
the record. `Replaces` is `None` for the first passing record.

Each canonical attestation remains valid only for the exact source archive it
names; it never extends its review claim to a later source revision. A later
reviewed source revision is covered only by its own successor-conformance
statement. Replacing a canonical record is required only when the canonical
DOC-00 evidence set or its governance contract changes, and uses a later Git
commit at the same canonical path with a new source identity. Its `Replaces`
value is exactly
`<Record ID> at archive digest <64 lowercase hexadecimal SHA-256>` and names
the different earlier archive digest; the earlier record remains recoverable
from Git history.

The G0 record uses `Record ID: DOC-CONF-24` for the first passing evidence set
and every later replacement. A later `DOC-CONF-25` or higher identifier names
the source-conformance successor only; it does not rename the canonical G0
record. Therefore every G0 replacement names
`DOC-CONF-24 at archive digest <prior digest>` in `Replaces`.

The replacement source-freeze commit has at most one parent and preserves
every canonical attestation tree entry from that parent byte-for-byte,
including its file mode. At every earlier history head, the canonical set is
either wholly absent or contains all 22 records. A present set must resolve to
one common last-modified evidence commit whose direct parent is its source
counterpart, whose only changes are those 22 records, and whose source tree,
schema, and replacement bindings are valid. Exact pull-request mode requires
the active replacement pair's parent to equal its recorded `Source commit` and
reconstructs its archive digest. Earlier pairs already integrated by the
required rebase method are accepted as content-equivalent counterparts only
when their evidence commits are reachable from
`NEMOSYNE_TRUSTED_PRIOR_HEAD`. CI binds that value to the pull-request base or
the pre-push integration head; local validation resolves `origin/main` when the
variable is absent. Every counterpart still requires the exact recorded source
tree, byte-identical receipt archive bindings, and direct source/evidence
ordering. Explicit linear-integration mode applies that same counterpart rule
to the active pair and does not reconstruct a commit-metadata-sensitive archive
from the rewritten commit. Historical G0
check requirements are read as exactly one module-level literal binding per
contract name from that set's own source checker; every additional AST binding
is invalid, current policy is not substituted, and no historical checker code
is executed. The protected digest names likewise have exactly one module-level
lowercase SHA-256 string literal and no other binding or mutation in every
current or historical checker revision. Attribute or subscript stores of
protected names, wildcard imports, and direct, imported, or transitively
aliased dynamic namespace or code-execution primitives are invalid. Current
protected-region and G0 validation use parsed literal values rather than
mutable module globals.
Pull-request digest comparison uses the trusted comparison-base checker to
parse both revisions, and append-only pull-request comparison uses that same
extracted comparison-base checker; the head checker does not validate either
claim about itself. Extracted comparators run with isolated Python startup so
caller-controlled module search paths, user sites, and `sitecustomize` cannot
alter them. A wholly absent set is a valid first-attestation state only when no
canonical path has earlier reachable history. All checker Git subprocesses
disable replacement objects and strip caller-controlled Git environment
overrides. Strict receipt validation rejects nonempty legacy grafts and shallow
repositories so reachable history cannot be substituted or truncated.
Documentation CI checks out full history before strict validation. Validation
traverses every commit reachable from HEAD and every merge parent with memoized
canonical receipt-tree states;
it rejects partial states, deletion after introduction, and any nonidentical
merge-parent states unless a two-parent preserving merge selects the exact
evidence commit as its second parent and that set replaces the first-parent
set. Linear integration additionally requires single-parent source/evidence
counterparts and unchanged source-tree and receipt bindings.
Validation follows source/evidence pairs recursively to that genuine
first-attestation state. Each historical source also carries this schema as a
non-executable regular file and both governance programs as executable regular
files. Every successor source preserves each existing finding row and complete
conformance section as an exact ordered prefix of both the pull-request
comparison base and the recursively validated predecessor source; only new
sequential entries may be appended. `Replaces` is derived from the validated
predecessor set, never from mutable content introduced by the new source freeze
or an earlier preparatory commit. After the canonical DOC-00 evidence pair,
linear integration permits reviewed specification and decision changes only
when every intervening commit preserves those append-only histories and
appends exactly one next conformance receipt when, and only when, the reviewed
archive changes. Canonical attestations and governance programs remain
unchanged.
