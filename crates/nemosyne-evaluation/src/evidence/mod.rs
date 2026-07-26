//! Versioned pre-access evidence and experiment-receipt contracts.
//!
//! This module owns only offline evidence custody. It does not grant runtime
//! authority, evaluate product behavior, or interpret experiment outcomes.

mod canonical;
mod crypto;
mod error;
mod manifest;
mod receipt;
mod rejection;
mod types;
mod witness;

pub use error::{AdmissionJoinField, EvidenceError, GuardEvidenceError, RejectionJoinField};
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
