use std::fmt;

use super::{GenerationAttemptId, PilotCondition, PilotTaskId};

/// Validation error for an internal synthetic-pilot artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticPilotError {
    /// A required textual or byte field is empty.
    EmptyField(&'static str),
    /// A stable numeric identifier is zero.
    ZeroIdentifier(&'static str),
    /// A generation-attempt identifier appears more than once.
    DuplicateAttempt(GenerationAttemptId),
    /// A task identifier appears more than once.
    DuplicateTask(PilotTaskId),
    /// An accepted generation attempt does not have exactly one matching task.
    AcceptedAttemptMismatch(GenerationAttemptId),
    /// A task does not reference its matching accepted generation attempt.
    TaskProvenanceMismatch(PilotTaskId),
    /// A corpus is paired with a generation log other than the one it binds.
    CorpusGenerationLogMismatch,
    /// A condition set is paired with a corpus other than the one it binds.
    ConditionCorpusMismatch,
    /// A task-condition artifact appears more than once.
    DuplicateConditionArtifact(PilotTaskId, PilotCondition),
    /// A task-condition artifact refers to an unknown task.
    UnknownConditionTask(PilotTaskId),
    /// A task lacks one of the seven pilot-only condition artifacts.
    MissingConditionArtifact(PilotTaskId, PilotCondition),
    /// Attention-bearing pilot conditions do not share one exact positive token count.
    AttentionTokenMismatch(PilotTaskId),
    /// A runner condition order is incomplete or contains a duplicate.
    InvalidConditionOrder,
    /// A runner seed appears more than once or the seed schedule is empty.
    InvalidSeedSchedule,
    /// An observation refers to a task not present in the frozen corpus.
    UnknownObservationTask(PilotTaskId),
    /// An observation seed is absent from the frozen runner manifest.
    UnknownObservationSeed(u64),
    /// A task-condition-seed observation appears more than once.
    DuplicateObservation(PilotTaskId, PilotCondition, u64),
    /// A completed receipt lacks a required task-condition-seed observation.
    MissingObservation(PilotTaskId, PilotCondition, u64),
    /// A completed receipt contains an unavailable cell.
    UnavailableCompletedCell(PilotTaskId, PilotCondition, u64),
}

impl fmt::Display for SyntheticPilotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for SyntheticPilotError {}
