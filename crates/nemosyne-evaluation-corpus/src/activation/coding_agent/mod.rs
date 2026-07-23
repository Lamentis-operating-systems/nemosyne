use super::revision_one::{CorpusDefinition, build_revision_one};
use super::{ActivationEvidenceCorpus, CorpusError, EvidenceLevel};

const ABSENT: EvidenceLevel = EvidenceLevel::Absent;
const LOW: EvidenceLevel = EvidenceLevel::Low;
const MEDIUM: EvidenceLevel = EvidenceLevel::Medium;
const HIGH: EvidenceLevel = EvidenceLevel::High;
const MAXIMAL: EvidenceLevel = EvidenceLevel::Maximal;

macro_rules! levels {
    (
        trigger: $trigger:expr,
        observed_state: $observed_state:expr,
        active_outcome: $active_outcome:expr,
        capability: $capability:expr,
        constraint: $constraint:expr $(,)?
    ) => {
        $crate::activation::revision_one::RevisionOneVector::new(
            $trigger,
            $observed_state,
            $active_outcome,
            $capability,
            $constraint,
        )
    };
}

macro_rules! judgments {
    (
        trigger: $trigger:expr => ($trigger_facts:expr, $trigger_rationale:expr),
        observed_state: $observed_state:expr =>
            ($observed_state_facts:expr, $observed_state_rationale:expr),
        active_outcome: $active_outcome:expr =>
            ($active_outcome_facts:expr, $active_outcome_rationale:expr),
        capability: $capability:expr => ($capability_facts:expr, $capability_rationale:expr),
        constraint: $constraint:expr => ($constraint_facts:expr, $constraint_rationale:expr) $(,)?
    ) => {
        $crate::activation::revision_one::RevisionOneJudgments::new(
            $crate::activation::revision_one::ChannelJudgmentDefinition::new(
                $trigger,
                $trigger_facts,
                $trigger_rationale,
            ),
            $crate::activation::revision_one::ChannelJudgmentDefinition::new(
                $observed_state,
                $observed_state_facts,
                $observed_state_rationale,
            ),
            $crate::activation::revision_one::ChannelJudgmentDefinition::new(
                $active_outcome,
                $active_outcome_facts,
                $active_outcome_rationale,
            ),
            $crate::activation::revision_one::ChannelJudgmentDefinition::new(
                $capability,
                $capability_facts,
                $capability_rationale,
            ),
            $crate::activation::revision_one::ChannelJudgmentDefinition::new(
                $constraint,
                $constraint_facts,
                $constraint_rationale,
            ),
        )
    };
}

mod calibration;
mod held_out;
mod schema;

/// Builds the immutable revision-1 coding-agent activation evidence corpus.
///
/// The artifact contains constructed semantic evidence, structurally separated
/// calibration and held-out evaluator suites, and explicit reference parameter
/// sets. The held-out partition is not statistically blind. The artifact does
/// not contain empirical observations or recommended activation parameters.
///
/// # Errors
///
/// Returns [`CorpusError`] when repository-authored corpus data violates its
/// structural, provenance, split, pairing, or evaluator-compatibility contract.
pub fn coding_agent_v1() -> Result<ActivationEvidenceCorpus, CorpusError> {
    build_revision_one(coding_agent_definition())
}

pub(super) fn coding_agent_definition() -> CorpusDefinition {
    let mut scenarios = calibration::scenarios();
    scenarios.extend(held_out::scenarios());

    CorpusDefinition {
        channels: schema::channels(),
        categories: schema::categories(),
        references: schema::references(),
        scenarios,
    }
}
