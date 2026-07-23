use nemosyne_evaluation::activation::ScenarioId;

use super::super::revision_one::ScenarioDefinition;
use super::super::{CorpusSplit, ScenarioCategoryId, SemanticCaseId};
use super::schema::{candidate, fact, judgment, preference, scenario};
use super::{ABSENT, HIGH, LOW, MAXIMAL, MEDIUM};

pub(super) fn scenarios() -> Vec<ScenarioDefinition> {
    vec![
        use_remote_reader(),
        use_local_reader(),
        implement_and_test(),
        perform_read_only_review(),
        update_normative_documentation(),
        correct_readme_only(),
        preserve_current_api(),
        remove_current_api(),
    ]
}

fn use_remote_reader() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2001),
        SemanticCaseId::new(200),
        ScenarioCategoryId::new(40),
        CorpusSplit::HeldOut,
        "Use available remote reader",
        "Equivalent source views exist, but only the remote reader is currently available.",
        vec![
            fact(
                1,
                "Remote and local views contain equivalent current source information.",
            ),
            fact(2, "Remote read access is available."),
            fact(3, "Local checkout access is unavailable."),
            fact(4, "The current task is to inspect the source information."),
            fact(5, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: HIGH => (
                &[4],
                "The source-inspection request directly frames the decision."
            ),
            observed_state: ABSENT => (
                &[1],
                "The source views expose equivalent current state; only their access capabilities differ."
            ),
            active_outcome: HIGH => (
                &[1, 4],
                "Obtaining the equivalent current source information directly advances the inspection outcome."
            ),
            capability: MAXIMAL => (
                &[2, 3],
                "Only the remote reader is available, so capability determines which equivalent view can be inspected."
            ),
            constraint: ABSENT => (
                &[5],
                "No explicit constraint governs the source-selection decision."
            ),
        )),
        vec![
            candidate(
                1,
                "Read the remote source view",
                judgments!(
                    trigger: HIGH => (
                        &[4],
                        "Reading the remote view directly performs the requested source inspection."
                    ),
                    observed_state: ABSENT => (
                        &[1],
                        "The observed-state channel is inactive because both views contain equivalent current information."
                    ),
                    active_outcome: HIGH => (
                        &[1, 4],
                        "Reading this equivalent view directly supplies the source information sought by the task."
                    ),
                    capability: MAXIMAL => (
                        &[2],
                        "Remote read access is immediately available."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Read the local source view",
                judgments!(
                    trigger: HIGH => (
                        &[4],
                        "Reading the local view directly addresses the requested source inspection."
                    ),
                    observed_state: ABSENT => (
                        &[1],
                        "The observed-state channel is inactive because both views contain equivalent current information."
                    ),
                    active_outcome: HIGH => (
                        &[1, 4],
                        "If accessible, the equivalent local view would directly supply the requested source information."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "Local checkout access is unavailable, so this action cannot currently be performed."
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
            &[1, 2, 3],
            "The currently executable source should precede an inaccessible equivalent.",
        ),
    )
}

fn use_local_reader() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2002),
        SemanticCaseId::new(200),
        ScenarioCategoryId::new(40),
        CorpusSplit::HeldOut,
        "Use available local reader",
        "Equivalent source views exist, but only the local reader is currently available.",
        vec![
            fact(
                1,
                "Remote and local views contain equivalent current source information.",
            ),
            fact(2, "Remote read access is unavailable."),
            fact(3, "Local checkout access is available."),
            fact(4, "The current task is to inspect the source information."),
            fact(5, "No explicit active constraint applies to this decision."),
        ],
        judgment(judgments!(
            trigger: HIGH => (
                &[4],
                "The source-inspection request directly frames the decision."
            ),
            observed_state: ABSENT => (
                &[1],
                "The source views expose equivalent current state; only their access capabilities differ."
            ),
            active_outcome: HIGH => (
                &[1, 4],
                "Obtaining the equivalent current source information directly advances the inspection outcome."
            ),
            capability: MAXIMAL => (
                &[2, 3],
                "Only the local reader is available, so capability determines which equivalent view can be inspected."
            ),
            constraint: ABSENT => (
                &[5],
                "No explicit constraint governs the source-selection decision."
            ),
        )),
        vec![
            candidate(
                1,
                "Read the remote source view",
                judgments!(
                    trigger: HIGH => (
                        &[4],
                        "Reading the remote view directly addresses the requested source inspection."
                    ),
                    observed_state: ABSENT => (
                        &[1],
                        "The observed-state channel is inactive because both views contain equivalent current information."
                    ),
                    active_outcome: HIGH => (
                        &[1, 4],
                        "If accessible, the equivalent remote view would directly supply the requested source information."
                    ),
                    capability: ABSENT => (
                        &[2],
                        "Remote read access is unavailable, so this action cannot currently be performed."
                    ),
                    constraint: ABSENT => (
                        &[5],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Read the local source view",
                judgments!(
                    trigger: HIGH => (
                        &[4],
                        "Reading the local view directly performs the requested source inspection."
                    ),
                    observed_state: ABSENT => (
                        &[1],
                        "The observed-state channel is inactive because both views contain equivalent current information."
                    ),
                    active_outcome: HIGH => (
                        &[1, 4],
                        "Reading this equivalent view directly supplies the source information sought by the task."
                    ),
                    capability: MAXIMAL => (
                        &[3],
                        "Local checkout access is immediately available."
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
            &[1, 2, 3],
            "The currently executable source should precede an inaccessible equivalent.",
        ),
    )
}

fn implement_and_test() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2101),
        SemanticCaseId::new(210),
        ScenarioCategoryId::new(30),
        CorpusSplit::HeldOut,
        "Implement and run focused test",
        "The task requests implementation, and repository verification requires the executable focused test.",
        vec![
            fact(1, "The user requests implementation of the change."),
            fact(2, "A focused test covers the changed behavior."),
            fact(3, "The focused test can be executed."),
            fact(
                4,
                "Repository verification requires running the focused test for this implementation.",
            ),
            fact(5, "Static review is also currently available."),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[1],
                "The explicit implementation request is the dominant cue for selecting the next work."
            ),
            observed_state: HIGH => (
                &[2],
                "The focused test that covers the change directly shapes the current implementation decision."
            ),
            active_outcome: HIGH => (
                &[1, 2],
                "Completing the implementation with focused verification directly shapes the requested outcome."
            ),
            capability: ABSENT => (
                &[3, 5],
                "Both candidate actions are currently available, so capability does not affect this decision."
            ),
            constraint: MAXIMAL => (
                &[4],
                "The focused-test requirement is a hard constraint on completing the implementation."
            ),
        )),
        vec![
            candidate(
                1,
                "Implement the change and run the focused test",
                judgments!(
                    trigger: MAXIMAL => (
                        &[1],
                        "Implementing the change is the exact operation named by the request."
                    ),
                    observed_state: MAXIMAL => (
                        &[2, 3],
                        "This candidate uses the exact available test that covers the changed behavior."
                    ),
                    active_outcome: MAXIMAL => (
                        &[1, 2],
                        "Implementation plus its focused verification constitutes the requested completed outcome."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both candidate actions are currently available."
                    ),
                    constraint: MAXIMAL => (
                        &[4],
                        "Implementation followed by the focused test exactly satisfies the verification requirement."
                    ),
                ),
            ),
            candidate(
                2,
                "Perform static review only",
                judgments!(
                    trigger: MEDIUM => (
                        &[1],
                        "Static review addresses the broad change but does not perform the requested implementation."
                    ),
                    observed_state: LOW => (
                        &[2],
                        "The focused test bears only indirectly on review that neither runs it nor implements the change."
                    ),
                    active_outcome: MEDIUM => (
                        &[1],
                        "Static review can make a secondary contribution but does not deliver the requested implementation."
                    ),
                    capability: ABSENT => (
                        &[5],
                        "The capability channel is inactive because both candidate actions are currently available."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "Static review without implementation or test execution violates the verification requirement."
                    ),
                ),
            ),
        ],
        preference(
            1,
            2,
            &[1, 2, 3, 4],
            "The requested implementation and required focused verification should precede review-only work.",
        ),
    )
}

