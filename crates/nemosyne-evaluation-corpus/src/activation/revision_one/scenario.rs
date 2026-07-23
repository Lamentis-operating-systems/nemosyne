use std::collections::BTreeSet;

use nemosyne_core::activation::{ActivationCandidate, CandidateId, ChannelId, ChannelSignal};
use nemosyne_evaluation::activation::{
    EvaluationScenario, EvidenceGate, ExpectedPreference, ScenarioId,
};

use super::{
    ALL_LEVELS, JudgmentDefinition, REVISION_ONE_CHANNELS, ScenarioDefinition, TRIGGER_CHANNEL_ID,
    unit, validate_text,
};
use crate::activation::{
    AnchoredValue, CandidateEvidence, CorpusError, CorpusSplit, EvidenceChannelDefinition,
    EvidenceLevel, FactId, GateEvidence, JudgmentApplicability, PreferenceEvidence,
    ScenarioCategory, ScenarioCategoryId, ScenarioEvidence, ScenarioFact,
};

pub(super) fn validate_channels(
    channels: &mut Vec<EvidenceChannelDefinition>,
) -> Result<(), CorpusError> {
    channels.sort_unstable_by_key(EvidenceChannelDefinition::channel_id);
    if let Some(pair) = channels
        .windows(2)
        .find(|pair| pair[0].channel_id() == pair[1].channel_id())
    {
        return Err(CorpusError::DuplicateChannel {
            channel_id: pair[0].channel_id(),
        });
    }
    let channel_ids: Box<[_]> = channels
        .iter()
        .map(EvidenceChannelDefinition::channel_id)
        .collect();
    if channel_ids.as_ref() != REVISION_ONE_CHANNELS {
        return Err(CorpusError::InvalidChannelSet { channel_ids });
    }

    for channel in channels {
        validate_text(
            format!("channel {} key", channel.channel_id().get()),
            channel.key(),
        )?;
        validate_text(
            format!("channel {} gate meaning", channel.channel_id().get()),
            channel.gate_meaning(),
        )?;
        validate_text(
            format!("channel {} signal meaning", channel.channel_id().get()),
            channel.signal_meaning(),
        )?;
        for level in ALL_LEVELS {
            validate_text(
                format!(
                    "channel {} {:?} gate anchor",
                    channel.channel_id().get(),
                    level
                ),
                channel.gate_anchor(level),
            )?;
            validate_text(
                format!(
                    "channel {} {:?} signal anchor",
                    channel.channel_id().get(),
                    level
                ),
                channel.signal_anchor(level),
            )?;
        }
    }

    Ok(())
}

pub(super) fn validate_categories(
    categories: &mut Vec<ScenarioCategory>,
) -> Result<(), CorpusError> {
    categories.sort_unstable_by_key(ScenarioCategory::category_id);
    if let Some(pair) = categories
        .windows(2)
        .find(|pair| pair[0].category_id() == pair[1].category_id())
    {
        return Err(CorpusError::DuplicateCategory {
            category_id: pair[0].category_id(),
        });
    }

    for category in categories {
        validate_text(
            format!("category {} key", category.category_id().get()),
            category.key(),
        )?;
        validate_text(
            format!("category {} description", category.category_id().get()),
            category.description(),
        )?;
    }

    Ok(())
}

