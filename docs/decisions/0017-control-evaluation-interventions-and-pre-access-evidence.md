# 0017: Control evaluation interventions and pre-access evidence

Status: Accepted
Date: 2026-07-24

## Context

The V1 proof program uses deliberately different attention conditions to test
product headroom and expectation effects before implementation and before
sealed release evaluation. The original contract did not define a
constructible equal-length neutral carrier, omitted a matched-size placebo
from G1, required a deliberately wrong expectation to cause harm, and allowed
rejected-attempt evidence to retain raw input bytes. It also described absence
of outcome access and arithmetic without a concrete custody witness.

## Decision

G1 uses a closed seven-condition design with prompt-only, situation-only,
matched-size no-memory placebo, focus with a neutral expectation carrier,
correct expectation, deliberately wrong expectation, and explicit abstention.
The four focus-derived conditions use one frozen tokenizer, placement, focus,
carrier grammar, and exact attention-token count. The neutral carrier contains
no expectation proposition.

Focus headroom must beat both the prompt and situation baselines and the
matched-size placebo under the frozen multiplicity procedure. A deliberately
wrong expectation remains a negative control, never a product configuration.
It must be distinguishable from correct and abstaining conditions, but the
protocol must upper-bound its harm, anchoring, and leakage; it must never
require a positive minimum of harm or anchoring.

Every mandatory cohort, model family, bridge variant, latent-query count, seed,
precision pair, language, gate, and fallback stage is covered by one
pre-outcome family-wise or sequential error-control procedure.

A rejected pre-access attempt retains no raw attempted bytes or parse prefix.
It records only typed attempt/stage/field data, a digest and length for the
complete input when available or consumed prefix otherwise, completeness, and
allowlisted identities established before rejection. The digest-only
commitment can reveal equality and enable dictionary verification for
predictable inputs; it is protected evidence for integrity correlation, not a
confidentiality mechanism or a claim that no information about the input
remains. A custodian-signed guard witness binds the sealed source,
capability-issuance state, and append-only outcome-access and analysis-job
ledger boundaries. This is conditional evidence that no outcome capability or
outcome-dependent analysis was admitted under the declared custody system, not
a universal proof that no arithmetic occurred.
A G1 access grant also binds a complete signed G1 execution record before any
outcome becomes available.

The proof program is the sole normative owner of the G1 and G9 condition,
estimand, threshold-domain, evidence, and access contracts. Delivery documents
reference stable proof anchors and state operational consequences without
copying their mathematics.

## Rationale

Matched controls isolate semantic value from token length and generic framing.
Upper-bounded negative controls test sensitivity without making unsafe behavior
a success criterion. Digest-only commitments and explicit ledger witnesses
retain correlation evidence without persisting raw input; the digest leakage
above remains within controlled evidence custody. Neither mechanism claims
stronger proof than the declared custody system can provide. One normative
mathematical owner prevents drift.

## Alternatives

- Keep six G1 conditions and compare focus only with shorter baselines. This
  cannot separate focus value from attention length or framing.
- Require wrong expectations to increase harm. This makes unsafe behavior a
  release-enabling result.
- Store rejected raw bytes for audit. This unnecessarily persists secrets and
  untrusted payloads.
- Duplicate formulas in the delivery program. This creates competing normative
  owners.

## Consequences

G1 manifests, fixtures, receipts, and gates must support seven conditions and a
neutral carrier. Existing thresholds remain unresolved until prospectively
frozen. Evidence tooling must implement the commitment and guard-witness
contracts before opening outcomes. Historical conformance records remain
unchanged and a new current-source record documents the correction.
