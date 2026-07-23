use std::error::Error;

use nemosyne_core::activation::{
    ActivationCandidate, CandidateId, ChannelId, ChannelSignal, UnitInterval,
};
use nemosyne_evaluation::activation::{
    ActivationParameter, ActivationParameters, EvaluationError, EvaluationScenario,
    EvaluationSuite, EvidenceGate, EvidenceParameter, ExpectedPreference, PreferenceOutcome,
    ScenarioId, evaluate_parameters,
};

use super::corpus::{build_partition, canonicalize_algebraic_preference_coefficients};
use super::{ALL_LEVELS, RevisionOneVector, build_revision_one};
use crate::activation::coding_agent::coding_agent_definition;
use crate::activation::{
    CorpusError, CorpusSplit, EvidenceChannelDefinition, EvidenceLevel, FactId,
    FactReferenceLocation, ReferenceId, ScenarioCategory, ScenarioCategoryId,
};

#[test]
fn rejects_duplicate_and_invalid_channel_schemas() {
    let mut duplicate = coding_agent_definition();
    duplicate.channels.push(duplicate.channels[0].clone());
    assert_eq!(
        build_revision_one(duplicate),
        Err(CorpusError::DuplicateChannel {
            channel_id: ChannelId::new(10),
        })
    );

    let mut missing = coding_agent_definition();
    missing.channels.remove(0);
    assert_eq!(
        build_revision_one(missing),
        Err(CorpusError::InvalidChannelSet {
            channel_ids: [
                ChannelId::new(20),
                ChannelId::new(30),
                ChannelId::new(40),
                ChannelId::new(50),
            ]
            .into(),
        })
    );
}

#[test]
fn permuted_authoring_collections_produce_the_same_complete_artifact() {
    let canonical =
        build_revision_one(coding_agent_definition()).expect("canonical definition is valid");
    let mut definition = coding_agent_definition();
    definition.channels.reverse();
    definition.categories.reverse();
    definition.references.reverse();
    let scenario = definition
        .scenarios
        .iter_mut()
        .find(|scenario| scenario.scenario_id == ScenarioId::new(1001))
        .expect("permutation probe scenario exists");
    scenario
        .gates
        .channels
        .set_fact_ids(ChannelId::new(10), &[5, 2]);
    scenario
        .candidates
        .iter_mut()
        .find(|candidate| candidate.candidate_id == CandidateId::new(1))
        .expect("permutation probe candidate exists")
        .judgment
        .channels
        .set_fact_ids(ChannelId::new(10), &[5, 2]);
    scenario.preferences[0].fact_ids = &[3, 2, 1];
    definition.scenarios.reverse();
    for scenario in &mut definition.scenarios {
        scenario.facts.reverse();
        scenario.candidates.reverse();
        scenario.preferences.reverse();
    }

    let permuted = build_revision_one(definition).expect("permuted definition is valid");
    assert_eq!(permuted, canonical);
    assert_eq!(
        permuted.regression_fingerprint(),
        canonical.regression_fingerprint()
    );

    for (canonical_reference, permuted_reference) in
        canonical.references().iter().zip(permuted.references())
    {
        for (canonical_partition, permuted_partition) in [
            (canonical.calibration(), permuted.calibration()),
            (canonical.held_out(), permuted.held_out()),
        ] {
            assert_eq!(
                evaluate_parameters(
                    canonical_reference.parameters(),
                    canonical_partition.suite(),
                )
                .expect("canonical reference evaluates"),
                evaluate_parameters(permuted_reference.parameters(), permuted_partition.suite(),)
                    .expect("permuted reference evaluates"),
            );
        }
    }
}

#[test]
fn rejects_invalid_metadata_and_category_references() {
    let mut invalid_text = coding_agent_definition();
    invalid_text.categories[0] =
        crate::activation::ScenarioCategory::new(ScenarioCategoryId::new(10), " padded ", "valid");
    assert!(matches!(
        build_revision_one(invalid_text),
        Err(CorpusError::InvalidText { .. })
    ));

    let mut duplicate_category = coding_agent_definition();
    duplicate_category
        .categories
        .push(duplicate_category.categories[0].clone());
    assert_eq!(
        build_revision_one(duplicate_category),
        Err(CorpusError::DuplicateCategory {
            category_id: ScenarioCategoryId::new(10),
        })
    );

    let mut unknown_category = coding_agent_definition();
    unknown_category.scenarios[0].category_id = ScenarioCategoryId::new(999);
    assert_eq!(
        build_revision_one(unknown_category),
        Err(CorpusError::UnknownCategory {
            scenario_id: ScenarioId::new(1001),
            category_id: ScenarioCategoryId::new(999),
        })
    );
}

