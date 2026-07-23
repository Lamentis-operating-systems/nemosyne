mod corpus;
mod scenario;

use std::collections::BTreeSet;

use nemosyne_core::activation::{CandidateId, ChannelId, UnitInterval};
use nemosyne_evaluation::activation::ScenarioId;

use self::corpus::{build_partition, build_references, validate_corpus_relationships};
use self::scenario::{materialize_scenario, validate_categories, validate_channels};
use super::{
    ActivationEvidenceCorpus, CorpusError, CorpusRevision, CorpusSplit, EvidenceChannelDefinition,
    EvidenceLevel, FactId, ReferenceId, ScenarioCategory, ScenarioCategoryId, SemanticCaseId,
};

const TRIGGER_CHANNEL_ID: ChannelId = ChannelId::new(10);
const REVISION_ONE_CHANNELS: [ChannelId; 5] = [
    ChannelId::new(10),
    ChannelId::new(20),
    ChannelId::new(30),
    ChannelId::new(40),
    ChannelId::new(50),
];
const ALL_LEVELS: [EvidenceLevel; 5] = [
    EvidenceLevel::Absent,
    EvidenceLevel::Low,
    EvidenceLevel::Medium,
    EvidenceLevel::High,
    EvidenceLevel::Maximal,
];

#[derive(Clone)]
pub(super) struct CorpusDefinition {
    pub channels: Vec<EvidenceChannelDefinition>,
    pub categories: Vec<ScenarioCategory>,
    pub references: Vec<ReferenceDefinition>,
    pub scenarios: Vec<ScenarioDefinition>,
}

#[derive(Clone)]
pub(super) struct ReferenceDefinition {
    pub reference_id: ReferenceId,
    pub key: &'static str,
    pub rationale: &'static str,
    pub weights: RevisionOneVector,
}

#[derive(Clone)]
pub(super) struct ScenarioDefinition {
    pub scenario_id: ScenarioId,
    pub semantic_case_id: SemanticCaseId,
    pub category_id: ScenarioCategoryId,
    pub split: CorpusSplit,
    pub title: &'static str,
    pub situation: &'static str,
    pub facts: Vec<FactDefinition>,
    pub gates: JudgmentDefinition,
    pub candidates: Vec<CandidateDefinition>,
    pub preferences: Vec<PreferenceDefinition>,
}

#[derive(Clone)]
pub(super) struct FactDefinition {
    pub fact_id: FactId,
    pub statement: &'static str,
}

#[derive(Clone)]
pub(super) struct JudgmentDefinition {
    pub channels: RevisionOneJudgments,
}

#[derive(Clone, Copy)]
pub(super) struct ChannelJudgmentDefinition {
    pub level: EvidenceLevel,
    pub fact_ids: &'static [u64],
    pub rationale: &'static str,
}

