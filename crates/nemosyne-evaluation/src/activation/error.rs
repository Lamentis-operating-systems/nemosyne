use std::error::Error;
use std::fmt;

use nemosyne_core::activation::{ActivationError, CandidateId, ChannelId};

use super::ScenarioId;

/// An invalid activation-evaluation input or failed scenario evaluation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum EvaluationError {
    /// An activation parameter channel is defined more than once.
    DuplicateParameterChannel {
        /// The duplicated channel.
        channel_id: ChannelId,
    },
    /// No evidence parameter has a positive weight.
    NoPositiveEvidenceWeight,
    /// A scenario defines one evidence gate more than once.
    DuplicateEvidenceGate {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The duplicated channel.
        channel_id: ChannelId,
    },
    /// A scenario contains fewer than two candidates.
    TooFewCandidates {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The supplied candidate count.
        count: usize,
    },
    /// A candidate identifier occurs more than once in a scenario.
    DuplicateCandidate {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The duplicated candidate.
        candidate_id: CandidateId,
    },
    /// A scenario defines no expected preference.
    NoPreferences {
        /// The affected scenario.
        scenario_id: ScenarioId,
    },
    /// A preference compares a candidate with itself.
    SelfPreference {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The self-compared candidate.
        candidate_id: CandidateId,
    },
    /// A preference refers to a candidate absent from its scenario.
    UnknownPreferenceCandidate {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The unknown candidate.
        candidate_id: CandidateId,
    },
    /// The same expected preference occurs more than once.
    DuplicatePreference {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The candidate expected to rank higher.
        preferred: CandidateId,
        /// The candidate expected to rank lower.
        other: CandidateId,
    },
    /// A preference is implied by an alternate path in the preference graph.
    RedundantPreference {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The candidate expected to rank higher.
        preferred: CandidateId,
        /// The candidate expected to rank lower.
        other: CandidateId,
    },
    /// A scenario's expected preferences contain a directed cycle.
    CyclicPreferences {
        /// The affected scenario.
        scenario_id: ScenarioId,
    },
    /// An evaluation suite contains no scenario.
    EmptySuite,
    /// A scenario identifier occurs more than once in a suite.
    DuplicateScenario {
        /// The duplicated scenario.
        scenario_id: ScenarioId,
    },
    /// A scenario omits a gate required by an evidence parameter.
    MissingEvidenceGate {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The evidence channel without a gate.
        channel_id: ChannelId,
    },
    /// A scenario supplies a gate for an unknown or inhibition channel.
    UnexpectedEvidenceGate {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The unexpected gate channel.
        channel_id: ChannelId,
    },
    /// The activation kernel rejected one scenario.
    Activation {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The underlying activation error.
        source: ActivationError,
    },
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateParameterChannel { channel_id } => write!(
                formatter,
                "activation parameter channel {} is defined more than once",
                channel_id.get()
            ),
            Self::NoPositiveEvidenceWeight => {
                formatter.write_str("activation parameters contain no positive evidence weight")
            }
            Self::DuplicateEvidenceGate {
                scenario_id,
                channel_id,
            } => write!(
                formatter,
                "scenario {} defines evidence gate {} more than once",
                scenario_id.get(),
                channel_id.get()
            ),
            Self::TooFewCandidates { scenario_id, count } => write!(
                formatter,
                "scenario {} contains {count} candidates; at least two are required",
                scenario_id.get()
            ),
            Self::DuplicateCandidate {
                scenario_id,
                candidate_id,
            } => write!(
                formatter,
                "scenario {} contains candidate {} more than once",
                scenario_id.get(),
                candidate_id.get()
            ),
            Self::NoPreferences { scenario_id } => write!(
                formatter,
                "scenario {} contains no expected preference",
                scenario_id.get()
            ),
            Self::SelfPreference {
                scenario_id,
                candidate_id,
            } => write!(
                formatter,
                "scenario {} compares candidate {} with itself",
                scenario_id.get(),
                candidate_id.get()
            ),
            Self::UnknownPreferenceCandidate {
                scenario_id,
                candidate_id,
            } => write!(
                formatter,
                "scenario {} preference refers to unknown candidate {}",
                scenario_id.get(),
                candidate_id.get()
            ),
            Self::DuplicatePreference {
                scenario_id,
                preferred,
                other,
            } => write!(
                formatter,
                "scenario {} preference {} over {} is defined more than once",
                scenario_id.get(),
                preferred.get(),
                other.get()
            ),
            Self::RedundantPreference {
                scenario_id,
                preferred,
                other,
            } => write!(
                formatter,
                "scenario {} preference {} over {} is transitively redundant",
                scenario_id.get(),
                preferred.get(),
                other.get()
            ),
            Self::CyclicPreferences { scenario_id } => write!(
                formatter,
                "scenario {} expected preferences contain a cycle",
                scenario_id.get()
            ),
            Self::EmptySuite => formatter.write_str("evaluation suite contains no scenario"),
            Self::DuplicateScenario { scenario_id } => write!(
                formatter,
                "scenario {} occurs more than once in the evaluation suite",
                scenario_id.get()
            ),
            Self::MissingEvidenceGate {
                scenario_id,
                channel_id,
            } => write!(
                formatter,
                "scenario {} is missing evidence gate {}",
                scenario_id.get(),
                channel_id.get()
            ),
            Self::UnexpectedEvidenceGate {
                scenario_id,
                channel_id,
            } => write!(
                formatter,
                "scenario {} supplies unexpected evidence gate {}",
                scenario_id.get(),
                channel_id.get()
            ),
            Self::Activation {
                scenario_id,
                source,
            } => write!(
                formatter,
                "scenario {} could not be evaluated: {source}",
                scenario_id.get()
            ),
        }
    }
}

impl Error for EvaluationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Activation { source, .. } => Some(source),
            _ => None,
        }
    }
}
