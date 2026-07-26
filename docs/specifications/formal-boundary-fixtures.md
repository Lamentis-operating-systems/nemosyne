# Formal boundary fixtures

Status: Experimental

## Purpose

This specification defines the BND-01 implementation of
`IF-BOUNDARY-FIXTURES`. The non-published
`nemosyne-boundary-fixtures` crate provides executable synthetic positive and
counterexample observations for F1 through F17 without selecting product
runtime technology.

The normative meanings of F1 through F17 remain owned by the
[V1 proof program](v1-proof-program.md). This fixture layer does not redefine
those obligations, implement a product compiler, construct a TGT-00 envelope,
execute EVD-02, or produce a G1 outcome.

## Definitions

### Ownership and dependency boundary

BND-01 is the sole semantic owner of the fixture catalog. It consumes only the
existing EVD-01 `ArtifactContentId` identity type. The crate:

- has no dependency from a production runtime crate;
- exposes no store, model, network, management, action-selection, experiment
  admission, or outcome-access capability;
- emits no producer or promotion receipt;
- does not select persistence, transport, model, renderer, or execution
  technology; and
- may be used only as offline verification support until a real interface
  producer and its receipt exist.

TGT-00 and EVD-02 contracts, types, thresholds, and dispositions remain
unchanged. In particular, a valid boundary catalog is neither a signed G1
envelope nor a `ValidForOutcomeAccess` value, and it cannot be interpreted as
G1 `Pass`, `Fail`, or `Inconclusive`.

### Catalog contract

`BoundaryFixtureCatalogV1` contains exactly one `Positive` fixture and one
`Counterexample` fixture for each closed `FixtureObligation::F1` through
`FixtureObligation::F17`.

Every fixture has:

- a unique lowercase kebab-case label;
- one closed obligation;
- one positive or counterexample kind;
- one typed `BoundaryObservationV1`; and
- for a counterexample only, one exact expected `BoundaryViolation`.

Catalog construction:

1. orders fixtures by obligation, kind, and label;
2. rejects duplicate labels;
3. executes every fixture and requires its declared outcome;
4. rejects missing or duplicate obligation/kind coverage; and
5. derives one SHA-256 `ArtifactContentId` from domain-separated canonical
   catalog bytes.

The identity is an EVD-01-compatible in-process fixture identity, not a
persistent wire-format commitment. Permuting cases preserves the identity.
Changing a fixture observation, label, expected violation, or coverage changes
the identity subject to the hash-collision assumption.

### Fixture matrix

| Obligation | Positive fixture | Counterexample | Checked boundary |
| --- | --- | --- | --- |
| F1 | `f1-prompt-preserved` | `f1-prompt-normalized` | Original prompt is the exact final byte suffix |
| F2 | `f2-authorized-before-relevance` | `f2-unauthorized-source-selected` | Authorization precedes relevance and every selected source is authorized |
| F3 | `f3-single-pinned-snapshot` | `f3-mixed-snapshot-revisions` | Semantic reads and exact slots use one pinned revision |
| F4 | `f4-read-only-compile-capabilities` | `f4-semantic-write-capability` | Compile receives only authorized reads and content-free coordination writes |
| F5 | `f5-focus-authority-lowered` | `f5-plan-authority-amplified` | Focus/expectation plan authority does not exceed any essential support |
| F6 | `f6-assertions-have-plan-support` | `f6-unknown-assertion-support` | Every assertion binds an existing planned item |
| F7 | `f7-complete-output-in-budget` | `f7-truncated-over-budget-output` | Complete post-substitution output fits without truncation or false empty success |
| F8 | `f8-bounded-monotonic-activation` | `f8-positive-evidence-lowers-activation` | Bounded activation and the exercised evidence/inhibition monotonic relation |
| F9 | `f9-buffered-validated-result` | `f9-error-exposes-prefix` | Success is completely validated and errors expose no semantic bytes |
| F10 | `f10-one-owner-per-truth` | `f10-plan-meaning-has-two-owners` | Prompt, ranking, and plan meaning each have one owner |
| F11 | `f11-authenticate-before-identity` | `f11-identity-before-authenticity` | Manifest authenticity precedes artifact identity |
| F12 | `f12-shared-set-and-exact-slot` | `f12-rebound-exact-slot` | Focus, expectation, and planner share witness/lineage and exact slot owner/content joins |
| F13 | `f13-duplicate-does-not-amplify` | `f13-duplicate-amplifies-support` | A duplicate in one dependency group cannot amplify support |
| F14 | `f14-complete-evidence-share-family` | `f14-evidence-share-promoted-to-probability` | Exact family shares are complete and remain evidence shares |
| F15 | `f15-alternatives-and-abstention-preserved` | `f15-action-role-crosses-plan-boundary` | Material transition alternatives, explicit abstention, bounded plan roles, and no action selection |
| F16 | `f16-offline-immutable-assessment` | `f16-runtime-assessment-mutates-memory` | Assessment remains offline and leaves prior evidence and memory unchanged |
| F17 | `f17-renderer-only-lexicalizes-plan` | `f17-renderer-adds-action-and-source-access` | Renderer assertions, exact slots, plan bindings, and prohibited capabilities remain closed |