fn perform_read_only_review() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2102),
        SemanticCaseId::new(210),
        ScenarioCategoryId::new(30),
        CorpusSplit::HeldOut,
        "Perform read-only review",
        "The task requests read-only review and does not authorize execution or mutation.",
        vec![
            fact(1, "The user requests read-only review."),
            fact(
                2,
                "Implementation and test execution are outside the task scope.",
            ),
            fact(3, "Static inspection is available."),
            fact(
                4,
                "The read-only work mode explicitly prohibits mutation and execution.",
            ),
            fact(
                5,
                "No observed workspace, runtime, or competing instruction-authority state distinguishes the candidates.",
            ),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[1],
                "The explicit read-only review request is the dominant decision cue."
            ),
            observed_state: ABSENT => (
                &[5],
                "No observed workspace, runtime, or competing instruction-authority state affects the read-only work-mode decision."
            ),
            active_outcome: HIGH => (
                &[1, 2],
                "Producing a read-only assessment directly shapes the requested outcome."
            ),
            capability: HIGH => (
                &[3, 4],
                "Current permissions allow static inspection while excluding mutation and execution."
            ),
            constraint: MAXIMAL => (
                &[4],
                "The explicit prohibition on mutation and execution is a hard active constraint."
            ),
        )),
        vec![
            candidate(
                1,
                "Implement the change and run the focused test",
                judgments!(
                    trigger: LOW => (
                        &[1, 2],
                        "Implementation shares the reviewed subject but is peripheral to a read-only request."
                    ),
                    observed_state: ABSENT => (
                        &[5],
                        "The observed-state channel is inactive because no workspace, runtime, or competing instruction-authority state affects the decision."
                    ),
                    active_outcome: LOW => (
                        &[1, 2],
                        "Implementation may relate to later work but contributes only indirectly to the requested review."
                    ),
                    capability: ABSENT => (
                        &[2],
                        "Execution and mutation are not authorized within the current task."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "Implementation and execution conflict with the explicit read-only constraint."
                    ),
                ),
            ),
            candidate(
                2,
                "Perform static review only",
                judgments!(
                    trigger: MAXIMAL => (
                        &[1],
                        "Static review is the exact focus of the explicit request."
                    ),
                    observed_state: ABSENT => (
                        &[5],
                        "The observed-state channel is inactive because no workspace, runtime, or competing instruction-authority state affects the decision."
                    ),
                    active_outcome: MAXIMAL => (
                        &[1],
                        "A static assessment constitutes the requested read-only outcome."
                    ),
                    capability: MAXIMAL => (
                        &[3],
                        "Static inspection is immediately available."
                    ),
                    constraint: MAXIMAL => (
                        &[4],
                        "Static inspection exactly preserves the explicit read-only constraint."
                    ),
                ),
            ),
        ],
        preference(
            2,
            1,
            &[1, 2, 3],
            "Read-only review should precede unauthorized implementation.",
        ),
    )
}

