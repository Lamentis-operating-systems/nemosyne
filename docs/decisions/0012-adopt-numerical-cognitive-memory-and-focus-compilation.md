# 0012: Adopt numerical cognitive memory and focus compilation

Status: Accepted
Date: 2026-07-23

## Context

Decision 0011 fixes Nemosyne V1 as a local, read-only attention compiler and
requires structured numerical relevance computation after natural-language
ingress. It deliberately leaves the memory representation, signal semantics,
multi-memory integration, and the state passed to a renderer open.

The intended product is not ordinary retrieval-augmented generation. A compile
call must use the prompt and caller-supplied situation as cues into one logical
memory universe, weigh different aspects of several memories, preserve
conflicts and exact values, and construct a compact focus state. Passing raw
memory prose or one undifferentiated text embedding to a language model would
lose this separation and make the renderer responsible for retrieval and
priority policy.

Research on encoding specificity, temporal context, base-level activation,
goal-dependent accessibility, bounded working memory, associative activation,
and partner-specific common ground supports context-dependent competition as a
useful engineering inspiration. It does not supply one biologically exact
formula, optimal parameter values, or evidence that a software vector is a
human engram.

## Decision

Adopt a hybrid numerical cognitive-memory architecture as the V1 implementation
hypothesis.

The authoritative memory revision preserves immutable record versions,
canonical proposition identities, provenance, source authority, authorization,
validity, supersession, uncertainty, conflicts, dependency groups, and exact
loss-sensitive values. These values use typed identifiers, scalars, timestamps,
coordinates, enumerations, relations, and byte-preserving payloads. They remain
available even when no original prose is retained.

Every computational memory representation is numerical. A memory exposes
separate, versioned facets for semantic content, episodic context, temporal
context, spatial context, goals, procedures or action relevance, social
perspective, and salience or hazard relevance where those facets exist. A
facet is a typed vector plus presence, confidence, encoder identity, and
bindings to the authoritative proposition and exact values it represents.
Missing, unknown, inapplicable, and zero are distinct states. Vectors from
different spaces are never added or compared without an explicit registered
transform.

Prompt, situation statements, and metadata are encoded into a query with the
same relevant facet types. The original prompt bytes and every loss-sensitive
request value remain outside the lossy query representation.

Compilation applies hard authorization, disclosure, deletion, and
representation-validity rules before soft relevance. Current validity,
supersession, and instruction authority then determine whether a record may
support a current normative instruction. A request that explicitly asks about
history may retrieve an older or superseded revision as qualified historical
evidence, but temporal context cannot revive that revision's current
authority. Compilation then derives independently inspectable, calibrated
evidence signals for cue fit, temporal context, base availability, active-goal
fit, procedural fit, hazard fit, and social perspective where supported by the
input. It may derive bounded associative activation over typed relations.
Signals that lack an authored derivation contract are errors or absent; they
are not guessed.

Use the existing deterministic activation kernel as the first ranking
mechanism to evaluate for this architecture:

\[
E_i =
\frac{\sum_c w_c g_c e_{i,c}}
     {\sum_c w_c g_c},
\qquad
R_i = \prod_j(1-\lambda_jp_{i,j}),
\qquad
A_i=E_iR_i.
\]

The cognitive research motivates channel hypotheses and qualitative
counterexamples, not the runtime values of \(w_c\), \(g_c\), or
\(\lambda_j\). Parameters and normalization transforms are immutable,
versioned compiler artifacts selected only through disjoint evaluation.
Activation remains a bounded engineering score, not truth, probability,
safety, importance in every context, or a model of neuronal firing.

After ranking, perform request-local **focus consolidation**. This operation
groups selected facets by canonical proposition and source-dependency group,
prevents duplicate copies of one source from amplifying a proposition,
preserves independent corroboration, and keeps unresolved contradictory
propositions separate. It never averages incompatible meanings into a
compromise. It selects a bounded set that balances activation, required
coverage, non-redundancy, conflict visibility, authority, uncertainty, and
output cost.

Focus consolidation creates a canonical numerical `AttentionPlan` whose roles
distinguish:

