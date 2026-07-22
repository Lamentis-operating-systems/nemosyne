use nemosyne_core::activation::{ActivationCandidate, CandidateId, ChannelId, UnitInterval};

use super::{EvaluationError, preference_graph::validate_preferences};

/// A numeric identifier for one evaluation scenario.
///
/// The value is opaque. Zero has no special meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScenarioId(u64);

impl ScenarioId {
    /// Creates a scenario identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying numeric value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// A situation-independent weight for one evidence channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvidenceParameter {
    channel_id: ChannelId,
    weight: UnitInterval,
}

impl EvidenceParameter {
    /// Creates an evidence parameter.
    #[must_use]
    pub const fn new(channel_id: ChannelId, weight: UnitInterval) -> Self {
        Self { channel_id, weight }
    }

    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the situation-independent evidence weight.
    #[must_use]
    pub const fn weight(&self) -> UnitInterval {
        self.weight
    }
}

/// A situation-independent strength for one inhibition channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InhibitionParameter {
    channel_id: ChannelId,
    strength: UnitInterval,
}

impl InhibitionParameter {
    /// Creates an inhibition parameter.
    #[must_use]
    pub const fn new(channel_id: ChannelId, strength: UnitInterval) -> Self {
        Self {
            channel_id,
            strength,
        }
    }

    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the situation-independent inhibition strength.
    #[must_use]
    pub const fn strength(&self) -> UnitInterval {
        self.strength
    }
}

/// One situation-independent evidence or inhibition parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum ActivationParameter {
    /// A positive-evidence weight.
    Evidence(EvidenceParameter),
    /// A multiplicative-inhibition strength.
    Inhibition(InhibitionParameter),
}

impl ActivationParameter {
    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        match self {
            Self::Evidence(parameter) => parameter.channel_id,
            Self::Inhibition(parameter) => parameter.channel_id,
        }
    }

    /// Returns the evidence parameter when present.
    #[must_use]
    pub const fn evidence(&self) -> Option<&EvidenceParameter> {
        match self {
            Self::Evidence(parameter) => Some(parameter),
            Self::Inhibition(_) => None,
        }
    }

    /// Returns the inhibition parameter when present.
    #[must_use]
    pub const fn inhibition(&self) -> Option<&InhibitionParameter> {
        match self {
            Self::Evidence(_) => None,
            Self::Inhibition(parameter) => Some(parameter),
        }
    }
}

impl From<EvidenceParameter> for ActivationParameter {
    fn from(parameter: EvidenceParameter) -> Self {
        Self::Evidence(parameter)
    }
}

impl From<InhibitionParameter> for ActivationParameter {
    fn from(parameter: InhibitionParameter) -> Self {
        Self::Inhibition(parameter)
    }
}

/// A canonical set of situation-independent activation parameters.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationParameters {
    parameters: Box<[ActivationParameter]>,
}

impl ActivationParameters {
    /// Creates and canonicalizes activation parameters.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate channel identifiers or when no evidence
    /// parameter has a positive weight.
    pub fn new(mut parameters: Vec<ActivationParameter>) -> Result<Self, EvaluationError> {
        parameters.sort_unstable_by_key(ActivationParameter::channel_id);
        if let Some(pair) = parameters
            .windows(2)
            .find(|pair| pair[0].channel_id() == pair[1].channel_id())
        {
            return Err(EvaluationError::DuplicateParameterChannel {
                channel_id: pair[0].channel_id(),
            });
        }
        if !parameters.iter().any(|parameter| {
            parameter
                .evidence()
                .is_some_and(|evidence| evidence.weight().get() > 0.0)
        }) {
            return Err(EvaluationError::NoPositiveEvidenceWeight);
        }

        Ok(Self {
            parameters: parameters.into_boxed_slice(),
        })
    }

    /// Returns parameters in ascending channel-identifier order.
    #[must_use]
    pub fn parameters(&self) -> &[ActivationParameter] {
        &self.parameters
    }
}

/// A situation-dependent gate for one evidence channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvidenceGate {
    channel_id: ChannelId,
    gate: UnitInterval,
}

impl EvidenceGate {
    /// Creates an evidence gate.
    #[must_use]
    pub const fn new(channel_id: ChannelId, gate: UnitInterval) -> Self {
        Self { channel_id, gate }
    }

    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the situation-dependent gate value.
    #[must_use]
    pub const fn gate(&self) -> UnitInterval {
        self.gate
    }
}

/// An expected strict ranking relationship between two candidates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpectedPreference {
    preferred: CandidateId,
    other: CandidateId,
}

