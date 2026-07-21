//! Deterministic activation scoring and ranking for normalized numeric signals.

mod error;
mod model;
mod ranking;

pub use error::ActivationError;
pub use model::{
    ActivationCandidate, ActivationChannel, ActivationExplanation, ActivationProfile, CandidateId,
    ChannelId, ChannelSignal, EvidenceChannel, EvidenceContribution, InhibitionChannel,
    InhibitionContribution, RankedActivation, UnitInterval,
};
pub use ranking::{explain_activation, rank_activations};
