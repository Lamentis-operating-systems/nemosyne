use std::error::Error;
use std::fmt;

use super::{CandidateId, ChannelId};

/// An invalid activation input or incompatible candidate/profile structure.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ActivationError {
    /// A numeric input is non-finite or outside `[0, 1]`.
    InvalidUnitInterval {
        /// The rejected numeric value.
        value: f64,
    },
    /// A profile defines a channel identifier more than once.
    DuplicateProfileChannel {
        /// The duplicated channel identifier.
        channel_id: ChannelId,
    },
    /// Positive evidence parameters produced an unrepresentable effective weight.
    EffectiveWeightUnderflow {
        /// The affected evidence channel.
        channel_id: ChannelId,
    },
    /// No evidence channel has a positive effective weight.
    NoEffectiveEvidence,
    /// A candidate supplies a channel identifier more than once.
    DuplicateCandidateChannel {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The duplicated channel identifier.
        channel_id: ChannelId,
    },
    /// A candidate identifier occurs more than once in one ranking request.
    DuplicateCandidate {
        /// The duplicated candidate identifier.
        candidate_id: CandidateId,
    },
    /// A candidate omits a signal required by the profile.
    MissingSignal {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The missing channel identifier.
        channel_id: ChannelId,
    },
    /// A candidate supplies a signal not defined by the profile.
    UnexpectedSignal {
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The unexpected channel identifier.
        channel_id: ChannelId,
    },
}

impl fmt::Display for ActivationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUnitInterval { value } => {
                write!(formatter, "value {value:?} is not finite or outside [0, 1]")
            }
            Self::DuplicateProfileChannel { channel_id } => write!(
                formatter,
                "profile channel {} is defined more than once",
                channel_id.get()
            ),
            Self::EffectiveWeightUnderflow { channel_id } => write!(
                formatter,
                "evidence channel {} has positive weight and gate whose product underflows to zero",
                channel_id.get()
            ),
            Self::NoEffectiveEvidence => {
                formatter.write_str("profile has no positive effective evidence weight")
            }
            Self::DuplicateCandidateChannel {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} supplies channel {} more than once",
                candidate_id.get(),
                channel_id.get()
            ),
            Self::DuplicateCandidate { candidate_id } => write!(
                formatter,
                "candidate {} occurs more than once",
                candidate_id.get()
            ),
            Self::MissingSignal {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} is missing channel {}",
                candidate_id.get(),
                channel_id.get()
            ),
            Self::UnexpectedSignal {
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "candidate {} supplies unexpected channel {}",
                candidate_id.get(),
                channel_id.get()
            ),
        }
    }
}

impl Error for ActivationError {}
