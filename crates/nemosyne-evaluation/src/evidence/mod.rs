//! Versioned pre-access evidence and experiment-receipt contracts.
//!
//! This module owns only offline evidence custody. It does not grant runtime
//! authority, evaluate product behavior, or interpret experiment outcomes.

mod canonical;
mod crypto;
mod error;
mod g1;
mod manifest;
mod receipt;
mod rejection;
mod types;
mod witness;

pub use error::{AdmissionJoinField, EvidenceError, GuardEvidenceError, RejectionJoinField};
pub use g1::{
    G1ArtifactBindingV1, G1ArtifactKind, G1AttentionMatchingV1, G1Baseline, G1Condition,
    G1ConditionArtifactV1, G1CriticalFailureBoundV1, G1CriticalFailureClass, G1DesignV1, G1Domain,
    G1EnvelopeError, G1ExecutionBindingV1, G1ExecutionIdentity, G1ExpectationRole,
    G1ExposureRequirementV1, G1ExposureScope, G1LeakageClass, G1PopulationV1,
    G1RunArtifactBindingV1, G1RunArtifactKind, G1SubgroupV1, G1TaskId, G1TaskV1, G1ThresholdKey,
    G1ThresholdV1, G1WrongControl, SignedG1EvaluationEnvelopeV1, finalize_g1_run_manifest,
};
pub use manifest::{RunManifestClaimsV1, SignedRunManifestV1, ValidForOutcomeAccess};
pub use receipt::{ExperimentReceiptPayloadV1, ValidExperimentReceiptV1};
pub use rejection::{
    PreAccessCustodyFailureRecord, PreAccessRejectionReceipt, PreAccessValidationResult,
    RejectedAttemptV1, ValidatorContext, admit_for_outcome_access, finalize_rejection,
};
pub use types::{
    ArtifactContentId, AttemptId, AttemptedArtifactKind, CapabilityIssuanceState, CustodianId,
    EstablishedIdentity, EstablishedIdentityKind, EvaluatorId, EvidenceDigest, EvidenceDisposition,
    EvidenceIdentity, EvidenceSchemaVersion, EvidenceSignature, GuardImplementationId,
    InputCommitment, InputCompleteness, LedgerBoundary, LedgerCommitment, PrincipalId,
    RejectionReason, SchemaId, SealedSourceId, SealedSourceState, TrustedTimestamp,
    ValidationField, ValidationStage, ValidationWindow, ValidatorId, ValidatorImplementationId,
    VerifyingKeyBytes,
};
pub use witness::{
    GuardAuthorityV1, GuardSubjectV1, GuardWitnessClaimsV1, GuardWitnessEvidence, GuardWitnessV1,
    RejectedGuardSubjectV1, ValidatedRunGuardSubjectV1,
};
