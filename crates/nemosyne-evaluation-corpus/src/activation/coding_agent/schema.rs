use nemosyne_core::activation::{CandidateId, ChannelId};
use nemosyne_evaluation::activation::ScenarioId;

use super::super::revision_one::{
    CandidateDefinition, FactDefinition, JudgmentDefinition, PreferenceDefinition,
    ReferenceDefinition, RevisionOneJudgments, ScenarioDefinition,
};
use super::super::{
    CorpusSplit, EvidenceChannelDefinition, FactId, ReferenceId, ScenarioCategory,
    ScenarioCategoryId, SemanticCaseId,
};
use super::{ABSENT, MAXIMAL};

pub(super) fn channels() -> Vec<EvidenceChannelDefinition> {
    vec![
        EvidenceChannelDefinition::new(
            ChannelId::new(10),
            "trigger_alignment",
            "Relevance of the explicit request to the current decision.",
            "Fit between the candidate and the explicit request.",
            [
                "The explicit request does not affect the current decision.",
                "The explicit request has only peripheral bearing on the decision.",
                "The explicit request materially frames part of the decision.",
                "The explicit request directly frames the decision.",
                "The explicit request is the dominant decision cue.",
            ],
            [
                "Unrelated to the request.",
                "Shares only a peripheral topic.",
                "Addresses the broad task.",
                "Directly addresses the operation.",
                "Is the exact requested focus.",
            ],
        ),
        EvidenceChannelDefinition::new(
            ChannelId::new(20),
            "observed_state_alignment",
            "Relevance of observed workspace, runtime, or instruction-authority state.",
            "Fit between the candidate and the observed state.",
            [
                "Observed state does not affect the current decision.",
                "Old or indirect state has only peripheral bearing on the decision.",
                "Material current state informs part of the decision.",
                "Current observed state directly shapes the decision.",
                "The exact observed state is the dominant decision cue.",
            ],
            [
                "Unsupported by current observations.",
                "Related through old or indirect state.",
                "Consistent with material current state.",
                "Directly supported by current state.",
                "Identifies the exact observed state.",
            ],
        ),
        EvidenceChannelDefinition::new(
            ChannelId::new(30),
            "active_outcome_alignment",
            "Relevance of the currently active task outcome.",
            "Contribution of the candidate to the active task outcome.",
            [
                "The active task outcome does not affect the current decision.",
                "The active task outcome has only peripheral bearing on the decision.",
                "The active task outcome materially frames part of the decision.",
                "The active task outcome directly shapes the decision.",
                "The active task outcome is the dominant decision cue.",
            ],
            [
                "Does not advance the outcome.",
                "Has an indirect possible contribution.",
                "Makes a material secondary contribution.",
                "Directly advances the outcome.",
                "Constitutes the active task outcome.",
            ],
        ),
        EvidenceChannelDefinition::new(
            ChannelId::new(40),
            "capability_fit",
            "Relevance of current tools, permissions, and executable actions.",
            "Feasibility of using the candidate under current capabilities.",
            [
                "Current capabilities do not affect the decision.",
                "Current capabilities have only peripheral bearing on the decision.",
                "Current capabilities materially narrow some options.",
                "Current capabilities directly constrain the feasible options.",
                "Available or missing capability dominates the decision.",
            ],
            [
                "Cannot currently be applied.",
                "Requires substantial missing capability.",
                "Is partially executable.",
                "Is executable with minor friction.",
                "Is immediately executable.",
            ],
        ),
        EvidenceChannelDefinition::new(
            ChannelId::new(50),
            "constraint_alignment",
            "Relevance of an explicit active constraint.",
            "Compatibility of the candidate with that constraint.",
            [
                "No explicit active constraint applies to the decision.",
                "An explicit constraint has only peripheral bearing on the decision.",
                "An explicit constraint materially governs part of the decision.",
                "An explicit constraint directly governs the decision.",
                "A hard active constraint dominates the decision.",
            ],
            [
                "Conflicts with the active constraint.",
                "Satisfies a peripheral part.",
                "Satisfies a material part.",
                "Directly satisfies the constraint.",
                "Exactly preserves the constraint.",
            ],
        ),
    ]
}

pub(super) fn categories() -> Vec<ScenarioCategory> {
    vec![
        ScenarioCategory::new(
            ScenarioCategoryId::new(10),
            "active_constraints",
            "An explicit rule changes which focus item should dominate.",
        ),
        ScenarioCategory::new(
            ScenarioCategoryId::new(20),
            "observed_state",
            "Current workspace, runtime, or instruction-authority state distinguishes candidates.",
        ),
        ScenarioCategory::new(
            ScenarioCategoryId::new(30),
            "requested_outcome",
            "The requested scope or work mode distinguishes candidates.",
        ),
        ScenarioCategory::new(
            ScenarioCategoryId::new(40),
            "operational_feasibility",
            "Current execution capability or work phase distinguishes candidates.",
        ),
    ]
}

pub(super) fn references() -> Vec<ReferenceDefinition> {
    vec![
        ReferenceDefinition {
            reference_id: ReferenceId::new(10),
            key: "trigger_only",
            rationale: "Measures the corpus using only explicit-request alignment.",
            weights: levels!(
                trigger: MAXIMAL,
                observed_state: ABSENT,
                active_outcome: ABSENT,
                capability: ABSENT,
                constraint: ABSENT,
            ),
        },
        ReferenceDefinition {
            reference_id: ReferenceId::new(20),
            key: "uniform_evidence",
            rationale: "Measures the corpus with equal weight on every evidence channel.",
            weights: levels!(
                trigger: MAXIMAL,
                observed_state: MAXIMAL,
                active_outcome: MAXIMAL,
                capability: MAXIMAL,
                constraint: MAXIMAL,
            ),
        },
    ]
}

#[allow(clippy::too_many_arguments)]
pub(super) fn scenario(
    scenario_id: ScenarioId,
    semantic_case_id: SemanticCaseId,
    category_id: ScenarioCategoryId,
    split: CorpusSplit,
    title: &'static str,
    situation: &'static str,
    facts: Vec<FactDefinition>,
    gates: JudgmentDefinition,
    candidates: Vec<CandidateDefinition>,
    preference: PreferenceDefinition,
) -> ScenarioDefinition {
    ScenarioDefinition {
        scenario_id,
        semantic_case_id,
        category_id,
        split,
        title,
        situation,
        facts,
        gates,
        candidates,
        preferences: vec![preference],
    }
}

pub(super) fn fact(fact_id: u64, statement: &'static str) -> FactDefinition {
    FactDefinition {
        fact_id: FactId::new(fact_id),
        statement,
    }
}

pub(super) fn judgment(channels: RevisionOneJudgments) -> JudgmentDefinition {
    JudgmentDefinition { channels }
}

pub(super) fn candidate(
    candidate_id: u64,
    label: &'static str,
    channels: RevisionOneJudgments,
) -> CandidateDefinition {
    CandidateDefinition {
        candidate_id: CandidateId::new(candidate_id),
        label,
        judgment: judgment(channels),
    }
}

pub(super) fn preference(
    preferred: u64,
    other: u64,
    fact_ids: &'static [u64],
    rationale: &'static str,
) -> PreferenceDefinition {
    PreferenceDefinition {
        preferred: CandidateId::new(preferred),
        other: CandidateId::new(other),
        fact_ids,
        rationale,
    }
}
