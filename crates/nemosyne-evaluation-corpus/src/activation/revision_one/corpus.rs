use std::collections::{BTreeMap, BTreeSet};

use nemosyne_core::activation::CandidateId;
use nemosyne_evaluation::activation::{
    ActivationParameter, ActivationParameters, EvaluationSuite, EvidenceParameter,
    evaluate_parameters,
};

use super::scenario::MaterializedScenario;
use super::{ReferenceDefinition, unit, validate_text};
use crate::activation::{
    AnchoredValue, CandidateEvidence, CorpusError, CorpusPartition, CorpusSplit,
    EvidenceChannelDefinition, EvidenceLevel, PreferenceEvidence, ReferenceParameterSet,
    ScenarioCategory, ScenarioEvidence, SemanticCaseId,
};

pub(super) fn validate_corpus_relationships(
    scenarios: &[MaterializedScenario],
    categories: &[ScenarioCategory],
) -> Result<(), CorpusError> {
    let mut scenario_ids = BTreeSet::new();
    let mut cases: BTreeMap<SemanticCaseId, Vec<&MaterializedScenario>> = BTreeMap::new();
    let mut category_splits = BTreeSet::new();

    for scenario in scenarios {
        if !scenario_ids.insert(scenario.evidence.scenario_id()) {
            return Err(CorpusError::DuplicateScenario {
                scenario_id: scenario.evidence.scenario_id(),
            });
        }
        cases
            .entry(scenario.evidence.semantic_case_id())
            .or_default()
            .push(scenario);
        category_splits.insert((scenario.evidence.category_id(), scenario.split));
    }

    for (semantic_case_id, pair) in cases {
        if pair.len() != 2 {
            return Err(CorpusError::InvalidPairCardinality {
                semantic_case_id,
                count: pair.len(),
            });
        }
        if pair[0].split != pair[1].split {
            return Err(CorpusError::CrossSplitSemanticCase { semantic_case_id });
        }
        if pair[0].evidence.category_id() != pair[1].evidence.category_id() {
            return Err(CorpusError::CrossCategorySemanticCase { semantic_case_id });
        }
        if candidate_identity(&pair[0].evidence) != candidate_identity(&pair[1].evidence) {
            return Err(CorpusError::PairedCandidateMismatch { semantic_case_id });
        }
        if !preferences_are_reversed(&pair[0].evidence, &pair[1].evidence) {
            return Err(CorpusError::PairedPreferenceMismatch { semantic_case_id });
        }
        if authored_shape(&pair[0].evidence) == authored_shape(&pair[1].evidence) {
            return Err(CorpusError::IndistinctPair { semantic_case_id });
        }
    }

    validate_cross_split_preference_shapes(scenarios)?;

    for category in categories {
        for split in [CorpusSplit::Calibration, CorpusSplit::HeldOut] {
            if !category_splits.contains(&(category.category_id(), split)) {
                return Err(CorpusError::MissingCategoryCoverage {
                    category_id: category.category_id(),
                    split,
                });
            }
        }
    }

    Ok(())
}

fn validate_cross_split_preference_shapes(
    scenarios: &[MaterializedScenario],
) -> Result<(), CorpusError> {
    let mut calibration_shapes = BTreeMap::new();
    for scenario in scenarios
        .iter()
        .filter(|scenario| scenario.split == CorpusSplit::Calibration)
    {
        for preference in scenario.evidence.preferences() {
            calibration_shapes
                .entry(preference_shape(&scenario.evidence, preference))
                .or_insert(scenario.evidence.scenario_id());
        }
    }

    for scenario in scenarios
        .iter()
        .filter(|scenario| scenario.split == CorpusSplit::HeldOut)
    {
        for preference in scenario.evidence.preferences() {
            if let Some(&calibration_scenario_id) =
                calibration_shapes.get(&preference_shape(&scenario.evidence, preference))
            {
                return Err(CorpusError::CrossSplitPreferenceShape {
                    calibration_scenario_id,
                    held_out_scenario_id: scenario.evidence.scenario_id(),
                });
            }
        }
    }

    Ok(())
}

fn candidate_identity(evidence: &ScenarioEvidence) -> Vec<(CandidateId, &str)> {
    evidence
        .candidates()
        .iter()
        .map(|candidate| (candidate.candidate_id(), candidate.label()))
        .collect()
}

fn preferences_are_reversed(first: &ScenarioEvidence, second: &ScenarioEvidence) -> bool {
    if first.preferences().len() != second.preferences().len() {
        return false;
    }
    let mut reversed: Vec<_> = first
        .preferences()
        .iter()
        .map(|preference| {
            let expectation = preference.expectation();
            (expectation.other(), expectation.preferred())
        })
        .collect();
    reversed.sort_unstable();
    let second_preferences: Vec<_> = second
        .preferences()
        .iter()
        .map(|preference| {
            let expectation = preference.expectation();
            (expectation.preferred(), expectation.other())
        })
        .collect();
    reversed == second_preferences
}

fn authored_shape(evidence: &ScenarioEvidence) -> AuthoredShape {
    AuthoredShape {
        gates: evidence
            .gates()
            .values()
            .iter()
            .map(AnchoredValue::level)
            .collect(),
        candidates: evidence
            .candidates()
            .iter()
            .map(|candidate| {
                (
                    candidate.candidate_id(),
                    candidate
                        .signals()
                        .iter()
                        .map(AnchoredValue::level)
                        .collect(),
                )
            })
            .collect(),
    }
}

