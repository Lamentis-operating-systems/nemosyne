use std::fmt;

use super::{
    EvidenceError,
    canonical::{Encoder, digest},
};

const COMPLETE_INPUT_DOMAIN: &[u8] = b"nemosyne.evidence.complete-input.v1";
const INCOMPLETE_INPUT_DOMAIN: &[u8] = b"nemosyne.evidence.consumed-prefix.v1";
pub(super) const MAX_PRINCIPAL_COUNT: usize = 256;
pub(super) const MAX_ESTABLISHED_IDENTITY_COUNT: usize = 64;

macro_rules! fixed_bytes_type {
    ($(#[$meta:meta])* $name:ident, $length:expr) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; $length]);

        impl $name {
            /// Constructs the value from its exact bytes.
            #[must_use]
            pub const fn from_bytes(bytes: [u8; $length]) -> Self {
                Self(bytes)
            }

            /// Returns the exact bytes.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; $length] {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}(", stringify!($name))?;
                for byte in self.0.iter().take(4) {
                    write!(formatter, "{byte:02x}")?;
                }
                formatter.write_str("…)")
            }
        }
    };
}

fixed_bytes_type!(
    /// Opaque identity of one validation attempt.
    AttemptId,
    32
);
fixed_bytes_type!(
    /// Domain-separated content identity of a signed artifact.
    ArtifactContentId,
    32
);
fixed_bytes_type!(
    /// SHA-256 evidence digest.
    EvidenceDigest,
    32
);
fixed_bytes_type!(
    /// Ed25519 evidence signature.
    EvidenceSignature,
    64
);
fixed_bytes_type!(
    /// Ed25519 verifying-key bytes.
    VerifyingKeyBytes,
    32
);
fixed_bytes_type!(
    /// Opaque allowlisted evidence identity.
    EvidenceIdentity,
    32
);
fixed_bytes_type!(
    /// Opaque validation or analysis principal identity.
    PrincipalId,
    32
);
fixed_bytes_type!(
    /// Opaque sealed outcome-source identity.
    SealedSourceId,
    32
);
fixed_bytes_type!(
    /// Versioned schema identity.
    SchemaId,
    32
);
fixed_bytes_type!(
    /// Guard implementation identity.
    GuardImplementationId,
    32
);
fixed_bytes_type!(
    /// Custodian identity derived from its verifying key.
    CustodianId,
    32
);
fixed_bytes_type!(
    /// Validator identity derived from its verifying key.
    ValidatorId,
    32
);
fixed_bytes_type!(
    /// Evaluator identity derived from its verifying key.
    EvaluatorId,
    32
);
fixed_bytes_type!(
    /// Validator implementation identity.
    ValidatorImplementationId,
    32
);
fixed_bytes_type!(
    /// Append-only ledger boundary commitment.
    LedgerCommitment,
    32
);

/// A trusted UTC instant represented as whole Unix seconds.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TrustedTimestamp(u64);

impl TrustedTimestamp {
    /// Constructs a timestamp from whole Unix seconds.
    #[must_use]
    pub const fn from_unix_seconds(seconds: u64) -> Self {
        Self(seconds)
    }

    /// Returns whole Unix seconds.
    #[must_use]
    pub const fn unix_seconds(self) -> u64 {
        self.0
    }
}

/// Supported evidence-envelope wire schema versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceSchemaVersion {
    /// Initial evidence-envelope schema.
    V1,
}

impl EvidenceSchemaVersion {
    /// Returns the stable numeric encoding.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        match self {
            Self::V1 => 1,
        }
    }
}

impl TryFrom<u16> for EvidenceSchemaVersion {
    type Error = EvidenceError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            version => Err(EvidenceError::UnknownSchemaVersion { version }),
        }
    }
}

/// Terminal experiment disposition retained after valid outcome admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceDisposition {
    /// All frozen gates passed.
    Pass,
    /// At least one frozen gate failed.
    Fail,
    /// Required evidence was invalid, missing, empty, or underexposed before
    /// affected arithmetic.
    Inconclusive,
}

