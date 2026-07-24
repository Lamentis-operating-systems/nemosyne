# V1 product contract

Status: Proposed

## Purpose

This specification defines the intended product boundary for Nemosyne V1.
Nemosyne is a local attention compiler: one call combines an immutable user
prompt, a concise caller-supplied description of the current situation,
request metadata, and the user's local memory to produce one compiled text for
an AI system.

The primary V1 user is one local user integrating Nemosyne with an AI agent.
The job is to make the agent attend to response-changing situation and memory
context and, when observed transition evidence supports it, to qualified
competing expectations about the present or a later state. Nemosyne does this
without rewriting the user's request, dumping raw history, answering the
request in advance, or choosing an action. The invocation contract is
domain-independent, but the first domain eligible for a supported and
validated claim is coding agents. Use in other domains remains experimental
until separately evaluated.

This document specifies the wanted result and its observable requirements. It
does not make internal technology choices part of the stable product surface.
Decisions 0014 and 0015 select the current V1 implementation hypothesis:
numerical cognitive-memory activation, parallel focus and expectation
formation, a qualified combined plan, and evidence-based selection between a
deterministic lexicalizer and a local vector-prefix candidate. The linked
architecture specifications define that path without changing the single-call
result. No current implementation is claimed to satisfy this contract.

## Definitions

The **caller** is the local user or an AI agent, application, or function acting
on that user's behalf. V1 assumes one trusted local user principal. A caller
must be trusted by that user to receive the attention text derived from the
user's memory. Multi-user and delegated-authorization models are outside this
contract.

A **compile request** contains:

- one original user prompt;
- zero to three concise natural-language situation statements; and
- request metadata that identifies the declared contextual time and may
  identify the location, host application, workspace, project, output
  language, or other explicit context.

Situation statements describe the caller's relevant view of the current
external state. V1 does not discover repositories, browsers, applications, or
other environmental state autonomously. Natural-language statements are the
external V1 input convenience. The selected internal architecture encodes them
as typed numerical facets while retaining the exact request values required by
this contract.

After ingress, relevance computation operates on a structured numerical state
made from typed vectors, scalars, identifiers, and numerical relations. Prompt
and situation prose are input sources, not the computational memory,
activation, or focus state. The architecture retains canonical propositions,
identity, provenance, time, authorization and authority facts, and other exact
values outside lossy vectors so attention claims remain supportable and
derived representations remain rebuildable. Exact encoder artifacts and
release dimensions are configuration choices governed by the numerical-memory
specification and proof program.

The **local memory database** persists one user-owned logical memory universe
at the Nemosyne installation. One logical universe does not require one table,
file, index, or physical representation. It may contain memories associated
with different projects, applications, places, times, people, goals, and
situations. These associations are relevance cues rather than hard retrieval
namespaces.

An **authorized memory** is a memory the local user permits Nemosyne to use and
to disclose, in derived form, to the trusted caller. Privacy, consent, and
access-control rules may exclude a memory before relevance is evaluated.
Permission to read a memory does not establish its truth, validity, or
instruction authority.

The **attention text** is a concise, evidence-bound description of one or both
of:

- the current focus and background that materially change how the original
  prompt should be interpreted; and
- one or more qualified present-state, passive-successor, or conditional
  outcome expectations when authorized observed-transition evidence supports
  them.

It may be empty when neither branch justifies additional context. An
expectation-only result is valid only when the expectation carries its complete
situation, condition, horizon, alternative, and uncertainty scope.

An expectation remains a hypothesis with its condition, horizon,
counterevidence, and uncertainty. It is not a fact, goal, answer, action,
command, safety claim, or probability. Attention is not a concatenation of raw
memory records, a human inner monologue, a human or model chain of thought, or
a claim about consciousness.

### Successful output

The **compiled prompt** is the only successful V1 result:

```text
attention:
{attention text}

user prompt:
{original user prompt}
```

