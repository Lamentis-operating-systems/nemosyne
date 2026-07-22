use nemosyne_core::activation::{ActivationError, CandidateId, ChannelId};
use nemosyne_evaluation::activation::{
    ActivationParameters, EvaluationError, EvaluationScenario, EvaluationSuite, ScenarioId,
    evaluate_parameters,
};

use super::{candidate, evidence, gate, inhibition, parameters, preference, scenario, suite};

#[test]
fn parameters_reject_duplicate_channels_and_absent_positive_evidence() {
    assert!(matches!(
        ActivationParameters::new(vec![evidence(1, 1.0), inhibition(1, 0.5)]),
        Err(EvaluationError::DuplicateParameterChannel { channel_id })
            if channel_id == ChannelId::new(1)
    ));
    assert!(matches!(
        ActivationParameters::new(vec![evidence(1, 0.0), inhibition(2, 0.5)]),
        Err(EvaluationError::NoPositiveEvidenceWeight)
    ));
}

#[test]
fn suites_reject_empty_and_duplicate_scenario_sets() {
    assert!(matches!(
        EvaluationSuite::new(vec![]),
        Err(EvaluationError::EmptySuite)
    ));

    let first = scenario(
        7,
        vec![gate(1, 1.0)],
        vec![candidate(1, &[(1, 1.0)]), candidate(2, &[(1, 0.0)])],
        vec![preference(1, 2)],
    );
    let second = first.clone();
    assert!(matches!(
        EvaluationSuite::new(vec![first, second]),
        Err(EvaluationError::DuplicateScenario { scenario_id })
            if scenario_id == ScenarioId::new(7)
    ));
}

#[test]
fn scenarios_require_two_candidates_and_one_preference() {
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(1),
            vec![gate(1, 1.0)],
            vec![candidate(1, &[(1, 1.0)])],
            vec![preference(1, 1)],
        ),
        Err(EvaluationError::TooFewCandidates { .. })
    ));
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(1),
            vec![gate(1, 1.0)],
            vec![candidate(1, &[(1, 1.0)]), candidate(2, &[(1, 0.0)])],
            vec![],
        ),
        Err(EvaluationError::NoPreferences { .. })
    ));
}

#[test]
fn scenarios_reject_duplicate_gates_and_candidates() {
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(1),
            vec![gate(1, 0.5), gate(1, 1.0)],
            vec![candidate(1, &[(1, 1.0)]), candidate(2, &[(1, 0.0)])],
            vec![preference(1, 2)],
        ),
        Err(EvaluationError::DuplicateEvidenceGate { channel_id, .. })
            if channel_id == ChannelId::new(1)
    ));

    let duplicate = candidate(3, &[(1, 0.5)]);
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(1),
            vec![gate(1, 1.0)],
            vec![duplicate.clone(), duplicate],
            vec![preference(3, 4)],
        ),
        Err(EvaluationError::DuplicateCandidate { candidate_id, .. })
            if candidate_id == CandidateId::new(3)
    ));
}

#[test]
fn scenarios_reject_invalid_preference_graphs() {
    let candidates = vec![
        candidate(1, &[(1, 0.9)]),
        candidate(2, &[(1, 0.5)]),
        candidate(3, &[(1, 0.1)]),
    ];

    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(4),
            vec![gate(1, 1.0)],
            candidates.clone(),
            vec![preference(1, 9)],
        ),
        Err(EvaluationError::UnknownPreferenceCandidate { candidate_id, .. })
            if candidate_id == CandidateId::new(9)
    ));
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(4),
            vec![gate(1, 1.0)],
            candidates.clone(),
            vec![preference(1, 1)],
        ),
        Err(EvaluationError::SelfPreference { candidate_id, .. })
            if candidate_id == CandidateId::new(1)
    ));
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(4),
            vec![gate(1, 1.0)],
            candidates.clone(),
            vec![preference(1, 2), preference(1, 2)],
        ),
        Err(EvaluationError::DuplicatePreference { .. })
    ));
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(4),
            vec![gate(1, 1.0)],
            candidates.clone(),
            vec![preference(1, 2), preference(2, 1)],
        ),
        Err(EvaluationError::CyclicPreferences { .. })
    ));
    assert!(matches!(
        EvaluationScenario::new(
            ScenarioId::new(4),
            vec![gate(1, 1.0)],
            candidates,
            vec![preference(1, 2), preference(2, 3), preference(3, 1)],
        ),
        Err(EvaluationError::CyclicPreferences { scenario_id })
            if scenario_id == ScenarioId::new(4)
    ));
}

