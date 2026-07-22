use nemosyne_core::activation::CandidateId;
use nemosyne_evaluation::activation::{PreferenceOutcome, evaluate_parameters};

use super::{
    assert_close, candidate, evidence, gate, inhibition, parameters, preference, scenario, suite,
};

#[test]
fn hand_calculated_multi_scenario_report_reproduces_all_metrics() {
    let parameters = parameters(vec![
        evidence(1, 0.75),
        evidence(2, 0.25),
        inhibition(3, 0.5),
    ]);
    let suite = suite(vec![
        scenario(
            10,
            vec![gate(1, 1.0), gate(2, 1.0)],
            vec![
                candidate(1, &[(1, 0.8), (2, 0.4), (3, 0.2)]),
                candidate(2, &[(1, 0.4), (2, 0.4), (3, 0.0)]),
                candidate(3, &[(1, 0.8), (2, 0.4), (3, 0.2)]),
                candidate(4, &[(1, 0.9), (2, 0.9), (3, 0.0)]),
            ],
            vec![preference(1, 2), preference(1, 3), preference(1, 4)],
        ),
        scenario(
            20,
            vec![gate(1, 0.0), gate(2, 1.0)],
            vec![
                candidate(11, &[(1, 0.0), (2, 0.8), (3, 0.0)]),
                candidate(12, &[(1, 0.0), (2, 1.0), (3, 0.6)]),
            ],
            vec![preference(11, 12)],
        ),
    ]);

    let report = evaluate_parameters(&parameters, &suite).expect("evaluation must succeed");

    assert_eq!(report.scenario_count(), 2);
    assert_eq!(report.preference_count(), 4);
    assert_eq!(report.satisfied_count(), 2);
    assert_eq!(report.tied_count(), 1);
    assert_eq!(report.violated_count(), 1);
    assert_eq!(report.passed_scenario_count(), 1);
    assert_close(report.micro_accuracy(), 0.5);
    assert_close(report.macro_accuracy(), 2.0 / 3.0);
    assert_close(report.scenario_pass_rate(), 0.5);

    let first = &report.scenarios()[0];
    assert_eq!(first.preference_count(), 3);
    assert_eq!(first.satisfied_count(), 1);
    assert_eq!(first.tied_count(), 1);
    assert_eq!(first.violated_count(), 1);
    assert_close(first.accuracy(), 1.0 / 3.0);
    assert!(!first.passed());

    assert_eq!(first.ranking()[0].candidate_id(), CandidateId::new(4));
    assert_close(first.ranking()[0].score().get(), 0.9);
    assert_eq!(first.ranking()[1].candidate_id(), CandidateId::new(1));
    assert_close(first.ranking()[1].score().get(), 0.63);
    assert_eq!(first.ranking()[2].candidate_id(), CandidateId::new(3));
    assert_close(first.ranking()[2].score().get(), 0.63);
    assert_eq!(first.ranking()[3].candidate_id(), CandidateId::new(2));
    assert_close(first.ranking()[3].score().get(), 0.4);

    let preference_results = first.preferences();
    assert_eq!(preference_results[0].expectation(), preference(1, 2));
    assert_eq!(
        preference_results[0].outcome(),
        PreferenceOutcome::Satisfied
    );
    assert_close(preference_results[0].preferred_score().get(), 0.63);
    assert_close(preference_results[0].other_score().get(), 0.4);
    assert_eq!(preference_results[1].outcome(), PreferenceOutcome::Tied);
    assert_eq!(preference_results[2].outcome(), PreferenceOutcome::Violated);

    let second = &report.scenarios()[1];
    assert_eq!(second.satisfied_count(), 1);
    assert_eq!(second.tied_count(), 0);
    assert_eq!(second.violated_count(), 0);
    assert_eq!(second.accuracy(), 1.0);
    assert!(second.passed());
    assert_close(second.ranking()[0].score().get(), 0.8);
    assert_close(second.ranking()[1].score().get(), 0.7);
}