The bytes belonging to the original prompt remain unchanged inside this
framing. The framing itself is not part of the original prompt.
The compiled prompt is outbound text for the target AI system, not a
machine-readable interchange envelope. Consumers must not recover fields by
parsing the two textual headers.

The serialization is the UTF-8 concatenation:

```text
"attention:\n"
+ (if attention is empty:
     ""
   else:
     attention text without a leading or trailing line break + "\n")
+ "\nuser prompt:\n"
+ original prompt bytes
```

No suffix is added after the original prompt. Attention text uses LF line
breaks. The exact empty-attention prefix is
`"attention:\n\nuser prompt:\n"`; there is exactly one empty line between the
headers, not two. V1 guarantees this construction and prompt preservation, not
recovery of the original fields from the compiled output alone.

The **compile operation** reads memory and produces the compiled prompt. It
does not invoke the target AI system and does not create, update, persistently
consolidate, or delete memories. It may consolidate compatible support and
conflict into request-local propositions that disappear when the call ends.
One call observes one immutable logical revision of the local memory, including
records, provenance, validity, supersession, and the authorization view used
for the call.

## Requirement catalogue

These identifiers provide stable traceability into architecture,
implementation, tests, and evaluation. The detailed clauses in this
specification remain normative.

| ID | Requirement |
| --- | --- |
| `V1-R01` | Accept one authentic original prompt, zero to three situation statements, one resolved contextual time, and explicit optional request metadata; obtain caller identity and authorization time from a separate trusted local invocation context. |
| `V1-R02` | Return one complete compiled text or one explicit error; keep compile and adapter transport failures distinct, and use no request-time randomness in a V1-deployable configuration. |
| `V1-R03` | Preserve the original prompt byte-for-byte inside the exact required framing and add no suffix. |
| `V1-R04` | Compile read-only against one immutable logical revision without persistent side effects. |
| `V1-R05` | Apply authorization before relevance while keeping every authorized memory logically eligible across contextual associations. |
| `V1-R06` | Preserve source support, authority ceilings, validity, uncertainty, and material conflict without promoting data into instructions. |
| `V1-R07` | Produce concise, evidence-bound focus and/or qualified expectation context, or faithful empty attention; never produce an answer, unsupported claim, or raw context dump. |
| `V1-R08` | Enforce the declared language and finite attention budget, including faithful empty attention and explicit insufficient-budget failure. |
| `V1-R09` | Keep persistent memory local and perform compilation without network access or compiler-initiated disclosure beyond the authorized local caller receiving the compiled result. |
| `V1-R10` | Perform no autonomous environment discovery, downstream AI invocation, or automatic memory learning during compilation. |
| `V1-R11` | Use structured numerical relevance state after ingress while retaining required exact and authoritative evidence. |
| `V1-R12` | Limit the first supported and validated product claim to the declared coding-agent evidence boundary. |
| `V1-R13` | Keep initialization, memory creation, import, correction, deletion, export, consolidation, and maintenance outside the compile operation under separate contracts. |
| `V1-R14` | Represent predictive evidence as versioned transition memories that distinguish before-state, condition, outcome representation, horizon, validity, reliability, provenance, dependency, and observation status. |
| `V1-R15` | Keep focus, expectation, goal, action, answer, fact, and probability as distinct semantic types throughout compilation and rendering. |
| `V1-R16` | Permit zero to a configured finite number of competing evidence-bound expectations rather than forcing one outcome. |
| `V1-R17` | Preserve each expectation's kind, condition, horizon, source support, counterevidence, uncertainty, exact values, and authority ceiling. |
| `V1-R18` | Prevent duplicated or known-dependent records from multiplying their total predictive-support budget. |
| `V1-R19` | Treat normalized relative support only as an evidence share until a separately accepted calibrated probability model passes disjoint evaluation. |
| `V1-R20` | Abstain from a positive expectation when otherwise valid evidence is insufficient, declared retrieval or coverage is below its sufficiency policy, or a well-formed material alternative family cannot fit its expectation limit faithfully; preserve malformed canonicalization, dependency, lineage, and global attention-budget failures as typed errors. |
| `V1-R21` | Preserve material incompatible alternatives and unknown or omitted support; never hide them through top-k renormalization or prose selection. |
| `V1-R22` | Keep assessment of later observations outside the compile API and product result; conformance evaluation may compare immutable expectation fixtures without mutating them, and only a separate explicit compile may reconsider alternatives. |
| `V1-R23` | Never treat renderer output, downstream agent output, or a prior prediction as persistent memory truth without a separate independently authorized observation. |
| `V1-R24` | Keep the renderer a local lexicalizer of a qualified plan; it must not generate semantic expectations, probabilities, answers, or actions. |
| `V1-R25` | Enforce finite configured limits on inputs, candidates, transitions, outcome groups, alternatives, exact values, computation, model resources, and attention output. |

