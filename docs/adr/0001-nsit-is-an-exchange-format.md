# ADR 0001: NSIT is an exchange format

- **Status:** Accepted
- **Date:** 2026-07-20
- **Owners:** @EliasPapavlassopoulos
- **Supersedes:** None

## Context

AI systems can spend substantial output tokens reproducing source-level syntax and
repeated structural context. Text minification removes useful names and relationships;
generic syntax trees are often verbose. We want to test whether a stable identity model
and compact notation can reduce total model tokens while still producing ordinary,
reviewable source changes.

Making NSIT the authoritative source immediately would require developers, builds,
debuggers, and release tooling to depend on an unproven format. It would also make
recovery from incomplete adapters or information loss harder.

## Options

1. **Keep conventional source only.** Lowest adoption cost, but no dedicated compact
   representation for model interaction.
2. **Use NSIT as a derived exchange format.** Conventional source remains authoritative;
   NSIT is generated for model tasks and decoded into a normal source diff.
3. **Make NSIT authoritative.** Conventional source becomes a generated projection of
   NSIT.

## Decision

NSIT begins as a **derived exchange format**.

Language-specific adapters map a declared source subset into a canonical program model
and compact NSIT view. AI-authored NSIT changes must validate and become an ordinary
source diff. Native parsers, compilers, tests, and repository review decide whether the
change is accepted. Unsupported constructs fail explicitly rather than being guessed
or discarded.

NSIT artifacts may be cached or inspected, but the conventional source, build
configuration, and tests remain authoritative. This decision does not choose the first
source language, concrete NSIT syntax, tokenizer, model, or exact round-trip fidelity
level.

## Consequences

### Benefits

- Projects can evaluate NSIT without migrating their source of truth.
- Every proposal remains inspectable with existing language and Git tooling.
- Failure or removal of NSIT does not strand the codebase.
- Token-efficiency claims can be compared directly with existing workflows.

### Costs and limits

- Each supported language needs a precise adapter and explicit unsupported cases.
- The original source still exists, so NSIT is not storage compression.
- Schema, symbol tables, prompts, retries, and reconstruction are part of total cost.
- Compilation and tests reduce risk but do not prove full behavioral equivalence.
- Exact comments, formatting, macros, reflection, or generated code may require fidelity
  metadata or remain outside an initial subset.

## Validation

Before claiming a useful result, a proof of concept must publish:

- the supported language and syntax subset;
- corpus and task definitions;
- NSIT schema, adapter, and version;
- token counts for named tokenizers, including all required context and retries;
- original-source, minified-source, and conventional-AST baselines;
- structural round-trip outcomes and known information loss;
- native parse, build, test, and source-diff results; and
- failure cases, not only successful examples.

The decision should be revisited only if evidence shows that making NSIT authoritative
would provide a material benefit that cannot be achieved safely as an exchange format.