fn update_normative_documentation() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2201),
        SemanticCaseId::new(220),
        ScenarioCategoryId::new(10),
        CorpusSplit::HeldOut,
        "Update normative contract",
        "A public behavior change triggers the repository's specification requirement.",
        vec![
            fact(1, "The change modifies public Rust behavior."),
            fact(
                2,
                "Repository governance requires a specification update for that change.",
            ),
            fact(3, "An unrelated README rewrite is possible."),
            fact(
                4,
                "The current task is to implement the public behavior change.",
            ),
            fact(
                5,
                "The requested file and normative specification are editable.",
            ),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[4],
                "The explicit implementation task is the dominant cue for choosing the work scope."
            ),
            observed_state: HIGH => (
                &[1],
                "The public-behavior nature of the current change directly shapes the work."
            ),
            active_outcome: HIGH => (
                &[1, 4],
                "Implementing the public behavior change directly defines the requested outcome."
            ),
            capability: ABSENT => (
                &[3, 5],
                "All candidate edits are currently feasible, so capability does not affect the required work scope."
            ),
            constraint: HIGH => (
                &[2],
                "The specification requirement directly governs how the behavior change must be completed."
            ),
        )),
        vec![
            candidate(
                1,
                "Update the requested file and normative specification",
                judgments!(
                    trigger: HIGH => (
                        &[4],
                        "Updating the requested file directly addresses the implementation task."
                    ),
                    observed_state: HIGH => (
                        &[1],
                        "The combined update directly reflects the current public-behavior change."
                    ),
                    active_outcome: HIGH => (
                        &[1, 4],
                        "The requested-file update directly advances the public behavior outcome."
                    ),
                    capability: ABSENT => (
                        &[5],
                        "The capability channel is inactive because all candidate edits are currently feasible."
                    ),
                    constraint: MAXIMAL => (
                        &[2],
                        "Including the normative specification exactly satisfies the active documentation requirement."
                    ),
                ),
            ),
            candidate(
                2,
                "Update only the requested file",
                judgments!(
                    trigger: MAXIMAL => (
                        &[4],
                        "Updating only the requested file is the most literal execution of the implementation request."
                    ),
                    observed_state: MAXIMAL => (
                        &[1],
                        "The candidate changes the exact artifact implicated by the current public-behavior state."
                    ),
                    active_outcome: MAXIMAL => (
                        &[1, 4],
                        "The requested-file edit constitutes the requested behavior implementation."
                    ),
                    capability: ABSENT => (
                        &[5],
                        "The capability channel is inactive because all candidate edits are currently feasible."
                    ),
                    constraint: ABSENT => (
                        &[2],
                        "Omitting the normative specification fails to satisfy the active governance constraint."
                    ),
                ),
            ),
            candidate(
                3,
                "Rewrite an unrelated README section",
                judgments!(
                    trigger: LOW => (
                        &[3, 4],
                        "The README rewrite shares repository context but is peripheral to the requested behavior change."
                    ),
                    observed_state: LOW => (
                        &[1, 3],
                        "The unrelated README has only an indirect connection to the current public-behavior state."
                    ),
                    active_outcome: ABSENT => (
                        &[3, 4],
                        "Rewriting unrelated documentation does not advance the requested behavior implementation."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because all candidate edits are currently feasible."
                    ),
                    constraint: ABSENT => (
                        &[2, 3],
                        "An unrelated README rewrite does not satisfy the required normative specification update."
                    ),
                ),
            ),
        ],
        preference(
            1,
            2,
            &[1, 2],
            "The governed code-and-specification change should precede code-only work.",
        ),
    )
}

