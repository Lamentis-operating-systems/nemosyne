# 0001: Establish documentation governance

Status: Accepted
Date: 2026-07-21

## Context

Nemosyne will contain mathematical algorithms and evolving research assumptions. Pull request discussions and code alone do not preserve their definitions, rationale, limitations, and verification requirements reliably.

## Decision

Keep current behavioral and mathematical contracts in versioned specifications, preserve significant choices as immutable decision records, document public Rust APIs with Rustdoc, and verify normative claims with tests where practical. Every pull request must declare its documentation impact. Continuous integration validates the document structure and requires a specification change whenever production Rust source changes.

## Rationale

This separates current truth from historical rationale while keeping both beside the implementation. Machine-enforced structure prevents silent omissions without introducing an external documentation system.

## Alternatives

A single architecture document would mix current contracts with historical reasoning. Pull request discussion alone is difficult to discover later. An external wiki can drift independently from the repository. Requiring a full RFC for every change would add unnecessary process.

## Consequences

Production Rust changes carry an explicit specification cost. Accepted decisions and superseded specifications remain available as history. Continuous integration can enforce structure and declared impact, but it cannot prove semantic correctness or resist an authorized contributor who deliberately weakens the checker itself; stronger adversarial protection would require an independently managed required workflow or mandatory review.
