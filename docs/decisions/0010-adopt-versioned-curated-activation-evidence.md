# 0010: Adopt a versioned curated activation evidence corpus

Status: Accepted
Date: 2026-07-23

## Context

The activation kernel deterministically ranks already-normalized signals, and
the parameter evaluator measures agreement with strict numeric preferences.
Neither contract establishes what a channel means, why an authored signal has a
particular value, or why one candidate should outrank another in a concrete
situation.

Calibrating parameters now would optimize ungrounded numbers. Implementing a
signal encoder now would provide no independent semantic target against which
its output could be judged. The project first needs a small, reviewable evidence
artifact that preserves the distinction between authored meaning, numeric
annotation, ranking, and evaluation.

## Decision

Add a non-published `nemosyne-evaluation-corpus` workspace crate with path
dependencies on `nemosyne-core` and `nemosyne-evaluation`. Keep both existing
crates unchanged.

Publish one fallible constructor for corpus revision `1`, scoped to constructed
coding-agent situations. Bind every numeric gate, candidate signal, and expected
preference to reviewable scenario facts and rationale. Preserve provenance for
each individual channel judgment rather than only for a complete vector; do not
generate channel provenance by copying vector-wide metadata. Use a fixed
five-level dyadic authoring grid with separate gate and candidate-signal rubrics
instead of arbitrary floating-point values. Represent a candidate channel under
an absent gate explicitly as `Inactive` with canonical numeric zero, rather than
misstating that zero as an active fit judgment.

Define five corpus-local positive-evidence channels: trigger alignment,
observed-state alignment, alignment with the currently active task outcome,
capability fit, and constraint alignment. The active outcome may be supplied by
the surrounding task state rather than repeated by the current trigger. Do not
add an inhibition channel until a concrete failure of positive evidence gives
it independent semantics.

Partition semantic cases explicitly into calibration and held-out evidence.
Keep every paired contrast in one split, require broad category coverage in
both splits, reject matching canonical algebraic preference signatures across
the two partitions, and treat all provenance as constructed rather than
empirical. Under idealized nonnegative real arithmetic without inhibition,
canonicalize one-sign inequalities by signed support and mixed-sign
inequalities by their positive ray. Treat this as a conservative structural
guard, not exact `f64` evaluator equivalence or complete leakage protection.
Preserve merged revisions unchanged and expose a complete non-cryptographic
regression fingerprint. Once held-out results influence later work, require
previously unauthored cases for a later independent claim.

Require channel, category, and reference stable keys to be byte-exact,
case-sensitive, and unique within their namespaces. Report invalid fact
references with their exact gate, candidate-signal, or preference location.
Give every supported activation-parameter kind an explicit fingerprint
encoding and fail loudly when protocol version `1` encounters an unsupported
kind.

Provide two explicit reference parameter sets: trigger-only evidence and
uniform evidence. Neither is a runtime default or selected parameter set.
Validate numeric compatibility by delegating to the existing evaluator, without
copying its ranking or metric logic.
Freeze complete per-scenario reference observations: ordered candidate
identifiers, exact final-score bits, expected preferences, and observed
outcomes. Derive aggregate regression expectations from those fixtures.

Describe revision-1 channel coverage as authored semantic coverage, not
independent discrimination. Record one local sensitivity observation for cases
2201 and 2202: uniform weights satisfy both preferences; changing only
constraint weight from one to zero makes case 2201 violated while case 2202
remains satisfied. Do not interpret this fixed comparison as a controlled
intervention, causal effect, channel independence, identifiability, or
generalization.

The detailed contract is maintained in
[`curated-activation-evidence.md`](../specifications/curated-activation-evidence.md).

## Rationale

A separate crate keeps reusable offline evidence out of the runtime kernel and
out of the generic evaluator. It is justified because later signal derivation
and calibration can consume the same versioned artifact together with its
provenance; a test-only fixture would not provide that boundary.

The discrete grid makes authored judgments comparable and avoids false
precision while mapping exactly to the kernel's unit interval. Separate
rubrics distinguish situation-level channel relevance from candidate-level fit.
Paired contrasts exercise related situations with reversed expected orders
without claiming a controlled intervention. Semantic-case isolation and
integer algebraic preference signatures together reject direct semantic
variants and the covered structural duplicates across partitions. The
signature deliberately ignores floating-point normalization and rounding, so
equal signatures need not produce equal score bits or preference outcomes.

