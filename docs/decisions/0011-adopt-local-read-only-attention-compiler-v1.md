# 0011: Adopt a local read-only attention compiler for V1

Status: Superseded
Date: 2026-07-23
Superseded by: 0014-adopt-memory-grounded-predictive-attention.md

## Context

The repository contains a deterministic numeric activation kernel, an offline
parameter evaluator, and a small synthetic coding-agent evidence corpus. These
components deliberately begin after situation-dependent signals have already
been derived and end before selected information becomes an attention text.
They do not yet define a Nemosyne product.

The broader project intent is to make relevant memory available to an AI
system without rewriting the user's request or dumping raw history into its
context. Before selecting storage, retrieval, representations, models, or
process topology, the project needs one stable target that those later designs
can be evaluated against.

The primary V1 user is one local user integrating Nemosyne with an AI agent.
The invocation contract may also be used by an application, function, or
terminal user. The caller can already describe the immediate situation more
accurately than a first version could infer it by observing arbitrary external
systems. The user also requires persistent memory to remain local and the
compile path to use that memory without changing it.

Human memory provides inspiration for context-dependent activation across
experiences, but not an implementation blueprint. In particular, a current
project is a strong retrieval cue, not a sufficient reason to make all memories
from other projects inaccessible.

## Decision

Define Nemosyne V1 as one local, read-only attention-compilation operation.

The operation accepts:

- an immutable original user prompt;
- zero to three concise caller-supplied situation statements; and
- explicit metadata including the current time and optional contextual cues
  such as location, application, workspace, or project.

The situation statements are an external V1 convenience. After ingress,
relevance computation uses structured numerical state composed of typed
vectors, scalars, and numerical relations rather than prose state objects.
Exact encoders, dimensions, numeric schemas, and the parallel retention of
lossless source data remain open for the architecture phase.

It reads one persistent, user-owned local memory database representing one
logical memory universe and returns exactly one compiled text:

```text
attention:
{attention text}

user prompt:
{original user prompt}
```

The embedded original prompt is byte-identical to the input. The attention
text describes the current focus and relevant background; it does not answer
the prompt, invent unsupported information, or reproduce raw memory records.
Empty attention is valid when neither request context nor authorized memory
supports useful additional focus.

Compilation observes one immutable logical revision covering memory records,
provenance, validity, supersession, and authorization state. It does not mutate
memory and does not invoke the target AI system. It performs no autonomous
environment discovery and requires no network service. Memory-management
operations are separate contracts and are not defined by this compile decision.

Treat project, workspace, application, time, location, and related metadata as
relevance cues. Do not use them as implicit hard memory partitions. All
authorized memories remain logically eligible; explicit privacy, consent, and
access-control rules remain hard boundaries.

V1 assumes one trusted local user principal. The local user authorizes the
caller to receive attention derived from authorized memory. Multi-user and
delegated-authorization models remain outside V1. The caller owns any later
disclosure of the compiled prompt to a local or remote AI system.
The caller preserves the authentic user-prompt origin and must not submit the
compiled text with greater downstream authority than the original prompt.

Keep the invocation surface domain-independent, but limit the initial supported
and validated product claim to coding-agent use. Other domains may call the
same interface experimentally but require independent evidence before they are
claimed as supported.

Maintain the complete observable target in
[`v1-product-contract.md`](../specifications/v1-product-contract.md). Keep that
specification `Proposed` until an implementation exists. Subsequent decisions
must define architecture, mathematics, data contracts, proof obligations, and
evaluation. They may refine the open observable limits recorded by the
specification but must not silently broaden or weaken its core boundary.
Implementation and contract tests permit only `Experimental` status;
`Validated` requires the sealed end-to-end evidence and every predeclared
release gate in the specification.

## Rationale

A single compile operation gives terminals, functions, applications, and AI
agents one consistent integration model. Returning text rather than invoking a
model keeps Nemosyne independent from the caller's AI provider and message
transport.