#[test]
fn rejects_duplicate_stable_keys_in_each_namespace() {
    let mut duplicate_channel_key = coding_agent_definition();
    let replacement = {
        let duplicate_key = duplicate_channel_key.channels[0].key();
        let channel = &duplicate_channel_key.channels[1];
        EvidenceChannelDefinition::new(
            channel.channel_id(),
            duplicate_key,
            channel.gate_meaning(),
            channel.signal_meaning(),
            ALL_LEVELS.map(|level| channel.gate_anchor(level)),
            ALL_LEVELS.map(|level| channel.signal_anchor(level)),
        )
    };
    duplicate_channel_key.channels[1] = replacement;
    assert_eq!(
        build_revision_one(duplicate_channel_key),
        Err(CorpusError::DuplicateChannelKey {
            key: "trigger_alignment".into(),
        })
    );

    let mut duplicate_category_key = coding_agent_definition();
    let replacement = {
        let duplicate_key = duplicate_category_key.categories[0].key();
        let category = &duplicate_category_key.categories[1];
        ScenarioCategory::new(
            category.category_id(),
            duplicate_key,
            category.description(),
        )
    };
    duplicate_category_key.categories[1] = replacement;
    assert_eq!(
        build_revision_one(duplicate_category_key),
        Err(CorpusError::DuplicateCategoryKey {
            key: "active_constraints".into(),
        })
    );

    let mut duplicate_reference_key = coding_agent_definition();
    duplicate_reference_key.references[1].key = duplicate_reference_key.references[0].key;
    assert_eq!(
        build_revision_one(duplicate_reference_key),
        Err(CorpusError::DuplicateReferenceKey {
            key: "trigger_only".into(),
        })
    );
}

#[test]
fn rejects_duplicate_unknown_and_empty_fact_evidence() {
    let mut duplicate = coding_agent_definition();
    let duplicate_fact = duplicate.scenarios[0].facts[0].clone();
    duplicate.scenarios[0].facts.push(duplicate_fact);
    assert_eq!(
        build_revision_one(duplicate),
        Err(CorpusError::DuplicateFact {
            scenario_id: ScenarioId::new(1001),
            fact_id: FactId::new(1),
        })
    );

    let mut unknown = coding_agent_definition();
    unknown.scenarios[0]
        .gates
        .channels
        .set_fact_ids(ChannelId::new(10), &[999]);
    assert_eq!(
        build_revision_one(unknown),
        Err(CorpusError::UnknownFact {
            scenario_id: ScenarioId::new(1001),
            location: FactReferenceLocation::Gate {
                channel_id: ChannelId::new(10),
            },
            fact_id: FactId::new(999),
        })
    );

    let mut empty = coding_agent_definition();
    empty.scenarios[0].candidates[0]
        .judgment
        .channels
        .set_fact_ids(ChannelId::new(10), &[]);
    assert_eq!(
        build_revision_one(empty),
        Err(CorpusError::EmptyFactReferences {
            scenario_id: ScenarioId::new(1001),
            location: FactReferenceLocation::CandidateSignal {
                candidate_id: CandidateId::new(1),
                channel_id: ChannelId::new(10),
            },
        })
    );
}

#[test]
fn rejects_duplicate_fact_references() {
    let mut duplicate = coding_agent_definition();
    duplicate.scenarios[0].preferences[0].fact_ids = &[1, 1];
    assert_eq!(
        build_revision_one(duplicate),
        Err(CorpusError::DuplicateFactReference {
            scenario_id: ScenarioId::new(1001),
            location: FactReferenceLocation::Preference {
                preferred: CandidateId::new(1),
                other: CandidateId::new(2),
            },
            fact_id: FactId::new(1),
        })
    );
}