## Preconditions

- The original prompt and situation statements are valid UTF-8, remain within
  finite configured input limits, and each contain at least one non-whitespace
  character.
- The request contains no more than three situation statements.
- The trusted caller preserves the origin of the current user prompt and does
  not present generated agent text as authenticated user input.
- The request supplies one unambiguous RFC 3339 contextual time with explicit
  offset. The compiler never substitutes the ambient clock.
- Authorization, disclosure expiry, and security policy use a separate trusted
  invocation time. Caller-supplied contextual time cannot revive expired
  access.
- Optional metadata is supplied explicitly. Absence of optional metadata is
  not evidence for a guessed value.
- The local memory installation is readable. An initialized but empty memory
  universe is valid.
- Every memory considered by compilation has passed the applicable
  authorization boundary.
- A positive expectation additionally requires eligible observed-transition
  evidence with compatible kind, condition, horizon, provenance, dependency,
  and numerical schemas. Absence of such evidence is valid and may yield
  focus-only or empty attention.
- The caller is trusted by the local user to receive the derived attention
  content.
- A finite attention-size budget is available. Its unit and V1 limit remain
  open.

Invalid input, inaccessible persistent state, or an inability to produce a
faithful result causes an explicit failure. A failed call does not return a
partial compiled prompt.

## Invariants

### Single-call behavior

The compile API produces one complete in-memory compiled prompt or one explicit
error. An adapter begins exposing the compiled prompt only after compilation
succeeds, keeps diagnostics and errors outside the successful product result,
and reports a delivery failure as an unsuccessful invocation. This does not
claim that every external transport can deliver an entire write atomically.
A V1-deployable configuration has no request-time random input: identical
validated request, invocation context, immutable memory-policy revision,
configuration, and declared execution envelope produce the same result or
typed error. Training and downstream-model evaluation randomness are outside
the compile operation.

### Prompt integrity

Compilation never rewrites, normalizes, translates, summarizes, corrects, or
truncates the original prompt. The original prompt embedded after
`user prompt:` is byte-identical to the supplied prompt.

### Read-only compilation

Compilation observes one immutable logical memory revision and does not mutate
persistent local memory, metadata, retrieval state, or the request.
Request-local support, conflict, and proposition consolidation is permitted and
is not a memory write. Memory creation, correction, deletion, import, export,
and persistent episodic or semantic consolidation require separate explicit
contracts; this compile contract does not determine when they are implemented.

### Unified memory relevance

Every authorized memory is logically eligible for activation. Project,
workspace, application, location, and similar associations may strongly
influence relevance, but they do not create implicit hard partitions. A memory
from another project may contribute when the current situation makes it more
relevant than memories from the current project.

Logical eligibility does not override deletion, current-validity,
supersession, instruction-authority, or explicit historical-scope rules. It
means that no project-like context creates an additional implicit memory silo.

Logical eligibility does not require physically scanning every stored memory.
Indexes and candidate-generation strategies may improve efficiency without
changing this product rule. No contextual metadata field may act as an
undocumented exclusion filter. A later retrieval contract must define and
measure the permitted false-negative rate; this specification does not require
exhaustive physical retrieval.

