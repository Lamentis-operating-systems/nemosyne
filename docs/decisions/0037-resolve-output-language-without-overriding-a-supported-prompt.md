# 0037: Resolve output language without overriding a supported prompt

Status: Accepted
Date: 2026-07-25

## Context

The product contract requires attention text to use the original prompt's
language and permits an explicit output-language field for an unsupported,
undetermined, neutral, or mixed-language prompt. The reference API instead
stated that any supplied `LanguageTag` selects the output language. A caller
could therefore supply a German tag with a clearly resolved English prompt and
receive two incompatible specified outcomes.

## Decision

`LanguageTag` construction validates only the versioned intrinsic BCP 47
syntax owned by the public request schema. It neither consults the installed
language schema nor establishes support. After authenticated language-schema
preflight, the compiler runs the pinned language resolver exactly once over
the original prompt, even when an explicit tag is present, and separately
canonicalizes that syntactically valid tag to a declared language identity or
classifies it as unsupported. Identity comparison is exact after that
canonicalization; no component introduces an ambient dialect, locale, or
fallback-equivalence rule.

Resolution uses this total table:

| Prompt resolver result | Explicit tag | Result |
| --- | --- | --- |
| exactly one supported identity `p` | absent | `p` |
| exactly one supported identity `p` | supported identity `p` | `p` |
| exactly one supported identity `p` | different supported identity | `RequestIncompatible` |
| exactly one supported identity `p` | syntactically valid but unsupported identity | `UnsupportedLanguage` |
| unsupported, undetermined, neutral, or mixed | one supported identity `e` | `e` |
| unsupported | absent | `UnsupportedLanguage` |
| undetermined, neutral, or mixed | absent | `UnsupportedLanguage` |
| unsupported, undetermined, neutral, or mixed | syntactically valid but unsupported identity | `UnsupportedLanguage` |

A syntactically malformed tag remains an intrinsic `CompileRequestError`.
After authenticated language-schema preflight, the compatibility boundary
retains these closed typed sources:

- `ExplicitLanguageConflictsWithPrompt` maps to `RequestIncompatible`, CLI
  exit `5`;
- `RequestedLanguageUnsupported`, `PromptLanguageUnsupported`, and
  `PromptLanguageNotUniquelyResolved` map to `UnsupportedLanguage`, CLI exit
  `2`.

Source precedence is total. A present syntactically valid but unsupported
explicit tag selects `RequestedLanguageUnsupported`, irrespective of the
prompt resolver result. Otherwise a different supported explicit identity
against one supported prompt identity selects
`ExplicitLanguageConflictsWithPrompt`. With no explicit tag,
`PromptLanguageUnsupported` applies only to an unsupported prompt result and
`PromptLanguageNotUniquelyResolved` applies only to an undetermined, neutral,
or mixed result. A supported explicit fallback makes either prompt-result
class successful. A missing, unauthenticated, or corrupt language schema is
`ArtifactUnavailable` before these compatibility outcomes, not a
language-support result. Neither language error may be relabeled as a
planning or renderer failure.

The selected `ResolvedOutputLanguage` controls generated attention only. It
never translates, normalizes, or rewrites the retained original prompt.
Only the original prompt enters detection; situation statements, metadata,
memory, process locale, and ambient platform state cannot change the resolver
result.
Planning, adapters, optional decoding, validation, serialization, and the CLI
consume the sealed result and perform no second detection, support lookup,
override, or fallback.

## Rationale

The table preserves the product promise for a clearly supported prompt while
keeping the explicit field useful when the prompt itself cannot select one
supported language. Exact canonical identity makes the rule reproducible and
prevents platform locale behavior from changing output.

## Alternatives

- **Always let the explicit tag override the prompt.** Rejected because it
  contradicts the same-language product contract and can silently switch the
  focus text away from the user's clear language.
- **Always ignore the explicit tag.** Rejected because neutral, mixed, or
  unsupported prompts would lose the declared compatibility path.
- **Treat every conflict as `UnsupportedLanguage`.** Rejected because both
  identities may be supported; the problem is incompatibility, not support.
- **Use loose primary-subtag equivalence.** Rejected because it leaves dialect
  and locale precedence dependent on an unspecified library policy.

## Consequences

Product, API, CLI, proof, and delivery contracts must use the same table.
Golden tests cover every row, the complete source precedence, intrinsic
syntax versus installed support, canonical-equivalent tags,
canonical-distinct supported tags, malformed and unsupported tags, neutral
and mixed prompts, prompt-byte preservation, single resolver invocation, and
the absence of downstream language re-resolution.

Because this accepted public-contract refinement changes reviewed source and
the governed finding and decision inventories, the same revision appends
`DOC-CONF-26` and advances only the corresponding checker constants and
regression fixtures. It does not select a concrete language detector, supported
language set, renderer, or empirical multilingual threshold.

This decision refines Decisions 0018 and 0020 only by totalizing the
pre-planning language-resolution boundary. Their canonical planning identity,
intrinsic request, byte-preservation, cancellation, and downstream-authority
rules remain unchanged.
