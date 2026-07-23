use nemosyne_evaluation::activation::{
    EvaluationReport, PreferenceOutcome, PreferenceOutcome::*, ScenarioId,
};
use nemosyne_evaluation_corpus::activation::CorpusSplit;

use super::{corpus, report};

#[test]
fn references_produce_frozen_scenario_observations() {
    let corpus = corpus();
    let observations = [
        (
            "trigger_only",
            CorpusSplit::Calibration,
            TRIGGER_CALIBRATION,
        ),
        ("trigger_only", CorpusSplit::HeldOut, TRIGGER_HELD_OUT),
        (
            "uniform_evidence",
            CorpusSplit::Calibration,
            UNIFORM_CALIBRATION,
        ),
        ("uniform_evidence", CorpusSplit::HeldOut, UNIFORM_HELD_OUT),
    ];

    for (reference, split, expected) in observations {
        let partition = match split {
            CorpusSplit::Calibration => corpus.calibration(),
            CorpusSplit::HeldOut => corpus.held_out(),
        };
        assert_report(
            reference,
            split,
            &report(&corpus, reference, partition),
            expected,
        );
    }
}

#[test]
fn references_are_distinct_measurement_points() {
    let corpus = corpus();

    assert_ne!(
        report(&corpus, "trigger_only", corpus.calibration()),
        report(&corpus, "uniform_evidence", corpus.calibration())
    );
    assert_ne!(
        report(&corpus, "trigger_only", corpus.held_out()),
        report(&corpus, "uniform_evidence", corpus.held_out())
    );
}

#[test]
fn repeated_reference_evaluation_is_deterministic() {
    let first = corpus();
    let second = corpus();

    for reference in ["trigger_only", "uniform_evidence"] {
        assert_eq!(
            report(&first, reference, first.calibration()),
            report(&second, reference, second.calibration())
        );
        assert_eq!(
            report(&first, reference, first.held_out()),
            report(&second, reference, second.held_out())
        );
    }
}

fn assert_report(
    reference: &str,
    split: CorpusSplit,
    report: &EvaluationReport,
    expected: &[ExpectedScenario],
) {
    let context = format!("{reference}/{split:?}");
    assert_eq!(
        report.scenarios().len(),
        expected.len(),
        "{context}: scenario count"
    );

    for (scenario, expected) in report.scenarios().iter().zip(expected) {
        let context = format!("{context}/{}", expected.scenario_id);
        assert_eq!(
            scenario.scenario_id(),
            ScenarioId::new(expected.scenario_id),
            "{context}: scenario order"
        );
        assert_eq!(
            scenario.ranking().len(),
            expected.ranking.len(),
            "{context}: ranking length"
        );

        for (activation, &(candidate_id, score_bits)) in
            scenario.ranking().iter().zip(expected.ranking)
        {
            assert_eq!(
                activation.candidate_id().get(),
                candidate_id,
                "{context}: candidate order"
            );
            assert_eq!(
                activation.evidence_score().get().to_bits(),
                score_bits,
                "{context}: evidence score"
            );
            assert_eq!(
                activation.retention().get().to_bits(),
                1.0_f64.to_bits(),
                "{context}: retention"
            );
            assert_eq!(
                activation.score().get().to_bits(),
                score_bits,
                "{context}: final score"
            );
        }

        assert_eq!(
            scenario.preferences().len(),
            1,
            "{context}: preference count"
        );
        let preference = scenario.preferences()[0];
        assert_eq!(
            (
                preference.expectation().preferred().get(),
                preference.expectation().other().get(),
                preference.outcome(),
            ),
            (expected.preferred, expected.other, expected.outcome),
            "{context}: preference"
        );
        assert_eq!(
            preference.preferred_score().get().to_bits(),
            expected.score_bits(expected.preferred),
            "{context}: preferred score"
        );
        assert_eq!(
            preference.other_score().get().to_bits(),
            expected.score_bits(expected.other),
            "{context}: other score"
        );
    }

    let counts = expected.iter().fold((0, 0, 0), |mut counts, scenario| {
        match scenario.outcome {
            Satisfied => counts.0 += 1,
            Tied => counts.1 += 1,
            Violated => counts.2 += 1,
        }
        counts
    });
    assert_eq!(report.satisfied_count(), counts.0, "{context}: satisfied");
    assert_eq!(report.tied_count(), counts.1, "{context}: tied");
    assert_eq!(report.violated_count(), counts.2, "{context}: violated");
    assert_eq!(
        report.preference_count(),
        expected.len(),
        "{context}: preference total"
    );
}

