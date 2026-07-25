# 0038: Enforce adapter least privilege and query conditioning

Status: Accepted
Date: 2026-07-25

## Context

Decision 0034 separates adapter-visible numerical conditioning from
validator-only authority, provenance, and policy state. The active renderer
specification nevertheless placed the authority ceiling and
current-versus-historical usage control in each adapter-visible renderer item,
embedded that category in the latent-prefix candidate, and stated that
candidate families consume the complete conditioning composite.

The same candidate was called query-conditioned even though its defined item
projection and resampler had no mathematical path from \(Q_F\) or
\(\mathcal Q_F\). Holding the item matrix fixed therefore made the specified
candidate invariant under changes to the numerical query context. This also
left the query-only product path and query-removal ablation undefined.

## Decision

Every learned candidate consumes only `AdapterConditioningViewV1` \(C_A\) and
`AdapterConfigurationViewV1` \(C_R^A\). The complete \(C_F\) remains an
orchestrator-owned checked composite from which the disjoint adapter and
validator views are borrowed; it is not a model input.

The canonical authority ceiling \(\mathcal U_i\), current-versus-historical
usage policy, provenance, custody, exclusions, and other authority controls
remain exclusively in \(C_V^{sem}\), \(C_V^{bind}\), or `X_L`. They do not
enter the adapter-visible renderer item \(\rho_i\), any learned embedding,
feature tensor, decoder input, or adapter resource function. When chronology
is independently selected as renderable meaning, planning represents only its
non-authoritative temporal semantics through a typed temporal facet or
required qualifier \(\mathcal Q_i\). Independent validation continues to
enforce the authority ceiling.

`VF-LATENT-PREFIX-01` remains an unselected experiment candidate, but its
mathematical manifest becomes structurally query-conditioned. A registered
projection maps the one complete adapter-safe \(Q_F\) projection followed by
the canonical \(\mathcal Q_F\) entries into a finite cue matrix \(H_Q\).
Each cue row contains only typed numerical facets, finite scalars, presence
masks, and a closed safe query, task, request-evidence, or
situation-evidence role. It contains no text, prompt token, exact payload,
identity, provenance, authority, policy, or validator control.

The candidate concatenates cue rows before canonical renderer-item rows:

\[
H_C=
\begin{bmatrix}
H_Q\\
H_R
\end{bmatrix}.
\]

Its latent resampler cross-attends to \(H_C\) under one complete row-presence
mask. Masked rows are excluded before the softmax maximum and denominator.
The canonical cue sequence contains exactly the real \(Q_F\) and
\(\mathcal Q_F\) rows, without physical padding, and every real cue row is
present in product mode. Any nonfinite projected state or
scaled-dot-product score is rejected before normalization or output.
The query-projection schema fixes source-role and facet order, dimensions,
ranges, masks, dtype, accumulation, and numerical policy; \(K_R\) binds those
choices, every query projector and embedding, a finite cue-row ceiling, and
the complete candidate tensor inventory. Query rows never receive plan-item
handles and cannot become attribution or support-trace targets.

Product mode requires its checked \(Q_F\) cue row. Query-only evaluation keeps
\(H_Q\) and removes memory-origin rows. The memory-only condition is an
evaluation-only ablation that masks the complete cue branch before
normalization while freezing the remaining candidate. It diagnoses the
adapter's direct query path and does not make upstream planning
query-independent.

Candidate time and space functions consume only \(C_A\) and \(C_R^A\), count
cue rows separately, and use the combined row count for cross-attention.
Qualification must test query mutations while holding \(H_R\), its item mask,
all non-query \(C_A\) fields, and \(C_R^A\) fixed; complete evaluation-only
cue masking; canonical source-order invariance; query-only operation without
memory-origin evidence but with a nonempty request/situation renderer
projection; intermediate overflow rejection; and the prohibition on cue
attribution.

This decision refines Decision 0034 without superseding it. It does not select
`VF-LATENT-PREFIX-01`, a renderer, model, checkpoint, dimension, threshold, or
runtime.

## Rationale

Keeping authority outside the learned path preserves the intended separation
between relevance, truth, authorization, and lexicalization. A separately
validated output may preserve an authority ceiling without giving that policy
state to the generator as a semantic feature.

A dedicated cue matrix is the smallest candidate-local correction that
preserves multiple typed request/situation inputs, explicit masks, canonical
ordering, the query-only path, and a meaningful query-removal ablation. A
single pooled query addition would leave those obligations implicit.

## Alternatives

- **Embed the authority category but prohibit learned ordering.** Rejected
  because the category would still be a model feature and would contradict the
  least-privilege view.
- **Give every family the complete \(C_F\).** Rejected because it exposes
  validator-only state and makes the two-view boundary ineffective.
- **Call the latent-prefix candidate memory-conditioned only.** Rejected
  because it would not implement the candidate's declared query-only product
  path or the product hypothesis already selected by Decision 0034.
- **Add one pooled query vector to the initial latent state.** Rejected for
  this manifest because it obscures multiple cue rows, masking, ordering, and
  the direct query-removal ablation.

## Consequences

Renderer contract tests must prove adapter-view compile isolation, absence of
authority tensors and vocabularies, exact cue and combined-matrix shapes,
fixed ordering, mask-before-softmax behavior, product-mode query presence,
query-only operation, evaluation-only cue removal, rejection of masked real
cue rows and nonfinite intermediate states, and cue exclusion from attribution
and support traces.

The two frozen-source review findings are recorded as `FND-379` and
`FND-380`. Because this accepted contract refinement changes reviewed source
and the governed finding, conformance, and decision inventories, the same
revision appends `DOC-CONF-27` and advances only the corresponding checker
constants and regression fixtures. It does not rewrite Decision 0034,
rebaseline protected history, rename the stable `DOC-CONF-24` G0 record, or
claim implementation or empirical evidence.