fn correct_readme_only() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2202),
        SemanticCaseId::new(220),
        ScenarioCategoryId::new(10),
        CorpusSplit::HeldOut,
        "Correct README typo",
        "A README typo changes no behavior and requires no normative specification update.",
        vec![
            fact(1, "The requested change corrects one README typo."),
            fact(2, "The typo correction changes no public behavior."),
            fact(3, "An unrelated README rewrite is possible."),
            fact(4, "No explicit active constraint applies to this decision."),
            fact(
                5,
                "The requested README file and normative specification are editable.",
            ),
        ],
        judgment(judgments!(
            trigger: MAXIMAL => (
                &[1],
                "The exact README typo correction is the dominant cue for choosing the work."
            ),
            observed_state: MEDIUM => (
                &[2],
                "The absence of a public-behavior change materially narrows the appropriate documentation scope."
            ),
            active_outcome: HIGH => (
                &[1, 2],
                "A narrow text correction directly shapes the requested outcome."
            ),
            capability: ABSENT => (
                &[3, 5],
                "All candidate documentation edits are feasible, so capability does not affect the proper scope."
            ),
            constraint: ABSENT => (
                &[4],
                "No explicit constraint governs the typo-correction decision."
            ),
        )),
        vec![
            candidate(
                1,
                "Update the requested file and normative specification",
                judgments!(
                    trigger: HIGH => (
                        &[1],
                        "The candidate includes the requested typo correction but adds an unrequested normative update."
                    ),
                    observed_state: MEDIUM => (
                        &[2],
                        "Updating the specification is only partly consistent with the observed absence of a behavior change."
                    ),
                    active_outcome: HIGH => (
                        &[1],
                        "The candidate directly corrects the typo despite its unnecessary additional work."
                    ),
                    capability: ABSENT => (
                        &[5],
                        "The capability channel is inactive because all candidate edits are currently feasible."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                2,
                "Update only the requested file",
                judgments!(
                    trigger: MAXIMAL => (
                        &[1],
                        "Updating only the requested README file is the exact task focus."
                    ),
                    observed_state: MAXIMAL => (
                        &[1, 2],
                        "A file-only correction exactly fits the observed text-only, non-behavioral change."
                    ),
                    active_outcome: MAXIMAL => (
                        &[1],
                        "The narrow file edit constitutes the complete requested typo correction."
                    ),
                    capability: ABSENT => (
                        &[5],
                        "The capability channel is inactive because all candidate edits are currently feasible."
                    ),
                    constraint: ABSENT => (
                        &[4],
                        "The constraint channel is inactive because no explicit constraint applies."
                    ),
                ),
            ),
            candidate(
                3,
                "Rewrite an unrelated README section",
                judgments!(
                    trigger: LOW => (
                        &[1, 3],
                        "The unrelated rewrite shares the README context but is peripheral to the specific typo request."
                    ),
                    observed_state: LOW => (
                        &[2, 3],
                        "The broader rewrite has only an indirect relationship to the observed text-only change."
                    ),
                    active_outcome: ABSENT => (
                        &[1, 3],
                        "Rewriting an unrelated section does not advance the requested typo correction."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because all candidate edits are currently feasible."
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
            "The narrow requested correction should precede an unnecessary normative change.",
        ),
    )
}

fn preserve_current_api() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2301),
        SemanticCaseId::new(230),
        ScenarioCategoryId::new(20),
        CorpusSplit::HeldOut,
        "Follow current API preservation",
        "The latest authoritative correction requires preserving the public API.",
        vec![
            fact(1, "An older instruction proposed removing the public API."),
            fact(
                2,
                "The latest user correction requires preserving the public API.",
            ),
            fact(3, "Both implementation paths remain feasible."),
        ],
        judgment(judgments!(
            trigger: HIGH => (
                &[2],
                "The latest user correction directly frames the public-API decision."
            ),
            observed_state: MAXIMAL => (
                &[1, 2],
                "The exact authority state distinguishes the latest correction from the older conflicting instruction."
            ),
            active_outcome: HIGH => (
                &[2],
                "Preserving the public API directly shapes the requested outcome."
            ),
            capability: ABSENT => (
                &[3],
                "Both implementation paths are equally feasible, so capability does not affect the decision."
            ),
            constraint: MAXIMAL => (
                &[2],
                "The explicit preservation requirement is the dominant constraint on the API decision."
            ),
        )),
        vec![
            candidate(
                1,
                "Preserve the public API",
                judgments!(
                    trigger: MAXIMAL => (
                        &[2],
                        "Preserving the public API is the exact operation required by the current correction."
                    ),
                    observed_state: MAXIMAL => (
                        &[1, 2],
                        "Preservation exactly follows the latest instruction rather than the superseded alternative."
                    ),
                    active_outcome: MAXIMAL => (
                        &[2],
                        "Preserving the public API constitutes the requested outcome."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both implementation paths are equally feasible."
                    ),
                    constraint: MAXIMAL => (
                        &[2],
                        "This candidate exactly satisfies the explicit preservation requirement."
                    ),
                ),
            ),
            candidate(
                2,
                "Remove the public API",
                judgments!(
                    trigger: LOW => (
                        &[1, 2],
                        "Removal shares the public-API topic but contradicts the operation required by the current correction."
                    ),
                    observed_state: LOW => (
                        &[1, 2],
                        "Removal is supported only by the older instruction and conflicts with the latest authority state."
                    ),
                    active_outcome: ABSENT => (
                        &[1, 2],
                        "Removing the API does not advance the current API-preservation outcome."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both implementation paths are equally feasible."
                    ),
                    constraint: ABSENT => (
                        &[2],
                        "Removal conflicts with the explicit preservation requirement."
                    ),
                ),
            ),
        ],
        preference(
            1,
            2,
            &[1, 2],
            "The latest authoritative correction should supersede the older instruction.",
        ),
    )
}