impl ExpectedPreference {
    /// Creates an expected preference.
    ///
    /// The containing scenario validates that both candidates exist and are
    /// distinct.
    #[must_use]
    pub const fn new(preferred: CandidateId, other: CandidateId) -> Self {
        Self { preferred, other }
    }

    /// Returns the candidate expected to rank higher.
    #[must_use]
    pub const fn preferred(&self) -> CandidateId {
        self.preferred
    }

    /// Returns the candidate expected to rank lower.
    #[must_use]
    pub const fn other(&self) -> CandidateId {
        self.other
    }
}

/// One numeric evaluation scenario.
#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationScenario {
    scenario_id: ScenarioId,
    gates: Box<[EvidenceGate]>,
    candidates: Box<[ActivationCandidate]>,
    preferences: Box<[ExpectedPreference]>,
}

impl EvaluationScenario {
    /// Creates and validates a numeric evaluation scenario.
    ///
    /// Candidate signal compatibility and exact gate correspondence with an
    /// activation parameter set are validated during evaluation.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate gates or candidates, fewer than two
    /// candidates, absent preferences, invalid candidate references, duplicate
    /// preferences, cycles, or transitively redundant preferences.
    pub fn new(
        scenario_id: ScenarioId,
        mut gates: Vec<EvidenceGate>,
        mut candidates: Vec<ActivationCandidate>,
        mut preferences: Vec<ExpectedPreference>,
    ) -> Result<Self, EvaluationError> {
        gates.sort_unstable_by_key(EvidenceGate::channel_id);
        if let Some(pair) = gates
            .windows(2)
            .find(|pair| pair[0].channel_id() == pair[1].channel_id())
        {
            return Err(EvaluationError::DuplicateEvidenceGate {
                scenario_id,
                channel_id: pair[0].channel_id(),
            });
        }

        candidates.sort_unstable_by_key(ActivationCandidate::candidate_id);
        if candidates.len() < 2 {
            return Err(EvaluationError::TooFewCandidates {
                scenario_id,
                count: candidates.len(),
            });
        }
        if let Some(pair) = candidates
            .windows(2)
            .find(|pair| pair[0].candidate_id() == pair[1].candidate_id())
        {
            return Err(EvaluationError::DuplicateCandidate {
                scenario_id,
                candidate_id: pair[0].candidate_id(),
            });
        }

        preferences.sort_unstable_by_key(preference_key);
        validate_preferences(scenario_id, &candidates, &preferences)?;

        Ok(Self {
            scenario_id,
            gates: gates.into_boxed_slice(),
            candidates: candidates.into_boxed_slice(),
            preferences: preferences.into_boxed_slice(),
        })
    }

    /// Returns the scenario identifier.
    #[must_use]
    pub const fn scenario_id(&self) -> ScenarioId {
        self.scenario_id
    }

    /// Returns gates in ascending channel-identifier order.
    #[must_use]
    pub fn gates(&self) -> &[EvidenceGate] {
        &self.gates
    }

    /// Returns candidates in ascending candidate-identifier order.
    #[must_use]
    pub fn candidates(&self) -> &[ActivationCandidate] {
        &self.candidates
    }

    /// Returns preferences ordered by preferred and then other candidate ID.
    #[must_use]
    pub fn preferences(&self) -> &[ExpectedPreference] {
        &self.preferences
    }
}

/// A canonical non-empty collection of evaluation scenarios.
#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationSuite {
    scenarios: Box<[EvaluationScenario]>,
}

impl EvaluationSuite {
    /// Creates and canonicalizes an evaluation suite.
    ///
    /// # Errors
    ///
    /// Returns an error when the input is empty or contains a duplicate
    /// scenario identifier.
    pub fn new(mut scenarios: Vec<EvaluationScenario>) -> Result<Self, EvaluationError> {
        if scenarios.is_empty() {
            return Err(EvaluationError::EmptySuite);
        }
        scenarios.sort_unstable_by_key(EvaluationScenario::scenario_id);
        if let Some(pair) = scenarios
            .windows(2)
            .find(|pair| pair[0].scenario_id == pair[1].scenario_id)
        {
            return Err(EvaluationError::DuplicateScenario {
                scenario_id: pair[0].scenario_id,
            });
        }

        Ok(Self {
            scenarios: scenarios.into_boxed_slice(),
        })
    }

    /// Returns scenarios in ascending identifier order.
    #[must_use]
    pub fn scenarios(&self) -> &[EvaluationScenario] {
        &self.scenarios
    }
}

fn preference_key(preference: &ExpectedPreference) -> (CandidateId, CandidateId) {
    (preference.preferred, preference.other)
}
