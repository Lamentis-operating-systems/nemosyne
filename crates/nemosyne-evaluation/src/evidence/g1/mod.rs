//! Typed, pre-outcome G1 evaluation-envelope construction.

mod envelope;
mod error;
mod model;
mod run;

pub use envelope::{G1DesignV1, SignedG1EvaluationEnvelopeV1};
pub use error::G1EnvelopeError;
pub use model::{
    G1ArtifactBindingV1, G1ArtifactKind, G1AttentionMatchingV1, G1Baseline, G1Condition,
    G1ConditionArtifactV1, G1CriticalFailureBoundV1, G1CriticalFailureClass, G1Domain,
    G1ExecutionBindingV1, G1ExecutionIdentity, G1ExpectationRole, G1ExposureRequirementV1,
    G1ExposureScope, G1LeakageClass, G1PopulationV1, G1RunArtifactBindingV1, G1RunArtifactKind,
    G1SubgroupV1, G1TaskId, G1TaskV1, G1ThresholdKey, G1ThresholdV1, G1WrongControl,
};
pub use run::{AdmittedG1RunV1, finalize_g1_run_manifest};
