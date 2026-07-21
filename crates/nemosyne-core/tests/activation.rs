//! Public-boundary integration tests for deterministic activation ranking.

use nemosyne_core::activation::{
    ActivationCandidate, ActivationError, ActivationProfile, CandidateId, ChannelId, ChannelSignal,
    EvidenceChannel, InhibitionChannel, UnitInterval, rank_activations,
};

const TOLERANCE: f64 = 1.0e-12;

fn unit(value: f64) -> UnitInterval {
    UnitInterval::new(value).expect("test fixture must be in the unit interval")
}

fn evidence(channel_id: u64, weight: f64, gate: f64) -> EvidenceChannel {
    EvidenceChannel::new(ChannelId::new(channel_id), unit(weight), unit(gate))
}

fn inhibition(channel_id: u64, strength: f64) -> InhibitionChannel {
    InhibitionChannel::new(ChannelId::new(channel_id), unit(strength))
}

fn signal(channel_id: u64, value: f64) -> ChannelSignal {
    ChannelSignal::new(ChannelId::new(channel_id), unit(value))
}

fn candidate(
    candidate_id: u64,
    evidence_signals: &[(u64, f64)],
    inhibition_signals: &[(u64, f64)],
) -> ActivationCandidate {
    ActivationCandidate::new(
        CandidateId::new(candidate_id),
        evidence_signals
            .iter()
            .map(|&(channel_id, value)| signal(channel_id, value))
            .collect(),
        inhibition_signals
            .iter()
            .map(|&(channel_id, value)| signal(channel_id, value))
            .collect(),
    )
    .expect("test candidate must not contain duplicate channels")
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "expected {expected:.16}, got {actual:.16}"
    );
}

#[test]
fn unit_interval_accepts_boundaries_and_canonicalizes_negative_zero() {
    assert_eq!(unit(0.0).get().to_bits(), 0.0_f64.to_bits());
    assert_eq!(unit(1.0).get(), 1.0);
    assert_eq!(unit(-0.0).get().to_bits(), 0.0_f64.to_bits());
}