impl EvidenceDisposition {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::Pass => 1,
            Self::Fail => 2,
            Self::Inconclusive => 3,
        }
    }
}

/// The closed attempted artifact domain.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AttemptedArtifactKind {
    /// A candidate G1 design envelope.
    G1Envelope,
    /// A complete G1 run manifest.
    G1RunManifest,
    /// A candidate-independent G9 protocol.
    G9Protocol,
    /// A complete post-verification G9 run manifest.
    G9RunManifest,
}

impl AttemptedArtifactKind {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::G1Envelope => 1,
            Self::G1RunManifest => 2,
            Self::G9Protocol => 3,
            Self::G9RunManifest => 4,
        }
    }

    pub(super) const fn is_run_manifest(self) -> bool {
        matches!(self, Self::G1RunManifest | Self::G9RunManifest)
    }
}

/// A closed bounded-parsing stage.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ValidationStage {
    /// Version and outer framing.
    Envelope,
    /// Canonical artifact structure.
    Structure,
    /// Pre-access guard evidence.
    Guard,
    /// Final manifest admission.
    Admission,
}

impl ValidationStage {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::Envelope => 1,
            Self::Structure => 2,
            Self::Guard => 3,
            Self::Admission => 4,
        }
    }
}

/// A closed field at which validation stopped.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ValidationField {
    /// Schema version.
    Schema,
    /// Artifact kind.
    ArtifactKind,
    /// Attempt identity.
    Attempt,
    /// Canonical manifest content.
    Manifest,
    /// Guard witness.
    GuardWitness,
    /// Sealed source.
    SealedSource,
    /// Validation or analysis principal set.
    Principals,
    /// Ledger boundary.
    Ledger,
    /// Signature.
    Signature,
}

impl ValidationField {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::Schema => 1,
            Self::ArtifactKind => 2,
            Self::Attempt => 3,
            Self::Manifest => 4,
            Self::GuardWitness => 5,
            Self::SealedSource => 6,
            Self::Principals => 7,
            Self::Ledger => 8,
            Self::Signature => 9,
        }
    }
}

/// Whether a commitment covers the complete attempted input or only the
/// consumed prefix available before bounded parsing stopped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputCompleteness {
    /// The digest covers the complete attempted input.
    Complete,
    /// The digest covers only the consumed prefix.
    Incomplete {
        /// Stage at which parsing stopped.
        stage: ValidationStage,
        /// Field at which parsing stopped.
        field: ValidationField,
    },
}

/// A non-retaining commitment to complete attempted bytes or a consumed
/// prefix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputCommitment {
    digest: EvidenceDigest,
    byte_length: u64,
    completeness: InputCompleteness,
}

impl InputCommitment {
    /// Commits to complete attempted input bytes without retaining them.
    pub fn complete(bytes: &[u8]) -> Result<Self, EvidenceError> {
        Ok(Self {
            digest: digest(COMPLETE_INPUT_DOMAIN, bytes),
            byte_length: u64::try_from(bytes.len()).map_err(|_| {
                EvidenceError::PayloadTooLarge {
                    actual: bytes.len(),
                    maximum: usize::MAX,
                }
            })?,
            completeness: InputCompleteness::Complete,
        })
    }

    /// Commits to the consumed prefix available when bounded parsing stopped.
    pub fn incomplete(
        consumed_prefix: &[u8],
        stage: ValidationStage,
        field: ValidationField,
    ) -> Result<Self, EvidenceError> {
        Ok(Self {
            digest: digest(INCOMPLETE_INPUT_DOMAIN, consumed_prefix),
            byte_length: u64::try_from(consumed_prefix.len()).map_err(|_| {
                EvidenceError::PayloadTooLarge {
                    actual: consumed_prefix.len(),
                    maximum: usize::MAX,
                }
            })?,
            completeness: InputCompleteness::Incomplete { stage, field },
        })
    }

