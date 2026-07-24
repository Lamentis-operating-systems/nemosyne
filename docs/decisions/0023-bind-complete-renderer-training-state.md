# 0023: Bind complete renderer training state

Status: Accepted
Date: 2026-07-24

## Context

The renderer contract identified a trained numerical bridge, a derived model
with learned appended exact-slot rows, and an optional LoRA adapter. It did not
state a disjoint ownership partition for those trainable tensors or require
qualification evidence to bind their complete composition. Materially
different learned state could therefore appear to share one renderer
configuration or reuse evidence.

## Decision

Renderer training artifacts have disjoint tensor ownership:

- \((e_s,r_{e_s})\) owns the derived model artifact, including the trained
  appended exact-slot input rows and, when untied, output rows. Original model
  rows remain byte-identical to the pinned base revision.
- \((\phi_s,r_{\phi_s})\) owns the complete numerical bridge checkpoint,
  including every learned projector, categorical embedding, normalization,
  latent query, resampler, language-conditioning, and attribution tensor. It
  contains no model vocabulary row.
- \((\Delta_s,r_{\Delta_s})\), when present, owns only the permitted LoRA
  updates and contains no base-model, derived-row, or bridge tensor.

Each artifact carries a canonical tensor-name inventory with shape, dtype, and
trainability or freeze disposition. Authenticated \(K_R\) binds all applicable
members of the composite
\((e_s,r_{e_s},\phi_s,r_{\phi_s},\Delta_s,r_{\Delta_s})\), plus the base model
and tokenizer identities. Descriptive architecture fields cannot substitute
for these content-identified artifacts.

Qualification applies to that exact composite. Adding, deleting, renaming,
reshaping, retyping, moving between owners, or changing any trained tensor
creates a different \(K_R\), `RendererConfigurationId`, candidate
configuration, and qualification obligation. Partial, reconstructed,
overlapping, or differently partitioned artifacts cannot reuse prior
qualification evidence.

## Rationale

Disjoint ownership makes the complete learned state enumerable and prevents
both omission and double ownership. Binding the composite into the renderer
configuration makes deterministic replay and qualification claims refer to
the same immutable bytes.

## Alternatives

- Put every trained tensor into one monolithic checkpoint. This would erase
  the existing model, bridge, and optional-LoRA lifecycle boundaries.
- Duplicate appended slot rows in both the derived model and bridge
  checkpoint. This would require a byte-equality join and retain two owners for
  one tensor.
- Bind only architecture descriptors and revisions. Those values do not prove
  which learned bytes were executed.

## Consequences

Training, packaging, loading, qualification, and release manifests must
preserve the partition and bind the exact composite. Phase A and Phase B
implementations need negative fixtures for omitted, duplicated, moved, or
mutated tensors. This decision selects identity and evidence semantics only; it
does not qualify a model or choose learned weights.
