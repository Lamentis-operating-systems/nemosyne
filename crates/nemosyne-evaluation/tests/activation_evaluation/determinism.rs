use nemosyne_evaluation::activation::{PreferenceOutcome, evaluate_parameters};

use super::{candidate, evidence, gate, inhibition, parameters, preference, scenario, suite};

#[test]
fn every_permutation_boundary_produces_an_identical_report() {
    let first_parameters = parameters(vec![
        inhibition(3, 0.4),
        evidence(2, 0.25),
        evidence(1, 0.75),
    ]);
    let second_parameters = parameters(vec![
        evidence(1, 0.75),
        evidence(2, 0.25),
        inhibition(3, 0.4),
    ]);

    let first_suite = suite(vec![
        scenario(
            20,
            vec![gate(2, 0.5), gate(1, 1.0)],
            vec![
                candidate(2, &[(3, 0.1), (1, 0.4), (2, 0.8)]),
                candidate(1, &[(2, 0.2), (3, 0.5), (1, 0.9)]),
                candidate(3, &[(1, 0.1), (2, 0.1), (3, 0.0)]),
            ],
            vec![preference(2, 3), preference(1, 2)],
        ),
        scenario(
            10,
            vec![gate(2, 1.0), gate(1, 0.5)],
            vec![
                candidate(5, &[(3, 0.0), (2, 0.3), (1, 0.9)]),
                candidate(4, &[(1, 0.2), (3, 0.2), (2, 0.8)]),
            ],
            vec![preference(5, 4)],
        ),
    ]);
    let second_suite = suite(vec![
        scenario(
            10,
            vec![gate(1, 0.5), gate(2, 1.0)],
            vec![
                candidate(4, &[(2, 0.8), (1, 0.2), (3, 0.2)]),
                candidate(5, &[(1, 0.9), (2, 0.3), (3, 0.0)]),
            ],
            vec![preference(5, 4)],
        ),
        scenario(
            20,
            vec![gate(1, 1.0), gate(2, 0.5)],
            vec![
                candidate(3, &[(3, 0.0), (2, 0.1), (1, 0.1)]),
                candidate(1, &[(1, 0.9), (2, 0.2), (3, 0.5)]),
                candidate(2, &[(2, 0.8), (1, 0.4), (3, 0.1)]),
            ],
            vec![preference(1, 2), preference(2, 3)],
        ),
    ]);

    let first = evaluate_parameters(&first_parameters, &first_suite)
        .expect("first evaluation must succeed");
    let second = evaluate_parameters(&second_parameters, &second_suite)
        .expect("second evaluation must succeed");

    assert_eq!(first, second);
    assert_eq!(first.scenarios()[0].scenario_id().get(), 10);
    assert_eq!(first.scenarios()[1].scenario_id().get(), 20);
    assert_eq!(
        first.scenarios()[1].preferences()[0].outcome(),
        PreferenceOutcome::Satisfied
    );
}

#[test]
fn constructors_expose_canonical_parameter_scenario_and_preference_order() {
    let parameters = parameters(vec![inhibition(9, 0.5), evidence(4, 0.4), evidence(1, 0.6)]);
    assert_eq!(
        parameters
            .parameters()
            .iter()
            .map(|parameter| parameter.channel_id().get())
            .collect::<Vec<_>>(),
        vec![1, 4, 9]
    );

    let suite = suite(vec![
        scenario(
            8,
            vec![gate(4, 1.0), gate(1, 1.0)],
            vec![
                candidate(3, &[(9, 0.0), (4, 0.3), (1, 0.2)]),
                candidate(1, &[(1, 0.9), (4, 0.8), (9, 0.0)]),
                candidate(2, &[(4, 0.5), (9, 0.0), (1, 0.6)]),
            ],
            vec![preference(2, 3), preference(1, 2)],
        ),
        scenario(
            2,
            vec![gate(1, 1.0), gate(4, 1.0)],
            vec![
                candidate(5, &[(1, 0.8), (4, 0.8), (9, 0.0)]),
                candidate(6, &[(1, 0.4), (4, 0.4), (9, 0.0)]),
            ],
            vec![preference(5, 6)],
        ),
    ]);

    assert_eq!(
        suite
            .scenarios()
            .iter()
            .map(|scenario| scenario.scenario_id().get())
            .collect::<Vec<_>>(),
        vec![2, 8]
    );
    let scenario = &suite.scenarios()[1];
    assert_eq!(
        scenario
            .gates()
            .iter()
            .map(|gate| gate.channel_id().get())
            .collect::<Vec<_>>(),
        vec![1, 4]
    );
    assert_eq!(
        scenario
            .candidates()
            .iter()
            .map(|candidate| candidate.candidate_id().get())
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert_eq!(
        scenario
            .preferences()
            .iter()
            .map(|preference| (preference.preferred().get(), preference.other().get()))
            .collect::<Vec<_>>(),
        vec![(1, 2), (2, 3)]
    );
}
