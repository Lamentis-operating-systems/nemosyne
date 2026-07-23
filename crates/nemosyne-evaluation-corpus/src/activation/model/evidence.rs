use nemosyne_core::activation::{CandidateId, ChannelId};
use nemosyne_evaluation::activation::{ExpectedPreference, ScenarioId};

use super::{EvidenceLevel, FactId, ScenarioCategoryId, ScenarioProvenance, SemanticCaseId};

/// Whether a channel judgment describes active evidence or a canonical inactive
/// evaluator slot.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum JudgmentApplicability {
    /// The level is an authored judgment under an active channel.
    Applicable,
    /// The corresponding gate is absent, so the numeric zero is structural
    /// rather than a candidate-fit judgment.
    Inactive,
}

/// The corpus-local semantics and authoring rubric for one evidence channel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceChannelDefinition {
    channel_id: ChannelId,
    key: Box<str>,
    gate_meaning: Box<str>,
    signal_meaning: Box<str>,
    gate_anchors: [Box<str>; 5],
    signal_anchors: [Box<str>; 5],
}

impl EvidenceChannelDefinition {
    pub(in crate::activation) fn new(
        channel_id: ChannelId,
        key: &str,
        gate_meaning: &str,
        signal_meaning: &str,
        gate_anchors: [&str; 5],
        signal_anchors: [&str; 5],
    ) -> Self {
        Self {
            channel_id,
            key: key.into(),
            gate_meaning: gate_meaning.into(),
            signal_meaning: signal_meaning.into(),
            gate_anchors: gate_anchors.map(Into::into),
            signal_anchors: signal_anchors.map(Into::into),
        }
    }

    /// Returns the numeric channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the stable corpus-local channel key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the situation-level meaning of the channel gate.
    #[must_use]
    pub fn gate_meaning(&self) -> &str {
        &self.gate_meaning
    }

    /// Returns the candidate-level meaning of the channel signal.
    #[must_use]
    pub fn signal_meaning(&self) -> &str {
        &self.signal_meaning
    }

    /// Returns the channel-specific gate description for an evidence level.
    #[must_use]
    pub fn gate_anchor(&self, level: EvidenceLevel) -> &str {
        &self.gate_anchors[level.index()]
    }

    /// Returns the channel-specific candidate-signal description for an
    /// evidence level.
    #[must_use]
    pub fn signal_anchor(&self, level: EvidenceLevel) -> &str {
        &self.signal_anchors[level.index()]
    }
}

/// One broad scenario category used to report corpus coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioCategory {
    category_id: ScenarioCategoryId,
    key: Box<str>,
    description: Box<str>,
}

impl ScenarioCategory {
    pub(in crate::activation) fn new(
        category_id: ScenarioCategoryId,
        key: &str,
        description: &str,
    ) -> Self {
        Self {
            category_id,
            key: key.into(),
            description: description.into(),
        }
    }

    /// Returns the category identifier.
    #[must_use]
    pub const fn category_id(&self) -> ScenarioCategoryId {
        self.category_id
    }

    /// Returns the stable category key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the category description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// One constructed fact used to justify scenario judgments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioFact {
    fact_id: FactId,
    statement: Box<str>,
}

impl ScenarioFact {
    pub(in crate::activation) fn new(fact_id: FactId, statement: &str) -> Self {
        Self {
            fact_id,
            statement: statement.into(),
        }
    }

    /// Returns the scenario-local fact identifier.
    #[must_use]
    pub const fn fact_id(&self) -> FactId {
        self.fact_id
    }

    /// Returns the constructed fact statement.
    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }
}

/// One channel value selected from the corpus authoring grid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchoredValue {
    channel_id: ChannelId,
    level: EvidenceLevel,
    applicability: JudgmentApplicability,
    fact_ids: Box<[FactId]>,
    rationale: Box<str>,
}

impl AnchoredValue {
    pub(in crate::activation) fn new(
        channel_id: ChannelId,
        level: EvidenceLevel,
        applicability: JudgmentApplicability,
        fact_ids: Box<[FactId]>,
        rationale: &str,
    ) -> Self {
        Self {
            channel_id,
            level,
            applicability,
            fact_ids,
            rationale: rationale.into(),
        }
    }

    /// Returns the channel identifier.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Returns the authored evidence level when applicable, or the canonical
    /// structural zero when [`Self::applicability`] is
    /// [`JudgmentApplicability::Inactive`].
    #[must_use]
    pub const fn level(&self) -> EvidenceLevel {
        self.level
    }

    /// Returns whether the value is an active judgment or a canonical inactive
    /// evaluator slot.
    #[must_use]
    pub const fn applicability(&self) -> JudgmentApplicability {
        self.applicability
    }

