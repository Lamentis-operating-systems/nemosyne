# 0049: Gate focus generation with applicability and typed readiness

Status: Accepted
Date: 2026-07-27

## Context

The V1 contracts preserve authority, validity, supersession, and conflict, but
they do not yet name one pre-generation boundary that decides whether a memory
is applicable to the current subject and project. This leaves room for a focus
adapter or prose renderer to resolve structured status, revision, validity,
replacement, authority, or priority conflicts implicitly.

Isolated internal synthetic vector experiments indicate that generic semantic
embeddings lose exact names, numbers, deadlines, negations, and guardrails;
model-native vector sequences retain more behavior but are model-bound and
comparatively large; aggressive compression loses substantial relative
stability. These observations are feasibility direction only. They are not G1
or V1 evidence and do not select an adapter, model, representation, or
threshold.

## Decision

V1 evaluates one `ApplicabilityBoundaryV1` before focus-candidate generation.
It contains structured, non-text controls for subject and project scope,
record status, revision or cycle, validity interval and state,
supersession/replacement, and authority and priority. Numerical similarity,
an embedding, an adapter, and prose cannot create, replace, or relax these
controls.

The boundary returns exactly one closed `ApplicabilityOutcomeV1`:

- `Applicable(ApplicableFocusInputV1)`;
- `Abstain(AbstainReasonV1)`; or
- `Conflict(ConflictSetV1)`.

`Abstain` and `Conflict` are deterministic terminal semantic outcomes for the
focus branch. They are data rather than generator prompts. No LLM, adapter,
lexicalizer, validator, or downstream rendering step can turn either outcome
into `Applicable`, choose a conflicting value, or emit a positive focus claim.
Malformed inputs, broken lineage, or capacity failures remain typed errors and
are not relabeled as abstention.

Only `Applicable` may enter focus-candidate generation. Planning subsequently
returns one closed `FocusReadinessOutcomeV1`: `Ready` contains a
`TypedFocusStructureV1`, while the earlier terminal `Abstain` or `Conflict`
passes through unchanged. The structure is finite, canonical, and validatable
before optional prose rendering. It carries typed focus roles, admitted
support handles, required qualifiers and relations, conflict/omission state,
authority ceiling, and exact-slot descriptors; it contains no prose field.
Rendering may realize only a validated `Ready` structure. A deterministic
non-LLM realization is the first baseline; any learned renderer remains
optional and evidence-gated.

The existing `CORE-01` package is the next bounded implementation package. It
owns only the dependency-light validated control and outcome vocabulary in
`nemosyne-core`; it does not evaluate memory, construct focus, render prose, or
select an adapter. `CORE-02` composes the controls into shared records.
`PLAN-01` evaluates applicability before candidate generation, and `PLAN-02`
constructs and validates the typed focus structure before `REN-01` can render.

## Rationale

Hard applicability is not semantic relevance. Separating it prevents a fluent
model from converting obsolete, out-of-scope, lower-authority, or mutually
incompatible material into plausible focus text. A typed pre-render structure
also makes positive, negative, and counterexample fixtures possible without
depending on model behavior.

Reusing `CORE-01` avoids adding a largely administrative package to the
already closed delivery registry while still placing the reusable types before
memory records, planning, and renderer work.

## Alternatives

- **Let the focus adapter infer applicability.** Rejected because model output
  is neither an authority decision nor a deterministic conflict resolver.
- **Encode controls only in vectors.** Rejected because the isolated
  experiments show loss of exact guardrails and provide no proof that a
  learned representation preserves policy semantics.
- **Render prose first and validate it afterward.** Rejected because prose
  validation cannot reliably reconstruct every omitted structured control.
- **Add a new top-level delivery package.** Rejected because `CORE-01` already
  owns dependency-light canonical primitives and is the earliest eligible
  implementation boundary.

## Consequences

`CORE-01` must provide package-local positive, negative, and counterexample
fixtures for every control and terminal outcome. Later focus and renderer
packages must prove that terminal outcomes bypass adapter execution and that
only a validated typed structure reaches optional prose realization.

The experiment observations remain in the research ledger with an explicit
non-proof boundary. No experiment metric becomes a V1 threshold or promotion
receipt through this decision.
