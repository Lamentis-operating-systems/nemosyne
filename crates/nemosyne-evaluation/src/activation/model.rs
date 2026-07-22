use nemosyne_core::activation::{
    ActivationCandidate, CandidateId, ChannelId, RankedActivation, UnitInterval,
};

use super::EvaluationError;

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
    /// preferences, or a cyclic preference graph.
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
        if preferences.is_empty() {
            return Err(EvaluationError::NoPreferences { scenario_id });
        }
        for preference in &preferences {
            if preference.preferred == preference.other {
                return Err(EvaluationError::SelfPreference {
                    scenario_id,
                    candidate_id: preference.preferred,
                });
            }
            for candidate_id in [preference.preferred, preference.other] {
                if candidates
                    .binary_search_by_key(&candidate_id, ActivationCandidate::candidate_id)
                    .is_err()
                {
                    return Err(EvaluationError::UnknownPreferenceCandidate {
                        scenario_id,
                        candidate_id,
                    });
                }
            }
        }
        if let Some(pair) = preferences.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(EvaluationError::DuplicatePreference {
                scenario_id,
                preferred: pair[0].preferred,
                other: pair[0].other,
            });
        }
        if preferences_contain_cycle(&candidates, &preferences) {
            return Err(EvaluationError::CyclicPreferences { scenario_id });
        }

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

/// The result of one strict expected preference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreferenceOutcome {
    /// The preferred candidate has a strictly higher activation score.
    Satisfied,
    /// Both candidates have exactly equal activation scores.
    Tied,
    /// The preferred candidate has a lower activation score.
    Violated,
}

/// The evaluated scores and outcome for one expected preference.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreferenceEvaluation {
    expectation: ExpectedPreference,
    preferred_score: UnitInterval,
    other_score: UnitInterval,
    outcome: PreferenceOutcome,
}

impl PreferenceEvaluation {
    pub(super) const fn new(
        expectation: ExpectedPreference,
        preferred_score: UnitInterval,
        other_score: UnitInterval,
        outcome: PreferenceOutcome,
    ) -> Self {
        Self {
            expectation,
            preferred_score,
            other_score,
            outcome,
        }
    }

    /// Returns the evaluated expectation.
    #[must_use]
    pub const fn expectation(&self) -> ExpectedPreference {
        self.expectation
    }

    /// Returns the preferred candidate's activation score.
    #[must_use]
    pub const fn preferred_score(&self) -> UnitInterval {
        self.preferred_score
    }

    /// Returns the other candidate's activation score.
    #[must_use]
    pub const fn other_score(&self) -> UnitInterval {
        self.other_score
    }

    /// Returns the strict comparison outcome.
    #[must_use]
    pub const fn outcome(&self) -> PreferenceOutcome {
        self.outcome
    }
}

/// The complete deterministic evaluation of one scenario.
#[derive(Clone, Debug, PartialEq)]
pub struct ScenarioEvaluation {
    scenario_id: ScenarioId,
    ranking: Box<[RankedActivation]>,
    preferences: Box<[PreferenceEvaluation]>,
    satisfied_count: usize,
    tied_count: usize,
    violated_count: usize,
}

impl ScenarioEvaluation {
    pub(super) fn new(
        scenario_id: ScenarioId,
        ranking: Box<[RankedActivation]>,
        preferences: Box<[PreferenceEvaluation]>,
        satisfied_count: usize,
        tied_count: usize,
        violated_count: usize,
    ) -> Self {
        debug_assert_eq!(
            satisfied_count + tied_count + violated_count,
            preferences.len()
        );
        Self {
            scenario_id,
            ranking,
            preferences,
            satisfied_count,
            tied_count,
            violated_count,
        }
    }

    /// Returns the scenario identifier.
    #[must_use]
    pub const fn scenario_id(&self) -> ScenarioId {
        self.scenario_id
    }

    /// Returns the complete kernel ranking for this scenario.
    #[must_use]
    pub fn ranking(&self) -> &[RankedActivation] {
        &self.ranking
    }

    /// Returns evaluated preferences in canonical order.
    #[must_use]
    pub fn preferences(&self) -> &[PreferenceEvaluation] {
        &self.preferences
    }

    /// Returns the number of evaluated preferences.
    #[must_use]
    pub const fn preference_count(&self) -> usize {
        self.preferences.len()
    }

    /// Returns the number of satisfied preferences.
    #[must_use]
    pub const fn satisfied_count(&self) -> usize {
        self.satisfied_count
    }

    /// Returns the number of tied preferences.
    #[must_use]
    pub const fn tied_count(&self) -> usize {
        self.tied_count
    }

    /// Returns the number of violated preferences.
    #[must_use]
    pub const fn violated_count(&self) -> usize {
        self.violated_count
    }

