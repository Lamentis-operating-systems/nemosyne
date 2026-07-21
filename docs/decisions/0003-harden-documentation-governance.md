# 0003: Harden documentation governance

Status: Accepted
Date: 2026-07-21

## Context

The initial checker enforced the intended workflow but left gaps around renames, nested records, supersession links, immutable history, and local verification.

## Decision

Use flat specification and decision directories containing only regular Markdown files. Classify both sides of renames, treat every GitHub Actions workflow as governance, permit only the exact metadata transition when superseding an accepted decision or existing specification, validate replacement chains, and require a new accepted decision for governance changes. Keep structural validation separate from change-aware validation against a committed branch and pull request body.

## Rationale

These rules make the documented invariants match the cases enforced by continuous integration while keeping the workflow deterministic and local.

## Alternatives

Recursive documentation would require identical path semantics throughout every validator. Heuristic history comparisons and existence-only replacement checks would remain ambiguous. Treating the structural check as pull-request validation would continue to hide missing change context.

## Consequences

Supersession requires an explicit replacement created in the same change. Documentation records cannot be nested or represented by symbolic links. Contributors must commit their work before running the local change-aware check.
