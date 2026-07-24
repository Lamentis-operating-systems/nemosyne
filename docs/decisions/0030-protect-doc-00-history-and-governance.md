# 0030: Protect DOC-00 history and governance

Status: Accepted
Date: 2026-07-25

## Context

DOC-00 depends on append-only finding and conformance history and replaceable,
content-bound attestations. The structural checker verified the current
finding ledger and conformance receipt but did not mechanically protect the
bytes of `FND-001..151` or `DOC-CONF-01..21`. Strict receipt validation also
resolved the predecessor of a replacement from the new source commit itself.
A source-freeze commit or an earlier preparatory commit could therefore
rewrite or remove prior canonical evidence and make that altered state appear
to be the replacement predecessor.

The delivery-program checker and its regression test define documentation
governance, but the change-aware documentation policy did not classify those
two paths as governance surfaces. The current conformance inventory also
mentioned more counts than the checker bound. Their expanded history and
attestation regressions also made the documentation job's five-minute timeout
too close to observed local suite duration to remain a reliable failure bound.

## Decision

The structural checker binds the exact UTF-8 byte regions containing
`FND-001..151` and `DOC-CONF-01..21` to two frozen SHA-256 digests. Any byte
change in either historical region fails validation. CRLF checkout translation
is normalized to canonical LF text bytes before locating and hashing those
regions, so the same tracked content validates on every supported platform.

Once Decision 0030 and both digest constants exist in the comparison base,
changing either digest requires this record to make its sole permitted
historical transition from `Accepted` to `Superseded`, name a decision added
by the same change, and bind that replacement in `Accepted` status. Merely
adding an unrelated accepted decision does not authorize a digest rebaseline.

Every finding row and complete conformance section already present in a pull
request's comparison base remains an exact ordered prefix of the successor.
Strict replacement validation applies the same comparison between each
recursively validated predecessor source and its successor source. Existing
entries may not be removed, reordered, or rewritten; only new sequential
entries may be appended. Canonical conformance-section text excludes only the
trailing CR/LF separator before the next canonical heading.

Strict receipt validation treats the 22 canonical attestation paths as one
complete set. At every inspected history head the set is either wholly absent
or wholly present; a partial set is invalid. Absence is a valid
first-attestation state only when no canonical path has earlier reachable
history. A present set must resolve to one common canonical evidence commit
and its direct source parent, retain the same tree entries through every later
bound-path-preserving commit, and satisfy the same source, archive, schema,
and replacement bindings as the current set. Its required G0 checks and
change-aware prefix are parsed as bounded literals from its own attested
source checker without executing historical code. Each name has exactly one
module-level literal binding, and any additional AST binding or mutation is
invalid. The receipt README must contain the complete normalized recursive
history contract. These source-bound contracts let later legitimate governance
additions coexist with immutable earlier evidence. Validation follows
predecessor source/evidence pairs recursively until it reaches the genuinely
absent first-attestation state.

Validation traverses every reachable merge parent rather than Git's
path-simplified history view and memoizes canonical receipt-tree states. A
partial state or disappearance after introduction is invalid on every branch.
Nonidentical parent states may converge only through the preserving two-parent
integration shape: the selected complete evidence commit is the second parent
and its replacement bindings match the first-parent state. This also rejects a
merge that hides an earlier complete or partial set by selecting an absent or
unrelated parent.

Every Git subprocess launched by the delivery-program checker disables
replacement-object semantics through both the command-line option and process
environment, strips inherited `GIT_*` repository/object/config overrides, and
pins the inspected worktree. Strict receipt validation rejects nonempty legacy
`info/grafts` files and shallow repositories rather than treating a substituted
or truncated boundary as a genuine root. The Documentation CI job therefore
checks out the complete repository history with `fetch-depth: 0`; jobs that do
not run strict lineage validation remain shallow by default.

Every historical source pair must also carry the canonical receipt-schema
README as a non-executable regular file and both governance programs as
executable regular files. Historical validation applies the same schema
semantics as current validation before accepting the pair.

Each protected-history digest name has exactly one module-level lowercase
SHA-256 string literal and no other binding or mutation in both the current
checker and every inspected historical checker revision. Change-aware
validation parses those literals as data rather than scraping a textual
assignment range, and recursive attestation validation never executes
historical checker code. Wildcard imports and direct dynamic namespace or
code-execution primitives are invalid in the bounded checker AST. Current
protected-region hashing and current G0 receipt validation use the parsed
literal values rather than rereading mutable module globals.

For pull-request digest comparison, the trusted comparison-base checker parses
both the base and head checker revisions. The modified head checker is never
trusted to report its own digest declarations. Executing the already trusted
comparison-base comparator is distinct from recursively executing an arbitrary
attested historical checker.

Once this decision exists in the comparison base, pull-request append-only
validation also extracts and executes the comparison-base checker rather than
the modified head checker. Every extracted comparator starts through isolated
Python mode without site initialization, so `PYTHONPATH`, user-site packages,
and `sitecustomize` cannot modify its imports or startup behavior.
Comparator helpers return status and data only. The parent validation shell
records every parser failure before cleanup and return; failure accounting
never occurs only inside command substitution, where a subshell-local counter
would be discarded.

The AST contract rejects protected names stored through object attributes or
subscripts as well as direct name stores. It resolves direct, imported, and
transitive aliases of namespace and code-execution primitives before examining
calls, so aliasing `globals`, `setattr`, `exec`, or equivalent builtins cannot
evade the single-literal-binding rule for either protected digests or G0
contract values.