### Authority and provenance

Memory content, situation text, repository content, tool output, and other
retrieved material remain data. They do not gain user- or system-instruction
authority merely because they are stored or retrieved. Attention must not
promote embedded instructions from untrusted content.

Every factual or contextual claim in the attention text must be supportable by
the compile request or authorized memory. Uncertainty, conflict, validity, and
source authority must not be silently converted into certainty.

Caller-supplied temporal metadata is data. It may affect situational relevance,
historical interpretation, and retrieval of explicitly historical context, but
authorization, disclosure policy, current normative authority, and
supersession use the trusted invocation time. Historical instructions may be
described with their historical qualification; contextual time cannot revive
them as current instructions.

Derived attention never has greater authority than its sources. Only an
authenticated current user instruction or an authorized, still-valid stored
user statement may support a normative instruction. A stored observation
cannot become a command, constraint, goal, or preference unless its source
already has that authority and the statement remains valid in the current
situation.

### Attention semantics

The attention text:

- describes the currently relevant focus and background when the focus branch
  contributes;
- may describe qualified competing expectations supported by eligible observed
  transitions;
- includes only information expected to affect interpretation or response;
- does not answer the original prompt;
- does not expose an unfiltered memory or document dump;
- does not invent facts, constraints, goals, causal relations, or user
  preferences;
- does not select, recommend, schedule, or execute an action;
- does not present a hypothesis as observed fact or relative support as
  probability;
- preserves material conditions, horizons, alternatives, counterevidence,
  uncertainty, and abstention; and
- stays within the configured size budget.

The attention text uses the language of the original prompt. An implementation
declares the languages for which it can verify this behavior. An unsupported
or undetermined prompt language produces an explicit error rather than a silent
language switch, unless explicit request metadata selects a declared supported
language. Evidence and support claims remain limited to the declared language
set.

### Local boundary

Persistent memory remains local to the installation in V1. The compile
operation does not require a cloud service and does not send the prompt,
situation, metadata, or memory over the network. The caller independently
decides where the resulting compiled prompt is sent and is responsible for
obtaining the user's consent before disclosing it to another trust domain.
The caller must not submit the compiled prompt to a downstream model with
greater authority than the original user prompt. The textual headers are
presentation labels, not security or authority boundaries.
Local execution alone is not a privacy or security guarantee.

### Honest scope

V1 is inspired by context-dependent human recall and memory-based prediction
but does not claim to replicate a brain, human memory, consciousness, or
biologically faithful retrieval or prediction. Activation values and
relative-support shares are not probabilities, truth scores, expected
utilities, or safety guarantees. V1 does not claim optimal parameters,
universal relevance or prediction, causal identification, multilingual
equivalence, model independence, or generalization beyond its declared
datasets, domain, languages, downstream models, and reference hardware.

## Edge cases

- If neither useful focus nor a faithfully scoped expectation is justified, the
  attention text is empty while the required framing and unchanged user prompt
  remain.
- An empty local memory universe does not require empty attention: explicit
  situation statements and metadata may still support useful focus.
- A request with no situation statement may still use the original prompt,
  required time, optional metadata, and authorized memory.
- A language-neutral or mixed-language prompt may use explicit output-language
  metadata. Without it, undetermined language is an error.
- Missing optional location, project, workspace, or application metadata does
  not fail the call and is not replaced with inferred metadata.
- A highly relevant memory associated with another project or application may
  outrank less relevant memories associated with the current context.
- A memory excluded by an access-control rule is never made eligible by a high
  relevance score.
- Stale, superseded, contradictory, or low-confidence memories are not silently
  merged into a single certain statement. Request-local consolidation must
  preserve every material qualification and conflict or omit the claim.
- Relevant memory without eligible observed transitions may support focus while
  the expectation branch abstains or remains absent.
