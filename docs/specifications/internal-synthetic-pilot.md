# Internal synthetic pilot

Status: Experimental

## Purpose

This specification defines a small offline pilot for the question:

> Does a generated focus context make an agent follow relevant prior
> constraints more reliably than no context or a same-size placebo?

The pilot is classified exactly as `InternalSyntheticPilot/NonPromotional`.
It exercises generated-task provenance, seven structural input variants,
pre-outcome freezing, controlled observation capture, and descriptive
reporting. It does not produce independent evidence, a product-headroom claim,
a release gate, or any formal G1 artifact or disposition.

No generation or downstream model is selected or invoked by the repository.
Concrete generated material remains blocked until an operator supplies and
approves the exact model/version plus its cost and privacy disclosure.

## Definitions

`ModelCostPrivacyDisclosureV1` names the provider, model, immutable version or
weight identity, maximum cost, data destination, retention policy, and privacy
implication. Every field is mandatory. It contains no credential.

`GenerationManifestV1` additionally binds the exact generation-prompt bytes,
tokenizer, decoding configuration, seed schedule, tool/network policy, and
runtime identity.

`GenerationLogV1` retains every attempted generator input, complete output
bytes, and one terminal classification:

- accepted as the sole source of one generated task;
- retained but rejected under the frozen selection rule; or
- generation error with the exact captured classification.

`PilotCorpusV1` requires a one-to-one join between every accepted attempt and
one `GeneratedPilotTaskV1`. Each task contains the exact prompt, situation,
relevant prior constraints, scoring labels, and source-attempt identity.
These are synthetic pilot labels, not independently authored case truth.

`PilotCondition` has seven pilot-scoped labels:

| Label | Structural variant |
| --- | --- |
| `pilot_prompt` | Original generated prompt without added context |
| `pilot_situation` | Prompt plus generated situation without prior-constraint context |
| `pilot_placebo` | Situation plus same-size irrelevant synthetic context |
| `pilot_focus` | Generated focus plus a neutral expectation carrier |
| `pilot_correct` | The same focus plus a generated correct expectation |
| `pilot_wrong` | The same focus plus a generated deliberately wrong expectation |
| `pilot_abstain` | The same focus plus explicit expectation abstention |

`PilotConditionSetV1` requires one exact artifact for every task and condition.
Prompt and situation have no attention-token count. The other five variants
have one common positive attention-token count within each task.

`PilotScoringManifestV1` freezes the deterministic scoring implementation,
constraint-following rule, task-completion rule, unavailable-cell rule, and
descriptive aggregation rule. The primary observation is whether the relevant
prior constraint remained satisfied. Task completion is retained separately
so constraint preservation cannot hide unusable task behavior.

`PilotRunnerIdentityV1` joins the source commit, runner implementation, runtime
environment, and outcome-sink identity. `PilotRunnerManifestV1` freezes that
identity with the complete seven-condition order, unique seed schedule,
cell-isolation policy, and unavailable-cell capture policy. Outcome-driven
regeneration is unconditionally encoded as forbidden.

`FrozenSyntheticPilotV1` content-identifies and exactly joins the generation
manifest, generation log, corpus, condition set, scoring manifest, and runner
manifest before observations can enter the API. It is non-cloneable and is
consumed by receipt finalization.

`SyntheticPilotReceiptV1` has only `Completed`, `Invalid`, and `Aborted`
dispositions. A completed receipt requires one scored observation for every
task-condition-seed cell. Invalid and aborted receipts retain partial
observations and a nonempty reason. Every receipt binds the frozen pilot root,
observation root, per-condition counts, and the fixed
`InternalSyntheticPilot/NonPromotional` labels.

The only reported rate is the descriptive fraction of scored cells that
followed the relevant prior constraint. There is no threshold, inferential
decision, or promotion operation.

## Preconditions

- A human operator approves one exact generation-model disclosure before any
  model invocation or generated-material acceptance.
- The approved maximum cost, data destination, retention behavior, and privacy
  implication are accurate for that exact model/version and invocation path.
- Generator credentials remain outside every manifest, log, corpus, output,
  and receipt.
- The generation prompt, generation procedure, task-selection rule, scoring
  procedure, runner, seeds, corpus, and seven condition artifacts are fixed
  before any downstream outcome is accessed.
- Every attempted generation is retained, including rejected outputs and
  generator errors.
- Inputs contain no private user memory, secret, proprietary repository
  content, or participant data unless a separate consent and data-governance
  contract authorizes that exact use.
- A controlled runner starts each task-condition-seed cell from the frozen
  environment under the declared isolation and capture policies.

## Invariants

- The pilot module is separate from `nemosyne_evaluation::evidence` and accepts
  none of its formal admission or receipt types.
- Pilot conditions use only the `pilot_*` labels.
- No pilot object can construct or be promoted into a formal evaluation
  envelope or receipt.
- Every artifact root is domain-separated SHA-256 over explicit
  length-delimited bytes. Changing a model version, attempt, task, label,
  condition input, scoring rule, runner field, observation, or disposition
  changes the corresponding root subject to the hash-collision assumption.
- A generation log is bound to one generation manifest, a corpus to one log,
  and a condition set to one corpus. Cross-bound composition rejects.
- Receipt construction consumes the non-cloneable frozen pilot.
- A completed receipt contains no unavailable cell. A partial or procedurally
  invalid run remains `Invalid` or `Aborted`; it is never silently completed.
- Regeneration, task replacement, relabeling, scorer changes, or condition
  changes after outcome access require a new pilot lineage. Results from the
  old and new lineages cannot be pooled by this API.
- Descriptive counts and rates carry no independent, causal, product, release,
  or generalization claim.

## Edge cases

- Empty disclosure, prompt, task, scoring, runner, reason, or required
  observation bytes reject.
- Zero task or generation-attempt identifiers reject.
- Duplicate attempts, tasks, condition artifacts, seeds, and observations
  reject.
- An accepted attempt without exactly one matching task, or a task without a
  matching accepted attempt, rejects.
- Missing structural variants, unknown tasks, absent attention token counts,
  or unequal attention token counts reject before freezing.
- Unknown observation tasks or seeds reject.
- Missing or unavailable cells prevent `Completed`.
- A model API alias without an immutable version is insufficient. The
  disclosure must instead identify the strongest reproducible version
  guarantee available and explicitly record the remaining drift risk.

## Verification

`crates/nemosyne-evaluation/tests/internal_synthetic_pilot.rs` uses local
structural fixtures only. It verifies:

- complete seven-condition construction;
- retention of accepted and rejected generation attempts;
- descriptive completed receipts and fixed non-promotional labels;
- rejection of incomplete completion;
- partial invalid receipts;
- identity change after model-version change;
- rejection of a missing condition; and
- rejection of cross-bound corpus and condition artifacts.

These tests are executable evidence for the structural contract only. They do
not contain AI-generated pilot material or downstream agent outcomes.

## Open questions

- Exact generation provider, model, immutable version, maximum cost, data
  destination, retention behavior, and privacy implication.
- Exact generation prompt and selection rule.
- Size and task-family composition of the first generated corpus.
- Exact deterministic scorer implementation and controlled runner.
- Durable external publication, trusted timing, signing, and custody for pilot
  roots. Current content roots bind in-process inputs but do not prove when
  they were created or who controlled external access.

All of these must be fixed before an actual pilot run. No current repository
artifact answers the pilot question.

## References

- [G1 execution admission harness](g1-execution-admission-harness.md)
- [V1 proof program](v1-proof-program.md)
- [Decision 0048](../decisions/0048-adopt-a-non-promotional-internal-synthetic-pilot.md)