Stable keys are public machine-readable identities and therefore require the
same unambiguous construction boundary as numeric identifiers. Exact fact
locations make authoring defects actionable. Failing on an unknown fingerprint
input is safer than producing a plausible collision. Per-scenario bit fixtures
detect ranking and arithmetic drift that aggregate counts can hide.

Reference successes, ties, and violations are retained as observations. No
outcome is a corpus-validity condition: selecting cases to force reference
failure would be as circular as selecting cases to force reference success.
Revision `1` reference reports were visible during implementation, so its
held-out partition validates workflow separation but is not blind evidence of
performance or generalization. Independent semantic authoring for a later
selection claim requires previously unauthored cases frozen before their
reports are inspected.

## Alternatives

- **Implement a parameter calibrator next.** Rejected because no grounded,
  disjoint evidence exists yet.
- **Implement prompt, vector, or world-state signal derivation next.** Rejected
  because its outputs would lack an independent evaluation target.
- **Store the cases only as evaluator integration-test fixtures.** Rejected
  because future offline tools need reusable suites and semantic provenance.
- **Add the corpus to `nemosyne-evaluation`.** Rejected because the evaluator is
  domain-independent mechanism, while the corpus is versioned domain evidence.
- **Use arbitrary `f64` annotations.** Rejected because values such as `0.73`
  would imply unsupported precision.
- **Use only one reference parameter set.** Rejected because trigger-only and
  uniform evidence expose different, useful failure modes without selecting an
  optimum.
- **Add inhibition for constraint violations.** Rejected until a concrete
  positive-evidence failure establishes its semantics and interaction with
  correlated evidence.
- **Randomly split scenario rows.** Rejected because paired variants
  would leak one semantic source across calibration and held-out evidence.
- **Rely only on semantic-case identifiers for split isolation.** Rejected
  because unrelated labels can still encode the same evaluated inequality.
- **Treat equal algebraic signatures as exact evaluator equivalence.** Rejected
  because floating-point normalization and rounding provide concrete
  counterexamples.
- **Remove the algebraic split guard.** Rejected because its conservative
  structural protection remains useful when its limits are explicit.
- **Rewrite cases to isolate one channel at a time.** Rejected because this
  would create new evidence instead of describing revision `1` honestly.
- **Claim that the fixed constraint ablation identifies causality.** Rejected
  because two fixed parameter points do not establish a data-generating process
  or control every semantic factor.
- **Enumerate exact `f64` behavior for every parameter tuple.** Rejected because
  the public parameter domain admits an astronomically large finite set of
  representable tuples.
- **Keep only aggregate reference snapshots.** Rejected because offsetting
  scenario changes can leave aggregate counts unchanged.
- **Reject every equal numeric value across splits.** Rejected because
  unassessed candidates and other unevaluated aspects may legitimately share a
  coarse representation; the guard is scoped to canonical algebraic preference
  signatures.
- **Claim empirical ground truth.** Rejected because revision `1` contains
  project-authored synthetic judgments.
- **Add serialization, a database, or a CLI.** Rejected because the first
  artifact is small, static, and consumed in-process.

## Consequences

The workspace gains a third, offline-only crate and no external dependency.
Corpus revision `1` is intentionally small and may reveal ambiguous or
insufficient channel semantics. Any later evidence change creates a new
revision and preserves revision `1`. Observing held-out results permanently
precludes treating revised versions of those same semantic cases as new
independent evidence. The regression fingerprint detects accidental revision
rewrites but is not an integrity or authenticity mechanism. A future
activation-parameter kind requires an explicit fingerprint-protocol decision
before use. The exact regression fixtures intentionally make arithmetic drift
visible across supported platforms.

The corpus establishes reproducible semantic provenance and reference reports,
not real-world validity, statistical generalization, safety, optimal channels,
calibrated parameters, channel independence, identifiability, or causal effects.
The algebraic split guard may conservatively reject cases that differ under
exact evaluator arithmetic. The next runtime task may define one narrow,
deterministic signal-derivation contract against this corpus. Parameter
calibration remains a later, separate decision.