#[test]
fn rejects_duplicate_scenarios_and_cross_split_cases() {
    let mut duplicate = coding_agent_definition();
    duplicate.scenarios.push(duplicate.scenarios[0].clone());
    assert_eq!(
        build_revision_one(duplicate),
        Err(CorpusError::DuplicateScenario {
            scenario_id: ScenarioId::new(1001),
        })
    );

    let mut cross_split = coding_agent_definition();
    cross_split.scenarios[1].split = CorpusSplit::HeldOut;
    assert!(matches!(
        build_revision_one(cross_split),
        Err(CorpusError::CrossSplitSemanticCase { .. })
    ));
}

#[test]
fn rejects_cross_category_cases_and_invalid_pair_cardinality() {
    let mut cross_category = coding_agent_definition();
    cross_category.scenarios[1].category_id = ScenarioCategoryId::new(20);
    assert_eq!(
        build_revision_one(cross_category),
        Err(CorpusError::CrossCategorySemanticCase {
            semantic_case_id: crate::activation::SemanticCaseId::new(100),
        })
    );

    let mut invalid_cardinality = coding_agent_definition();
    invalid_cardinality.scenarios.remove(1);
    assert_eq!(
        build_revision_one(invalid_cardinality),
        Err(CorpusError::InvalidPairCardinality {
            semantic_case_id: crate::activation::SemanticCaseId::new(100),
            count: 1,
        })
    );
}

#[test]
fn rejects_pair_identity_and_preference_drift() {
    let mut candidate_drift = coding_agent_definition();
    candidate_drift.scenarios[1].candidates[0].label = "Different candidate identity";
    assert!(matches!(
        build_revision_one(candidate_drift),
        Err(CorpusError::PairedCandidateMismatch { .. })
    ));

    let mut preference_drift = coding_agent_definition();
    preference_drift.scenarios[1].preferences[0].preferred = CandidateId::new(1);
    preference_drift.scenarios[1].preferences[0].other = CandidateId::new(2);
    assert!(matches!(
        build_revision_one(preference_drift),
        Err(CorpusError::PairedPreferenceMismatch { .. })
    ));
}

#[test]
fn rejects_indistinct_pairs_and_missing_category_coverage() {
    let mut indistinct = coding_agent_definition();
    let first = indistinct.scenarios[0].clone();
    indistinct.scenarios[1].gates.channels = first.gates.channels;
    for (candidate, first_candidate) in indistinct.scenarios[1]
        .candidates
        .iter_mut()
        .zip(first.candidates)
    {
        candidate.judgment.channels = first_candidate.judgment.channels;
    }
    assert!(matches!(
        build_revision_one(indistinct),
        Err(CorpusError::IndistinctPair { .. })
    ));

    let mut missing_coverage = coding_agent_definition();
    missing_coverage
        .scenarios
        .retain(|scenario| scenario.semantic_case_id.get() != 200);
    assert_eq!(
        build_revision_one(missing_coverage),
        Err(CorpusError::MissingCategoryCoverage {
            category_id: ScenarioCategoryId::new(40),
            split: CorpusSplit::HeldOut,
        })
    );
}

#[test]
fn rejects_cross_split_algebraic_preference_signature_leakage() {
    let mut leaked = coding_agent_definition();
    let calibration = leaked.scenarios[0].clone();
    let held_out = leaked
        .scenarios
        .iter_mut()
        .find(|scenario| scenario.scenario_id == ScenarioId::new(2201))
        .expect("held-out fixture exists");
    held_out.gates.channels = calibration.gates.channels;
    for (candidate, calibration_candidate) in
        held_out.candidates.iter_mut().zip(calibration.candidates)
    {
        candidate.judgment.channels = calibration_candidate.judgment.channels;
    }

    assert_eq!(
        build_revision_one(leaked),
        Err(CorpusError::CrossSplitAlgebraicPreferenceSignature {
            calibration_scenario_id: ScenarioId::new(1001),
            held_out_scenario_id: ScenarioId::new(2201),
        })
    );
}