#[test]
fn scenarios_reject_transitively_redundant_preferences() {
    let result = EvaluationScenario::new(
        ScenarioId::new(5),
        vec![gate(1, 1.0)],
        vec![
            candidate(1, &[(1, 0.9)]),
            candidate(2, &[(1, 0.5)]),
            candidate(3, &[(1, 0.1)]),
        ],
        vec![preference(1, 2), preference(2, 3), preference(1, 3)],
    );

    assert!(matches!(
        result,
        Err(EvaluationError::RedundantPreference {
            scenario_id,
            preferred,
            other,
        }) if scenario_id == ScenarioId::new(5)
            && preferred == CandidateId::new(1)
            && other == CandidateId::new(3)
    ));
}

#[test]
fn scenarios_detect_cycles_in_disconnected_components() {
    let result = EvaluationScenario::new(
        ScenarioId::new(6),
        vec![gate(1, 1.0)],
        vec![
            candidate(1, &[(1, 0.9)]),
            candidate(2, &[(1, 0.8)]),
            candidate(3, &[(1, 0.7)]),
            candidate(4, &[(1, 0.6)]),
            candidate(5, &[(1, 0.5)]),
        ],
        vec![
            preference(1, 2),
            preference(3, 4),
            preference(4, 5),
            preference(5, 3),
        ],
    );

    assert!(matches!(
        result,
        Err(EvaluationError::CyclicPreferences { scenario_id })
            if scenario_id == ScenarioId::new(6)
    ));
}

#[test]
fn evaluation_requires_exactly_the_evidence_gates_defined_by_parameters() {
    let parameters = parameters(vec![evidence(1, 1.0), inhibition(2, 0.5)]);

    let missing = suite(vec![scenario(
        7,
        vec![],
        vec![
            candidate(1, &[(1, 1.0), (2, 0.0)]),
            candidate(2, &[(1, 0.0), (2, 0.0)]),
        ],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&parameters, &missing),
        Err(EvaluationError::MissingEvidenceGate {
            scenario_id,
            channel_id,
        }) if scenario_id == ScenarioId::new(7) && channel_id == ChannelId::new(1)
    ));

    let unknown = suite(vec![scenario(
        8,
        vec![gate(1, 1.0), gate(9, 0.5)],
        vec![
            candidate(1, &[(1, 1.0), (2, 0.0)]),
            candidate(2, &[(1, 0.0), (2, 0.0)]),
        ],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&parameters, &unknown),
        Err(EvaluationError::UnexpectedEvidenceGate {
            scenario_id,
            channel_id,
        }) if scenario_id == ScenarioId::new(8) && channel_id == ChannelId::new(9)
    ));

    let inhibition_gate = suite(vec![scenario(
        9,
        vec![gate(1, 1.0), gate(2, 0.5)],
        vec![
            candidate(1, &[(1, 1.0), (2, 0.0)]),
            candidate(2, &[(1, 0.0), (2, 0.0)]),
        ],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&parameters, &inhibition_gate),
        Err(EvaluationError::UnexpectedEvidenceGate {
            scenario_id,
            channel_id,
        }) if scenario_id == ScenarioId::new(9) && channel_id == ChannelId::new(2)
    ));
}

#[test]
fn zero_weight_evidence_channels_remain_part_of_the_exact_schema() {
    let parameters = parameters(vec![evidence(1, 1.0), evidence(2, 0.0)]);
    let missing_gate = suite(vec![scenario(
        10,
        vec![gate(1, 1.0)],
        vec![
            candidate(1, &[(1, 1.0), (2, 0.0)]),
            candidate(2, &[(1, 0.0), (2, 0.0)]),
        ],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&parameters, &missing_gate),
        Err(EvaluationError::MissingEvidenceGate {
            scenario_id,
            channel_id,
        }) if scenario_id == ScenarioId::new(10) && channel_id == ChannelId::new(2)
    ));

    let missing_signal = suite(vec![scenario(
        11,
        vec![gate(1, 1.0), gate(2, 0.0)],
        vec![candidate(1, &[(1, 1.0)]), candidate(2, &[(1, 0.0)])],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&parameters, &missing_signal),
        Err(EvaluationError::Activation {
            scenario_id,
            source: ActivationError::MissingSignal {
                candidate_id,
                channel_id,
            },
        }) if scenario_id == ScenarioId::new(11)
            && candidate_id == CandidateId::new(1)
            && channel_id == ChannelId::new(2)
    ));
}