    /// Returns the facts cited for this individual channel judgment.
    #[must_use]
    pub fn fact_ids(&self) -> &[FactId] {
        &self.fact_ids
    }

    /// Returns the rationale for this individual channel judgment.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

/// Situation-level gate judgments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GateEvidence {
    values: Box<[AnchoredValue]>,
}

impl GateEvidence {
    pub(in crate::activation) const fn new(values: Box<[AnchoredValue]>) -> Self {
        Self { values }
    }

    /// Returns anchored gate values in ascending channel order.
    #[must_use]
    pub fn values(&self) -> &[AnchoredValue] {
        &self.values
    }
}

/// One candidate focus item and its annotated signal vector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateEvidence {
    candidate_id: CandidateId,
    label: Box<str>,
    signals: Box<[AnchoredValue]>,
}

impl CandidateEvidence {
    pub(in crate::activation) fn new(
        candidate_id: CandidateId,
        label: &str,
        signals: Box<[AnchoredValue]>,
    ) -> Self {
        Self {
            candidate_id,
            label: label.into(),
            signals,
        }
    }

    /// Returns the candidate identifier.
    #[must_use]
    pub const fn candidate_id(&self) -> CandidateId {
        self.candidate_id
    }

    /// Returns the concise candidate meaning.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns anchored candidate signals in ascending channel order.
    #[must_use]
    pub fn signals(&self) -> &[AnchoredValue] {
        &self.signals
    }
}

/// One expected strict preference and its semantic basis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreferenceEvidence {
    expectation: ExpectedPreference,
    fact_ids: Box<[FactId]>,
    rationale: Box<str>,
}

impl PreferenceEvidence {
    pub(in crate::activation) fn new(
        expectation: ExpectedPreference,
        fact_ids: Box<[FactId]>,
        rationale: &str,
    ) -> Self {
        Self {
            expectation,
            fact_ids,
            rationale: rationale.into(),
        }
    }

    /// Returns the expected strict preference.
    #[must_use]
    pub const fn expectation(&self) -> ExpectedPreference {
        self.expectation
    }

    /// Returns facts cited by the expected preference.
    #[must_use]
    pub fn fact_ids(&self) -> &[FactId] {
        &self.fact_ids
    }

    /// Returns the preference rationale.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

/// Reviewable semantic evidence for one numeric evaluation scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioEvidence {
    scenario_id: ScenarioId,
    semantic_case_id: SemanticCaseId,
    category_id: ScenarioCategoryId,
    title: Box<str>,
    situation: Box<str>,
    provenance: ScenarioProvenance,
    facts: Box<[ScenarioFact]>,
    gates: GateEvidence,
    candidates: Box<[CandidateEvidence]>,
    preferences: Box<[PreferenceEvidence]>,
}

impl ScenarioEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::activation) fn new(
        scenario_id: ScenarioId,
        semantic_case_id: SemanticCaseId,
        category_id: ScenarioCategoryId,
        title: &str,
        situation: &str,
        facts: Box<[ScenarioFact]>,
        gates: GateEvidence,
        candidates: Box<[CandidateEvidence]>,
        preferences: Box<[PreferenceEvidence]>,
    ) -> Self {
        Self {
            scenario_id,
            semantic_case_id,
            category_id,
            title: title.into(),
            situation: situation.into(),
            provenance: ScenarioProvenance::Constructed,
            facts,
            gates,
            candidates,
            preferences,
        }
    }

    /// Returns the numeric scenario identifier.
    #[must_use]
    pub const fn scenario_id(&self) -> ScenarioId {
        self.scenario_id
    }

    /// Returns the semantic source identifier.
    #[must_use]
    pub const fn semantic_case_id(&self) -> SemanticCaseId {
        self.semantic_case_id
    }

    /// Returns the broad category identifier.
    #[must_use]
    pub const fn category_id(&self) -> ScenarioCategoryId {
        self.category_id
    }

    /// Returns the scenario title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the constructed situation description.
    #[must_use]
    pub fn situation(&self) -> &str {
        &self.situation
    }

    /// Returns the declared provenance class.
    #[must_use]
    pub const fn provenance(&self) -> ScenarioProvenance {
        self.provenance
    }

    /// Returns facts in ascending identifier order.
    #[must_use]
    pub fn facts(&self) -> &[ScenarioFact] {
        &self.facts
    }

    /// Returns the annotated gate vector.
    #[must_use]
    pub const fn gates(&self) -> &GateEvidence {
        &self.gates
    }

    /// Returns candidate evidence in ascending candidate order.
    #[must_use]
    pub fn candidates(&self) -> &[CandidateEvidence] {
        &self.candidates
    }

    /// Returns preference evidence in canonical evaluator order.
    #[must_use]
    pub fn preferences(&self) -> &[PreferenceEvidence] {
        &self.preferences
    }
}
