# Experimental Alpha path

Status: Proposed

## Purpose

This specification defines an isolated implementation-exploration path for the
applicability and typed-readiness boundary selected by Decision 0049. It
permits a small deterministic prototype before formal G1 passes without
altering or competing with the canonical V1 delivery program.

Alpha is not V1, G1/G3 evidence, a product capability, or a release candidate.
The existing 54 delivery packages, dependency graph, gate definitions,
`CORE-01` eligibility, and release rules remain unchanged.

## Definitions

`nemosyne-experimental-alpha` is the only permitted Alpha crate. It is a
workspace member with `publish = false`. It may depend on dependency-light
canonical primitives already implemented in `nemosyne-core`; canonical,
formal-evaluation, boundary-fixture, release, and published crates cannot
depend on Alpha or re-export it.

Alpha's public vocabulary is explicitly experimental and contains no public
type, trait, function, module, constant, or re-export whose identifier ends in
`V1`. Its permitted surface is limited to:

- bounded typed controls for exact subject/project scope, active state,
  revision or cycle, validity, replacement/supersession, authority/priority,
  and explicit conflict;
- a closed Alpha applicability outcome with `Applicable`, `Abstain`, and
  `Conflict`;
- a closed Alpha readiness outcome in which only `Applicable` may produce a
  bounded, canonical, prose-free typed focus structure and the two terminal
  outcomes pass through unchanged;
- typed construction/evaluation errors distinct from semantic abstention and
  conflict; and
- a deterministic non-LLM baseline over the ready typed structure.

The baseline is a structural oracle for fixtures. It may serialize only the
admitted typed roles, support handles, qualifiers, relations, omissions,
authority ceiling, and exact-slot descriptors in canonical order. It may not
infer a fact, resolve a conflict, relax a qualifier, select an action, or
present itself as product prose.

The permitted dependency direction is:

```text
nemosyne-experimental-alpha ──depends on──> nemosyne-core

canonical V1 / evaluation / fixtures / release ──X──> Alpha
Alpha artifacts ──X──> formal evidence or release state
```

Alpha is not added to the 54-package delivery DAG.

## Preconditions

- The implementation pull request freezes a finite fixture manifest before
  reporting any Alpha result.
- Every fixture is local, synthetic, content-identified, and contains no
  private user memory, credential, participant data, or claimed case truth.
- Every control has an explicit finite representation and bound. Missing,
  malformed, duplicate, contradictory, and over-limit inputs return typed
  errors under one documented deterministic precedence.
- Outcome semantics and precedence are specified before implementation. The
  current proposal does not select them merely from feasibility metrics.

## Invariants

- Alpha cannot construct, import, implement, alias, wrap, serialize, or
  deserialize a formal G1 envelope, admission capability, receipt,
  `Pass`/`Fail`/`Inconclusive` disposition, V1 package receipt, promotion
  permit, release receipt, or release state.
- Alpha completion cannot satisfy a dependency, entrance condition,
  acceptance row, gate, or requirement in the V1 delivery program.
- `Abstain` and `Conflict` are terminal. Neither readiness nor the deterministic
  baseline can convert them to `Applicable` or emit a positive focus.
- Only a valid `Applicable` value can produce a ready typed structure.
- The same valid input produces byte-identical outcome and baseline bytes
  across input permutations admitted as semantically equivalent.
- Numerical similarity, embeddings, adapters, and prose are absent from the
  applicability decision.
- Alpha contains no persistence, database, migration, product API, CLI,
  network, telemetry, LLM/model selection or invocation, tokenizer, learned
  adapter, external provider integration, packaging, publication, or release
  path.
- Alpha observations and fixtures are `ExperimentalAlpha/NonPromotional`.
  They are not converted, copied, cited, or summarized as formal proof.

## Edge cases

- Empty and whitespace-only required identifiers reject.
- Unknown enum values, invalid bounds, duplicate controls, broken replacement
  lineage, and incompatible control combinations reject without a partial
  semantic outcome.
- Scope mismatch, inactive/invalid/replaced records, and unresolved maximal
  conflicts require explicit fixtures; their exact outcome and precedence must
  be frozen in the implementation specification before code is accepted.
- Empty admitted support, over-capacity focus structure, unknown role,
  unsupported relation, qualifier loss, and noncanonical ordering reject.
- Baseline invocation on `Abstain`, `Conflict`, or an invalid structure is
  unrepresentable or returns a typed error without output bytes.

## Verification

A later Alpha implementation pull request is acceptable only when all of the
following are machine checked:

1. `cargo metadata --format-version 1 --no-deps` and a repository boundary
   checker prove exactly one package named `nemosyne-experimental-alpha`,
   `publish = false`, no canonical workspace package depends on it, and its
   dependency closure contains only the explicitly allowed workspace
   primitives.
2. The boundary checker scans Alpha's public Rust surface and rejects public
   `*V1` identifiers, formal evidence/release imports or terms, forbidden
   dependencies, build/publish metadata, and forbidden capability modules.
3. Package-local tests cover each control with positive, negative, boundary,
   malformed, and permutation fixtures; every closed applicability outcome;
   terminal `Abstain`/`Conflict` propagation; applicable-only readiness;
   canonical typed-structure ordering; and byte-identical deterministic
   baseline output.
4. Counterexample tests prove that conflict cannot be selected away,
   replacement cannot be ignored, lower authority cannot override a higher
   admissible value, qualifier/support loss rejects, and no terminal or error
   case yields baseline bytes.
5. A documentation-policy regression proves that adding Alpha changes none of
   the 54 package rows, their dependency/graph counts, G1/G3 gate text,
   `CORE-01` predecessors, or release requirements.
6. The full repository checks in `AGENTS.md` pass.

These checks establish structural isolation and deterministic behavior only.
They do not establish usefulness, product headroom, empirical validity,
security, cognitive validity, or release readiness.

## Open questions

- The exact finite Alpha control schemas, limits, error variants, and
  deterministic error/outcome precedence.
- The exact bounded typed-focus roles and canonical baseline encoding.
- The frozen fixture manifest and minimum counterexample matrix.

These are blockers for the implementation pull request. Persistence, product
API, LLM/model choice, adapter integration, and release are outside this
specification rather than open Alpha questions.

## References

- [Decision 0049](../decisions/0049-gate-focus-generation-with-applicability-and-typed-readiness.md)
- [Decision 0050](../decisions/0050-isolate-a-non-promotional-experimental-alpha-path.md)
- [Focus-and-expectation planning](focus-and-expectation-planning.md)
- [Internal synthetic pilot](internal-synthetic-pilot.md)
- [V1 delivery program](v1-delivery-program.md)
