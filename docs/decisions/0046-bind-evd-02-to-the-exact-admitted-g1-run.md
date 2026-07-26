# 0046: Bind EVD-02 to the exact admitted G1 run

Status: Accepted
Date: 2026-07-26

## Context

EVD-01 authenticates a complete run manifest and guard witness, while TGT-00
constructs the typed signed G1 envelope embedded in that manifest. A later
EVD-02 analyzer must not accept those values separately and accidentally or
deliberately evaluate an admitted run against another envelope or execution
instance.

The repository does not yet contain the independently authored tasks,
condition artifacts, justified thresholds, executable analysis procedure,
review and custody identities, or controlled execution inputs required for an
empirical G1 run.

## Decision

Introduce `AdmittedG1RunV1` in `nemosyne-evaluation::evidence::g1`.
Construction consumes `ValidForOutcomeAccess`, verifies the supplied signed G1
envelope, reconstructs the canonical G1 run payload from that envelope and the
supplied execution binding, and requires exact equality with the admitted
manifest payload.

Retain the complete typed envelope and execution binding and expose them only
for read access together with the admission. Add no outcome-analysis or
experiment-receipt finalizer in this change.

Treat the current empirical EVD-02 execution as blocked. Synthetic tests may
exercise structural binding but cannot provide authored inputs or empirical
evidence.

## Rationale

Exact payload reconstruction reuses the existing canonical TGT-00 encoding and
EVD-01 admission instead of creating a second identity system. Consuming the
admission prevents one successful admission from being silently rebound.
Withholding pass, fail, and `Inconclusive` construction prevents arbitrary
opaque bytes or self-authored fixtures from masquerading as proof-owned G1
analysis.

## Alternatives

- Execute G1 with repository test fixtures. Rejected because those fixtures
  are self-authored structural examples, not independent controlled evidence.
- Accept an opaque caller-supplied result and disposition. Rejected because it
  would permit unvalidated `Pass` receipts.
- Implement the complete statistical analyzer now. Rejected because the
  concrete prospective artifacts and executable analysis procedure do not yet
  exist.
- Parse and duplicate the signed-envelope fields from the admitted payload.
  Rejected because canonical reconstruction and exact byte comparison are
  smaller and avoid a second decoder contract.
- Report missing pre-access design or custody artifacts as `Inconclusive`.
  Rejected because the proof program reserves `Inconclusive` for specified
  failures after valid outcome admission.

## Consequences

A later EVD-02 implementation has one fail-closed admitted-run input boundary.
The current package cannot produce G1 pass, fail, or `Inconclusive` evidence
and therefore does not unblock TGT-01 or any implementation package that
depends on passing G1.

The external task corpus, condition bytes, analysis artifacts, identities,
custody system, controlled runner, and receipt schema remain separate
prospective work.
