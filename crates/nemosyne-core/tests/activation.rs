//! Integration tests for the public activation contract.

use nemosyne_core::activation::{
    ActivationCandidate, ActivationChannel, ActivationProfile, CandidateId, ChannelId,
    ChannelSignal, EvidenceChannel, InhibitionChannel, UnitInterval,
};

#[path = "activation/determinism.rs"]
mod determinism;
#[path = "activation/scenario.rs"]
mod scenario;
#[path = "activation/scoring.rs"]
mod scoring;
#[path = "activation/validation.rs"]
mod validation;
#[path = "activation/values_profile.rs"]
mod values_profile;

fn unit(value: f64) -> UnitInterval {
    UnitInterval::new(value).expect("fixture value must be in the unit interval")
}

fn evidence(id: u64, weight: f64, gate: f64) -> ActivationChannel {
    EvidenceChannel::new(ChannelId::new(id), unit(weight), unit(gate)).into()
}

fn inhibition(id: u64, strength: f64) -> ActivationChannel {
    InhibitionChannel::new(ChannelId::new(id), unit(strength)).into()
}

fn profile(channels: Vec<ActivationChannel>) -> ActivationProfile {
    ActivationProfile::new(channels).expect("profile fixture must be valid")
}

fn candidate(id: u64, signals: &[(u64, f64)]) -> ActivationCandidate {
    ActivationCandidate::new(
        CandidateId::new(id),
        signals
            .iter()
            .map(|(channel_id, value)| {
                ChannelSignal::new(ChannelId::new(*channel_id), unit(*value))
            })
            .collect(),
    )
    .expect("candidate fixture must be valid")
}

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1e-12;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}