Caller-supplied situation statements make the first system useful without
premature repository, browser, operating-system, or application sensors. A
three-statement limit keeps the boundary focused while metadata carries
structured context that should not depend on prose.

Read-only compilation makes a request reversible and testable. It prevents an
ordinary prompt from silently changing persistent memory and allows future
memory lifecycle operations to receive their own provenance, consent, and
correction contracts.

One logical memory universe preserves cross-context recall. Treating context
associations as activation cues allows a memory from another project to become
relevant without requiring a full physical scan. Authorization remains a
separate prior boundary because cognitive relevance cannot grant access.

Local persistence gives the user ownership of the memory store and prevents
the compiler itself from requiring disclosure to a cloud service. It is not,
by itself, a privacy or security guarantee. The trusted caller still controls
whether the final compiled prompt is sent to a local or remote AI system.

Defining observable behavior before internal architecture prevents current
kernel experiments, corpus-local channels, candidate technologies, or
human-memory analogies from becoming unsupported product commitments.

## Alternatives

- **Make the interface coding-agent-specific.** Rejected because the compile
  contract is domain-independent. The initial supported and validated claim is
  nevertheless limited to coding-agent use.
- **Partition memory strictly by project or workspace.** Rejected as the
  general relevance policy because it prevents useful cross-context recall.
  Explicit authorization partitions remain valid.
- **Infer the complete situation from repositories, browsers, tools, and the
  operating system.** Deferred because it would multiply integrations before
  the compilation hypothesis is tested.
- **Combine compilation with automatic memory writes.** Rejected because a
  normal prompt must not silently modify persistent user memory.
- **Return retrieved records or a raw context dump.** Rejected because the
  product output is a concise description of focus, not unprocessed storage
  content.
- **Rewrite or summarize the original prompt.** Rejected because the user's
  request is immutable input and must remain inspectable.
- **Invoke the downstream AI model inside Nemosyne.** Rejected because model
  selection and transport belong to the caller.
- **Require a cloud memory or rendering service.** Rejected for V1 because
  persistent memory and compilation are local.
- **Make a structured trace the primary successful output.** Rejected because
  the required integration result is one complete text. A separate diagnostic
  or test boundary must preserve claim-to-source evidence without changing
  that result.
- **Select SQLite, embeddings, vector-symbolic representations, Qwen, MLX, or
  another architecture now.** Deferred until data, retrieval, rendering,
  mathematical, and evaluation contracts are specified.
- **Claim biological fidelity.** Rejected because human memory is inspiration,
  not evidence that the software reproduces a brain.

## Consequences

The next phase can design the complete architecture against an accepted core
boundary instead of deriving product behavior from implementation convenience.
It must specify the memory lifecycle, retrieval and activation mathematics,
representation boundaries, rendering, trust model, proof obligations,
evaluation suites, and release thresholds.

V1 does not automatically observe the caller's environment, invoke an AI
model, or learn from compilation calls. Callers must provide the immediate
situation and metadata explicitly.

Cross-context memory is allowed by default after authorization. The eventual
retrieval architecture must therefore support efficient candidate generation
without treating convenience indexes as semantic access boundaries.

One compile call must observe one immutable logical memory revision. The later
storage contract must provide this consistency without being forced to use a
particular database or transaction implementation.

Local persistence creates future obligations for storage permissions,
encryption decisions, migrations, corruption handling, inspection, export,
correction, and deletion. This decision does not yet select their
implementation.

The single text result constrains user-facing integration but does not prohibit
internal structured states, explanations, or offline evaluation artifacts.
Those interfaces require later decisions. The textual headers are presentation,
not a parseable transport or an authority boundary; callers must treat the
compiled prompt as one outbound text.

This decision establishes a target, not implementation or validation evidence.
The product specification remains proposed, and the existing numerical
components do not yet satisfy the end-to-end contract.