Historical receipt reconstruction reads each Git blob as raw UTF-8 bytes and
applies the same byte-preserving parser used for current receipts. Generic Git
command helpers that trim output are not valid receipt readers; leading or
trailing whitespace cannot become valid merely because a receipt moved into
history.

The bounded checker AST rejects computed attribute or subscript access and
mutation through namespace-bearing objects, including module dictionaries and
builtin dictionaries. It likewise rejects computed lookup or aliasing of
namespace and code-execution primitives. A checker contract may use only the
closed statically identifiable subset validated by the trusted base checker;
concatenated keys cannot defer a protected mutation or executable lookup to
runtime.

The canonical archive command uses both `GIT_NO_REPLACE_OBJECTS=1` and
`git --no-replace-objects`. The receipt schema checker binds that complete
normalized command, including the immutable Git view, rather than validating
only the archive algorithm label.

The next source-freeze commit preserves every canonical attestation tree entry
from its parent byte-for-byte, including file mode. It may neither add, remove,
rewrite, nor rebase an existing canonical attestation. Its evidence child then
replaces exactly that recursively validated predecessor and names its prior
archive digest.

`DOC-CONF-22` binds the complete current structural inventory: package,
dependency-table, graph, interface, wave, finding, conformance, review,
specification, and decision counts including the accepted and superseded
decision split. The checker validates the canonical inventory statement
exactly once.

Changes to `scripts/check-v1-delivery-program.py` and
`scripts/test-v1-delivery-program-check.sh` are documentation-governance
changes. The change-aware documentation policy requires a new accepted
decision for either path, including additions, modifications, deletions, and
renames.

Regression tests cover both protected-history regions, unauthorized digest
rebaselines, CRLF checkout translation, append-only finding and conformance
history, complete current-inventory drift, partial receipt sets, source-freeze
and preparatory-history deletion or rewriting, historical G0 contract
evolution and rebinding, malformed prior schema or governance modes, recursive
predecessor tampering, present/absent and complete/partial merge histories,
divergent complete histories, protected-digest rebinding, replacement-object
history substitution, legacy grafts, hostile Git environment overrides,
shallow history, a lying head append-only checker, hostile Python startup
customization, attribute and subscript stores of protected names,
alias-mediated namespace and code-execution mutations, computed namespace
keys, computed execution lookup, raw historical receipt whitespace,
archive-command drift, and undecided changes to either governance path.

The Documentation CI job retains one shared fail-fast boundary but raises its
timeout from five to fifteen minutes. The limit accommodates the complete
fixture-based governance suites and strict recursive receipt validation while
remaining finite; it does not relax, skip, or reorder any required check.

## Rationale

An append-only claim is meaningful only when old bytes and the complete
predecessor chain are mechanically protected. Separate finding and conformance
digests make content drift visible; explicit supersession makes intentional
rebaselining reviewable. Prefix comparison extends that protection to every
newly attested entry without requiring a digest rebaseline for each legitimate
append. Recursively resolving complete source/evidence pairs prevents either
the source freeze or an earlier preparatory commit from redefining its
predecessor. Walking every merge parent avoids the TREESAME path simplification
that can otherwise erase reachable evidence from a path-limited Git log.
Single-binding digest extraction keeps the value reviewed by change-aware
policy identical to the value used at runtime. Classifying the checker and
tests as governance prevents silent weakening of the enforcement mechanism.
Disabling replacement objects, removing caller-controlled Git environment
overrides, rejecting graft files, and rejecting shallow clones ensure that
“reachable history” means the repository's stored commit DAG rather than a
locally substituted or truncated view.
Using the base checker for both pull-request comparisons prevents a proposed
checker change from declaring its own history valid. Isolated Python startup
closes the corresponding interpreter-startup injection path. Binding the full
archive command keeps the human recipe and executable archive semantics in one
reviewed contract.

## Alternatives

- Rely on reviewer comparison of historical conformance text. This is
  repeatable but not deterministic enforcement.
- Resolve replacement history from the source commit. This lets the commit
  being validated redefine the history it claims to preserve.
- Validate only selected current counts. Unchecked inventory claims can drift
  while structural validation still passes.
- Treat checker changes as ordinary scripts. Those scripts directly determine
  whether DOC-00 evidence and governance are accepted.
- Accept Git replacement objects, legacy grafts, environment-selected object
  stores, or shallow boundaries. Any can make a forged local history appear
  complete while hiding stored predecessors.
- Keep the five-minute Documentation timeout. The two principal governance
  suites already consumed most of that budget locally before checkout and the
  remaining structural, strict, and change-aware checks.
- Split or weaken the governance suites to preserve the shorter timeout. That
  would add scheduling complexity or reduce evidence without improving the
  contract.

## Consequences

The protected finding or conformance digest can change only through a future
decision that explicitly supersedes this governance rule; ordinary append-only
additions do not alter either digest. Every later source comparison also pays
the cost of exact prefix validation, and replacement receipt evidence carries
the cost of recursively verifying every earlier complete canonical pair.
Checker and regression-test maintenance always carries an accepted decision
record and matching documentation impact. The Documentation job may now run
for up to fifteen minutes before GitHub terminates it, but every failed check
still stops the job immediately.
