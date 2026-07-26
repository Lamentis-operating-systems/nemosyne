use std::{error::Error, fmt};

use super::{
    G1ArtifactKind, G1Condition, G1CriticalFailureClass, G1Domain, G1ExposureScope,
    G1RunArtifactKind, G1SubgroupV1, G1TaskId, G1ThresholdKey,
};
use crate::evidence::EvidenceError;

/// Failure to construct or authenticate a pre-outcome G1 envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum G1EnvelopeError {
    /// A required opaque identity is the all-zero sentinel.
    EmptyIdentity,
    /// A bounded collection is empty.
    EmptyCollection,
    /// A bounded collection exceeds its limit.
    TooManyItems {
        /// Supplied item count.
        actual: usize,
        /// Maximum accepted count.
        maximum: usize,
    },
    /// One condition occurs more than once.
    DuplicateCondition {
        /// Duplicated condition.
        condition: G1Condition,
    },
    /// A required condition is missing.
    MissingCondition {
        /// Missing condition.
        condition: G1Condition,
    },
    /// One required artifact kind occurs more than once.
    DuplicateArtifact {
        /// Duplicated artifact kind.
        kind: G1ArtifactKind,
    },
    /// A required artifact kind is missing.
    MissingArtifact {
        /// Missing artifact kind.
        kind: G1ArtifactKind,
    },
    /// One required run artifact kind occurs more than once.
    DuplicateRunArtifact {
        /// Duplicated run artifact kind.
        kind: G1RunArtifactKind,
    },
    /// A required run artifact kind is missing.
    MissingRunArtifact {
        /// Missing run artifact kind.
        kind: G1RunArtifactKind,
    },
    /// Attention matching has a zero or inconsistent bound.
    InvalidAttentionBounds,
    /// One task identifier occurs more than once.
    DuplicateTask {
        /// Duplicated task.
        task_id: G1TaskId,
    },
    /// A task has a zero design-weight numerator.
    ZeroDesignWeight {
        /// Invalid task.
        task_id: G1TaskId,
    },
    /// The exact rational design-weight mass is not one.
    InvalidDesignWeightMass,
    /// An expectation-eligible task is outside the context-dependent domain.
    ExpectationTaskOutsideDependentDomain {
        /// Invalid task.
        task_id: G1TaskId,
    },
    /// One required population domain is empty.
    EmptyDomain {
        /// Empty domain.
        domain: G1Domain,
    },
    /// The expectation-eligible subset is empty.
    EmptyExpectationSubset,
    /// One exposure slice occurs more than once.
    DuplicateExposure {
        /// Duplicated scope.
        scope: G1ExposureScope,
        /// Optional subgroup for the scope.
        subgroup: Option<G1SubgroupV1>,
    },
    /// A required exposure slice is absent.
    MissingExposure {
        /// Missing scope.
        scope: G1ExposureScope,
        /// Optional subgroup for the scope.
        subgroup: Option<G1SubgroupV1>,
    },
    /// An exposure minimum is zero or exceeds designed membership.
    InvalidExposure {
        /// Invalid scope.
        scope: G1ExposureScope,
        /// Optional subgroup for the scope.
        subgroup: Option<G1SubgroupV1>,
    },
    /// One threshold key occurs more than once.
    DuplicateThreshold {
        /// Duplicated threshold.
        key: G1ThresholdKey,
    },
    /// A required threshold is absent.
    MissingThreshold {
        /// Missing threshold.
        key: G1ThresholdKey,
    },
    /// A threshold is non-finite or outside its proof-owned domain.
    InvalidThreshold {
        /// Invalid threshold.
        key: G1ThresholdKey,
    },
    /// One critical-failure class occurs more than once.
    DuplicateCriticalFailure {
        /// Duplicated class.
        class: G1CriticalFailureClass,
    },
    /// A required critical-failure class is absent.
    MissingCriticalFailure {
        /// Missing class.
        class: G1CriticalFailureClass,
    },
    /// A critical-failure exposure is zero.
    InvalidCriticalFailureExposure {
        /// Invalid class.
        class: G1CriticalFailureClass,
    },
    /// A critical-failure rate bound is non-finite or outside `(0, 1)`.
    InvalidCriticalFailureRate {
        /// Invalid class.
        class: G1CriticalFailureClass,
    },
    /// An embedded EVD-01 operation failed.
    Evidence(EvidenceError),
}

impl fmt::Display for G1EnvelopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for G1EnvelopeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Evidence(source) => Some(source),
            _ => None,
        }
    }
}

impl From<EvidenceError> for G1EnvelopeError {
    fn from(value: EvidenceError) -> Self {
        Self::Evidence(value)
    }
}