fn preference_shape(
    evidence: &ScenarioEvidence,
    preference: &PreferenceEvidence,
) -> PreferenceShape {
    let expectation = preference.expectation();
    let preferred = candidate_levels(evidence, expectation.preferred());
    let other = candidate_levels(evidence, expectation.other());
    let mut coefficients: Vec<_> = evidence
        .gates()
        .values()
        .iter()
        .map(AnchoredValue::level)
        .zip(preferred.iter().copied().zip(other))
        .map(|(gate, (preferred, other))| {
            level_index(gate) * (level_index(preferred) - level_index(other))
        })
        .collect();
    canonicalize_preference_coefficients(&mut coefficients);
    PreferenceShape { coefficients }
}

pub(super) fn canonicalize_preference_coefficients(coefficients: &mut [i16]) {
    let has_positive = coefficients.iter().any(|coefficient| *coefficient > 0);
    let has_negative = coefficients.iter().any(|coefficient| *coefficient < 0);

    match (has_positive, has_negative) {
        (true, false) => {
            for coefficient in coefficients {
                *coefficient = i16::from(*coefficient > 0);
            }
        }
        (false, true) => {
            for coefficient in coefficients {
                *coefficient = -i16::from(*coefficient < 0);
            }
        }
        (true, true) => normalize_positive_ray(coefficients),
        (false, false) => {}
    }
}

fn normalize_positive_ray(coefficients: &mut [i16]) {
    let divisor = coefficients
        .iter()
        .map(|coefficient| coefficient.unsigned_abs())
        .filter(|coefficient| *coefficient != 0)
        .reduce(greatest_common_divisor)
        .unwrap_or(1);
    for coefficient in coefficients {
        *coefficient /= divisor as i16;
    }
}

fn candidate_levels(evidence: &ScenarioEvidence, candidate_id: CandidateId) -> Vec<EvidenceLevel> {
    let index = evidence
        .candidates()
        .binary_search_by_key(&candidate_id, CandidateEvidence::candidate_id)
        .expect("validated preferences refer to canonical candidates");
    evidence.candidates()[index]
        .signals()
        .iter()
        .map(AnchoredValue::level)
        .collect()
}

fn level_index(level: EvidenceLevel) -> i16 {
    level.index() as i16
}

fn greatest_common_divisor(mut left: u16, mut right: u16) -> u16 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

pub(super) fn build_partition(
    split: CorpusSplit,
    scenarios: &[MaterializedScenario],
) -> Result<CorpusPartition, CorpusError> {
    let selected: Vec<_> = scenarios
        .iter()
        .filter(|scenario| scenario.split == split)
        .collect();
    if selected.is_empty() {
        return Err(CorpusError::EmptyPartition { split });
    }

    let evidence: Box<[_]> = selected
        .iter()
        .map(|scenario| scenario.evidence.clone())
        .collect();
    let suite = EvaluationSuite::new(
        selected
            .into_iter()
            .map(|scenario| scenario.evaluation.clone())
            .collect(),
    )
    .map_err(|source| CorpusError::Suite { split, source })?;

    Ok(CorpusPartition::new(split, evidence, suite))
}

pub(super) fn build_references(
    mut definitions: Vec<ReferenceDefinition>,
    channels: &[EvidenceChannelDefinition],
    calibration: &CorpusPartition,
    held_out: &CorpusPartition,
) -> Result<Box<[ReferenceParameterSet]>, CorpusError> {
    definitions.sort_unstable_by_key(|reference| reference.reference_id);
    if let Some(pair) = definitions
        .windows(2)
        .find(|pair| pair[0].reference_id == pair[1].reference_id)
    {
        return Err(CorpusError::DuplicateReference {
            reference_id: pair[0].reference_id,
        });
    }

    let channel_ids: Vec<_> = channels
        .iter()
        .map(EvidenceChannelDefinition::channel_id)
        .collect();
    let mut references = Vec::with_capacity(definitions.len());
    for definition in definitions {
        validate_text(
            format!("reference {} key", definition.reference_id.get()),
            definition.key,
        )?;
        validate_text(
            format!("reference {} rationale", definition.reference_id.get()),
            definition.rationale,
        )?;
        let parameters = ActivationParameters::new(
            channel_ids
                .iter()
                .copied()
                .zip(definition.weights.canonical_levels())
                .map(|(channel_id, level)| {
                    ActivationParameter::from(EvidenceParameter::new(channel_id, unit(level)))
                })
                .collect(),
        )
        .map_err(|source| CorpusError::ReferenceParameters {
            reference_id: definition.reference_id,
            source,
        })?;
        for partition in [calibration, held_out] {
            evaluate_parameters(&parameters, partition.suite()).map_err(|source| {
                CorpusError::ReferenceEvaluation {
                    reference_id: definition.reference_id,
                    split: partition.split(),
                    source,
                }
            })?;
        }
        references.push(ReferenceParameterSet::new(
            definition.reference_id,
            definition.key,
            definition.rationale,
            parameters,
        ));
    }

    Ok(references.into_boxed_slice())
}

#[derive(Eq, PartialEq)]
struct AuthoredShape {
    gates: Vec<EvidenceLevel>,
    candidates: Vec<(CandidateId, Vec<EvidenceLevel>)>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct PreferenceShape {
    coefficients: Vec<i16>,
}