#[test]
fn unit_interval_rejects_every_invalid_numeric_class() {
    for value in [-0.001, 1.001, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let error = UnitInterval::new(value).expect_err("value must be rejected");
        match error {
            ActivationError::InvalidUnitInterval { value: rejected } => {
                assert_eq!(rejected.to_bits(), value.to_bits());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}

#[test]
fn computes_the_hand_verified_breakdown() {
    let profile = ActivationProfile::new(
        vec![evidence(20, 0.75, 1.0), evidence(10, 0.25, 1.0)],
        vec![inhibition(30, 0.5)],
    )
    .expect("profile must be valid");
    let candidates = [candidate(7, &[(20, 0.8), (10, 0.0)], &[(30, 0.4)])];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    let result = &ranked[0];
    let breakdown = result.breakdown();

    assert_eq!(result.candidate_id(), CandidateId::new(7));
    assert_close(breakdown.evidence_score().get(), 0.6);
    assert_close(breakdown.retention().get(), 0.8);
    assert_close(breakdown.score().get(), 0.48);

    let evidence = breakdown.evidence_contributions();
    assert_eq!(evidence.len(), 2);
    assert_eq!(evidence[0].channel_id(), ChannelId::new(10));
    assert_close(evidence[0].weight().get(), 0.25);
    assert_close(evidence[0].gate().get(), 1.0);
    assert_close(evidence[0].signal().get(), 0.0);
    assert_close(evidence[0].effective_weight().get(), 0.25);
    assert_close(evidence[0].contribution().get(), 0.0);
    assert_eq!(evidence[1].channel_id(), ChannelId::new(20));
    assert_close(evidence[1].weight().get(), 0.75);
    assert_close(evidence[1].gate().get(), 1.0);
    assert_close(evidence[1].signal().get(), 0.8);
    assert_close(evidence[1].effective_weight().get(), 0.75);
    assert_close(evidence[1].contribution().get(), 0.6);

    let inhibition = breakdown.inhibition_contributions();
    assert_eq!(inhibition.len(), 1);
    assert_eq!(inhibition[0].channel_id(), ChannelId::new(30));
    assert_close(inhibition[0].strength().get(), 0.5);
    assert_close(inhibition[0].signal().get(), 0.4);
    assert_close(inhibition[0].retention_factor().get(), 0.8);
}

#[test]
fn no_inhibition_preserves_the_evidence_score() {
    let profile =
        ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![]).expect("profile must be valid");
    let candidates = [candidate(1, &[(1, 0.42)], &[])];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    let breakdown = ranked[0].breakdown();

    assert_close(breakdown.evidence_score().get(), 0.42);
    assert_close(breakdown.retention().get(), 1.0);
    assert_close(breakdown.score().get(), 0.42);
    assert!(breakdown.inhibition_contributions().is_empty());
}

#[test]
fn complete_inhibition_retains_the_zero_scored_candidate() {
    let profile = ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![inhibition(2, 1.0)])
        .expect("profile must be valid");
    let candidates = [candidate(9, &[(1, 1.0)], &[(2, 1.0)])];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    let breakdown = ranked[0].breakdown();

    assert_eq!(ranked.len(), 1);
    assert_eq!(ranked[0].candidate_id(), CandidateId::new(9));
    assert_close(breakdown.evidence_score().get(), 1.0);
    assert_close(breakdown.retention().get(), 0.0);
    assert_close(breakdown.score().get(), 0.0);
    assert_close(
        breakdown.inhibition_contributions()[0]
            .retention_factor()
            .get(),
        0.0,
    );
}

#[test]
fn disabled_evidence_channels_have_no_effect() {
    let profile = ActivationProfile::new(
        vec![
            evidence(3, 0.0, 1.0),
            evidence(1, 1.0, 1.0),
            evidence(2, 1.0, 0.0),
        ],
        vec![],
    )
    .expect("profile must have one effective channel");
    let candidates = [candidate(1, &[(3, 1.0), (2, 1.0), (1, 0.25)], &[])];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    let breakdown = ranked[0].breakdown();
    let contributions = breakdown.evidence_contributions();

    assert_close(breakdown.evidence_score().get(), 0.25);
    assert_close(breakdown.score().get(), 0.25);
    assert_eq!(contributions.len(), 3);
    assert_close(contributions[0].effective_weight().get(), 1.0);
    assert_close(contributions[0].contribution().get(), 0.25);
    assert_close(contributions[1].effective_weight().get(), 0.0);
    assert_close(contributions[1].contribution().get(), 0.0);
    assert_close(contributions[2].effective_weight().get(), 0.0);
    assert_close(contributions[2].contribution().get(), 0.0);
}

#[test]
fn rejects_profiles_without_effective_evidence() {
    let cases = [
        (vec![], vec![]),
        (vec![evidence(1, 0.0, 1.0)], vec![]),
        (vec![evidence(1, 1.0, 0.0)], vec![]),
        (
            vec![evidence(1, 0.0, 1.0), evidence(2, 1.0, 0.0)],
            vec![inhibition(3, 1.0)],
        ),
    ];

    for (evidence_channels, inhibition_channels) in cases {
        assert_eq!(
            ActivationProfile::new(evidence_channels, inhibition_channels),
            Err(ActivationError::NoEffectiveEvidence)
        );
    }
}

#[test]
fn handles_subnormal_effective_weights_without_losing_normalized_evidence() {
    let smallest_positive = f64::from_bits(1);
    let profile = ActivationProfile::new(vec![evidence(1, smallest_positive, 1.0)], vec![])
        .expect("subnormal effective weight remains positive");
    let candidates = [candidate(1, &[(1, 0.5)], &[])];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");

    assert_close(ranked[0].breakdown().evidence_score().get(), 0.5);
    assert_close(ranked[0].breakdown().score().get(), 0.5);
}

#[test]
fn rejects_positive_weight_and_gate_when_their_product_underflows_to_zero() {
    let profile = ActivationProfile::new(
        vec![evidence(1, f64::MIN_POSITIVE, f64::MIN_POSITIVE)],
        vec![],
    );

    assert_eq!(profile, Err(ActivationError::NoEffectiveEvidence));
}

#[test]
fn score_is_bounded_and_monotonic_over_a_numeric_grid() {
    let profile = ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![inhibition(2, 0.75)])
        .expect("profile must be valid");
    let grid = [0.0, 0.25, 0.5, 0.75, 1.0];

    for penalty in grid {
        let mut previous_score = None;
        for evidence_value in grid {
            let candidates = [candidate(1, &[(1, evidence_value)], &[(2, penalty)])];
            let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
            let score = ranked[0].breakdown().score().get();
            let expected = evidence_value * (1.0 - 0.75 * penalty);

            assert_close(score, expected);
            assert!(score.is_finite());
            assert!((0.0..=1.0).contains(&score));
            if let Some(previous) = previous_score {
                assert!(score >= previous);
            }
            previous_score = Some(score);
        }
    }

    for evidence_value in grid {
        let mut previous_score = None;
        for penalty in grid {
            let candidates = [candidate(1, &[(1, evidence_value)], &[(2, penalty)])];
            let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
            let score = ranked[0].breakdown().score().get();

            if let Some(previous) = previous_score {
                assert!(score <= previous);
            }
            previous_score = Some(score);
        }
    }
}

