use nemosyne_evaluation::activation::ScenarioId;

use super::super::revision_one::ScenarioDefinition;
use super::super::{CorpusSplit, ScenarioCategoryId, SemanticCaseId};
use super::schema::{candidate, fact, judgment, preference, scenario};
use super::{ABSENT, HIGH, LOW, MAXIMAL, MEDIUM};

pub(super) fn scenarios() -> Vec<ScenarioDefinition> {
    vec![
        preserve_modified_file(),
        regenerate_modified_file(),
        diagnose_refresh_path(),
        diagnose_bootstrap_path(),
        request_repair_method_a(),
        request_repair_method_b(),
        prioritize_diagnostic_outcome(),
        prioritize_repair_outcome(),
    ]
}

fn preserve_modified_file() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1001),
        SemanticCaseId::new(100),
        ScenarioCategoryId::new(10),
        CorpusSplit::Calibration,
        "Preserve modified target",
        "The target file has uncommitted work that the task requires preserving.",
        vec![
            fact(1, "The target file contains uncommitted changes."),
            fact(2, "The task explicitly requires preserving those changes."),
            fact(
                3,
                "Patching preserves existing content; regeneration replaces it.",
            ),
            fact(
                4,
                "Both operations are executable; patching has minor inspection friction while regeneration can run immediately.",
            ),
            fact(
                5,
                "Patching can complete the requested target change while preserving existing work; regeneration changes the target but loses that work.",
            ),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[2, 5],
                "The request directly concerns changing the target while preserving its existing work."
            ),
            observed_state: HIGH => (
                &[1],
                "The uncommitted target directly shapes which operation fits the current workspace state."
            ),
            active_outcome: MAXIMAL => (
                &[2, 5],
                "Changing the target while preserving its existing work is the complete requested outcome."
            ),
            capability: LOW => (
                &[4],
                "Both operations are executable, so their small difference in execution friction has only peripheral bearing on the choice."
            ),
            constraint: MAXIMAL => (
                &[2],
                "The explicit preservation requirement is a hard active constraint on the operation."
            ),
        )),
        vec![
            candidate(
                1,
                "Inspect and patch the existing file",
                judgments!(
                    trigger: MAXIMAL => (
                        &[2, 5],
                        "Patching is the exact requested focus because it changes the target while preserving its existing work."
                    ),
                    observed_state: MAXIMAL => (
                        &[1, 3],
                        "Patching is the operation that exactly fits the target's current uncommitted state."
                    ),
                    active_outcome: MAXIMAL => (
                        &[2, 5],
                        "The incremental patch can constitute the complete target change while preserving existing work."
                    ),
                    capability: HIGH => (
                        &[4],
                        "Patching is executable, although it requires inspecting and applying a focused edit."
                    ),
                    constraint: MAXIMAL => (
                        &[2, 3],
                        "Patching exactly satisfies the explicit requirement to preserve existing changes."
                    ),
                ),
            ),
            candidate(
                2,
                "Regenerate and replace the file",
                judgments!(
                    trigger: MEDIUM => (
                        &[2, 5],
                        "Regeneration addresses the broad target-change task but not the request to preserve existing work."
                    ),
                    observed_state: ABSENT => (
                        &[1, 3],
                        "Replacing the file is unsupported by the observed uncommitted state that would be lost."
                    ),
                    active_outcome: MEDIUM => (
                        &[2, 5],
                        "Complete replacement changes the target but only partially serves the requested outcome because it loses existing work."
                    ),
                    capability: MAXIMAL => (
                        &[4],
                        "Regeneration is immediately executable."
                    ),
                    constraint: ABSENT => (
                        &[2, 3],
                        "Replacing the file fails the explicit preservation requirement."
                    ),
                ),
            ),
        ],
        preference(
            1,
            2,
            &[1, 2, 3],
            "The explicit preservation requirement makes patching preferable to replacement.",
        ),
    )
}