    /// Returns the domain-separated digest.
    #[must_use]
    pub const fn digest(self) -> EvidenceDigest {
        self.digest
    }

    /// Returns the committed byte length.
    #[must_use]
    pub const fn byte_length(self) -> u64 {
        self.byte_length
    }

    /// Returns whether complete input or a consumed prefix was committed.
    #[must_use]
    pub const fn completeness(self) -> InputCompleteness {
        self.completeness
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.fixed(self.digest.as_bytes());
        encoder.u64(self.byte_length);
        match self.completeness {
            InputCompleteness::Complete => encoder.byte(1),
            InputCompleteness::Incomplete { stage, field } => {
                encoder.byte(2);
                encoder.byte(stage.tag());
                encoder.byte(field.tag());
            }
        }
    }
}

/// The allowlisted kind of an identity established before validation stopped.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EstablishedIdentityKind {
    /// Installed configuration.
    Configuration,
    /// Source root.
    SourceRoot,
    /// Dataset root.
    DatasetRoot,
    /// Candidate implementation.
    Implementation,
    /// Evidence custodian.
    Custody,
    /// Independent verifier.
    Verifier,
    /// Validation implementation.
    ValidationImplementation,
    /// Hardware class.
    HardwareClass,
    /// Operating system.
    OperatingSystem,
    /// Trusted-time source.
    TrustedTime,
    /// Signed artifact.
    SignedArtifact,
    /// Sealed outcome source.
    SealedSource,
}

impl EstablishedIdentityKind {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::Configuration => 1,
            Self::SourceRoot => 2,
            Self::DatasetRoot => 3,
            Self::Implementation => 4,
            Self::Custody => 5,
            Self::Verifier => 6,
            Self::ValidationImplementation => 7,
            Self::HardwareClass => 8,
            Self::OperatingSystem => 9,
            Self::TrustedTime => 10,
            Self::SignedArtifact => 11,
            Self::SealedSource => 12,
        }
    }
}

/// One canonical allowlisted identity established before validation stopped.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EstablishedIdentity {
    kind: EstablishedIdentityKind,
    identity: EvidenceIdentity,
}

impl EstablishedIdentity {
    /// Constructs an allowlisted established identity.
    #[must_use]
    pub const fn new(kind: EstablishedIdentityKind, identity: EvidenceIdentity) -> Self {
        Self { kind, identity }
    }

    /// Returns the allowlisted identity kind.
    #[must_use]
    pub const fn kind(self) -> EstablishedIdentityKind {
        self.kind
    }

    /// Returns the opaque identity.
    #[must_use]
    pub const fn identity(self) -> EvidenceIdentity {
        self.identity
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.byte(self.kind.tag());
        encoder.fixed(self.identity.as_bytes());
    }
}

/// Explicit state of a sealed source established before rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SealedSourceState {
    /// No sealed-source identity was established.
    Absent,
    /// This exact sealed-source identity was independently established.
    Established(SealedSourceId),
}

impl SealedSourceState {
    pub(super) fn encode(&self, encoder: &mut Encoder) {
        match self {
            Self::Absent => encoder.byte(1),
            Self::Established(identity) => {
                encoder.byte(2);
                encoder.fixed(identity.as_bytes());
            }
        }
    }
}

/// The closed reason for rejecting an attempted artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RejectionReason {
    /// Unsupported schema version.
    UnsupportedSchema,
    /// Malformed canonical structure.
    MalformedStructure,
    /// A required field is absent.
    MissingRequiredField,
    /// A field violates its bounded domain.
    InvalidField,
    /// A supplied reference does not recompute.
    ReferenceMismatch,
    /// A signature is invalid.
    InvalidSignature,
}

impl RejectionReason {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::UnsupportedSchema => 1,
            Self::MalformedStructure => 2,
            Self::MissingRequiredField => 3,
            Self::InvalidField => 4,
            Self::ReferenceMismatch => 5,
            Self::InvalidSignature => 6,
        }
    }
}

