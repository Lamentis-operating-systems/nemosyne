# NSIT

**Naome System Identity & Typography**

NSIT is an open, experimental exchange format for software. It tests whether AI
systems can read and propose changes to conventional source code with fewer total
model tokens by using a compact, canonical representation. The conventional source
remains the authoritative codebase.

> **Status: research / pre-alpha.** This repository currently defines the product
> contract and evaluation criteria. It does not contain a working encoder, decoder,
> supported language adapter, or released format. No token-reduction or semantic-
> preservation result has been demonstrated yet.

## The problem

Source code is written for humans, compilers, and existing tools. When an AI system
generates or edits it, output tokens are spent repeatedly on formatting, punctuation,
long identifiers, type syntax, imports, and structural context.

Minification shortens text but also removes information that helps models reason about
software. Generic AST formats preserve structure but are usually verbose. NSIT explores
a different tradeoff: preserve the program identities and relationships needed for a
change, encode them once, and express edits in a compact machine-oriented typography.

That is a hypothesis to measure, not a claim that all source code can be compressed or
that fewer characters always mean fewer model tokens.

## What “exchange format” means

NSIT sits between a normal codebase and an AI system:

```text
authoritative source
        │
        ▼
language adapter ──► canonical program model ──► compact NSIT view
                                                        │
                                                        ▼
                                                   AI proposal
                                                        │
                                                        ▼
native source diff ◄── validated NSIT change ◄── NSIT validator
        │
        ▼
compiler, tests, and repository review
```

The source files, build configuration, and tests remain authoritative. An NSIT document
is derived for a task and may be discarded afterward. An AI proposal is never accepted
because it is syntactically valid NSIT; it must be projected back into ordinary source
and reviewed with the language's native tools.

For example, a language adapter might represent a conventional function conceptually
as:

```text
fn invoiceTotal(items:list<Item>)->Money = sum(items.price)
```

This is illustrative only; it is not accepted NSIT syntax.

## Identity and typography

The name describes the two intended responsibilities:

- **System Identity** concerns how modules, types, symbols, fields, and relationships
  can be referenced without repeating their full source spelling. The required
  stability of those references remains a design question.
- **Typography** concerns a compact, deterministic notation for those identities and
  the operations that inspect or change them.

Compactness must not come from silently deleting information required to understand,
reconstruct, or validate a change.

## Initial contract

The first useful NSIT implementation must preserve these boundaries:

1. **Conventional source is authoritative.** NSIT is not the primary programming
   language or the only copy of a codebase.
2. **Adapters are language-specific.** Unsupported syntax must fail explicitly rather
   than be guessed or dropped.
3. **Changes are reversible and inspectable.** A proposed NSIT change becomes a normal
   source diff before acceptance.
4. **Verification uses native tools.** Parsing, type checking, compilation, tests, and
   repository policy remain part of the decision.
5. **Token savings include overhead.** Measurements include the schema, symbol tables,
   task context, model output, retries, and repair prompts needed to use NSIT.
6. **Claims are evidence-bound.** Support, equivalence, and compression claims name the
   language subset, corpus, tokenizer, model, and NSIT version that were measured.

The accepted rationale is recorded in
[ADR 0001](docs/adr/0001-nsit-is-an-exchange-format.md).

## What NSIT is not

NSIT is not currently:

- a general-purpose programming language;
- a source-code minifier or archive format;
- a universal cross-language transpiler;
- a replacement for Git, compilers, tests, or code review;
- proof that two programs have identical behavior; or
- a sandbox for executing untrusted code.

## How success will be evaluated

A proof of concept should start with one language and a declared syntax subset. It
must compare NSIT with the original source, minified source, and a conventional AST
encoding on the same tasks.

At minimum, results should report:

- input and output tokens for named tokenizers;
- all NSIT context required by the model;
- successful structural round trips for the supported subset;
- native parse, build, and test results after reconstruction;
- task success and repair attempts for AI-authored changes;
- diff locality for small edits; and
- information intentionally not preserved.

A smaller payload that causes more failed edits or hidden reconstruction cost is not a
successful result.

## Project documents

- [ADR 0001](docs/adr/0001-nsit-is-an-exchange-format.md) — why NSIT begins as an
  exchange format rather than a source of truth
- [Contributing](CONTRIBUTING.md) — contribution and evidence expectations
- [Security](SECURITY.md) — private vulnerability reporting and current non-claims
- [Governance](GOVERNANCE.md) — decision authority during pre-alpha
- [Code of Conduct](CODE_OF_CONDUCT.md) — community standards and reporting

## License

Apache License 2.0. See [LICENSE](LICENSE).