fn regenerate_modified_file() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1002),
        SemanticCaseId::new(100),
        ScenarioCategoryId::new(10),
        CorpusSplit::Calibration,
        "Regenerate authorized target",
        "The generated target may be replaced and the user authorizes regeneration.",
        vec![
            fact(1, "The target file is generated output."),
            fact(
                2,
                "The user explicitly requests and authorizes replacing the generated file.",
            ),
            fact(
                3,
                "Patching preserves existing content; regeneration replaces it.",
            ),
            fact(4, "No explicit active constraint applies to this decision."),
            fact(
                5,
                "Both operations are executable; patching requires inspection while regeneration can run directly.",
            ),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[2],
                "The request explicitly authorizes the replacement operation under consideration."
            ),
            observed_state: MAXIMAL => (
                &[1],
                "The fact that the target is generated output is the exact current state shaping the choice."
            ),
            active_outcome: MAXIMAL => (
                &[1, 2],
                "Replacing the generated target is the explicitly authorized outcome."
            ),
            capability: LOW => (
                &[5],
                "Both operations are executable, so their small difference in execution friction has only peripheral bearing on the choice."
            ),
            constraint: ABSENT => (
                &[4],
                "The constraint channel is inactive because no explicit constraint applies."
            ),
        )),
        vec![
            candidate(
                1,
                "Inspect and patch the existing file",
                judgments!(
                    trigger: MEDIUM => (
                        &[1, 2],
                        "Patching addresses the broad target-change task but not the explicitly authorized replacement operation."
                    ),
                    observed_state: LOW => (
                        &[1, 3],
                        "Patching is related to the generated file but is only indirectly compatible with its generated ownership."
                    ),
                    active_outcome: MEDIUM => (
                        &[2, 3],
                        "Patching can make a material target change but does not perform the authorized replacement."
                    ),
                    capability: HIGH => (
                        &[5],
                        "Patching is executable, although it requires a focused edit."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Regenerate and replace the file",
                judgments!(
                    trigger: MAXIMAL => (
                        &[2],
                        "Regeneration exactly performs the replacement operation the user authorized."
                    ),
                    observed_state: MAXIMAL => (
                        &[1, 3],
                        "Regeneration is the exact operation associated with the target's generated ownership."
                    ),
                    active_outcome: MAXIMAL => (
                        &[1, 2],
                        "Regeneration constitutes the authorized replacement outcome."
                    ),
                    capability: MAXIMAL => (
                        &[5],
                        "Regeneration is immediately executable."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
        ],
        preference(
            2,
            1,
            &[1, 2, 3],
            "Explicit authorization and generated ownership make regeneration preferable.",
        ),
    )
}

fn diagnose_refresh_path() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1101),
        SemanticCaseId::new(110),
        ScenarioCategoryId::new(20),
        CorpusSplit::Calibration,
        "Follow refresh diagnostic",
        "A current authentication diagnostic implicates the refresh path.",
        vec![
            fact(
                1,
                "The task is to diagnose the current authentication failure.",
            ),
            fact(2, "The current diagnostic identifies the refresh path."),
            fact(3, "Both authentication paths are inspectable."),
            fact(4, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[1],
                "The explicit request is solely to diagnose the current authentication failure."
            ),
            observed_state: MAXIMAL => (
                &[2],
                "The current diagnostic names the refresh path exactly."
            ),
            active_outcome: MAXIMAL => (
                &[1],
                "Producing a diagnosis is the complete requested outcome."
            ),
            capability: ABSENT => (
                &[3],
                "Both paths are equally inspectable, so capability does not distinguish the diagnostic decision."
            ),
            constraint: ABSENT => (
                &[4],
                "The constraint channel is inactive because no explicit constraint applies."
            ),
        )),
        vec![
            candidate(
                1,
                "Inspect the authentication refresh path",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Inspecting an authentication path directly addresses the requested diagnosis."
                    ),
                    observed_state: MAXIMAL => (
                        &[2],
                        "The candidate targets the exact path identified by the current diagnostic."
                    ),
                    active_outcome: HIGH => (
                        &[1, 2],
                        "Inspecting the implicated path directly advances the diagnosis."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both diagnostic paths are equally inspectable."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Inspect the session bootstrap path",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Inspecting an authentication path remains directly related to the diagnostic request."
                    ),
                    observed_state: LOW => (
                        &[2],
                        "The bootstrap path is in the same subsystem but is not implicated by the current diagnostic."
                    ),
                    active_outcome: HIGH => (
                        &[1],
                        "Inspecting the alternative path could still contribute directly to diagnosis."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both diagnostic paths are equally inspectable."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
        ],
        preference(
            1,
            2,
            &[1, 2],
            "The currently implicated path should precede a related alternative.",
        ),
    )
}