/// Closed state of outcome-capability issuance during a guard window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityIssuanceState {
    /// No outcome capability was issued.
    NotIssued,
}

impl CapabilityIssuanceState {
    pub(super) const fn tag(self) -> u8 {
        match self {
            Self::NotIssued => 1,
        }
    }
}

/// An inclusive trusted validation window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationWindow {
    start: TrustedTimestamp,
    end: TrustedTimestamp,
}

impl ValidationWindow {
    /// Constructs a nondecreasing trusted validation window.
    pub fn new(start: TrustedTimestamp, end: TrustedTimestamp) -> Result<Self, EvidenceError> {
        if end < start {
            return Err(EvidenceError::InvalidValidationWindow);
        }
        Ok(Self { start, end })
    }

    /// Returns the inclusive start.
    #[must_use]
    pub const fn start(self) -> TrustedTimestamp {
        self.start
    }

    /// Returns the inclusive end.
    #[must_use]
    pub const fn end(self) -> TrustedTimestamp {
        self.end
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.u64(self.start.unix_seconds());
        encoder.u64(self.end.unix_seconds());
    }
}

/// Head and tail commitments for one append-only ledger interval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LedgerBoundary {
    head: LedgerCommitment,
    tail: LedgerCommitment,
}

impl LedgerBoundary {
    /// Constructs a ledger boundary from opaque head and tail commitments.
    #[must_use]
    pub const fn new(head: LedgerCommitment, tail: LedgerCommitment) -> Self {
        Self { head, tail }
    }

    /// Returns the ledger head commitment.
    #[must_use]
    pub const fn head(self) -> LedgerCommitment {
        self.head
    }

    /// Returns the ledger tail commitment.
    #[must_use]
    pub const fn tail(self) -> LedgerCommitment {
        self.tail
    }

    pub(super) fn encode(&self, encoder: &mut Encoder) {
        encoder.fixed(self.head.as_bytes());
        encoder.fixed(self.tail.as_bytes());
    }
}

pub(super) fn canonical_principals(
    mut principals: Vec<PrincipalId>,
) -> Result<Box<[PrincipalId]>, EvidenceError> {
    if principals.is_empty() {
        return Err(EvidenceError::EmptyPrincipalSet);
    }
    if principals.len() > MAX_PRINCIPAL_COUNT {
        return Err(EvidenceError::TooManyPrincipals {
            actual: principals.len(),
            maximum: MAX_PRINCIPAL_COUNT,
        });
    }
    principals.sort_unstable();
    if principals.windows(2).any(|window| window[0] == window[1]) {
        return Err(EvidenceError::DuplicatePrincipal);
    }
    Ok(principals.into_boxed_slice())
}

pub(super) fn canonical_identities(
    mut identities: Vec<EstablishedIdentity>,
) -> Result<Box<[EstablishedIdentity]>, EvidenceError> {
    if identities.len() > MAX_ESTABLISHED_IDENTITY_COUNT {
        return Err(EvidenceError::TooManyEstablishedIdentities {
            actual: identities.len(),
            maximum: MAX_ESTABLISHED_IDENTITY_COUNT,
        });
    }
    identities.sort_unstable();
    if identities.windows(2).any(|window| window[0] == window[1]) {
        return Err(EvidenceError::DuplicateEstablishedIdentity);
    }
    Ok(identities.into_boxed_slice())
}

pub(super) fn encode_principals(principals: &[PrincipalId], encoder: &mut Encoder) {
    encoder
        .u32(u32::try_from(principals.len()).expect("validated principal set length fits in u32"));
    for principal in principals {
        encoder.fixed(principal.as_bytes());
    }
}

pub(super) fn schema_id(domain: &[u8]) -> SchemaId {
    SchemaId::from_bytes(*digest(b"nemosyne.evidence.schema-id.v1", domain).as_bytes())
}
