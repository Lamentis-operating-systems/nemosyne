use nemosyne_evaluation::activation::PreferenceOutcome;

use super::{corpus, report};

#[test]
fn references_produce_frozen_reconstructable_observations() {
    let corpus = corpus();
    let observations = [
        ("trigger_only", corpus.calibration(), (4, 4, 0)),
        ("trigger_only", corpus.held_out(), (5, 2, 1)),
        ("uniform_evidence", corpus.calibration(), (8, 0, 0)),
        ("uniform_evidence", corpus.held_out(), (8, 0, 0)),
    ];

    for (reference, partition, expected) in observations {
        let report = report(&corpus, reference, partition);
        assert_eq!(
            (
                report.satisfied_count(),
                report.tied_count(),
                report.violated_count()
            ),
            expected
        );
        assert_eq!(report.scenario_count(), 8);
        assert_eq!(report.preference_count(), 8);
        assert_eq!(
            report.satisfied_count() + report.tied_count() + report.violated_count(),
            report.preference_count()
        );

        let reconstructed = report
            .scenarios()
            .iter()
            .flat_map(|scenario| scenario.preferences())
            .fold((0, 0, 0), |mut counts, preference| {
                match preference.outcome() {
                    PreferenceOutcome::Satisfied => counts.0 += 1,
                    PreferenceOutcome::Tied => counts.1 += 1,
                    PreferenceOutcome::Violated => counts.2 += 1,
                }
                counts
            });
        assert_eq!(reconstructed, expected);
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