fn diagnose_bootstrap_path() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1102),
        SemanticCaseId::new(110),
        ScenarioCategoryId::new(20),
        CorpusSplit::Calibration,
        "Follow bootstrap diagnostic",
        "A current authentication diagnostic implicates the bootstrap path.",
        vec![
            fact(
                1,
                "The task is to diagnose the current authentication failure.",
            ),
            fact(
                2,
                "The current diagnostic identifies the session bootstrap path.",
            ),
            fact(3, "Both authentication paths are inspectable."),
            fact(4, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[1],
                "The explicit request is solely to diagnose the current authentication failure."
            ),
            observed_state: MAXIMAL => (
                &[2],
                "The current diagnostic names the session bootstrap path exactly."
            ),
            active_outcome: MAXIMAL => (
                &[1],
                "Producing a diagnosis is the complete requested outcome."
            ),
            capability: ABSENT => (
                &[3],
                "Both paths are equally inspectable, so capability does not distinguish the diagnostic decision."
            ),
            constraint: ABSENT => (
                &[4],
                "The constraint channel is inactive because no explicit constraint applies."
            ),
        )),
        vec![
            candidate(
                1,
                "Inspect the authentication refresh path",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Inspecting an authentication path remains directly related to the diagnostic request."
                    ),
                    observed_state: LOW => (
                        &[2],
                        "The refresh path is in the same subsystem but is not implicated by the current diagnostic."
                    ),
                    active_outcome: HIGH => (
                        &[1],
                        "Inspecting the alternative path could still contribute directly to diagnosis."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both diagnostic paths are equally inspectable."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Inspect the session bootstrap path",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Inspecting an authentication path directly addresses the requested diagnosis."
                    ),
                    observed_state: MAXIMAL => (
                        &[2],
                        "The candidate targets the exact path identified by the current diagnostic."
                    ),
                    active_outcome: HIGH => (
                        &[1, 2],
                        "Inspecting the implicated path directly advances the diagnosis."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both diagnostic paths are equally inspectable."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
        ],
        preference(
            2,
            1,
            &[1, 2],
            "The currently implicated path should precede a related alternative.",
        ),
    )
}

