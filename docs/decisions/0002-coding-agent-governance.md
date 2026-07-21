# 0002: Establish coding-agent governance

Status: Accepted
Date: 2026-07-21

## Context

Coding agents need concise repository-specific instructions that preserve Nemosyne's documentation contracts and verification requirements. Unchecked or conflicting instruction files could let that workflow drift.

## Decision

Maintain one repository-wide `AGENTS.md` at the root. Continuous integration requires its core sections, rejects additional agent instruction files, and requires a new decision record whenever documentation-governance files change.

## Rationale

A single root file gives agents one consistent workflow while keeping specifications and decision records authoritative. Requiring an explicit decision for governance changes makes policy evolution visible and reviewable.

## Alternatives

Relying only on contributor documentation would not provide persistent agent instructions. Allowing unrestricted nested files would make effective guidance harder to audit. Freezing the root file by hash would make legitimate maintenance unnecessarily rigid.

## Consequences

Future governance changes carry the cost of a decision record. Directory-specific agent guidance is unavailable until its precedence and validation rules are defined. Continuous integration can enforce structure and declared process, but not the semantic quality of instructions or resistance to an authorized contributor changing every guard together.