- the current situation and task;
- the dominant active goal;
- immediate constraints or hazards;
- response-changing background;
- secondary goals or influences;
- unresolved conflict and uncertainty;
- social perspective where supported;
- non-renderable, control-only explicit exclusions; and
- exact-value and proposition-support bindings.

The plan, not the renderer, decides what may be expressed and which
relationships are dominant or secondary. It contains no proposed answer and
does not choose or execute an action. Nemosyne influences a later model by
making selected context available; V1 does not claim to predict that model's
action distribution.

Treat persistent memory consolidation as a separate management operation.
Read-only compilation may combine memories only in request-local state and
must not update access history, strengthen a memory, create an abstraction, or
write a new record.

Describe the final text as an evidence-bound **focus narrative** or
**verbalized focus state**. Do not describe it as a human chain of thought,
inner monologue, consciousness trace, or reconstruction of subjective
experience.

Maintain the detailed contract and mathematical assumptions in
[`cognitive-memory-activation-and-focus.md`](../specifications/cognitive-memory-activation-and-focus.md).

## Rationale

Typed facets let the same situation emphasize different evidence: an immediate
hazard can gate procedural and temporal cues strongly, while a planning request
can emphasize goals, deadlines, and social commitments. One global embedding
cannot preserve these roles reliably.

Keeping an authoritative exact plane beside the rebuildable numerical plane
prevents a lossy vector from becoming the source of a name, path, time, number,
authority label, or provenance claim. It also permits re-encoding and
representation experiments without rewriting memory truth.

Proposition-level consolidation uses multiple memories without turning
duplication into evidence or hiding disagreement. A bounded plan gives the
renderer a small working context while retaining machine-checkable support and
exclusion boundaries.

Separating persistent consolidation from compile-time integration preserves
Decision 0011's read-only invariant and prevents successful retrieval from
creating a self-reinforcing memory loop.

The focus-narrative term preserves the product intent without asserting that
human cognition is continuously verbal or that generated language faithfully
reports either human or model-internal reasoning.

## Alternatives

- **Store and retrieve prose chunks.** Rejected as the V1 computational model
  because it delegates relevance, authority, and prioritization to the
  renderer and encourages a context dump.
- **Represent every memory by one semantic embedding.** Rejected because roles,
  exact values, conflicts, goals, time, and social perspective become
  inseparable and lossy.
- **Store only dense vectors.** Rejected because exact values, deletion,
  provenance, authority, and faithful support cannot be reconstructed
  reliably.
- **Use only symbolic triples or relational rows.** Rejected as the complete
  computational model because fuzzy cue matching and cross-context similarity
  remain necessary. Exact relational structures remain part of the
  authoritative plane.
- **Average all selected memory vectors into one vector.** Rejected because it
  loses roles and support bindings and can erase conflict through destructive
  interference.
- **Let the renderer select memories and infer priorities.** Rejected because
  it makes the untrusted generative stage the policy and retrieval authority.
- **Make every recent or frequently accessed memory dominant.** Rejected
  because base availability is only one cue and would create popularity loops.
- **Persist activation and compile-time consolidation automatically.** Rejected
  because the compile operation is read-only.
- **Select an action inside V1.** Rejected because the accepted product returns
  focus text and does not own downstream reasoning or execution.
- **Claim a brain simulation or human chain of thought.** Rejected because the
  cited cognitive models are partial and contested functional accounts, not an
  implementation blueprint or evidence of biological equivalence.

## Consequences

Future memory-management work must define how authoritative propositions,
exact values, source bytes, and derived facets are created, corrected, exported,
deleted, and rebuilt. A physical database engine and schema remain separate
choices, but no conforming compute path may treat stored prose as the activation
state or discard the exact support required by this decision.

Signal derivation, associative propagation, consolidation, and plan selection
each require independent specifications, simple baselines, counterexamples,
and frozen evidence. The accepted architecture selects what must be represented
and where decisions belong; it does not validate the selected mathematics or
parameter values.

The renderer receives a bounded numerical plan rather than the memory universe.
This creates a concrete adapter obligation addressed by Decision 0013 and
allows renderer failures to be isolated from retrieval and planning failures.

Nemosyne may be cognitively inspired while remaining operationally testable.
No release may claim human-like memory, thought, consciousness, action choice,
or general cognitive fidelity from this decision.
