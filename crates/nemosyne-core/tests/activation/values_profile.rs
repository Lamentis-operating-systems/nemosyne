use std::collections::HashSet;

use nemosyne_core::activation::{
    ActivationCandidate, ActivationChannel, ActivationError, ActivationProfile, CandidateId,
    ChannelId, ChannelSignal, EvidenceChannel, UnitInterval,
};

use super::{candidate, evidence, inhibition, unit};

#[test]
fn unit_interval_accepts_boundaries_and_canonicalizes_negative_zero() {
    assert_eq!(UnitInterval::new(0.0).expect("zero is valid").get(), 0.0);
    assert_eq!(UnitInterval::new(1.0).expect("one is valid").get(), 1.0);
    assert_eq!(
        UnitInterval::new(-0.0)
            .expect("negative zero is valid")
            .get()
            .to_bits(),
        0
    );
}

#[test]
fn unit_interval_rejects_every_invalid_numeric_class() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.1, 1.1] {
        assert!(matches!(
            UnitInterval::new(value),
            Err(ActivationError::InvalidUnitInterval { value: rejected })
                if rejected.to_bits() == value.to_bits()
        ));
    }
}

#[test]
fn identifiers_are_hashable_value_types() {
    let candidates = HashSet::from([CandidateId::new(3), CandidateId::new(3)]);
    let channels = HashSet::from([ChannelId::new(7), ChannelId::new(7)]);
    assert_eq!(candidates.len(), 1);
    assert_eq!(channels.len(), 1);
}

#[test]
fn profile_uses_one_global_canonical_channel_order() {
    let profile = ActivationProfile::new(vec![
        inhibition(4, 0.4),
        evidence(2, 0.5, 0.5),
        inhibition(1, 0.2),
        evidence(3, 1.0, 1.0),
    ])
    .expect("profile must be valid");

    assert_eq!(
        profile
            .channels()
            .iter()
            .map(ActivationChannel::channel_id)
            .map(ChannelId::get)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    assert!(profile.channels()[0].inhibition().is_some());
    assert!(profile.channels()[1].evidence().is_some());
}

#[test]
fn duplicate_profile_ids_are_rejected_across_channel_kinds() {
    assert_eq!(
        ActivationProfile::new(vec![evidence(1, 1.0, 1.0), inhibition(1, 0.5)]),
        Err(ActivationError::DuplicateProfileChannel {
            channel_id: ChannelId::new(1)
        })
    );
}

#[test]
fn profile_requires_effective_evidence() {
    assert_eq!(
        ActivationProfile::new(vec![inhibition(1, 0.5)]),
        Err(ActivationError::NoEffectiveEvidence)
    );
    assert_eq!(
        ActivationProfile::new(vec![evidence(1, 0.0, 1.0), evidence(2, 1.0, 0.0)]),
        Err(ActivationError::NoEffectiveEvidence)
    );
}

#[test]
fn positive_parameters_that_underflow_are_rejected() {
    let tiny = UnitInterval::new(f64::from_bits(1)).expect("smallest subnormal is valid");
    let channel = ActivationChannel::Evidence(EvidenceChannel::new(
        ChannelId::new(8),
        tiny,
        UnitInterval::new(0.5).expect("half is valid"),
    ));

    assert_eq!(
        ActivationProfile::new(vec![channel, evidence(9, 1.0, 1.0)]),
        Err(ActivationError::EffectiveWeightUnderflow {
            channel_id: ChannelId::new(8)
        })
    );

    let full_underflow = ActivationChannel::Evidence(EvidenceChannel::new(
        ChannelId::new(10),
        UnitInterval::new(f64::MIN_POSITIVE).expect("minimum normal is valid"),
        UnitInterval::new(f64::MIN_POSITIVE).expect("minimum normal is valid"),
    ));
    assert_eq!(
        ActivationProfile::new(vec![full_underflow]),
        Err(ActivationError::EffectiveWeightUnderflow {
            channel_id: ChannelId::new(10)
        })
    );
}

#[test]
fn representable_subnormal_effective_weight_keeps_its_evidence() {
    let smallest_positive = f64::from_bits(1);
    let profile = ActivationProfile::new(vec![evidence(1, smallest_positive, 1.0)])
        .expect("subnormal effective weight remains positive");
    let candidate = candidate(1, &[(1, 0.5)]);
    let ranked = nemosyne_core::activation::rank_activations(&profile, &[candidate])
        .expect("ranking must succeed");
    assert_eq!(ranked[0].evidence_score().get(), 0.5);
    assert_eq!(ranked[0].score().get(), 0.5);
}

#[test]
fn positive_effective_weight_that_underflows_during_normalization_is_rejected() {
    let smallest_positive = f64::from_bits(1);
    assert_eq!(
        ActivationProfile::new(vec![
            evidence(1, smallest_positive, 1.0),
            evidence(2, 1.0, 1.0),
            evidence(3, 1.0, 1.0),
        ]),
        Err(ActivationError::NormalizedWeightUnderflow {
            channel_id: ChannelId::new(1)
        })
    );
}

#[test]
fn candidate_uses_one_global_canonical_signal_order() {
    let candidate = candidate(5, &[(4, 0.4), (1, 0.1), (3, 0.3), (2, 0.2)]);
    assert_eq!(
        candidate
            .signals()
            .iter()
            .map(ChannelSignal::channel_id)
            .map(ChannelId::get)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn duplicate_candidate_signal_ids_are_rejected() {
    let duplicate = ChannelSignal::new(ChannelId::new(2), unit(0.5));
    assert_eq!(
        ActivationCandidate::new(CandidateId::new(4), vec![duplicate, duplicate]),
        Err(ActivationError::DuplicateCandidateChannel {
            candidate_id: CandidateId::new(4),
            channel_id: ChannelId::new(2)
        })
    );
}