#[test]
fn exact_score_tie_is_not_satisfied_by_candidate_id_order() {
    let parameters = parameters(vec![evidence(1, 1.0)]);
    let suite = suite(vec![scenario(
        1,
        vec![gate(1, 1.0)],
        vec![candidate(2, &[(1, 0.5)]), candidate(1, &[(1, 0.5)])],
        vec![preference(1, 2)],
    )]);

    let report = evaluate_parameters(&parameters, &suite).expect("evaluation must succeed");
    let result = &report.scenarios()[0];

    assert_eq!(result.ranking()[0].candidate_id(), CandidateId::new(1));
    assert_eq!(result.preferences()[0].outcome(), PreferenceOutcome::Tied);
    assert_eq!(result.satisfied_count(), 0);
    assert_eq!(result.tied_count(), 1);
    assert_eq!(result.accuracy(), 0.0);
    assert!(!result.passed());
}

#[test]
fn one_representable_score_step_is_a_strict_preference() {
    let lower = 0.5_f64;
    let higher = f64::from_bits(lower.to_bits() + 1);
    let parameters = parameters(vec![evidence(1, 1.0)]);
    let suite = suite(vec![scenario(
        1,
        vec![gate(1, 1.0)],
        vec![candidate(1, &[(1, higher)]), candidate(2, &[(1, lower)])],
        vec![preference(1, 2)],
    )]);

    let report = evaluate_parameters(&parameters, &suite).expect("evaluation must succeed");
    let result = &report.scenarios()[0].preferences()[0];
    assert_eq!(result.outcome(), PreferenceOutcome::Satisfied);
    assert!(result.preferred_score().get() > result.other_score().get());
}

#[test]
fn partial_order_scores_only_declared_preferences() {
    let parameters = parameters(vec![evidence(1, 1.0)]);
    let suite = suite(vec![scenario(
        1,
        vec![gate(1, 1.0)],
        vec![
            candidate(1, &[(1, 0.9)]),
            candidate(2, &[(1, 0.8)]),
            candidate(3, &[(1, 0.1)]),
        ],
        vec![preference(1, 2)],
    )]);

    let report = evaluate_parameters(&parameters, &suite).expect("evaluation must succeed");
    let result = &report.scenarios()[0];

    assert_eq!(result.ranking().len(), 3);
    assert_eq!(result.preferences().len(), 1);
    assert_eq!(report.preference_count(), 1);
    assert_eq!(report.satisfied_count(), 1);
    assert_eq!(report.micro_accuracy(), 1.0);
}

#[test]
fn two_explicit_parameter_sets_can_be_compared_on_the_same_suite() {
    let suite = suite(vec![scenario(
        1,
        vec![gate(1, 1.0), gate(2, 1.0)],
        vec![
            candidate(1, &[(1, 1.0), (2, 0.0)]),
            candidate(2, &[(1, 0.0), (2, 1.0)]),
        ],
        vec![preference(1, 2)],
    )]);
    let first = parameters(vec![evidence(1, 0.8), evidence(2, 0.2)]);
    let second = parameters(vec![evidence(1, 0.2), evidence(2, 0.8)]);

    let first_report = evaluate_parameters(&first, &suite).expect("evaluation must succeed");
    let second_report = evaluate_parameters(&second, &suite).expect("evaluation must succeed");

    assert_eq!(
        first_report.scenarios()[0].preferences()[0].outcome(),
        PreferenceOutcome::Satisfied
    );
    assert_eq!(first_report.micro_accuracy(), 1.0);
    assert_eq!(
        second_report.scenarios()[0].preferences()[0].outcome(),
        PreferenceOutcome::Violated
    );
    assert_eq!(second_report.micro_accuracy(), 0.0);
}

#[test]
fn outcome_counts_always_partition_all_preferences() {
    let parameters = parameters(vec![evidence(1, 1.0)]);
    let suite = suite(vec![scenario(
        1,
        vec![gate(1, 1.0)],
        vec![
            candidate(1, &[(1, 0.5)]),
            candidate(2, &[(1, 0.4)]),
            candidate(3, &[(1, 0.5)]),
            candidate(4, &[(1, 0.6)]),
        ],
        vec![preference(1, 2), preference(1, 3), preference(1, 4)],
    )]);

    let report = evaluate_parameters(&parameters, &suite).expect("evaluation must succeed");
    assert_eq!(
        report.satisfied_count() + report.tied_count() + report.violated_count(),
        report.preference_count()
    );
    for result in report.scenarios() {
        assert_eq!(
            result.satisfied_count() + result.tied_count() + result.violated_count(),
            result.preference_count()
        );
    }
}