#[test]
fn canonicalizes_one_sign_algebraic_signatures_by_signed_support() {
    let mut first_positive = [4, 4, 4, 4, 0];
    let mut second_positive = [12, 12, 9, 4, 0];
    canonicalize_algebraic_preference_coefficients(&mut first_positive);
    canonicalize_algebraic_preference_coefficients(&mut second_positive);
    assert_eq!(first_positive, [1, 1, 1, 1, 0]);
    assert_eq!(second_positive, first_positive);

    let mut first_negative = [-4, -4, 0, -8, 0];
    let mut second_negative = [-1, -9, 0, -2, 0];
    canonicalize_algebraic_preference_coefficients(&mut first_negative);
    canonicalize_algebraic_preference_coefficients(&mut second_negative);
    assert_eq!(first_negative, [-1, -1, 0, -1, 0]);
    assert_eq!(second_negative, first_negative);

    let mut zero = [0; 5];
    canonicalize_algebraic_preference_coefficients(&mut zero);
    assert_eq!(zero, [0; 5]);
}

#[test]
fn canonicalizes_mixed_sign_algebraic_signatures_by_positive_ray() {
    let mut first = [-4, 8, 12, 0, 0];
    let mut proportional = [-2, 4, 6, 0, 0];
    let mut different = [-4, 8, 3, 0, 0];
    canonicalize_algebraic_preference_coefficients(&mut first);
    canonicalize_algebraic_preference_coefficients(&mut proportional);
    canonicalize_algebraic_preference_coefficients(&mut different);

    assert_eq!(first, [-1, 2, 3, 0, 0]);
    assert_eq!(proportional, first);
    assert_eq!(different, [-4, 8, 3, 0, 0]);
    assert_ne!(different, first);
}

#[test]
fn algebraic_signature_does_not_claim_exact_f64_outcome_equivalence() {
    let mut small_difference = [1, 0, 0, 0, 0];
    let mut large_difference = [16, 0, 0, 0, 0];
    canonicalize_algebraic_preference_coefficients(&mut small_difference);
    canonicalize_algebraic_preference_coefficients(&mut large_difference);
    assert_eq!(small_difference, large_difference);

    let tied = evaluate_f64_boundary_case([0.25, 1.0], [0.5, 0.5], [0.25, 0.5]);
    assert_eq!(
        tied,
        (
            0x3fe0_0000_0000_0000,
            0x3fe0_0000_0000_0000,
            PreferenceOutcome::Tied,
        )
    );

    let satisfied = evaluate_f64_boundary_case([1.0, 1.0], [1.0, 0.5], [0.0, 0.5]);
    assert_eq!(
        satisfied,
        (
            0x3fe0_0000_0000_0001,
            0x3fe0_0000_0000_0000,
            PreferenceOutcome::Satisfied,
        )
    );
}

#[test]
fn rejects_an_inactive_trigger_gate_and_invalid_reference_parameters() {
    let mut inactive_trigger = coding_agent_definition();
    inactive_trigger.scenarios[0]
        .gates
        .channels
        .set_level(ChannelId::new(10), EvidenceLevel::Absent);
    assert_eq!(
        build_revision_one(inactive_trigger),
        Err(CorpusError::InactiveTriggerGate {
            scenario_id: ScenarioId::new(1001),
        })
    );

    let mut inactive_signal = coding_agent_definition();
    inactive_signal.scenarios[1].candidates[0]
        .judgment
        .channels
        .set_level(ChannelId::new(50), EvidenceLevel::Low);
    assert_eq!(
        build_revision_one(inactive_signal),
        Err(CorpusError::NonCanonicalInactiveSignal {
            scenario_id: ScenarioId::new(1002),
            candidate_id: CandidateId::new(1),
            channel_id: ChannelId::new(50),
        })
    );

    let mut invalid_reference = coding_agent_definition();
    invalid_reference.references[0].weights = RevisionOneVector::new(
        EvidenceLevel::Absent,
        EvidenceLevel::Absent,
        EvidenceLevel::Absent,
        EvidenceLevel::Absent,
        EvidenceLevel::Absent,
    );
    let error = build_revision_one(invalid_reference).expect_err("zero weights must be rejected");
    assert!(matches!(
        &error,
        CorpusError::ReferenceParameters {
            reference_id,
            ..
        } if *reference_id == ReferenceId::new(10)
    ));
    assert!(error.source().is_some());
}