    /// Returns the strict pairwise accuracy for this scenario.
    #[must_use]
    pub fn accuracy(&self) -> f64 {
        rate(self.satisfied_count, self.preference_count())
    }

    /// Returns whether every expected preference was satisfied.
    #[must_use]
    pub const fn passed(&self) -> bool {
        self.tied_count == 0 && self.violated_count == 0
    }
}

/// Aggregate and per-scenario activation-parameter evaluation results.
#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationReport {
    scenarios: Box<[ScenarioEvaluation]>,
    preference_count: usize,
    satisfied_count: usize,
    tied_count: usize,
    violated_count: usize,
    passed_scenario_count: usize,
    macro_accuracy: f64,
}

impl EvaluationReport {
    pub(super) fn new(
        scenarios: Box<[ScenarioEvaluation]>,
        preference_count: usize,
        satisfied_count: usize,
        tied_count: usize,
        violated_count: usize,
        passed_scenario_count: usize,
        macro_accuracy: f64,
    ) -> Self {
        debug_assert!(!scenarios.is_empty());
        debug_assert_eq!(
            satisfied_count + tied_count + violated_count,
            preference_count
        );
        debug_assert!(passed_scenario_count <= scenarios.len());
        Self {
            scenarios,
            preference_count,
            satisfied_count,
            tied_count,
            violated_count,
            passed_scenario_count,
            macro_accuracy: bounded_rate(macro_accuracy),
        }
    }

    /// Returns scenario reports in ascending scenario-identifier order.
    #[must_use]
    pub fn scenarios(&self) -> &[ScenarioEvaluation] {
        &self.scenarios
    }

    /// Returns the number of evaluated scenarios.
    #[must_use]
    pub const fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }

    /// Returns the total number of evaluated preferences.
    #[must_use]
    pub const fn preference_count(&self) -> usize {
        self.preference_count
    }

    /// Returns the total number of satisfied preferences.
    #[must_use]
    pub const fn satisfied_count(&self) -> usize {
        self.satisfied_count
    }

    /// Returns the total number of tied preferences.
    #[must_use]
    pub const fn tied_count(&self) -> usize {
        self.tied_count
    }

    /// Returns the total number of violated preferences.
    #[must_use]
    pub const fn violated_count(&self) -> usize {
        self.violated_count
    }

    /// Returns the number of scenarios whose preferences were all satisfied.
    #[must_use]
    pub const fn passed_scenario_count(&self) -> usize {
        self.passed_scenario_count
    }

    /// Returns strict accuracy across all expected preferences.
    #[must_use]
    pub fn micro_accuracy(&self) -> f64 {
        rate(self.satisfied_count, self.preference_count)
    }

    /// Returns the mean strict accuracy across scenarios.
    #[must_use]
    pub const fn macro_accuracy(&self) -> f64 {
        self.macro_accuracy
    }

    /// Returns the fraction of scenarios whose preferences were all satisfied.
    #[must_use]
    pub fn scenario_pass_rate(&self) -> f64 {
        rate(self.passed_scenario_count, self.scenario_count())
    }
}

fn preference_key(preference: &ExpectedPreference) -> (CandidateId, CandidateId) {
    (preference.preferred, preference.other)
}

fn preferences_contain_cycle(
    candidates: &[ActivationCandidate],
    preferences: &[ExpectedPreference],
) -> bool {
    let mut indegrees = vec![0_usize; candidates.len()];
    let mut outgoing = vec![Vec::new(); candidates.len()];

    for preference in preferences {
        let preferred_index = candidates
            .binary_search_by_key(&preference.preferred, ActivationCandidate::candidate_id)
            .expect("preference candidates were validated");
        let other_index = candidates
            .binary_search_by_key(&preference.other, ActivationCandidate::candidate_id)
            .expect("preference candidates were validated");
        outgoing[preferred_index].push(other_index);
        indegrees[other_index] += 1;
    }

    let mut available: Vec<usize> = indegrees
        .iter()
        .enumerate()
        .filter_map(|(index, degree)| (*degree == 0).then_some(index))
        .collect();
    let mut visited = 0;
    while let Some(index) = available.pop() {
        visited += 1;
        for &other_index in &outgoing[index] {
            indegrees[other_index] -= 1;
            if indegrees[other_index] == 0 {
                available.push(other_index);
            }
        }
    }

    visited != candidates.len()
}

fn rate(numerator: usize, denominator: usize) -> f64 {
    debug_assert!(denominator > 0);
    debug_assert!(numerator <= denominator);
    bounded_rate(numerator as f64 / denominator as f64)
}

fn bounded_rate(value: f64) -> f64 {
    debug_assert!(value.is_finite());
    canonical_zero(value.clamp(0.0, 1.0))
}

fn canonical_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}