#[test]
fn ranks_all_candidates_and_breaks_exact_ties_by_identifier() {
    let profile =
        ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![]).expect("profile must be valid");
    let candidates = [
        candidate(30, &[(1, 0.2)], &[]),
        candidate(20, &[(1, 0.8)], &[]),
        candidate(40, &[(1, 0.0)], &[]),
        candidate(10, &[(1, 0.8)], &[]),
    ];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    let identifiers: Vec<u64> = ranked
        .iter()
        .map(|result| result.candidate_id().get())
        .collect();

    assert_eq!(identifiers, vec![10, 20, 30, 40]);
    assert_close(ranked[3].breakdown().score().get(), 0.0);
}

#[test]
fn permutations_produce_identical_results_and_canonical_breakdowns() {
    let first_profile = ActivationProfile::new(
        vec![evidence(20, 0.7, 0.8), evidence(10, 0.3, 0.4)],
        vec![inhibition(40, 0.23), inhibition(30, 0.17)],
    )
    .expect("profile must be valid");
    let second_profile = ActivationProfile::new(
        vec![evidence(10, 0.3, 0.4), evidence(20, 0.7, 0.8)],
        vec![inhibition(30, 0.17), inhibition(40, 0.23)],
    )
    .expect("profile must be valid");

    let first_candidates = [
        candidate(2, &[(20, 0.44), (10, 0.62)], &[(40, 0.73), (30, 0.11)]),
        candidate(1, &[(20, 0.91), (10, 0.17)], &[(40, 0.31), (30, 0.29)]),
    ];
    let second_candidates = [
        candidate(1, &[(10, 0.17), (20, 0.91)], &[(30, 0.29), (40, 0.31)]),
        candidate(2, &[(10, 0.62), (20, 0.44)], &[(30, 0.11), (40, 0.73)]),
    ];

    let first = rank_activations(&first_profile, &first_candidates).expect("ranking must succeed");
    let second =
        rank_activations(&second_profile, &second_candidates).expect("ranking must succeed");

    assert_eq!(first, second);
    assert_eq!(
        first_profile
            .evidence_channels()
            .iter()
            .map(|channel| channel.channel_id().get())
            .collect::<Vec<_>>(),
        vec![10, 20]
    );
    assert_eq!(
        first_profile
            .inhibition_channels()
            .iter()
            .map(|channel| channel.channel_id().get())
            .collect::<Vec<_>>(),
        vec![30, 40]
    );

    for result in first {
        assert_eq!(
            result
                .breakdown()
                .evidence_contributions()
                .iter()
                .map(|contribution| contribution.channel_id().get())
                .collect::<Vec<_>>(),
            vec![10, 20]
        );
        assert_eq!(
            result
                .breakdown()
                .inhibition_contributions()
                .iter()
                .map(|contribution| contribution.channel_id().get())
                .collect::<Vec<_>>(),
            vec![30, 40]
        );
    }
}