fn request_repair_method_a() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1201),
        SemanticCaseId::new(120),
        ScenarioCategoryId::new(30),
        CorpusSplit::Calibration,
        "Use requested repair method A",
        "Two repair methods are equivalent, and the request explicitly selects method A.",
        vec![
            fact(
                1,
                "The user explicitly requests method A for the authentication repair.",
            ),
            fact(
                2,
                "Methods A and B are equally valid and produce the same repaired behavior.",
            ),
            fact(
                3,
                "The current authentication state supports methods A and B equally.",
            ),
            fact(4, "Methods A and B are equally executable."),
            fact(5, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[1],
                "The explicit method selection is the dominant cue for choosing between the equivalent repairs."
            ),
            observed_state: ABSENT => (
                &[3],
                "The current authentication state supports both repair methods equally and does not distinguish them."
            ),
            active_outcome: ABSENT => (
                &[2],
                "Both methods produce the same repaired outcome, so outcome does not distinguish the method choice."
            ),
            capability: ABSENT => (
                &[4],
                "Both repair methods are equally executable, so capability does not distinguish them."
            ),
            constraint: ABSENT => (
                &[5],
                "No explicit constraint governs the repair-method choice."
            ),
        )),
        vec![
            candidate(
                1,
                "Apply the authentication repair with method A",
                judgments!(
                    trigger: MAXIMAL => (
                        &[1],
                        "Method A is the exact repair method selected by the request."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the current state supports both methods equally."
                    ),
                    active_outcome: ABSENT => (
                        &[2],
                        "The outcome channel is inactive because both repair methods produce the same result."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both repair methods are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Apply the authentication repair with method B",
                judgments!(
                    trigger: MEDIUM => (
                        &[1, 2],
                        "Method B addresses the same broad repair task but is not the method selected by the request."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the current state supports both methods equally."
                    ),
                    active_outcome: ABSENT => (
                        &[2],
                        "The outcome channel is inactive because both repair methods produce the same result."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both repair methods are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
        ],
        preference(
            1,
            2,
            &[1, 2],
            "The explicitly requested method A should precede the equivalent unrequested method B.",
        ),
    )
}

fn request_repair_method_b() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1202),
        SemanticCaseId::new(120),
        ScenarioCategoryId::new(30),
        CorpusSplit::Calibration,
        "Use requested repair method B",
        "Two repair methods are equivalent, and the request explicitly selects method B.",
        vec![
            fact(
                1,
                "The user explicitly requests method B for the authentication repair.",
            ),
            fact(
                2,
                "Methods A and B are equally valid and produce the same repaired behavior.",
            ),
            fact(
                3,
                "The current authentication state supports methods A and B equally.",
            ),
            fact(4, "Methods A and B are equally executable."),
            fact(5, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[1],
                "The explicit method selection is the dominant cue for choosing between the equivalent repairs."
            ),
            observed_state: ABSENT => (
                &[3],
                "The current authentication state supports both repair methods equally and does not distinguish them."
            ),
            active_outcome: ABSENT => (
                &[2],
                "Both methods produce the same repaired outcome, so outcome does not distinguish the method choice."
            ),
            capability: ABSENT => (
                &[4],
                "Both repair methods are equally executable, so capability does not distinguish them."
            ),
            constraint: ABSENT => (
                &[5],
                "No explicit constraint governs the repair-method choice."
            ),
        )),
        vec![
            candidate(
                1,
                "Apply the authentication repair with method A",
                judgments!(
                    trigger: MEDIUM => (
                        &[1, 2],
                        "Method A addresses the same broad repair task but is not the method selected by the request."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the current state supports both methods equally."
                    ),
                    active_outcome: ABSENT => (
                        &[2],
                        "The outcome channel is inactive because both repair methods produce the same result."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both repair methods are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Apply the authentication repair with method B",
                judgments!(
                    trigger: MAXIMAL => (
                        &[1],
                        "Method B is the exact repair method selected by the request."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the current state supports both methods equally."
                    ),
                    active_outcome: ABSENT => (
                        &[2],
                        "The outcome channel is inactive because both repair methods produce the same result."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both repair methods are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
        ],
        preference(
            2,
            1,
            &[1, 2],
            "The explicitly requested method B should precede the equivalent unrequested method A.",
        ),
    )
}

fn prioritize_diagnostic_outcome() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1301),
        SemanticCaseId::new(130),
        ScenarioCategoryId::new(40),
        CorpusSplit::Calibration,
        "Prioritize diagnostic outcome",
        "The broad task is resolving an authentication failure, and the active outcome for this step is a diagnosis.",
        vec![
            fact(
                1,
                "The current task is to continue resolving the authentication failure.",
            ),
            fact(2, "The active outcome for this step is a diagnosis."),
            fact(
                3,
                "The current workspace contains both diagnostic inputs and a prepared repair, so observed state does not favor either action.",
            ),
            fact(
                4,
                "Diagnostic evidence collection and repair application are equally executable.",
            ),
            fact(5, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: HIGH => (
                &[1],
                "The broad failure-resolution task directly frames both candidate actions without selecting between them."
            ),
            observed_state: ABSENT => (
                &[3],
                "The neutral workspace state supports both actions equally and does not distinguish them."
            ),
            active_outcome: MAXIMAL => (
                &[2],
                "The diagnostic outcome is the dominant cue for choosing the next action."
            ),
            capability: ABSENT => (
                &[4],
                "Both actions are equally executable, so capability does not distinguish them."
            ),
            constraint: ABSENT => (
                &[5],
                "No explicit constraint governs the outcome choice."
            ),
        )),
        vec![
            candidate(
                1,
                "Collect diagnostic evidence",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Collecting diagnostic evidence directly addresses the broad failure-resolution task."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the neutral workspace supports both actions equally."
                    ),
                    active_outcome: MAXIMAL => (
                        &[2],
                        "Evidence collection constitutes the active diagnostic outcome."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both actions are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Apply the repair",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Applying the repair directly addresses the broad failure-resolution task."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the neutral workspace supports both actions equally."
                    ),
                    active_outcome: ABSENT => (
                        &[2],
                        "Applying the repair does not produce the active diagnostic outcome."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both actions are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
        ],
        preference(
            1,
            2,
            &[1, 2],
            "The action that constitutes the active diagnostic outcome should precede the alternative repair action.",
        ),
    )
}

fn prioritize_repair_outcome() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(1302),
        SemanticCaseId::new(130),
        ScenarioCategoryId::new(40),
        CorpusSplit::Calibration,
        "Prioritize repair outcome",
        "The broad task is resolving an authentication failure, and the active outcome for this step is applying the prepared repair.",
        vec![
            fact(
                1,
                "The current task is to continue resolving the authentication failure.",
            ),
            fact(
                2,
                "The active outcome for this step is applying the prepared repair.",
            ),
            fact(
                3,
                "The current workspace contains both diagnostic inputs and a prepared repair, so observed state does not favor either action.",
            ),
            fact(
                4,
                "Diagnostic evidence collection and repair application are equally executable.",
            ),
            fact(5, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: HIGH => (
                &[1],
                "The broad failure-resolution task directly frames both candidate actions without selecting between them."
            ),
            observed_state: ABSENT => (
                &[3],
                "The neutral workspace state supports both actions equally and does not distinguish them."
            ),
            active_outcome: MAXIMAL => (
                &[2],
                "The repair outcome is the dominant cue for choosing the next action."
            ),
            capability: ABSENT => (
                &[4],
                "Both actions are equally executable, so capability does not distinguish them."
            ),
            constraint: ABSENT => (
                &[5],
                "No explicit constraint governs the outcome choice."
            ),
        )),
        vec![
            candidate(
                1,
                "Collect diagnostic evidence",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Collecting diagnostic evidence directly addresses the broad failure-resolution task."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the neutral workspace supports both actions equally."
                    ),
                    active_outcome: ABSENT => (
                        &[2],
                        "Collecting diagnostic evidence does not constitute the active repair outcome."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both actions are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Apply the repair",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "Applying the repair directly addresses the broad failure-resolution task."
                    ),
                    observed_state: ABSENT => (
                        &[3],
                        "The observed-state channel is inactive because the neutral workspace supports both actions equally."
                    ),
                    active_outcome: MAXIMAL => (
                        &[2],
                        "Applying the prepared repair constitutes the active repair outcome."
                    ),
                    capability: ABSENT => (
                        &[4],
                        "The capability channel is inactive because both actions are equally executable."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
        ],
        preference(
            2,
            1,
            &[1, 2],
            "The action that constitutes the active repair outcome should precede the alternative diagnostic action.",
        ),
    )
}