impl ChannelJudgmentDefinition {
    pub(super) const fn new(
        level: EvidenceLevel,
        fact_ids: &'static [u64],
        rationale: &'static str,
    ) -> Self {
        Self {
            level,
            fact_ids,
            rationale,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct RevisionOneJudgments {
    trigger_alignment: ChannelJudgmentDefinition,
    observed_state_alignment: ChannelJudgmentDefinition,
    active_outcome_alignment: ChannelJudgmentDefinition,
    capability_fit: ChannelJudgmentDefinition,
    constraint_alignment: ChannelJudgmentDefinition,
}

impl RevisionOneJudgments {
    pub(super) const fn new(
        trigger_alignment: ChannelJudgmentDefinition,
        observed_state_alignment: ChannelJudgmentDefinition,
        active_outcome_alignment: ChannelJudgmentDefinition,
        capability_fit: ChannelJudgmentDefinition,
        constraint_alignment: ChannelJudgmentDefinition,
    ) -> Self {
        Self {
            trigger_alignment,
            observed_state_alignment,
            active_outcome_alignment,
            capability_fit,
            constraint_alignment,
        }
    }

    pub(super) const fn canonical(self) -> [ChannelJudgmentDefinition; 5] {
        [
            self.trigger_alignment,
            self.observed_state_alignment,
            self.active_outcome_alignment,
            self.capability_fit,
            self.constraint_alignment,
        ]
    }

    #[cfg(test)]
    pub(super) fn set_level(&mut self, channel_id: ChannelId, level: EvidenceLevel) {
        match channel_id.get() {
            10 => self.trigger_alignment.level = level,
            20 => self.observed_state_alignment.level = level,
            30 => self.active_outcome_alignment.level = level,
            40 => self.capability_fit.level = level,
            50 => self.constraint_alignment.level = level,
            _ => panic!("revision-1 test helper received an unknown channel"),
        }
    }

    #[cfg(test)]
    pub(super) fn set_fact_ids(&mut self, channel_id: ChannelId, fact_ids: &'static [u64]) {
        match channel_id.get() {
            10 => self.trigger_alignment.fact_ids = fact_ids,
            20 => self.observed_state_alignment.fact_ids = fact_ids,
            30 => self.active_outcome_alignment.fact_ids = fact_ids,
            40 => self.capability_fit.fact_ids = fact_ids,
            50 => self.constraint_alignment.fact_ids = fact_ids,
            _ => panic!("revision-1 test helper received an unknown channel"),
        }
    }
}

#[derive(Clone)]
pub(super) struct CandidateDefinition {
    pub candidate_id: CandidateId,
    pub label: &'static str,
    pub judgment: JudgmentDefinition,
}

#[derive(Clone)]
pub(super) struct PreferenceDefinition {
    pub preferred: CandidateId,
    pub other: CandidateId,
    pub fact_ids: &'static [u64],
    pub rationale: &'static str,
}

#[derive(Clone, Copy)]
pub(super) struct RevisionOneVector {
    trigger_alignment: EvidenceLevel,
    observed_state_alignment: EvidenceLevel,
    active_outcome_alignment: EvidenceLevel,
    capability_fit: EvidenceLevel,
    constraint_alignment: EvidenceLevel,
}

impl RevisionOneVector {
    pub(super) const fn new(
        trigger_alignment: EvidenceLevel,
        observed_state_alignment: EvidenceLevel,
        active_outcome_alignment: EvidenceLevel,
        capability_fit: EvidenceLevel,
        constraint_alignment: EvidenceLevel,
    ) -> Self {
        Self {
            trigger_alignment,
            observed_state_alignment,
            active_outcome_alignment,
            capability_fit,
            constraint_alignment,
        }
    }

    const fn canonical_levels(self) -> [EvidenceLevel; 5] {
        [
            self.trigger_alignment,
            self.observed_state_alignment,
            self.active_outcome_alignment,
            self.capability_fit,
            self.constraint_alignment,
        ]
    }
}

pub(super) fn build_revision_one(
    mut definition: CorpusDefinition,
) -> Result<ActivationEvidenceCorpus, CorpusError> {
    validate_channels(&mut definition.channels)?;
    validate_categories(&mut definition.categories)?;

    let category_ids: BTreeSet<_> = definition
        .categories
        .iter()
        .map(ScenarioCategory::category_id)
        .collect();
    let mut materialized = Vec::with_capacity(definition.scenarios.len());
    for scenario in definition.scenarios {
        materialized.push(materialize_scenario(
            scenario,
            &definition.channels,
            &category_ids,
        )?);
    }
    materialized.sort_unstable_by_key(|scenario| scenario.evidence.scenario_id());
    validate_corpus_relationships(&materialized, &definition.categories)?;

    let calibration = build_partition(CorpusSplit::Calibration, &materialized)?;
    let held_out = build_partition(CorpusSplit::HeldOut, &materialized)?;
    let references = build_references(
        definition.references,
        &definition.channels,
        &calibration,
        &held_out,
    )?;

    Ok(ActivationEvidenceCorpus::new(
        CorpusRevision::new(1),
        definition.channels.into_boxed_slice(),
        definition.categories.into_boxed_slice(),
        references,
        calibration,
        held_out,
    ))
}

fn validate_text(context: String, value: &str) -> Result<(), CorpusError> {
    if value.is_empty() || value.trim() != value {
        return Err(CorpusError::InvalidText {
            context: context.into_boxed_str(),
        });
    }
    Ok(())
}

fn unit(level: EvidenceLevel) -> UnitInterval {
    UnitInterval::new(level.as_f64()).expect("evidence levels are valid unit-interval values")
}

#[cfg(test)]
mod tests;