pub(super) fn materialize_scenario(
    mut definition: ScenarioDefinition,
    channels: &[EvidenceChannelDefinition],
    category_ids: &BTreeSet<ScenarioCategoryId>,
) -> Result<MaterializedScenario, CorpusError> {
    let scenario_id = definition.scenario_id;
    validate_text(
        format!("scenario {} title", scenario_id.get()),
        definition.title,
    )?;
    validate_text(
        format!("scenario {} situation", scenario_id.get()),
        definition.situation,
    )?;
    if !category_ids.contains(&definition.category_id) {
        return Err(CorpusError::UnknownCategory {
            scenario_id,
            category_id: definition.category_id,
        });
    }

    definition.facts.sort_unstable_by_key(|fact| fact.fact_id);
    if let Some(pair) = definition
        .facts
        .windows(2)
        .find(|pair| pair[0].fact_id == pair[1].fact_id)
    {
        return Err(CorpusError::DuplicateFact {
            scenario_id,
            fact_id: pair[0].fact_id,
        });
    }
    for fact in &definition.facts {
        validate_text(
            format!("scenario {} fact {}", scenario_id.get(), fact.fact_id.get()),
            fact.statement,
        )?;
    }
    let known_facts: BTreeSet<_> = definition.facts.iter().map(|fact| fact.fact_id).collect();

    let gate_values = anchored_gate_values(scenario_id, channels, definition.gates, &known_facts)?;
    let trigger_gate = gate_values
        .binary_search_by_key(&TRIGGER_CHANNEL_ID, AnchoredValue::channel_id)
        .ok()
        .map(|index| gate_values[index].level());
    if trigger_gate == Some(EvidenceLevel::Absent) {
        return Err(CorpusError::InactiveTriggerGate { scenario_id });
    }

    definition
        .candidates
        .sort_unstable_by_key(|candidate| candidate.candidate_id);
    let mut candidates = Vec::with_capacity(definition.candidates.len());
    let mut activation_candidates = Vec::with_capacity(definition.candidates.len());
    for candidate in definition.candidates {
        validate_text(
            format!(
                "scenario {} candidate {} label",
                scenario_id.get(),
                candidate.candidate_id.get()
            ),
            candidate.label,
        )?;
        let signals = anchored_signal_values(
            scenario_id,
            candidate.candidate_id,
            channels,
            candidate.judgment,
            &gate_values,
            &known_facts,
        )?;
        let activation_candidate = ActivationCandidate::new(
            candidate.candidate_id,
            signals
                .iter()
                .map(|signal| ChannelSignal::new(signal.channel_id(), unit(signal.level())))
                .collect(),
        )
        .map_err(|source| CorpusError::Activation {
            scenario_id,
            source,
        })?;
        candidates.push(CandidateEvidence::new(
            candidate.candidate_id,
            candidate.label,
            signals,
        ));
        activation_candidates.push(activation_candidate);
    }

    definition
        .preferences
        .sort_unstable_by_key(|preference| (preference.preferred, preference.other));
    let mut preferences = Vec::with_capacity(definition.preferences.len());
    let mut expected_preferences = Vec::with_capacity(definition.preferences.len());
    for preference in definition.preferences {
        validate_text(
            format!(
                "scenario {} preference {} over {} rationale",
                scenario_id.get(),
                preference.preferred.get(),
                preference.other.get()
            ),
            preference.rationale,
        )?;
        let fact_ids = validated_fact_references(scenario_id, preference.fact_ids, &known_facts)?;
        let expectation = ExpectedPreference::new(preference.preferred, preference.other);
        preferences.push(PreferenceEvidence::new(
            expectation,
            fact_ids,
            preference.rationale,
        ));
        expected_preferences.push(expectation);
    }

    let evidence = ScenarioEvidence::new(
        scenario_id,
        definition.semantic_case_id,
        definition.category_id,
        definition.title,
        definition.situation,
        definition
            .facts
            .into_iter()
            .map(|fact| ScenarioFact::new(fact.fact_id, fact.statement))
            .collect(),
        GateEvidence::new(gate_values),
        candidates.into_boxed_slice(),
        preferences.into_boxed_slice(),
    );
    let evaluation = EvaluationScenario::new(
        scenario_id,
        evidence
            .gates()
            .values()
            .iter()
            .map(|gate| EvidenceGate::new(gate.channel_id(), unit(gate.level())))
            .collect(),
        activation_candidates,
        expected_preferences,
    )
    .map_err(|source| CorpusError::Scenario {
        scenario_id,
        source,
    })?;

    Ok(MaterializedScenario {
        split: definition.split,
        evidence,
        evaluation,
    })
}

