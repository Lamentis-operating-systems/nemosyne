use nemosyne_core::activation::{RankedActivation, UnitInterval};

use super::{ExpectedPreference, ScenarioId};

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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct OutcomeCounts {
    satisfied: usize,
    tied: usize,
    violated: usize,
}

impl OutcomeCounts {
    fn from_preferences(preferences: &[PreferenceEvaluation]) -> Self {
        let mut counts = Self::default();
        for preference in preferences {
            match preference.outcome() {
                PreferenceOutcome::Satisfied => counts.satisfied += 1,
                PreferenceOutcome::Tied => counts.tied += 1,
                PreferenceOutcome::Violated => counts.violated += 1,
            }
        }
        counts
    }

    fn add_assign(&mut self, other: Self) {
        self.satisfied += other.satisfied;
        self.tied += other.tied;
        self.violated += other.violated;
    }

    const fn total(self) -> usize {
        self.satisfied + self.tied + self.violated
    }

    const fn passed(self) -> bool {
        self.tied == 0 && self.violated == 0
    }
}

/// The complete deterministic evaluation of one scenario.
#[derive(Clone, Debug, PartialEq)]
pub struct ScenarioEvaluation {
    scenario_id: ScenarioId,
    ranking: Box<[RankedActivation]>,
    preferences: Box<[PreferenceEvaluation]>,
    counts: OutcomeCounts,
}

impl ScenarioEvaluation {
    pub(super) fn new(
        scenario_id: ScenarioId,
        ranking: Box<[RankedActivation]>,
        preferences: Box<[PreferenceEvaluation]>,
    ) -> Self {
        let counts = OutcomeCounts::from_preferences(&preferences);
        Self {
            scenario_id,
            ranking,
            preferences,
            counts,
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
        self.counts.total()
    }

    /// Returns the number of satisfied preferences.
    #[must_use]
    pub const fn satisfied_count(&self) -> usize {
        self.counts.satisfied
    }

    /// Returns the number of tied preferences.
    #[must_use]
    pub const fn tied_count(&self) -> usize {
        self.counts.tied
    }

    /// Returns the number of violated preferences.
    #[must_use]
    pub const fn violated_count(&self) -> usize {
        self.counts.violated
    }

    /// Returns the strict pairwise accuracy for this scenario.
    #[must_use]
    pub fn accuracy(&self) -> f64 {
        rate(self.counts.satisfied, self.counts.total())
    }

    /// Returns whether every expected preference was satisfied.
    #[must_use]
    pub const fn passed(&self) -> bool {
        self.counts.passed()
    }
}

/// Aggregate and per-scenario activation-parameter evaluation results.
#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationReport {
    scenarios: Box<[ScenarioEvaluation]>,
    counts: OutcomeCounts,
    passed_scenario_count: usize,
    macro_accuracy: f64,
}

impl EvaluationReport {
    pub(super) fn new(scenarios: Box<[ScenarioEvaluation]>) -> Self {
        debug_assert!(!scenarios.is_empty());

        let mut counts = OutcomeCounts::default();
        let mut passed_scenario_count = 0;
        let mut scenario_accuracy_sum = 0.0;
        for scenario in &scenarios {
            counts.add_assign(scenario.counts);
            passed_scenario_count += usize::from(scenario.passed());
            scenario_accuracy_sum += scenario.accuracy();
        }
        let macro_accuracy = bounded_rate(scenario_accuracy_sum / scenarios.len() as f64);

        Self {
            scenarios,
            counts,
            passed_scenario_count,
            macro_accuracy,
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
        self.counts.total()
    }

    /// Returns the total number of satisfied preferences.
    #[must_use]
    pub const fn satisfied_count(&self) -> usize {
        self.counts.satisfied
    }

    /// Returns the total number of tied preferences.
    #[must_use]
    pub const fn tied_count(&self) -> usize {
        self.counts.tied
    }

    /// Returns the total number of violated preferences.
    #[must_use]
    pub const fn violated_count(&self) -> usize {
        self.counts.violated
    }

    /// Returns the number of scenarios whose preferences were all satisfied.
    #[must_use]
    pub const fn passed_scenario_count(&self) -> usize {
        self.passed_scenario_count
    }

    /// Returns strict accuracy across all expected preferences.
    #[must_use]
    pub fn micro_accuracy(&self) -> f64 {
        rate(self.counts.satisfied, self.counts.total())
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