#[test]
fn kernel_errors_preserve_the_scenario_context() {
    let parameters = parameters(vec![evidence(1, 1.0)]);
    let missing_signal = suite(vec![scenario(
        42,
        vec![gate(1, 1.0)],
        vec![candidate(1, &[]), candidate(2, &[])],
        vec![preference(1, 2)],
    )]);

    let missing_signal_error =
        evaluate_parameters(&parameters, &missing_signal).expect_err("evaluation must fail");
    assert!(std::error::Error::source(&missing_signal_error).is_some());
    assert!(matches!(
        missing_signal_error,
        EvaluationError::Activation {
            scenario_id,
            source: ActivationError::MissingSignal {
                candidate_id,
                channel_id,
            },
        } if scenario_id == ScenarioId::new(42)
            && candidate_id == CandidateId::new(1)
            && channel_id == ChannelId::new(1)
    ));

    let inactive = suite(vec![scenario(
        43,
        vec![gate(1, 0.0)],
        vec![candidate(1, &[(1, 1.0)]), candidate(2, &[(1, 0.0)])],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&parameters, &inactive),
        Err(EvaluationError::Activation {
            scenario_id,
            source: ActivationError::NoEffectiveEvidence,
        }) if scenario_id == ScenarioId::new(43)
    ));

    let unexpected_signal = suite(vec![scenario(
        44,
        vec![gate(1, 1.0)],
        vec![
            candidate(1, &[(1, 1.0), (9, 0.5)]),
            candidate(2, &[(1, 0.0), (9, 0.0)]),
        ],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&parameters, &unexpected_signal),
        Err(EvaluationError::Activation {
            scenario_id,
            source: ActivationError::UnexpectedSignal {
                candidate_id,
                channel_id,
            },
        }) if scenario_id == ScenarioId::new(44)
            && candidate_id == CandidateId::new(1)
            && channel_id == ChannelId::new(9)
    ));
}

#[test]
fn numeric_kernel_errors_preserve_the_scenario_context() {
    let smallest_positive = f64::from_bits(1);
    let effective_underflow_parameters = parameters(vec![evidence(1, smallest_positive)]);
    let effective_underflow = suite(vec![scenario(
        50,
        vec![gate(1, 0.5)],
        vec![candidate(1, &[(1, 1.0)]), candidate(2, &[(1, 0.0)])],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&effective_underflow_parameters, &effective_underflow),
        Err(EvaluationError::Activation {
            scenario_id,
            source: ActivationError::EffectiveWeightUnderflow { channel_id },
        }) if scenario_id == ScenarioId::new(50) && channel_id == ChannelId::new(1)
    ));

    let normalization_parameters = parameters(vec![
        evidence(1, smallest_positive),
        evidence(2, 1.0),
        evidence(3, 1.0),
    ]);
    let normalization_underflow = suite(vec![scenario(
        51,
        vec![gate(1, 1.0), gate(2, 1.0), gate(3, 1.0)],
        vec![
            candidate(1, &[(1, 1.0), (2, 1.0), (3, 1.0)]),
            candidate(2, &[(1, 0.0), (2, 0.0), (3, 0.0)]),
        ],
        vec![preference(1, 2)],
    )]);
    assert!(matches!(
        evaluate_parameters(&normalization_parameters, &normalization_underflow),
        Err(EvaluationError::Activation {
            scenario_id,
            source: ActivationError::NormalizedWeightUnderflow { channel_id },
        }) if scenario_id == ScenarioId::new(51) && channel_id == ChannelId::new(1)
    ));
}

#[test]
fn canonical_scenario_order_determines_the_first_activation_error() {
    let parameters = parameters(vec![evidence(1, 1.0)]);
    let later = scenario(
        20,
        vec![gate(1, 1.0)],
        vec![candidate(1, &[]), candidate(2, &[])],
        vec![preference(1, 2)],
    );
    let earlier = scenario(
        10,
        vec![gate(1, 0.0)],
        vec![candidate(1, &[(1, 1.0)]), candidate(2, &[(1, 0.0)])],
        vec![preference(1, 2)],
    );

    assert!(matches!(
        evaluate_parameters(&parameters, &suite(vec![later, earlier])),
        Err(EvaluationError::Activation {
            scenario_id,
            source: ActivationError::NoEffectiveEvidence,
        }) if scenario_id == ScenarioId::new(10)
    ));
}