#[test]
fn rejects_duplicate_profile_channels_within_and_across_categories() {
    let within_evidence =
        ActivationProfile::new(vec![evidence(1, 1.0, 1.0), evidence(1, 0.5, 1.0)], vec![]);
    assert_eq!(
        within_evidence,
        Err(ActivationError::DuplicateProfileChannel {
            channel_id: ChannelId::new(1)
        })
    );

    let within_inhibition = ActivationProfile::new(
        vec![evidence(1, 1.0, 1.0)],
        vec![inhibition(2, 0.5), inhibition(2, 0.75)],
    );
    assert_eq!(
        within_inhibition,
        Err(ActivationError::DuplicateProfileChannel {
            channel_id: ChannelId::new(2)
        })
    );

    let across_categories =
        ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![inhibition(1, 0.5)]);
    assert_eq!(
        across_categories,
        Err(ActivationError::DuplicateProfileChannel {
            channel_id: ChannelId::new(1)
        })
    );
}

#[test]
fn rejects_duplicate_candidate_channels_within_and_across_categories() {
    let duplicate_evidence = ActivationCandidate::new(
        CandidateId::new(7),
        vec![signal(1, 0.2), signal(1, 0.3)],
        vec![],
    );
    assert_eq!(
        duplicate_evidence,
        Err(ActivationError::DuplicateCandidateChannel {
            candidate_id: CandidateId::new(7),
            channel_id: ChannelId::new(1)
        })
    );

    let duplicate_inhibition = ActivationCandidate::new(
        CandidateId::new(7),
        vec![],
        vec![signal(2, 0.2), signal(2, 0.3)],
    );
    assert_eq!(
        duplicate_inhibition,
        Err(ActivationError::DuplicateCandidateChannel {
            candidate_id: CandidateId::new(7),
            channel_id: ChannelId::new(2)
        })
    );

    let duplicate_across_categories = ActivationCandidate::new(
        CandidateId::new(7),
        vec![signal(3, 0.2)],
        vec![signal(3, 0.3)],
    );
    assert_eq!(
        duplicate_across_categories,
        Err(ActivationError::DuplicateCandidateChannel {
            candidate_id: CandidateId::new(7),
            channel_id: ChannelId::new(3)
        })
    );
}

#[test]
fn rejects_duplicate_candidate_identifiers() {
    let profile =
        ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![]).expect("profile must be valid");
    let candidates = [
        candidate(5, &[(1, 0.2)], &[]),
        candidate(5, &[(1, 0.8)], &[]),
    ];

    assert_eq!(
        rank_activations(&profile, &candidates),
        Err(ActivationError::DuplicateCandidate {
            candidate_id: CandidateId::new(5)
        })
    );
}

#[test]
fn rejects_missing_and_unexpected_evidence_signals() {
    let profile = ActivationProfile::new(
        vec![evidence(1, 1.0, 1.0), evidence(2, 1.0, 1.0)],
        vec![inhibition(3, 1.0)],
    )
    .expect("profile must be valid");
    let missing = [candidate(8, &[(1, 0.5)], &[(3, 0.5)])];
    let unexpected = [candidate(8, &[(1, 0.5), (2, 0.5), (4, 0.5)], &[(3, 0.5)])];

    assert_eq!(
        rank_activations(&profile, &missing),
        Err(ActivationError::MissingEvidenceSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(2)
        })
    );
    assert_eq!(
        rank_activations(&profile, &unexpected),
        Err(ActivationError::UnexpectedEvidenceSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(4)
        })
    );
}

