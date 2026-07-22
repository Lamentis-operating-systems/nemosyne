use nemosyne_core::activation::{
    ActivationError, CandidateId, ChannelId, explain_activation, rank_activations,
};

use super::{candidate, evidence, inhibition, profile};

#[test]
fn missing_signal_is_rejected() {
    let profile = profile(vec![evidence(1, 1.0, 1.0), inhibition(2, 0.5)]);
    let candidates = [candidate(7, &[(1, 0.4)])];
    assert_eq!(
        rank_activations(&profile, &candidates),
        Err(ActivationError::MissingSignal {
            candidate_id: CandidateId::new(7),
            channel_id: ChannelId::new(2)
        })
    );
}

#[test]
fn unexpected_signal_is_rejected() {
    let profile = profile(vec![evidence(2, 1.0, 1.0)]);
    let candidates = [candidate(7, &[(1, 0.3), (2, 0.4)])];
    assert_eq!(
        rank_activations(&profile, &candidates),
        Err(ActivationError::UnexpectedSignal {
            candidate_id: CandidateId::new(7),
            channel_id: ChannelId::new(1)
        })
    );
}

#[test]
fn explanation_rejects_missing_and_unexpected_signals() {
    let profile = profile(vec![evidence(2, 1.0, 1.0), inhibition(4, 0.5)]);
    let missing = candidate(7, &[(2, 0.4)]);
    assert_eq!(
        explain_activation(&profile, &missing),
        Err(ActivationError::MissingSignal {
            candidate_id: CandidateId::new(7),
            channel_id: ChannelId::new(4)
        })
    );

    let unexpected = candidate(8, &[(1, 0.3), (2, 0.4), (4, 0.2)]);
    assert_eq!(
        explain_activation(&profile, &unexpected),
        Err(ActivationError::UnexpectedSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(1)
        })
    );
}

#[test]
fn interior_missing_and_trailing_unexpected_signals_are_rejected() {
    let profile = profile(vec![
        evidence(2, 1.0, 1.0),
        inhibition(4, 0.5),
        evidence(6, 1.0, 1.0),
    ]);
    assert_eq!(
        rank_activations(&profile, &[candidate(8, &[(2, 0.5), (6, 0.5)])]),
        Err(ActivationError::MissingSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(4)
        })
    );
    assert_eq!(
        rank_activations(
            &profile,
            &[candidate(8, &[(2, 0.5), (4, 0.5), (6, 0.5), (9, 0.5)])]
        ),
        Err(ActivationError::UnexpectedSignal {
            candidate_id: CandidateId::new(8),
            channel_id: ChannelId::new(9)
        })
    );
}

#[test]
fn duplicate_candidate_ids_are_rejected() {
    let profile = profile(vec![evidence(1, 1.0, 1.0)]);
    let candidates = [candidate(4, &[(1, 0.2)]), candidate(4, &[(1, 0.8)])];
    assert_eq!(
        rank_activations(&profile, &candidates),
        Err(ActivationError::DuplicateCandidate {
            candidate_id: CandidateId::new(4)
        })
    );
}

#[test]
fn empty_candidate_input_returns_empty_ranking() {
    let profile = profile(vec![evidence(1, 1.0, 1.0)]);
    assert_eq!(rank_activations(&profile, &[]), Ok(vec![]));
}
