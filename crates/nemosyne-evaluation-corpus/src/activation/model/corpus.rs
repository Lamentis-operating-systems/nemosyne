use nemosyne_evaluation::activation::{ActivationParameters, EvaluationSuite, ScenarioId};

use super::{
    CorpusRevision, CorpusSplit, EvidenceChannelDefinition, ReferenceId, ScenarioCategory,
    ScenarioEvidence,
};

/// One explicit parameter reference supplied with the corpus.
#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceParameterSet {
    reference_id: ReferenceId,
    key: Box<str>,
    rationale: Box<str>,
    parameters: ActivationParameters,
}

impl ReferenceParameterSet {
    pub(in crate::activation) fn new(
        reference_id: ReferenceId,
        key: &str,
        rationale: &str,
        parameters: ActivationParameters,
    ) -> Self {
        Self {
            reference_id,
            key: key.into(),
            rationale: rationale.into(),
            parameters,
        }
    }

    /// Returns the reference identifier.
    #[must_use]
    pub const fn reference_id(&self) -> ReferenceId {
        self.reference_id
    }

    /// Returns the stable reference key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the reason this reference is included.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }

    /// Returns the exact reference parameters.
    #[must_use]
    pub const fn parameters(&self) -> &ActivationParameters {
        &self.parameters
    }
}

/// One canonical corpus partition and its derived evaluator suite.
#[derive(Clone, Debug, PartialEq)]
pub struct CorpusPartition {
    split: CorpusSplit,
    evidence: Box<[ScenarioEvidence]>,
    suite: EvaluationSuite,
}

impl CorpusPartition {
    pub(in crate::activation) const fn new(
        split: CorpusSplit,
        evidence: Box<[ScenarioEvidence]>,
        suite: EvaluationSuite,
    ) -> Self {
        Self {
            split,
            evidence,
            suite,
        }
    }

    /// Returns the declared split.
    #[must_use]
    pub const fn split(&self) -> CorpusSplit {
        self.split
    }

    /// Returns scenario evidence in ascending scenario order.
    #[must_use]
    pub fn evidence(&self) -> &[ScenarioEvidence] {
        &self.evidence
    }

    /// Returns the evaluator suite derived from the semantic evidence.
    #[must_use]
    pub const fn suite(&self) -> &EvaluationSuite {
        &self.suite
    }

    /// Finds semantic evidence by scenario identifier.
    #[must_use]
    pub fn find(&self, scenario_id: ScenarioId) -> Option<&ScenarioEvidence> {
        self.evidence
            .binary_search_by_key(&scenario_id, ScenarioEvidence::scenario_id)
            .ok()
            .map(|index| &self.evidence[index])
    }
}

/// A versioned, canonical activation-evaluation evidence artifact.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivationEvidenceCorpus {
    revision: CorpusRevision,
    channels: Box<[EvidenceChannelDefinition]>,
    categories: Box<[ScenarioCategory]>,
    references: Box<[ReferenceParameterSet]>,
    calibration: CorpusPartition,
    held_out: CorpusPartition,
}

impl ActivationEvidenceCorpus {
    pub(in crate::activation) const fn new(
        revision: CorpusRevision,
        channels: Box<[EvidenceChannelDefinition]>,
        categories: Box<[ScenarioCategory]>,
        references: Box<[ReferenceParameterSet]>,
        calibration: CorpusPartition,
        held_out: CorpusPartition,
    ) -> Self {
        Self {
            revision,
            channels,
            categories,
            references,
            calibration,
            held_out,
        }
    }

    /// Returns the immutable corpus revision.
    #[must_use]
    pub const fn revision(&self) -> CorpusRevision {
        self.revision
    }

    /// Returns channel definitions in ascending identifier order.
    #[must_use]
    pub fn channels(&self) -> &[EvidenceChannelDefinition] {
        &self.channels
    }

    /// Returns scenario categories in ascending identifier order.
    #[must_use]
    pub fn categories(&self) -> &[ScenarioCategory] {
        &self.categories
    }

    /// Returns explicit references in ascending reference order.
    #[must_use]
    pub fn references(&self) -> &[ReferenceParameterSet] {
        &self.references
    }

    /// Returns the calibration partition.
    #[must_use]
    pub const fn calibration(&self) -> &CorpusPartition {
        &self.calibration
    }

    /// Returns the held-out partition.
    #[must_use]
    pub const fn held_out(&self) -> &CorpusPartition {
        &self.held_out
    }

    /// Returns a deterministic fingerprint of every publicly observable field.
    ///
    /// The fingerprint is a non-cryptographic regression value for detecting
    /// accidental edits to an immutable corpus revision. Equal artifacts
    /// produce equal values, but equality does not prove artifact identity,
    /// authenticity, or integrity.
    #[must_use]
    pub fn regression_fingerprint(&self) -> u64 {
        super::super::fingerprint::regression_fingerprint(self)
    }
}
