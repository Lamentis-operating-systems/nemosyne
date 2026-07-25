# 0019: Establish the render domain and bounded validation

Status: Superseded
Date: 2026-07-24
Superseded by: 0034-adopt-vector-conditioned-focus-adapter-boundary.md

## Context

The proposed renderer path left deterministic lexicalization, candidate type
ownership, slot-error precedence, and validation recomputation partially
implicit. The independent validator needs candidate and view types without
depending on the renderer implementation or receiving a raw plan. Re-running
semantic query encoding during validation would add cost and a second semantic
interpretation of one request.

## Decision

Package `REN-01` first freezes an accepted lexicalizer and render-domain
contract before adding renderer source. A dependency-light
`nemosyne-render-domain` crate owns opaque rendered and substituted candidate
types, validation-view interfaces, plan and renderer identities plus exact
canonical commitments, the authenticated slot registry, and checked
constructors. Renderer implementations emit untrusted drafts into those
constructors. The validator depends on the render domain and authenticated
configuration domain, never on a renderer implementation, compiler-private
context, or raw plan.

`UnknownSlot` means an emitted identifier has no authenticated registry record.
`ForbiddenSlot` means the record exists with the registry-wide `Forbidden`
disposition. A registry-authorized identifier that is absent from, or not
permitted by, the selected plan is instead a `SlotBindingMismatch`; a selected
binding whose promised surface cannot be resolved is
`ExactSurfaceUnavailable`. The eleven substitution classes have fixed class
priority; within a class the smallest canonical evidence key wins. Input order
is not observable.

`buildValidationContext` does not invoke the situation binder, semantic encoder,
or query construction again. It revalidates the retained request and canonical
ingress/query binding against the sealed query projections. It independently
recomputes the canonical plan envelope where required by Decision 0016, with
exactly two bounded passes, canonical input bytes, checked
byte/time/peak-space ceilings, and cancellation points.

The validation identity is always
`(PlanContentId, PlanCanonicalCommitment, RendererConfigurationId,
RendererCanonicalCommitment)`. Selectable latent-query counts are exactly
`8`, `16`, and `32`; `64` is diagnostic stress-only until a later decision.

## Rationale

A shared opaque domain keeps ownership narrow without coupling the validator to
generation code. Registry existence and authorization are different facts and
need different errors. Revalidation should prove integrity of retained
artifacts, not reinterpret semantics. Explicit bounds make the independent
collision checks measurable.

## Alternatives

- Let the validator depend on the renderer implementation. This weakens
  independence and creates a cyclic ownership boundary.
- Expose public candidate constructors. This permits forged validated-looking
  artifacts.
- Re-encode the request during validation. This duplicates semantic work and
  can disagree with the original sealed query.
- Treat `64` latent queries as selectable without qualification. This widens
  the candidate family without a frozen multiplicity or resource contract.

## Consequences

Renderer, validator, compiler, proof, performance, and delivery contracts must
use the shared render-domain boundary. `REN-01` contains an ADR/specification
checkpoint before source. Tests must exercise multi-invalid permutation,
identity commitment, no-reencoding, bounded double-pass, and dependency
isolation.
