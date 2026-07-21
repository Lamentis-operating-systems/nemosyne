use nemosyne_core::activation::{explain_activation, rank_activations};

use super::{assert_close, candidate, evidence, inhibition, profile};

#[test]
fn hand_calculated_score_is_reproduced() {
    let profile = profile(vec![evidence(1, 1.0, 1.0), inhibition(2, 0.5)]);
    let candidate = candidate(1, &[(1, 0.6), (2, 0.4)]);
    let activation = rank_activations(&profile, &[candidate]).expect("ranking must succeed")[0];
    assert_close(activation.evidence_score().get(), 0.6);
    assert_close(activation.retention().get(), 0.8);
    assert_close(activation.score().get(), 0.48);
}

#[test]
fn zero_evidence_and_absent_or_complete_inhibition_have_defined_results() {
    let no_inhibition = profile(vec![evidence(1, 1.0, 1.0)]);
    let zero = rank_activations(&no_inhibition, &[candidate(1, &[(1, 0.0)])])
        .expect("ranking must succeed")[0];
    assert_eq!(zero.evidence_score().get(), 0.0);
    assert_eq!(zero.retention().get(), 1.0);
    assert_eq!(zero.score().get(), 0.0);

    let complete = profile(vec![evidence(1, 1.0, 1.0), inhibition(2, 1.0)]);
    let inhibited = rank_activations(&complete, &[candidate(1, &[(1, 1.0), (2, 1.0)])])
        .expect("ranking must succeed")[0];
    assert_eq!(inhibited.retention().get(), 0.0);
    assert_eq!(inhibited.score().get(), 0.0);
}

#[test]
fn explicitly_inactive_evidence_channels_contribute_zero() {
    let profile = profile(vec![
        evidence(1, 0.0, 1.0),
        evidence(2, 1.0, 0.0),
        evidence(3, 0.5, 1.0),
    ]);
    let candidate = candidate(1, &[(1, 1.0), (2, 1.0), (3, 0.4)]);
    let explanation = explain_activation(&profile, &candidate).expect("explanation must succeed");
    assert_close(explanation.activation().score().get(), 0.4);
    assert_eq!(
        explanation.evidence_contributions()[0].contribution().get(),
        0.0
    );
    assert_eq!(
        explanation.evidence_contributions()[1].contribution().get(),
        0.0
    );
}

#[test]
fn evidence_is_monotone_and_inhibition_is_antitone() {
    let profile = profile(vec![evidence(1, 1.0, 1.0), inhibition(2, 0.7)]);
    let low_evidence = rank_activations(&profile, &[candidate(1, &[(1, 0.2), (2, 0.3)])])
        .expect("ranking must succeed")[0];
    let high_evidence = rank_activations(&profile, &[candidate(1, &[(1, 0.8), (2, 0.3)])])
        .expect("ranking must succeed")[0];
    let high_inhibition = rank_activations(&profile, &[candidate(1, &[(1, 0.8), (2, 0.9)])])
        .expect("ranking must succeed")[0];
    assert!(high_evidence.score().get() >= low_evidence.score().get());
    assert!(high_inhibition.score().get() <= high_evidence.score().get());
}

#[test]
fn score_is_exact_bounded_and_monotone_over_a_numeric_grid() {
    let profile = profile(vec![evidence(1, 1.0, 1.0), inhibition(2, 0.75)]);
    let grid = [0.0, 0.25, 0.5, 0.75, 1.0];

    for penalty in grid {
        let mut previous = 0.0;
        for evidence_value in grid {
            let activation = rank_activations(
                &profile,
                &[candidate(1, &[(1, evidence_value), (2, penalty)])],
            )
            .expect("ranking must succeed")[0];
            let score = activation.score().get();
            assert_close(score, evidence_value * (1.0 - 0.75 * penalty));
            assert!(score.is_finite() && (0.0..=1.0).contains(&score));
            assert!(score >= previous);
            previous = score;
        }
    }

    for evidence_value in grid {
        let mut previous = 1.0;
        for penalty in grid {
            let score = rank_activations(
                &profile,
                &[candidate(1, &[(1, evidence_value), (2, penalty)])],
            )
            .expect("ranking must succeed")[0]
                .score()
                .get();
            assert!(score <= previous);
            previous = score;
        }
    }
}