#[test]
fn rejects_interior_and_leading_evidence_set_mismatches() {
    let missing_profile = ActivationProfile::new(
        vec![
            evidence(2, 1.0, 1.0),
            evidence(4, 1.0, 1.0),
            evidence(6, 1.0, 1.0),
        ],
        vec![],
    )
    .expect("profile must be valid");
    let missing = [candidate(8, &[(2, 0.5), (6, 0.5)], &[])];

    assert_eq!(
        rank_activations(&missing_profile, &missing),
        Err(ActivationError::MissingEvidenceSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(4)
        })
    );

    let unexpected_profile =
        ActivationProfile::new(vec![evidence(2, 1.0, 1.0), evidence(4, 1.0, 1.0)], vec![])
            .expect("profile must be valid");
    let unexpected = [candidate(8, &[(1, 0.5), (2, 0.5), (4, 0.5)], &[])];

    assert_eq!(
        rank_activations(&unexpected_profile, &unexpected),
        Err(ActivationError::UnexpectedEvidenceSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(1)
        })
    );
}

#[test]
fn rejects_missing_and_unexpected_inhibition_signals() {
    let profile = ActivationProfile::new(
        vec![evidence(1, 1.0, 1.0), evidence(2, 1.0, 1.0)],
        vec![inhibition(3, 1.0)],
    )
    .expect("profile must be valid");
    let missing = [candidate(8, &[(1, 0.5), (2, 0.5)], &[])];
    let unexpected = [candidate(8, &[(1, 0.5), (2, 0.5)], &[(3, 0.5), (4, 0.5)])];

    assert_eq!(
        rank_activations(&profile, &missing),
        Err(ActivationError::MissingInhibitionSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(3)
        })
    );
    assert_eq!(
        rank_activations(&profile, &unexpected),
        Err(ActivationError::UnexpectedInhibitionSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(4)
        })
    );
}

#[test]
fn rejects_a_signal_supplied_in_the_wrong_channel_category() {
    let profile = ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![inhibition(2, 1.0)])
        .expect("profile must be valid");
    let candidates = [candidate(8, &[(1, 0.5), (2, 0.5)], &[])];

    assert_eq!(
        rank_activations(&profile, &candidates),
        Err(ActivationError::UnexpectedEvidenceSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(2)
        })
    );
}