fn anchored_gate_values(
    scenario_id: ScenarioId,
    channels: &[EvidenceChannelDefinition],
    judgment: JudgmentDefinition,
    known_facts: &BTreeSet<FactId>,
) -> Result<Box<[AnchoredValue]>, CorpusError> {
    materialize_judgment(
        scenario_id,
        None,
        channels,
        judgment,
        known_facts,
        &BTreeSet::new(),
    )
}

fn anchored_signal_values(
    scenario_id: ScenarioId,
    candidate_id: CandidateId,
    channels: &[EvidenceChannelDefinition],
    judgment: JudgmentDefinition,
    gates: &[AnchoredValue],
    known_facts: &BTreeSet<FactId>,
) -> Result<Box<[AnchoredValue]>, CorpusError> {
    let inactive_channels = gates
        .iter()
        .filter(|gate| gate.level() == EvidenceLevel::Absent)
        .map(AnchoredValue::channel_id)
        .collect();
    materialize_judgment(
        scenario_id,
        Some(candidate_id),
        channels,
        judgment,
        known_facts,
        &inactive_channels,
    )
}

fn materialize_judgment(
    scenario_id: ScenarioId,
    candidate_id: Option<CandidateId>,
    channels: &[EvidenceChannelDefinition],
    judgment: JudgmentDefinition,
    known_facts: &BTreeSet<FactId>,
    inactive_channels: &BTreeSet<ChannelId>,
) -> Result<Box<[AnchoredValue]>, CorpusError> {
    channels
        .iter()
        .zip(judgment.channels.canonical())
        .map(|(channel, authored)| {
            let fact_ids = validated_fact_references(scenario_id, authored.fact_ids, known_facts)?;
            let context = candidate_id.map_or_else(
                || {
                    format!(
                        "scenario {} gate channel {} rationale",
                        scenario_id.get(),
                        channel.channel_id().get()
                    )
                },
                |candidate_id| {
                    format!(
                        "scenario {} candidate {} channel {} rationale",
                        scenario_id.get(),
                        candidate_id.get(),
                        channel.channel_id().get()
                    )
                },
            );
            validate_text(context, authored.rationale)?;

            let applicability = if inactive_channels.contains(&channel.channel_id()) {
                if authored.level != EvidenceLevel::Absent {
                    return Err(CorpusError::NonCanonicalInactiveSignal {
                        scenario_id,
                        candidate_id: candidate_id
                            .expect("only candidate signals can have inactive channels"),
                        channel_id: channel.channel_id(),
                    });
                }
                JudgmentApplicability::Inactive
            } else {
                JudgmentApplicability::Applicable
            };

            Ok(AnchoredValue::new(
                channel.channel_id(),
                authored.level,
                applicability,
                fact_ids,
                authored.rationale,
            ))
        })
        .collect()
}

fn validated_fact_references(
    scenario_id: ScenarioId,
    fact_ids: &[u64],
    known_facts: &BTreeSet<FactId>,
) -> Result<Box<[FactId]>, CorpusError> {
    let mut references: Vec<_> = fact_ids.iter().copied().map(FactId::new).collect();
    if references.is_empty() {
        return Err(CorpusError::EmptyFactReferences { scenario_id });
    }
    references.sort_unstable();
    if let Some(pair) = references.windows(2).find(|pair| pair[0] == pair[1]) {
        return Err(CorpusError::DuplicateFactReference {
            scenario_id,
            fact_id: pair[0],
        });
    }
    for &fact_id in &references {
        if !known_facts.contains(&fact_id) {
            return Err(CorpusError::UnknownFact {
                scenario_id,
                fact_id,
            });
        }
    }
    Ok(references.into_boxed_slice())
}

#[derive(Clone)]
pub(super) struct MaterializedScenario {
    pub(super) split: CorpusSplit,
    pub(super) evidence: ScenarioEvidence,
    pub(super) evaluation: EvaluationScenario,
}