The F15 positive observation contains both a bounded frame with preserved
material alternatives and a separate explicit abstention role. Plan roles are
closed to focus, expectation, validator-only control, and abstention;
`ProhibitedActionSelection` exists only as counterexample input.

The F17 positive observation permits only selected plan propositions and
structural surfaces. Its counterexample exercises action recommendation,
unbound assertion, exact-slot rebinding, and memory-text capability. Other
closed prohibited capability variants remain available for later consumer
fixtures without granting those capabilities.

## Preconditions

- A positive fixture has no expected violation.
- A counterexample has exactly one expected violation.
- Observation kind must match the named obligation.
- Every obligation has exactly one fixture of each kind.

## Invariants

- Catalog order depends only on obligation, kind, and label.
- Input permutation preserves fixture order and content identity.
- Every accepted catalog contains exactly 34 fixtures.
- Every accepted positive fixture evaluates successfully.
- Every accepted counterexample returns its exact declared violation.
- A fixture correction changes the catalog identity subject to the
  hash-collision assumption.
- No fixture constructs or grants a runtime, management, network,
  action-selection, G1-envelope, or outcome-access capability.

## Edge cases

- Empty, non-lowercase, or non-kebab-case labels reject before catalog
  construction.
- Duplicate labels reject even when they cover different obligations.
- Missing or duplicate obligation/kind coverage rejects without a partial
  catalog.
- An observation variant used under the wrong obligation returns
  `ObservationKindMismatch`.
- Exact-slot acceptance requires both owner and content identity to match the
  independently retained binding.
- Alternative shares use exact integers and a positive common denominator.
- Activation observations use integer basis points in `[0, 1000]`; they are
  synthetic monotonic canaries, not the production activation algorithm.
- Catalog construction returns no partial catalog after any failure.

## Verification

`crates/nemosyne-boundary-fixtures/tests/bnd_01.rs` verifies:

- all 34 fixtures execute to their declared outcome;
- the exact declared 34-label inventory is present;
- every F1 through F17 obligation has exactly one positive fixture and one
  counterexample;
- input permutation preserves canonical order and catalog identity;
- a fixture correction changes the catalog identity;
- missing and duplicate coverage fail; and
- an obligation/observation mismatch and invalid fixture labels fail
  explicitly.

Workspace formatting, Clippy, Rustdoc, and tests additionally prove that the
fixture API is documented and builds against the current EVD-01 interface.

## Limitations and non-claims

These fixtures are abstract executable boundary canaries. They do not prove
that a future product implementation lacks an ambient capability, implements
every F1 through F17 sub-obligation, preserves timing noninterference, or
satisfies a model-dependent semantic validator. Each later implementation and
verification package must bind its concrete inputs and outputs to the
applicable fixture observation and add domain-specific cases.

Catalog completeness is structural BND-01 implementation evidence only. It is
not G1 or G9 evidence, empirical validation, security certification, release
authorization, product value evidence, or a claim that G2 has been promoted.

## Open questions

Concrete consumer bindings remain owned by their later implementation and
verification packages. No unresolved choice in this fixture package selects a
model, store, transport, runtime topology, security architecture, or G1
outcome.

## References

- [V1 delivery program](v1-delivery-program.md)
- [V1 proof program](v1-proof-program.md)
- [Decision 0044](../decisions/0044-adopt-authenticated-evd-01-evidence-envelope.md)
- [Decision 0047](../decisions/0047-adopt-content-identified-formal-boundary-fixtures.md)
