use std::error::Error;
use std::fmt;

use nemosyne_core::activation::{ActivationError, CandidateId, ChannelId};
use nemosyne_evaluation::activation::{EvaluationError, ScenarioId};

use super::{CorpusSplit, FactId, ReferenceId, ScenarioCategoryId, SemanticCaseId};

/// A defect in corpus metadata, structure, or derived evaluator input.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CorpusError {
    /// A required human-readable value is empty or padded with whitespace.
    InvalidText {
        /// A concise path to the invalid value.
        context: Box<str>,
    },
    /// An evidence channel occurs more than once.
    DuplicateChannel {
        /// The duplicated channel.
        channel_id: ChannelId,
    },
    /// The corpus channel set differs from the revision-1 schema.
    InvalidChannelSet {
        /// The observed channel identifiers.
        channel_ids: Box<[ChannelId]>,
    },
    /// A scenario category occurs more than once.
    DuplicateCategory {
        /// The duplicated category.
        category_id: ScenarioCategoryId,
    },
    /// A reference parameter identifier occurs more than once.
    DuplicateReference {
        /// The duplicated reference.
        reference_id: ReferenceId,
    },
    /// A corpus partition contains no scenario.
    EmptyPartition {
        /// The empty partition.
        split: CorpusSplit,
    },
    /// A scenario identifier occurs more than once in the corpus.
    DuplicateScenario {
        /// The duplicated scenario.
        scenario_id: ScenarioId,
    },
    /// A scenario-local fact identifier occurs more than once.
    DuplicateFact {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The duplicated fact.
        fact_id: FactId,
    },
    /// Scenario evidence cites a fact absent from that scenario.
    UnknownFact {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The unknown fact.
        fact_id: FactId,
    },
    /// One evidence item cites the same scenario fact more than once.
    DuplicateFactReference {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The repeated fact.
        fact_id: FactId,
    },
    /// One evidence item cites no scenario fact.
    EmptyFactReferences {
        /// The affected scenario.
        scenario_id: ScenarioId,
    },
    /// A scenario refers to an undefined broad category.
    UnknownCategory {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The unknown category.
        category_id: ScenarioCategoryId,
    },
    /// A semantic case occurs in both corpus partitions.
    CrossSplitSemanticCase {
        /// The leaked semantic source.
        semantic_case_id: SemanticCaseId,
    },
    /// A held-out preference repeats a complete calibration preference shape.
    CrossSplitPreferenceShape {
        /// The calibration scenario that first defines the numeric shape.
        calibration_scenario_id: ScenarioId,
        /// The held-out scenario that repeats the numeric shape.
        held_out_scenario_id: ScenarioId,
    },
    /// One semantic case refers to more than one broad category.
    CrossCategorySemanticCase {
        /// The inconsistent semantic source.
        semantic_case_id: SemanticCaseId,
    },
    /// A paired contrast does not contain exactly two scenarios.
    InvalidPairCardinality {
        /// The affected semantic source.
        semantic_case_id: SemanticCaseId,
        /// The observed scenario count.
        count: usize,
    },
    /// Paired scenarios do not retain the same candidate identities and labels.
    PairedCandidateMismatch {
        /// The affected semantic source.
        semantic_case_id: SemanticCaseId,
    },
    /// Paired scenarios do not contain exactly reversed preferences.
    PairedPreferenceMismatch {
        /// The affected semantic source.
        semantic_case_id: SemanticCaseId,
    },
    /// The two paired scenarios contain equal authored evidence.
    IndistinctPair {
        /// The affected semantic source.
        semantic_case_id: SemanticCaseId,
    },
    /// A category is absent from one required partition.
    MissingCategoryCoverage {
        /// The affected category.
        category_id: ScenarioCategoryId,
        /// The partition without coverage.
        split: CorpusSplit,
    },
    /// Revision 1 cannot evaluate the trigger-only reference in a scenario.
    InactiveTriggerGate {
        /// The affected scenario.
        scenario_id: ScenarioId,
    },
    /// An inactive candidate channel uses a nonzero signal level.
    NonCanonicalInactiveSignal {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The affected candidate.
        candidate_id: CandidateId,
        /// The inactive channel.
        channel_id: ChannelId,
    },
    /// Core candidate construction rejected authored signal evidence.
    Activation {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The underlying activation error.
        source: ActivationError,
    },
    /// Evaluator scenario construction rejected derived evidence.
    Scenario {
        /// The affected scenario.
        scenario_id: ScenarioId,
        /// The underlying evaluation error.
        source: EvaluationError,
    },
    /// Evaluator suite construction rejected one partition.
    Suite {
        /// The affected partition.
        split: CorpusSplit,
        /// The underlying evaluation error.
        source: EvaluationError,
    },
    /// Reference parameter construction failed.
    ReferenceParameters {
        /// The affected reference.
        reference_id: ReferenceId,
        /// The underlying evaluation error.
        source: EvaluationError,
    },
    /// A reference could not evaluate one complete partition.
    ReferenceEvaluation {
        /// The affected reference.
        reference_id: ReferenceId,
        /// The affected partition.
        split: CorpusSplit,
        /// The underlying evaluation error.
        source: EvaluationError,
    },
}

