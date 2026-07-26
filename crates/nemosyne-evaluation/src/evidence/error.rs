use std::{error::Error, fmt};

/// A fixed join field for successful outcome-access admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionJoinField {
    /// Opaque validation-attempt identity.
    AttemptId,
    /// Signed run-manifest content identity.
    RunManifestContentId,
    /// Signed run-manifest digest.
    RunManifestDigest,
    /// Signed run-manifest signature.
    RunManifestSignature,
    /// Sealed outcome-source identity.
    SealedSourceId,
    /// Validation-window start.
    ValidationWindowStart,
    /// Validation-window end.
    ValidationWindowEnd,
    /// Closed validation-principal set.
    ValidationPrincipalSet,
    /// Closed analysis-principal set.
    AnalysisPrincipalSet,
    /// Outcome-capability issuance state.
    OutcomeCapabilityIssuanceState,
    /// Outcome-access ledger head.
    OutcomeAccessLedgerHead,
    /// Outcome-access ledger tail.
    OutcomeAccessLedgerTail,
    /// Analysis-job ledger head.
    AnalysisJobLedgerHead,
    /// Analysis-job ledger tail.
    AnalysisJobLedgerTail,
}

/// A fixed join field for a guarded rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RejectionJoinField {
    /// Opaque validation-attempt identity.
    AttemptId,
    /// Attempted artifact kind.
    AttemptedKind,
    /// Complete-input or consumed-prefix commitment.
    InputCommitment,
    /// Explicit absent-or-established sealed-source state.
    EstablishedSealedSource,
}

/// The closed reason that required guard evidence could not support a terminal
/// pre-access result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardEvidenceError {
    /// No witness was supplied.
    Missing,
    /// Supplied witness material failed structural or signature validation.
    Invalid,
    /// The witness was valid but carried the other closed subject.
    WrongSubject,
    /// A rejection witness did not match the attempted rejection.
    RejectionMismatch {
        /// First mismatched field in fixed precedence.
        field: RejectionJoinField,
    },
    /// An admission witness did not match the validated run manifest.
    AdmissionMismatch {
        /// First mismatched field in fixed precedence.
        field: AdmissionJoinField,
    },
}

/// An evidence-envelope construction or validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceError {
    /// A validation window ends before it starts.
    InvalidValidationWindow,
    /// A principal set is empty.
    EmptyPrincipalSet,
    /// A principal occurs more than once.
    DuplicatePrincipal,
    /// A principal set exceeds its closed evidence-envelope bound.
    TooManyPrincipals {
        /// Supplied principal count.
        actual: usize,
        /// Maximum accepted count.
        maximum: usize,
    },
    /// An established identity occurs more than once.
    DuplicateEstablishedIdentity,
    /// An established-identity set exceeds its closed evidence-envelope bound.
    TooManyEstablishedIdentities {
        /// Supplied identity count.
        actual: usize,
        /// Maximum accepted count.
        maximum: usize,
    },
    /// The established sealed-source state is inconsistent with the allowlist.
    InvalidEstablishedSealedSource,
    /// An incomplete commitment names a different stop location than the
    /// rejected attempt.
    InconsistentCommitmentLocation,
    /// An encoded evidence schema version is unknown.
    UnknownSchemaVersion {
        /// Unsupported numeric schema version.
        version: u16,
    },
    /// An artifact kind cannot be used as a run manifest.
    ExpectedRunManifest,
    /// A payload is empty where a reconstructible payload is required.
    EmptyPayload,
    /// A bounded payload exceeds its contract.
    PayloadTooLarge {
        /// Supplied byte length.
        actual: usize,
        /// Maximum accepted byte length.
        maximum: usize,
    },
    /// The supplied signature or verifying key is invalid.
    InvalidSignature,
    /// The witness signer or guard implementation differs from the
    /// independently supplied authority.
    UntrustedGuardAuthority,
    /// A content identity or digest does not recompute from canonical bytes.
    ReferenceMismatch,
    /// A witness is required but missing.
    MissingGuardWitness,
    /// Supplied witness material is invalid.
    InvalidGuardWitness,
    /// A structurally valid witness has the wrong subject.
    WrongGuardSubject,
    /// A guarded rejection failed its fixed-precedence join.
    RejectionGuardMismatch {
        /// First mismatched field.
        field: RejectionJoinField,
    },
    /// A successful admission failed its fixed-precedence join.
    GuardWitnessMismatch {
        /// First mismatched field.
        field: AdmissionJoinField,
    },
}

impl fmt::Display for EvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValidationWindow => {
                formatter.write_str("validation window ends before it starts")
            }
            Self::EmptyPrincipalSet => formatter.write_str("principal set must not be empty"),
            Self::DuplicatePrincipal => formatter.write_str("principal set contains a duplicate"),
            Self::TooManyPrincipals { actual, maximum } => {
                write!(
                    formatter,
                    "principal count {actual} exceeds maximum {maximum}"
                )
            }
            Self::DuplicateEstablishedIdentity => {
                formatter.write_str("established identity set contains a duplicate")
            }
            Self::TooManyEstablishedIdentities { actual, maximum } => {
                write!(
                    formatter,
                    "established identity count {actual} exceeds maximum {maximum}"
                )
            }
            Self::InvalidEstablishedSealedSource => formatter
                .write_str("established sealed source is inconsistent with established identities"),
            Self::InconsistentCommitmentLocation => formatter
                .write_str("incomplete commitment location differs from rejection location"),
            Self::UnknownSchemaVersion { version } => {
                write!(formatter, "unknown evidence schema version {version}")
            }
            Self::ExpectedRunManifest => formatter.write_str("artifact kind is not a run manifest"),
            Self::EmptyPayload => formatter.write_str("payload must not be empty"),
            Self::PayloadTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "payload length {actual} exceeds maximum {maximum}"
                )
            }
            Self::InvalidSignature => formatter.write_str("signature is invalid"),
            Self::UntrustedGuardAuthority => {
                formatter.write_str("guard witness authority does not match trusted authority")
            }
            Self::ReferenceMismatch => {
                formatter.write_str("content identity or digest does not match canonical bytes")
            }
            Self::MissingGuardWitness => formatter.write_str("guard witness is missing"),
            Self::InvalidGuardWitness => formatter.write_str("guard witness is invalid"),
            Self::WrongGuardSubject => formatter.write_str("guard witness has the wrong subject"),
            Self::RejectionGuardMismatch { field } => {
                write!(formatter, "rejection guard mismatch at {field:?}")
            }
            Self::GuardWitnessMismatch { field } => {
                write!(formatter, "admission guard mismatch at {field:?}")
            }
        }
    }
}

impl Error for EvidenceError {}
