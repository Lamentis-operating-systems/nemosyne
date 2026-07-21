use nemosyne_core::activation::{CandidateId, explain_activation, rank_activations};

use super::{candidate, evidence, inhibition, profile};

#[test]
fn permuted_profiles_candidates_and_signals_produce_identical_results() {
    let first_profile = profile(vec![
        evidence(3, 0.4, 0.7),
        inhibition(2, 0.5),
        evidence(1, 0.8, 0.9),
    ]);
    let second_profile = profile(vec![
        evidence(1, 0.8, 0.9),
        evidence(3, 0.4, 0.7),
        inhibition(2, 0.5),
    ]);
    let first_candidates = [
        candidate(2, &[(3, 0.9), (1, 0.2), (2, 0.1)]),
        candidate(1, &[(1, 0.7), (2, 0.4), (3, 0.3)]),
    ];
    let second_candidates = [
        candidate(1, &[(3, 0.3), (1, 0.7), (2, 0.4)]),
        candidate(2, &[(2, 0.1), (3, 0.9), (1, 0.2)]),
    ];
    assert_eq!(
        rank_activations(&first_profile, &first_candidates),
        rank_activations(&second_profile, &second_candidates)
    );
}

#[test]
fn exact_ties_are_resolved_by_candidate_id() {
    let profile = profile(vec![evidence(1, 1.0, 1.0)]);
    let candidates = [candidate(9, &[(1, 0.5)]), candidate(3, &[(1, 0.5)])];
    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    assert_eq!(ranked[0].candidate_id(), CandidateId::new(3));
    assert_eq!(ranked[1].candidate_id(), CandidateId::new(9));
}

#[test]
fn ranking_contains_all_candidates_in_score_then_id_order() {
    let profile = profile(vec![evidence(1, 1.0, 1.0)]);
    let candidates = [
        candidate(30, &[(1, 0.2)]),
        candidate(20, &[(1, 0.8)]),
        candidate(40, &[(1, 0.0)]),
        candidate(10, &[(1, 0.8)]),
    ];
    let ranked = rank_activations(&profile, &candidates).expect("ranking must succeed");
    assert_eq!(
        ranked
            .iter()
            .map(|activation| activation.candidate_id().get())
            .collect::<Vec<_>>(),
        vec![10, 20, 30, 40]
    );
}

#[test]
fn explanation_lists_each_kind_in_canonical_channel_order() {
    let profile = profile(vec![
        inhibition(40, 0.2),
        evidence(20, 0.7, 0.8),
        inhibition(30, 0.1),
        evidence(10, 0.3, 0.4),
    ]);
    let candidate = candidate(1, &[(40, 0.3), (20, 0.9), (30, 0.2), (10, 0.1)]);
    let explanation = explain_activation(&profile, &candidate).expect("explanation must succeed");
    assert_eq!(
        explanation
            .evidence_contributions()
            .iter()
            .map(|entry| entry.channel_id().get())
            .collect::<Vec<_>>(),
        vec![10, 20]
    );
    assert_eq!(
        explanation
            .inhibition_contributions()
            .iter()
            .map(|entry| entry.channel_id().get())
            .collect::<Vec<_>>(),
        vec![30, 40]
    );
}