#[test]
fn accepts_complete_signals_in_arbitrary_input_order() {
    let profile = ActivationProfile::new(
        vec![evidence(2, 1.0, 1.0), evidence(1, 1.0, 1.0)],
        vec![inhibition(4, 1.0), inhibition(3, 1.0)],
    )
    .expect("profile must be valid");
    let candidates = [candidate(1, &[(2, 0.6), (1, 0.4)], &[(4, 0.2), (3, 0.1)])];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");

    assert_eq!(ranked.len(), 1);
    assert_eq!(
        candidates[0]
            .evidence_signals()
            .iter()
            .map(|signal| signal.channel_id().get())
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(
        candidates[0]
            .inhibition_signals()
            .iter()
            .map(|signal| signal.channel_id().get())
            .collect::<Vec<_>>(),
        vec![3, 4]
    );
    assert_close(candidates[0].evidence_signals()[0].value().get(), 0.4);
}

#[test]
fn empty_candidate_input_returns_an_empty_ranking() {
    let profile =
        ActivationProfile::new(vec![evidence(1, 1.0, 1.0)], vec![]).expect("profile must be valid");

    assert_eq!(rank_activations(&profile, &[]), Ok(vec![]));
}

#[test]
fn every_breakdown_reconstructs_its_score() {
    let profile = ActivationProfile::new(
        vec![
            evidence(3, 0.9, 0.7),
            evidence(1, 0.4, 0.8),
            evidence(2, 0.5, 0.3),
        ],
        vec![inhibition(5, 0.6), inhibition(4, 0.2)],
    )
    .expect("profile must be valid");
    let candidates = [
        candidate(1, &[(3, 0.2), (1, 0.7), (2, 0.9)], &[(5, 0.4), (4, 0.1)]),
        candidate(2, &[(3, 1.0), (1, 0.0), (2, 0.5)], &[(5, 0.8), (4, 0.6)]),
    ];
    let denominator: f64 = profile
        .evidence_channels()
        .iter()
        .map(|channel| channel.weight().get() * channel.gate().get())
        .sum();

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");

    for result in ranked {
        let breakdown = result.breakdown();
        let mut evidence_sum = 0.0;
        for contribution in breakdown.evidence_contributions() {
            let effective_weight = contribution.weight().get() * contribution.gate().get();
            let expected_contribution =
                effective_weight * contribution.signal().get() / denominator;
            assert_close(contribution.effective_weight().get(), effective_weight);
            assert_close(contribution.contribution().get(), expected_contribution);
            evidence_sum += contribution.contribution().get();
        }

        let mut retention_product = 1.0;
        for contribution in breakdown.inhibition_contributions() {
            let expected_factor = 1.0 - contribution.strength().get() * contribution.signal().get();
            assert_close(contribution.retention_factor().get(), expected_factor);
            retention_product *= contribution.retention_factor().get();
        }

        assert_close(breakdown.evidence_score().get(), evidence_sum);
        assert_close(breakdown.retention().get(), retention_product);
        assert_close(
            breakdown.score().get(),
            breakdown.evidence_score().get() * breakdown.retention().get(),
        );
        assert!(breakdown.evidence_score().get().is_finite());
        assert!(breakdown.retention().get().is_finite());
        assert!(breakdown.score().get().is_finite());
        assert!((0.0..=1.0).contains(&breakdown.evidence_score().get()));
        assert!((0.0..=1.0).contains(&breakdown.retention().get()));
        assert!((0.0..=1.0).contains(&breakdown.score().get()));
    }
}

#[test]
fn clamps_a_one_ulp_rounding_overflow_to_the_unit_interval() {
    let first_weight = f64::from_bits(0x3feb_c366_e555_2160);
    let second_weight = f64::from_bits(0x3fcf_3878_bbaf_3894);
    let profile = ActivationProfile::new(
        vec![
            evidence(1, first_weight, 1.0),
            evidence(2, second_weight, 1.0),
        ],
        vec![],
    )
    .expect("profile must be valid");
    let candidates = [candidate(1, &[(1, 1.0), (2, 1.0)], &[])];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    let breakdown = ranked[0].breakdown();
    let raw_sum: f64 = breakdown
        .evidence_contributions()
        .iter()
        .map(|contribution| contribution.contribution().get())
        .sum();

    assert!(raw_sum > 1.0);
    assert_eq!(breakdown.evidence_score().get(), 1.0);
    assert_eq!(breakdown.score().get(), 1.0);
    assert!(breakdown.score().get().is_finite());
}

#[test]
fn numeric_red_light_example_ranks_immediate_evidence_first() {
    // These caller-selected channels and values are an algorithm fixture, not
    // a safety policy or a claim that the kernel can identify safe behavior.
    let profile = ActivationProfile::new(
        vec![
            evidence(1, 0.4, 1.0),
            evidence(2, 0.3, 1.0),
            evidence(3, 0.2, 1.0),
            evidence(4, 0.2, 0.5),
        ],
        vec![inhibition(5, 0.5)],
    )
    .expect("profile must be valid");
    let candidates = [
        candidate(2, &[(1, 0.2), (2, 0.1), (3, 0.0), (4, 1.0)], &[(5, 0.4)]),
        candidate(1, &[(1, 1.0), (2, 1.0), (3, 1.0), (4, 0.5)], &[(5, 0.0)]),
    ];

    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");

    assert_eq!(ranked[0].candidate_id(), CandidateId::new(1));
    assert_close(ranked[0].breakdown().evidence_score().get(), 0.95);
    assert_close(ranked[0].breakdown().retention().get(), 1.0);
    assert_close(ranked[0].breakdown().score().get(), 0.95);
    assert_eq!(ranked[1].candidate_id(), CandidateId::new(2));
    assert_close(ranked[1].breakdown().evidence_score().get(), 0.21);
    assert_close(ranked[1].breakdown().retention().get(), 0.8);
    assert_close(ranked[1].breakdown().score().get(), 0.168);
}