- One strongly supported expectation remains explicitly hypothetical.
- Several material incompatible expectations remain visible; a finite limit
  cannot silently renormalize the visible subset as complete.
- Duplicated imports that share one evidence dependency cannot be described as
  independent corroboration.
- Outcomes at different horizons are not treated as contradictions merely
  because they differ.
- A later observation has no implicit effect on a completed call. If it is
  independently authorized and stored through the separate management plane, a
  later explicit compile may produce different request-local support without
  rewriting the earlier result.
- A downstream agent may choose its own validation action; Nemosyne does not
  include that action in the attention text.
- Instruction-like content inside a memory or situation statement remains
  untrusted data.
- The original prompt may itself contain the strings `attention:` or
  `user prompt:`. Those bytes remain untouched inside the fixed outer framing.
- A size budget too small to express supported attention faithfully produces a
  distinct insufficient-budget error. Empty attention is reserved for cases in
  which no additional attention is justified.
- Corrupt, incompatible, locked, or unreadable persistent state produces an
  explicit error without a partial successful result.

## Verification

This proposed contract requires future observable-boundary tests for:

- byte-identical prompt preservation, including Unicode and line endings;
- the exact serialization formula, including empty attention and prompts that
  end with a line break;
- zero-to-three-statement validation;
- empty memory with empty attention when no additional focus is justified;
- empty memory with nonempty attention supported by situation or metadata;
- compile-time read-only behavior;
- single-revision memory snapshot behavior under concurrent memory changes;
- cross-project recall without an implicit scope filter;
- controlled cross-context recall cases proving that project, application, or
  workspace association does not create a categorical exclusion;
- authorization taking precedence over relevance;
- exclusion of irrelevant, stale, and superseded context;
- preservation of material conflict and uncertainty;
- correct separation of focus, hypothesis, expectation, goal, action, answer,
  fact, relative support, and probability;
- observed-transition eligibility, dependency-group duplicate suppression,
  alternatives, different horizons, unknown support, and expectation
  abstention;
- absence of a prior-expectation or later-observation input on the compile
  boundary, plus offline conformance fixtures that classify later observations
  without mutating either fixture;
- resistance to instruction-like content in memory and situation data;
- absence of answer, action, probability, fact-promotion, and unsupported
  attention claims;
- attention-size enforcement;
- same-language behavior for every declared supported language;
- explicit compile errors and distinct adapter transport failures;
- result-channel isolation for every adopted adapter; and
- operation without network access.

Evaluation cases must label propositions that attention must include, may
include, and must exclude, including cases in which empty attention is
required. A separate diagnostic or test boundary must map each emitted
proposition to request or memory evidence and retain material conflicts and
qualifications. This evidence boundary does not change the single compiled text
returned by the product operation.

Before architecture selection is treated as a product commitment, an
expert-authored reference-attention comparison must demonstrate useful headroom
for the product premise. Expert-authored attention is a constrained reference,
not a proven optimum, and obeys the same semantics, size budget, language, and
downstream placement as system-generated attention.

End-to-end evaluation must compare at least:

- the unchanged prompt with no attention;
- the prompt with the same situation and metadata but no persistent memory;
- focus-only attention over the same eligible memories;
- focus plus a renderer-visible expectation abstention;
- expectation-only attention where the fixture permits it;
- focus-plus-expectation attention;
- focus plus a deliberately wrong dominant expectation;
- token-matched raw context;
- token-matched semantic-similarity retrieval;
- the strongest available deterministic non-oracle baseline;
- the candidate Nemosyne implementation; and
- expert-authored reference attention.

Factorial ablations must isolate the effects of situation, memory, and full
attention compilation. Every comparison uses the same downstream model,
message placement, decoding settings, and effective context or token budget.

A successful V1 release claim requires improvement on context-dependent coding
tasks over no attention and the strongest non-oracle baseline, a predeclared
non-inferiority bound on context-independent tasks, and predeclared maximum
harm rates against both required baselines and within every subgroup included
in the supported claim. Exact thresholds remain for the later evaluation
specification. Claims in another domain require separate evidence for that
domain.

