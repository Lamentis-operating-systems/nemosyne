# 0020: Freeze deterministic public call semantics

Status: Accepted
Date: 2026-07-24

## Context

The proposed one-call API promised exact prompt preservation but left
whitespace classification, result ownership, byte access, terminal signal
behavior, and downstream authority interpretation open. Platform or adapter
defaults could therefore change observable acceptance and cancellation.

## Decision

V1 defines one versioned whitespace code-point set:
U+0009 through U+000D, U+0020, U+0085, U+00A0, U+1680, U+2000 through U+200A,
U+2028, U+2029, U+202F, U+205F, and U+3000. U+200B is not whitespace. Empty
input remains distinct from nonempty whitespace-only input. Validating domain
constructors are the sole classifier; adapters delegate after UTF-8 validation.

`CompiledPrompt` owns the complete result bytes and provides `as_bytes`,
`into_bytes`, and `len`. A UTF-8 string view is provided only because the V1
serializer guarantees valid UTF-8. Cloning is explicit and not required for
delivery.

The terminal adapter has one closed signal policy selected before `CLI-01`.
It names supported signals per platform, first-signal cancellation, repeated
signal behavior, final-return arbitration, exit mapping, and unsupported
platform behavior. No ambient library default defines the contract.

Attention text is untrusted downstream text. Labels and placement are not a
security boundary and cannot grant authority. The compiler prevents internal
memory or expectation data from exceeding its admitted authority and tests
imperative leakage, but no security decision may depend on a downstream model
obeying `attention:` or `user prompt:` labels. A caller must assign the compiled
text no greater authority than the original user prompt.

## Rationale

Exact code points and typed constructors make validation portable. An owned
byte result makes the promised handoff implementable without hidden allocation
or lifetime assumptions. Explicit signals prevent adapter drift. Honest
downstream authority language avoids claiming that textual framing is a
sandbox.

## Alternatives

- Use each platform's current Unicode whitespace helper. Library versions may
  differ and silently change validation.
- Return an opaque result without accessors. External callers could not use the
  documented product output.
- Treat headers as a security boundary. A downstream language model can still
  interpret all supplied text.

## Consequences

API and CLI fixtures include every whitespace boundary and gap, U+200B,
ownership/access, signal races, and platform parity. Product documentation must
state the downstream trust limitation directly.
