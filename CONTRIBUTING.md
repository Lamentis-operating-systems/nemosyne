# Contributing

NSIT is pre-alpha. Contributions should make the product contract clearer or test a
specific part of the exchange-format hypothesis; volume of code is not a goal.

## Workflow

1. Search existing issues and Architecture Decision Records.
2. Discuss material format, authority, compatibility, or security changes before
   implementing them.
3. Work on a focused branch and open one coherent pull request.
4. State what was implemented, how it was verified, and what is not being claimed.
5. Let maintainers merge through the protected `main` branch.

Use private vulnerability reporting as described in [SECURITY.md](SECURITY.md); never
publish an unresolved vulnerability in a contribution.

## Evidence

Claims must name their scope and validation method. In particular:

- token claims include tokenizer, model, corpus, prompts, schema and symbol-table
  overhead, retries, and baseline;
- round-trip claims identify the supported language subset and information not
  preserved;
- behavior changes include tests that fail without the change;
- performance claims include a reproducible workload; and
- generated artifacts identify their authoritative input and regeneration command.

Passing CI is necessary but does not by itself establish semantic equivalence, safety,
or useful compression.

## Architecture decisions

Add or update an ADR for material changes to the NSIT contract, notation, identity
model, language adapters, trust boundaries, compatibility, verification, or release
policy. Follow [docs/adr/README.md](docs/adr/README.md).

## Dependencies and generated code

Justify every new dependency: purpose, maintenance, license, transitive/build effects,
authority, and removal cost. Pin or lock versions where the chosen ecosystem supports
it.

Do not manually edit a repository-tracked generated projection. Commit its
authoritative input, regeneration path, and output together. Temporary NSIT views and
proposed changes may be discarded unless a task or benchmark contract explicitly
requires retention. Generation must not embed secrets, local paths, or unexplained
nondeterminism.

## Contribution responsibility and rights

AI assistance is allowed. The contributor remains responsible for understanding,
licensing, reviewing, and independently validating the complete change. AI-generated
implementation and tests derived from the same assumptions are not independent
evidence.

Do not submit confidential data or content you do not have the right to contribute.
Unless explicitly stated otherwise, intentional contributions are submitted under the
repository's Apache-2.0 license as described by that license's contribution terms.

All participants must follow the [Code of Conduct](CODE_OF_CONDUCT.md).