#[derive(Clone, Copy)]
struct ExpectedScenario {
    scenario_id: u64,
    ranking: &'static [(u64, u64)],
    preferred: u64,
    other: u64,
    outcome: PreferenceOutcome,
}

impl ExpectedScenario {
    fn score_bits(self, candidate_id: u64) -> u64 {
        self.ranking
            .iter()
            .find_map(|&(observed_id, score_bits)| {
                (observed_id == candidate_id).then_some(score_bits)
            })
            .expect("expected preference candidate appears in the ranking")
    }
}

const fn expected(
    scenario_id: u64,
    ranking: &'static [(u64, u64)],
    preferred: u64,
    other: u64,
    outcome: PreferenceOutcome,
) -> ExpectedScenario {
    ExpectedScenario {
        scenario_id,
        ranking,
        preferred,
        other,
        outcome,
    }
}

const TRIGGER_CALIBRATION: &[ExpectedScenario] = &[
    expected(
        1001,
        &[(1, 0x3ff0_0000_0000_0000), (2, 0x3fe0_0000_0000_0000)],
        1,
        2,
        Satisfied,
    ),
    expected(
        1002,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fe0_0000_0000_0000)],
        2,
        1,
        Satisfied,
    ),
    expected(
        1101,
        &[(1, 0x3fe8_0000_0000_0000), (2, 0x3fe8_0000_0000_0000)],
        1,
        2,
        Tied,
    ),
    expected(
        1102,
        &[(1, 0x3fe8_0000_0000_0000), (2, 0x3fe8_0000_0000_0000)],
        2,
        1,
        Tied,
    ),
    expected(
        1201,
        &[(1, 0x3ff0_0000_0000_0000), (2, 0x3fe0_0000_0000_0000)],
        1,
        2,
        Satisfied,
    ),
    expected(
        1202,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fe0_0000_0000_0000)],
        2,
        1,
        Satisfied,
    ),
    expected(
        1301,
        &[(1, 0x3fe8_0000_0000_0000), (2, 0x3fe8_0000_0000_0000)],
        1,
        2,
        Tied,
    ),
    expected(
        1302,
        &[(1, 0x3fe8_0000_0000_0000), (2, 0x3fe8_0000_0000_0000)],
        2,
        1,
        Tied,
    ),
];

const TRIGGER_HELD_OUT: &[ExpectedScenario] = &[
    expected(
        2001,
        &[(1, 0x3fe8_0000_0000_0000), (2, 0x3fe8_0000_0000_0000)],
        1,
        2,
        Tied,
    ),
    expected(
        2002,
        &[(1, 0x3fe8_0000_0000_0000), (2, 0x3fe8_0000_0000_0000)],
        2,
        1,
        Tied,
    ),
    expected(
        2101,
        &[(1, 0x3ff0_0000_0000_0000), (2, 0x3fe0_0000_0000_0000)],
        1,
        2,
        Satisfied,
    ),
    expected(
        2102,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fd0_0000_0000_0000)],
        2,
        1,
        Satisfied,
    ),
    expected(
        2201,
        &[
            (2, 0x3ff0_0000_0000_0000),
            (1, 0x3fe8_0000_0000_0000),
            (3, 0x3fd0_0000_0000_0000),
        ],
        1,
        2,
        Violated,
    ),
    expected(
        2202,
        &[
            (2, 0x3ff0_0000_0000_0000),
            (1, 0x3fe8_0000_0000_0000),
            (3, 0x3fd0_0000_0000_0000),
        ],
        2,
        1,
        Satisfied,
    ),
    expected(
        2301,
        &[(1, 0x3ff0_0000_0000_0000), (2, 0x3fd0_0000_0000_0000)],
        1,
        2,
        Satisfied,
    ),
    expected(
        2302,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fd0_0000_0000_0000)],
        2,
        1,
        Satisfied,
    ),
];