Before a sealed evaluation set is opened or executed, its manifest and semantic
lineage split, the architecture revision, parameters, downstream model
versions, prompts, seeds, baselines, metrics, statistical treatment, release
thresholds, reference hardware, and resource budgets must be frozen. Sealed
cases must be previously unauthored, not derived from calibration lineages, and
labeled without access to candidate outputs or scores. Independent annotation
or adjudication resolves disputed labels.

Primary coding-agent outcomes come from executable tests, repository
invariants, and explicit instruction-compliance checks. Blinded human
adjudication may resolve subjective cases. An AI judge may supplement but not
solely determine a release claim.

The existing revision-1 synthetic activation corpus is regression evidence and
contains no observed-transition or expectation labels. It cannot satisfy this
independent end-to-end claim. Moving this specification
from `Proposed` to `Experimental` requires an implementation and
observable-boundary tests. Moving it to `Validated` additionally requires the
sealed end-to-end evidence and every predeclared release gate in this section.

The existing activation kernel and offline evaluation corpus provide evidence
only for their current numerical contracts. They do not verify this product
contract.

## Open questions

The linked architecture specifications now fix the intended numerical-memory,
predictive focus-and-expectation, renderer-baseline, vector-prefix candidate,
and model-qualification path.
Before implementation or a supported product claim, focused contracts must
still resolve:

- adoption and stabilization of the proposed programmatic and CLI contracts,
  concrete limits, and budget unit;
- memory provisioning, creation, import, correction, deletion, export,
  persistent consolidation, and migration;
- the physical database schema, engine, indexing strategy, encryption,
  permissions, and storage location;
- the artifact-manifest signing format, installation trust root, and
  authenticated update and rollback policy;
- the exact encoder checkpoints, vector dimensions, calibrated runtime
  parameters, candidate budgets, and accepted false-negative rates;
- transition and outcome schemas, compatibility functions, support and
  abstention thresholds, and time-later evidence;
- the final plan budget, model, quantization, runtime, supported languages,
  reference hardware, and resource thresholds selected by frozen evaluation;
- concurrency, process boundaries, packaging, and supported platforms;
- inspectability and diagnostics without changing the single successful
  output; and
- evaluation datasets, statistical thresholds, release gates, and
  reproducible evidence receipts.

## References

- [Superseded Decision 0011: Adopt a local read-only attention compiler for V1](../decisions/0011-adopt-local-read-only-attention-compiler-v1.md)
- [Superseded Decision 0012: Adopt numerical cognitive memory and focus compilation](../decisions/0012-adopt-numerical-cognitive-memory-and-focus-compilation.md)
- [Superseded Decision 0013: Adopt a vector-prefix local renderer qualification path](../decisions/0013-adopt-a-vector-prefix-local-renderer-qualification-path.md)
- [V1 reference architecture](v1-reference-architecture.md)
- [V1 proof program](v1-proof-program.md)
- [V1 delivery program](v1-delivery-program.md)
- [Cognitive memory activation and focus](cognitive-memory-activation-and-focus.md)
- [Predictive attention and expectation](predictive-attention-and-expectation.md)
- [Focus-and-expectation planning](focus-and-expectation-planning.md)
- [Vector-to-attention renderer](vector-to-attention-renderer.md)
- [Local renderer model qualification](local-renderer-model-qualification.md)
- [Situation-conditioned activation](situation-conditioned-activation.md)
- [Activation parameter evaluation](activation-parameter-evaluation.md)
- [Curated activation evidence](curated-activation-evidence.md)
- [Decision 0014: Adopt memory-grounded predictive attention](../decisions/0014-adopt-memory-grounded-predictive-attention.md)
- [Decision 0015: Render qualified focus-and-expectation plans](../decisions/0015-render-qualified-focus-and-expectation-plans.md)
- [Nemosyne README](../../README.md)