#[test]
fn rejects_duplicate_references_and_empty_partitions() {
    let mut duplicate_reference = coding_agent_definition();
    duplicate_reference
        .references
        .push(duplicate_reference.references[0].clone());
    assert_eq!(
        build_revision_one(duplicate_reference),
        Err(CorpusError::DuplicateReference {
            reference_id: ReferenceId::new(10),
        })
    );

    assert_eq!(
        build_partition(CorpusSplit::HeldOut, &[]),
        Err(CorpusError::EmptyPartition {
            split: CorpusSplit::HeldOut,
        })
    );
}

#[test]
fn preserves_evaluator_scenario_and_reference_evaluation_errors() {
    let mut invalid_scenario = coding_agent_definition();
    invalid_scenario.scenarios[0].candidates[1].candidate_id = CandidateId::new(1);
    let error =
        build_revision_one(invalid_scenario).expect_err("duplicate candidates must be rejected");
    assert!(matches!(
        &error,
        CorpusError::Scenario {
            scenario_id,
            source: EvaluationError::DuplicateCandidate { candidate_id, .. },
        } if *scenario_id == ScenarioId::new(1001) && *candidate_id == CandidateId::new(1)
    ));
    assert!(error.source().is_some());

    let mut invalid_reference_evaluation = coding_agent_definition();
    invalid_reference_evaluation.references[0].weights = RevisionOneVector::new(
        EvidenceLevel::Absent,
        EvidenceLevel::Absent,
        EvidenceLevel::Absent,
        EvidenceLevel::Absent,
        EvidenceLevel::Maximal,
    );
    let error = build_revision_one(invalid_reference_evaluation)
        .expect_err("constraint-only parameters cannot evaluate every scenario");
    assert!(matches!(
        &error,
        CorpusError::ReferenceEvaluation {
            reference_id,
            split: CorpusSplit::Calibration,
            ..
        } if *reference_id == ReferenceId::new(10)
    ));
    assert!(error.source().is_some());
}

fn evaluate_f64_boundary_case(
    gates: [f64; 2],
    preferred: [f64; 2],
    other: [f64; 2],
) -> (u64, u64, PreferenceOutcome) {
    let channel_ids = [ChannelId::new(10), ChannelId::new(20)];
    let parameters = ActivationParameters::new(
        channel_ids
            .into_iter()
            .zip([1.0e-16, 1.0])
            .map(|(channel_id, weight)| {
                ActivationParameter::from(EvidenceParameter::new(channel_id, interval(weight)))
            })
            .collect(),
    )
    .expect("boundary parameters are valid");
    let scenario = EvaluationScenario::new(
        ScenarioId::new(1),
        channel_ids
            .into_iter()
            .zip(gates)
            .map(|(channel_id, gate)| EvidenceGate::new(channel_id, interval(gate)))
            .collect(),
        vec![
            boundary_candidate(CandidateId::new(1), channel_ids, preferred),
            boundary_candidate(CandidateId::new(2), channel_ids, other),
        ],
        vec![ExpectedPreference::new(
            CandidateId::new(1),
            CandidateId::new(2),
        )],
    )
    .expect("boundary scenario is valid");
    let suite = EvaluationSuite::new(vec![scenario]).expect("boundary suite is valid");
    let report = evaluate_parameters(&parameters, &suite).expect("boundary case must evaluate");
    let preference = report.scenarios()[0].preferences()[0];
    (
        preference.preferred_score().get().to_bits(),
        preference.other_score().get().to_bits(),
        preference.outcome(),
    )
}

fn boundary_candidate(
    candidate_id: CandidateId,
    channel_ids: [ChannelId; 2],
    signals: [f64; 2],
) -> ActivationCandidate {
    ActivationCandidate::new(
        candidate_id,
        channel_ids
            .into_iter()
            .zip(signals)
            .map(|(channel_id, signal)| ChannelSignal::new(channel_id, interval(signal)))
            .collect(),
    )
    .expect("boundary candidate is valid")
}

fn interval(value: f64) -> UnitInterval {
    UnitInterval::new(value).expect("test value is in the unit interval")
}