const UNIFORM_CALIBRATION: &[ExpectedScenario] = &[
    expected(
        1001,
        &[(1, 0x3fef_8000_0000_0000), (2, 0x3fd4_0000_0000_0000)],
        1,
        2,
        Satisfied,
    ),
    expected(
        1002,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fdc_4ec4_ec4e_c4ed)],
        2,
        1,
        Satisfied,
    ),
    expected(
        1101,
        &[(1, 0x3fea_aaaa_aaaa_aaaa), (2, 0x3fe2_aaaa_aaaa_aaaa)],
        1,
        2,
        Satisfied,
    ),
    expected(
        1102,
        &[(2, 0x3fea_aaaa_aaaa_aaaa), (1, 0x3fe2_aaaa_aaaa_aaaa)],
        2,
        1,
        Satisfied,
    ),
    expected(
        1201,
        &[(1, 0x3ff0_0000_0000_0000), (2, 0x3fe0_0000_0000_0000)],
        1,
        2,
        Satisfied,
    ),
    expected(
        1202,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fe0_0000_0000_0000)],
        2,
        1,
        Satisfied,
    ),
    expected(
        1301,
        &[(1, 0x3fec_9249_2492_4924), (2, 0x3fd4_9249_2492_4924)],
        1,
        2,
        Satisfied,
    ),
    expected(
        1302,
        &[(2, 0x3fec_9249_2492_4924), (1, 0x3fd4_9249_2492_4924)],
        2,
        1,
        Satisfied,
    ),
];

const UNIFORM_HELD_OUT: &[ExpectedScenario] = &[
    expected(
        2001,
        &[(1, 0x3feb_3333_3333_3333), (2, 0x3fdc_cccc_cccc_cccc)],
        1,
        2,
        Satisfied,
    ),
    expected(
        2002,
        &[(2, 0x3feb_3333_3333_3333), (1, 0x3fdc_cccc_cccc_cccc)],
        2,
        1,
        Satisfied,
    ),
    expected(
        2101,
        &[(1, 0x3ff0_0000_0000_0000), (2, 0x3fd3_6db6_db6d_b6db)],
        1,
        2,
        Satisfied,
    ),
    expected(
        2102,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fc0_0000_0000_0000)],
        2,
        1,
        Satisfied,
    ),
    expected(
        2201,
        &[
            (1, 0x3fe9_d89d_89d8_9d8a),
            (2, 0x3fe8_9d89_d89d_89da),
            (3, 0x3fc1_3b13_b13b_13b2),
        ],
        1,
        2,
        Satisfied,
    ),
    expected(
        2202,
        &[
            (2, 0x3ff0_0000_0000_0000),
            (1, 0x3fe6_38e3_8e38_e38e),
            (3, 0x3fc5_5555_5555_5555),
        ],
        2,
        1,
        Satisfied,
    ),
    expected(
        2301,
        &[(1, 0x3ff0_0000_0000_0000), (2, 0x3fc0_0000_0000_0000)],
        1,
        2,
        Satisfied,
    ),
    expected(
        2302,
        &[(2, 0x3ff0_0000_0000_0000), (1, 0x3fc0_0000_0000_0000)],
        2,
        1,
        Satisfied,
    ),
];