impl fmt::Display for CorpusError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidText { context } => {
                write!(
                    formatter,
                    "corpus text at {context} is empty or not trimmed"
                )
            }
            Self::DuplicateChannel { channel_id } => write!(
                formatter,
                "corpus channel {} is defined more than once",
                channel_id.get()
            ),
            Self::InvalidChannelSet { channel_ids } => write!(
                formatter,
                "corpus channels differ from revision 1: {:?}",
                channel_ids
                    .iter()
                    .map(|channel_id| channel_id.get())
                    .collect::<Vec<_>>()
            ),
            Self::DuplicateCategory { category_id } => write!(
                formatter,
                "scenario category {} is defined more than once",
                category_id.get()
            ),
            Self::DuplicateReference { reference_id } => write!(
                formatter,
                "reference parameter set {} is defined more than once",
                reference_id.get()
            ),
            Self::EmptyPartition { split } => {
                write!(formatter, "{split:?} corpus partition contains no scenario")
            }
            Self::DuplicateScenario { scenario_id } => write!(
                formatter,
                "scenario {} occurs more than once in the corpus",
                scenario_id.get()
            ),
            Self::DuplicateFact {
                scenario_id,
                fact_id,
            } => write!(
                formatter,
                "scenario {} defines fact {} more than once",
                scenario_id.get(),
                fact_id.get()
            ),
            Self::UnknownFact {
                scenario_id,
                fact_id,
            } => write!(
                formatter,
                "scenario {} cites unknown fact {}",
                scenario_id.get(),
                fact_id.get()
            ),
            Self::DuplicateFactReference {
                scenario_id,
                fact_id,
            } => write!(
                formatter,
                "scenario {} cites fact {} more than once in one evidence item",
                scenario_id.get(),
                fact_id.get()
            ),
            Self::EmptyFactReferences { scenario_id } => write!(
                formatter,
                "scenario {} contains evidence without cited facts",
                scenario_id.get()
            ),
            Self::UnknownCategory {
                scenario_id,
                category_id,
            } => write!(
                formatter,
                "scenario {} refers to unknown category {}",
                scenario_id.get(),
                category_id.get()
            ),
            Self::CrossSplitSemanticCase { semantic_case_id } => write!(
                formatter,
                "semantic case {} occurs in both corpus partitions",
                semantic_case_id.get()
            ),
            Self::CrossSplitPreferenceShape {
                calibration_scenario_id,
                held_out_scenario_id,
            } => write!(
                formatter,
                "held-out scenario {} repeats an evaluated preference shape from calibration scenario {}",
                held_out_scenario_id.get(),
                calibration_scenario_id.get()
            ),
            Self::CrossCategorySemanticCase { semantic_case_id } => write!(
                formatter,
                "semantic case {} crosses scenario categories",
                semantic_case_id.get()
            ),
            Self::InvalidPairCardinality {
                semantic_case_id,
                count,
            } => write!(
                formatter,
                "semantic case {} contains {count} scenarios; exactly two are required",
                semantic_case_id.get()
            ),
            Self::PairedCandidateMismatch { semantic_case_id } => write!(
                formatter,
                "semantic case {} does not preserve candidate identities and labels",
                semantic_case_id.get()
            ),
            Self::PairedPreferenceMismatch { semantic_case_id } => write!(
                formatter,
                "semantic case {} does not contain exactly reversed preferences",
                semantic_case_id.get()
            ),
            Self::IndistinctPair { semantic_case_id } => write!(
                formatter,
                "semantic case {} contains no authored contrast",
                semantic_case_id.get()
            ),
            Self::MissingCategoryCoverage { category_id, split } => write!(
                formatter,
                "scenario category {} has no {split:?} evidence",
                category_id.get()
            ),
            Self::InactiveTriggerGate { scenario_id } => write!(
                formatter,
                "scenario {} has no effective trigger-alignment gate",
                scenario_id.get()
            ),
            Self::NonCanonicalInactiveSignal {
                scenario_id,
                candidate_id,
                channel_id,
            } => write!(
                formatter,
                "scenario {} candidate {} uses a nonzero signal for inactive channel {}",
                scenario_id.get(),
                candidate_id.get(),
                channel_id.get()
            ),
            Self::Activation {
                scenario_id,
                source,
            } => write!(
                formatter,
                "scenario {} contains invalid activation evidence: {source}",
                scenario_id.get()
            ),
            Self::Scenario {
                scenario_id,
                source,
            } => write!(
                formatter,
                "scenario {} contains invalid evaluation evidence: {source}",
                scenario_id.get()
            ),
            Self::Suite { split, source } => {
                write!(
                    formatter,
                    "{split:?} evidence does not form a suite: {source}"
                )
            }
            Self::ReferenceParameters {
                reference_id,
                source,
            } => write!(
                formatter,
                "reference parameter set {} is invalid: {source}",
                reference_id.get()
            ),
            Self::ReferenceEvaluation {
                reference_id,
                split,
                source,
            } => write!(
                formatter,
                "reference parameter set {} cannot evaluate {split:?} evidence: {source}",
                reference_id.get()
            ),
        }
    }
}

impl Error for CorpusError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Activation { source, .. } => Some(source),
            Self::Scenario { source, .. }
            | Self::Suite { source, .. }
            | Self::ReferenceParameters { source, .. }
            | Self::ReferenceEvaluation { source, .. } => Some(source),
            _ => None,
        }
    }
}