#[test]
fn explanation_reconstructs_bounded_finite_aggregates() {
    let profile = profile(vec![
        evidence(3, 0.9, 0.7),
        evidence(1, 0.4, 0.8),
        evidence(2, 0.5, 0.3),
        inhibition(5, 0.6),
        inhibition(4, 0.2),
    ]);
    let candidate = candidate(1, &[(3, 0.2), (1, 0.7), (2, 0.9), (5, 0.4), (4, 0.1)]);
    let explanation = explain_activation(&profile, &candidate).expect("explanation must succeed");
    let evidence_sum: f64 = explanation
        .evidence_contributions()
        .iter()
        .map(|entry| entry.contribution().get())
        .sum();
    let retention_product: f64 = explanation
        .inhibition_contributions()
        .iter()
        .map(|entry| entry.retention_factor().get())
        .product();
    let activation = explanation.activation();
    assert_close(activation.evidence_score().get(), evidence_sum);
    assert_close(activation.retention().get(), retention_product);
    assert_close(
        activation.score().get(),
        activation.evidence_score().get() * activation.retention().get(),
    );
    for value in [
        activation.evidence_score().get(),
        activation.retention().get(),
        activation.score().get(),
    ] {
        assert!(value.is_finite());
        assert!((0.0..=1.0).contains(&value));
    }
}

#[test]
fn prepared_normalized_weights_are_exposed_by_explanation() {
    let profile = profile(vec![evidence(1, 0.5, 0.5), evidence(2, 1.0, 0.75)]);
    let candidate = candidate(1, &[(1, 0.2), (2, 0.8)]);
    let explanation = explain_activation(&profile, &candidate).expect("explanation must succeed");
    let entries = explanation.evidence_contributions();
    assert_close(entries[0].effective_weight().get(), 0.25);
    assert_close(entries[0].normalized_weight().get(), 0.25);
    assert_close(entries[1].effective_weight().get(), 0.75);
    assert_close(entries[1].normalized_weight().get(), 0.75);
}

#[test]
fn compact_ranking_and_explanation_have_identical_aggregate_bits() {
    let profile = profile(vec![
        inhibition(4, 0.3),
        evidence(1, 0.7, 0.9),
        evidence(3, 0.4, 0.8),
        inhibition(2, 0.6),
    ]);
    let candidate = candidate(9, &[(1, 0.2), (2, 0.4), (3, 0.9), (4, 0.1)]);
    let ranked = rank_activations(&profile, std::slice::from_ref(&candidate))
        .expect("ranking must succeed")[0];
    let explained = explain_activation(&profile, &candidate)
        .expect("explanation must succeed")
        .activation()
        .to_owned();
    assert_eq!(ranked.candidate_id(), explained.candidate_id());
    assert_eq!(
        ranked.evidence_score().get().to_bits(),
        explained.evidence_score().get().to_bits()
    );
    assert_eq!(
        ranked.retention().get().to_bits(),
        explained.retention().get().to_bits()
    );
    assert_eq!(
        ranked.score().get().to_bits(),
        explained.score().get().to_bits()
    );
}

#[test]
fn one_ulp_rounding_overflow_is_clamped() {
    let first_weight = f64::from_bits(0x3feb_c366_e555_2160);
    let second_weight = f64::from_bits(0x3fcf_3878_bbaf_3894);
    let profile = profile(vec![
        evidence(1, first_weight, 1.0),
        evidence(2, second_weight, 1.0),
    ]);
    let activation = rank_activations(&profile, &[candidate(1, &[(1, 1.0), (2, 1.0)])])
        .expect("ranking must succeed")[0];
    assert_eq!(activation.evidence_score().get(), 1.0);
    assert_eq!(activation.score().get(), 1.0);
}