fn remove_current_api() -> ScenarioDefinition {
    scenario(
        ScenarioId::new(2302),
        SemanticCaseId::new(230),
        ScenarioCategoryId::new(20),
        CorpusSplit::HeldOut,
        "Follow current API removal",
        "The latest authoritative correction requires removing the public API.",
        vec![
            fact(
                1,
                "An older instruction proposed preserving the public API.",
            ),
            fact(
                2,
                "The latest user correction requires removing the public API.",
            ),
            fact(3, "Both implementation paths remain feasible."),
        ],
        judgment(judgments!(
            trigger: HIGH => (
                &[2],
                "The latest user correction directly frames the public-API decision."
            ),
            observed_state: MAXIMAL => (
                &[1, 2],
                "The exact authority state distinguishes the latest correction from the older conflicting instruction."
            ),
            active_outcome: HIGH => (
                &[2],
                "Removing the public API directly shapes the requested outcome."
            ),
            capability: ABSENT => (
                &[3],
                "Both implementation paths are equally feasible, so capability does not affect the decision."
            ),
            constraint: MAXIMAL => (
                &[2],
                "The explicit removal requirement is the dominant constraint on the API decision."
            ),
        )),
        vec![
            candidate(
                1,
                "Preserve the public API",
                judgments!(
                    trigger: LOW => (
                        &[1, 2],
                        "Preservation shares the public-API topic but contradicts the operation required by the current correction."
                    ),
                    observed_state: LOW => (
                        &[1, 2],
                        "Preservation is supported only by the older instruction and conflicts with the latest authority state."
                    ),
                    active_outcome: ABSENT => (
                        &[1, 2],
                        "Preserving the API does not advance the current API-removal outcome."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both implementation paths are equally feasible."
                    ),
                    constraint: ABSENT => (
                        &[2],
                        "Preservation conflicts with the explicit removal requirement."
                    ),
                ),
            ),
            candidate(
                2,
                "Remove the public API",
                judgments!(
                    trigger: MAXIMAL => (
                        &[2],
                        "Removing the public API is the exact operation required by the current correction."
                    ),
                    observed_state: MAXIMAL => (
                        &[1, 2],
                        "Removal exactly follows the latest instruction rather than the superseded alternative."
                    ),
                    active_outcome: MAXIMAL => (
                        &[2],
                        "Removing the public API constitutes the requested outcome."
                    ),
                    capability: ABSENT => (
                        &[3],
                        "The capability channel is inactive because both implementation paths are equally feasible."
                    ),
                    constraint: MAXIMAL => (
                        &[2],
                        "This candidate exactly satisfies the explicit removal requirement."
                    ),
                ),
            ),
        ],
        preference(
            2,
            1,
            &[1, 2],
            "The latest authoritative correction should supersede the older instruction.",
        ),
    )
}
