use ed25519_dalek::SigningKey;

use super::{
    ArtifactContentId, AttemptId, AttemptedArtifactKind, EstablishedIdentity,
    EstablishedIdentityKind, EvidenceDigest, EvidenceError, EvidenceIdentity, EvidenceSignature,
    GuardEvidenceError, GuardSubjectV1, GuardWitnessEvidence, InputCommitment, InputCompleteness,
    RejectionJoinField, RejectionReason, SchemaId, SealedSourceState, SignedRunManifestV1,
    TrustedTimestamp, ValidForOutcomeAccess, ValidationField, ValidationStage, ValidatorId,
    ValidatorImplementationId, VerifyingKeyBytes,
    canonical::{Encoder, digest},
    crypto,
    types::{canonical_identities, schema_id},
};

const REJECTION_SCHEMA_DOMAIN: &[u8] = b"nemosyne.evidence.rejection-receipt.schema.v1";
const REJECTION_CANONICAL_DOMAIN: &[u8] = b"nemosyne.evidence.rejection-receipt.canonical.v1";
const REJECTION_CONTENT_ID_DOMAIN: &[u8] = b"nemosyne.evidence.rejection-receipt.content-id.v1";
const REJECTION_DIGEST_DOMAIN: &[u8] = b"nemosyne.evidence.rejection-receipt.digest.v1";
const REJECTION_SIGNATURE_DOMAIN: &[u8] = b"nemosyne.evidence.rejection-receipt.signature.v1";
const CUSTODY_FAILURE_SCHEMA_DOMAIN: &[u8] = b"nemosyne.evidence.custody-failure.schema.v1";
const CUSTODY_FAILURE_CANONICAL_DOMAIN: &[u8] = b"nemosyne.evidence.custody-failure.canonical.v1";
const CUSTODY_FAILURE_SIGNATURE_DOMAIN: &[u8] = b"nemosyne.evidence.custody-failure.signature.v1";

/// One completely classified rejected pre-access attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedAttemptV1 {
    attempt_id: AttemptId,
    attempted_kind: AttemptedArtifactKind,
    input_commitment: InputCommitment,
    established_identities: Box<[EstablishedIdentity]>,
    established_sealed_source: SealedSourceState,
    stage: ValidationStage,
    field: ValidationField,
    reason: RejectionReason,
}

impl RejectedAttemptV1 {
    /// Constructs a canonical rejection attempt from allowlisted identities.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        attempt_id: AttemptId,
        attempted_kind: AttemptedArtifactKind,
        input_commitment: InputCommitment,
        established_identities: Vec<EstablishedIdentity>,
        established_sealed_source: SealedSourceState,
        stage: ValidationStage,
        field: ValidationField,
        reason: RejectionReason,
    ) -> Result<Self, EvidenceError> {
        if let InputCompleteness::Incomplete {
            stage: commitment_stage,
            field: commitment_field,
        } = input_commitment.completeness()
            && (commitment_stage != stage || commitment_field != field)
        {
            return Err(EvidenceError::InconsistentCommitmentLocation);
        }
        let established_identities = canonical_identities(established_identities)?;
        validate_sealed_source(established_sealed_source, &established_identities)?;
        Ok(Self {
            attempt_id,
            attempted_kind,
            input_commitment,
            established_identities,
            established_sealed_source,
            stage,
            field,
            reason,
        })
    }

    /// Returns the opaque attempt identity.
    #[must_use]
    pub const fn attempt_id(&self) -> AttemptId {
        self.attempt_id
    }

    /// Returns the attempted artifact kind.
    #[must_use]
    pub const fn attempted_kind(&self) -> AttemptedArtifactKind {
        self.attempted_kind
    }

    /// Returns the non-retaining input commitment.
    #[must_use]
    pub const fn input_commitment(&self) -> InputCommitment {
        self.input_commitment
    }

    /// Returns established allowlisted identities in canonical order.
    #[must_use]
    pub fn established_identities(&self) -> &[EstablishedIdentity] {
        &self.established_identities
    }

    /// Returns the explicit absent-or-established sealed-source state.
    #[must_use]
    pub const fn established_sealed_source(&self) -> SealedSourceState {
        self.established_sealed_source
    }

    /// Returns the validation stage that rejected the attempt.
    #[must_use]
    pub const fn stage(&self) -> ValidationStage {
        self.stage
    }

    /// Returns the validation field that rejected the attempt.
    #[must_use]
    pub const fn field(&self) -> ValidationField {
        self.field
    }

    /// Returns the closed rejection reason.
    #[must_use]
    pub const fn reason(&self) -> RejectionReason {
        self.reason
    }
}

/// Validator-owned signing context for terminal pre-access records.
pub struct ValidatorContext<'key> {
    trusted_time: TrustedTimestamp,
    implementation_id: ValidatorImplementationId,
    signing_key_bytes: &'key [u8; 32],
}

impl<'key> ValidatorContext<'key> {
    /// Constructs a validator context without retaining or copying the signing
    /// key into an evidence artifact.
    #[must_use]
    pub const fn new(
        trusted_time: TrustedTimestamp,
        implementation_id: ValidatorImplementationId,
        signing_key_bytes: &'key [u8; 32],
    ) -> Self {
        Self {
            trusted_time,
            implementation_id,
            signing_key_bytes,
        }
    }
}

/// Exactly one terminal pre-access validation result.
#[derive(Debug, Eq, PartialEq)]
pub enum PreAccessValidationResult<T> {
    /// A complete valid artifact or sealed admission.
    Valid(T),
    /// A rejection backed by an exactly matching valid rejection witness.
    Rejected(Box<PreAccessRejectionReceipt>),
    /// Validation stopped because required guard evidence was unavailable.
    CustodyEvidenceUnavailable(Box<PreAccessCustodyFailureRecord>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GuardWitnessReference {
    content_id: ArtifactContentId,
    digest: EvidenceDigest,
    signature: EvidenceSignature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ValidatorAuthentication {
    implementation_id: ValidatorImplementationId,
    trusted_time: TrustedTimestamp,
    validator_id: ValidatorId,
    validator_key: VerifyingKeyBytes,
}

/// A guarded rejection containing no raw attempted or witness bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreAccessRejectionReceipt {
    schema_id: SchemaId,
    attempt: RejectedAttemptV1,
    witness: GuardWitnessReference,
    content_id: ArtifactContentId,
    digest: EvidenceDigest,
    validator: ValidatorAuthentication,
    signature: EvidenceSignature,
}

impl PreAccessRejectionReceipt {
    /// Returns the evidence schema identity.
    #[must_use]
    pub const fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    /// Returns the rejected attempt metadata.
    #[must_use]
    pub const fn attempt(&self) -> &RejectedAttemptV1 {
        &self.attempt
    }

    /// Returns the matching guard-witness content identity.
    #[must_use]
    pub const fn witness_content_id(&self) -> ArtifactContentId {
        self.witness.content_id
    }

    /// Returns the matching guard-witness digest.
    #[must_use]
    pub const fn witness_digest(&self) -> EvidenceDigest {
        self.witness.digest
    }

    /// Returns the matching guard-witness signature.
    #[must_use]
    pub const fn witness_signature(&self) -> EvidenceSignature {
        self.witness.signature
    }

    /// Returns the receipt content identity.
    #[must_use]
    pub const fn content_id(&self) -> ArtifactContentId {
        self.content_id
    }

    /// Returns the receipt digest.
    #[must_use]
    pub const fn digest(&self) -> EvidenceDigest {
        self.digest
    }

    /// Returns the validator implementation identity.
    #[must_use]
    pub const fn validator_implementation_id(&self) -> ValidatorImplementationId {
        self.validator.implementation_id
    }

    /// Returns the trusted validation time.
    #[must_use]
    pub const fn trusted_time(&self) -> TrustedTimestamp {
        self.validator.trusted_time
    }

    /// Returns the validator identity derived from its verifying key.
    #[must_use]
    pub const fn validator_id(&self) -> ValidatorId {
        self.validator.validator_id
    }

    /// Returns the validator verifying key.
    #[must_use]
    pub const fn validator_key(&self) -> VerifyingKeyBytes {
        self.validator.validator_key
    }

    /// Returns the validator signature.
    #[must_use]
    pub const fn signature(&self) -> EvidenceSignature {
        self.signature
    }

    /// Recomputes content references and verifies the validator signature.
    pub fn verify(&self) -> Result<(), EvidenceError> {
        let canonical =
            canonical_rejection(self.schema_id, &self.attempt, self.witness, self.validator);
        let content_id = ArtifactContentId::from_bytes(
            *digest(REJECTION_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let digest = digest(REJECTION_DIGEST_DOMAIN, &canonical);
        if content_id != self.content_id || digest != self.digest {
            return Err(EvidenceError::ReferenceMismatch);
        }
        if self.validator.validator_id
            != ValidatorId::from_bytes(crypto::signer_id(self.validator.validator_key))
        {
            return Err(EvidenceError::ReferenceMismatch);
        }
        crypto::verify(
            REJECTION_SIGNATURE_DOMAIN,
            &canonical,
            self.validator.validator_key,
            self.signature,
        )
    }
}

/// Minimal validator-authenticated evidence that guard validation stopped.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreAccessCustodyFailureRecord {
    schema_id: SchemaId,
    attempt_id: AttemptId,
    attempted_kind: AttemptedArtifactKind,
    input_commitment: InputCommitment,
    established_identities: Box<[EstablishedIdentity]>,
    stage: ValidationStage,
    error: GuardEvidenceError,
    validator_implementation_id: ValidatorImplementationId,
    trusted_time: TrustedTimestamp,
    validator_id: ValidatorId,
    validator_key: VerifyingKeyBytes,
    signature: EvidenceSignature,
}

impl PreAccessCustodyFailureRecord {
    /// Returns the evidence schema identity.
    #[must_use]
    pub const fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    /// Returns the opaque attempt identity.
    #[must_use]
    pub const fn attempt_id(&self) -> AttemptId {
        self.attempt_id
    }

    /// Returns the attempted artifact kind.
    #[must_use]
    pub const fn attempted_kind(&self) -> AttemptedArtifactKind {
        self.attempted_kind
    }

    /// Returns the non-retaining input commitment.
    #[must_use]
    pub const fn input_commitment(&self) -> InputCommitment {
        self.input_commitment
    }

    /// Returns established allowlisted identities in canonical order.
    #[must_use]
    pub fn established_identities(&self) -> &[EstablishedIdentity] {
        &self.established_identities
    }

    /// Returns the stage at which guard validation stopped.
    #[must_use]
    pub const fn stage(&self) -> ValidationStage {
        self.stage
    }

    /// Returns the closed guard-evidence error.
    #[must_use]
    pub const fn error(&self) -> GuardEvidenceError {
        self.error
    }

    /// Returns the validator implementation identity.
    #[must_use]
    pub const fn validator_implementation_id(&self) -> ValidatorImplementationId {
        self.validator_implementation_id
    }

    /// Returns the trusted validator time.
    #[must_use]
    pub const fn trusted_time(&self) -> TrustedTimestamp {
        self.trusted_time
    }

    /// Returns the validator identity derived from its verifying key.
    #[must_use]
    pub const fn validator_id(&self) -> ValidatorId {
        self.validator_id
    }

    /// Returns the validator verifying key.
    #[must_use]
    pub const fn validator_key(&self) -> VerifyingKeyBytes {
        self.validator_key
    }

    /// Returns the validator signature.
    #[must_use]
    pub const fn signature(&self) -> EvidenceSignature {
        self.signature
    }

    /// Verifies the validator signature over the complete minimal record.
    pub fn verify(&self) -> Result<(), EvidenceError> {
        if self.validator_id != ValidatorId::from_bytes(crypto::signer_id(self.validator_key)) {
            return Err(EvidenceError::ReferenceMismatch);
        }
        crypto::verify(
            CUSTODY_FAILURE_SIGNATURE_DOMAIN,
            &canonical_custody_failure(self),
            self.validator_key,
            self.signature,
        )
    }
}

/// Finalizes a rejected attempt or returns a disjoint custody-failure record
/// when the required rejection witness is unavailable.
pub fn finalize_rejection<T>(
    attempt: RejectedAttemptV1,
    witness_evidence: &GuardWitnessEvidence,
    validator: &ValidatorContext<'_>,
) -> PreAccessValidationResult<T> {
    match checked_rejection_witness(&attempt, witness_evidence) {
        Ok(witness) => PreAccessValidationResult::Rejected(Box::new(sign_rejection(
            attempt, witness, validator,
        ))),
        Err(error) => PreAccessValidationResult::CustodyEvidenceUnavailable(Box::new(
            sign_custody_failure(&attempt, map_guard_error(error), validator),
        )),
    }
}

/// Admits a complete signed run manifest or returns a disjoint custody-failure
/// record when its required admission witness is unavailable.
pub fn admit_for_outcome_access(
    manifest: &SignedRunManifestV1,
    witness_evidence: &GuardWitnessEvidence,
    established_identities: Vec<EstablishedIdentity>,
    validator: &ValidatorContext<'_>,
) -> Result<PreAccessValidationResult<ValidForOutcomeAccess>, EvidenceError> {
    let identities = canonical_identities(established_identities)?;
    let source_state = SealedSourceState::Established(manifest.claims().sealed_source_id());
    validate_sealed_source(source_state, &identities)?;
    match ValidForOutcomeAccess::new(manifest, witness_evidence) {
        Ok(admission) => Ok(PreAccessValidationResult::Valid(admission)),
        Err(error) if is_guard_error(&error) => {
            let synthetic_attempt = RejectedAttemptV1 {
                attempt_id: manifest.claims().attempt_id(),
                attempted_kind: manifest.claims().kind(),
                input_commitment: manifest.input_commitment()?,
                established_identities: identities,
                established_sealed_source: source_state,
                stage: ValidationStage::Admission,
                field: ValidationField::GuardWitness,
                reason: RejectionReason::InvalidField,
            };
            Ok(PreAccessValidationResult::CustodyEvidenceUnavailable(
                Box::new(sign_custody_failure(
                    &synthetic_attempt,
                    map_guard_error(error),
                    validator,
                )),
            ))
        }
        Err(error) => Err(error),
    }
}

fn checked_rejection_witness<'a>(
    attempt: &RejectedAttemptV1,
    evidence: &'a GuardWitnessEvidence,
) -> Result<&'a super::GuardWitnessV1, EvidenceError> {
    let witness = evidence.require_valid()?;
    witness.verify()?;
    let GuardSubjectV1::RejectedAttempt(subject) = witness.claims().subject() else {
        return Err(EvidenceError::WrongGuardSubject);
    };
    let checks = [
        (
            attempt.attempt_id == witness.claims().attempt_id(),
            RejectionJoinField::AttemptId,
        ),
        (
            attempt.attempted_kind == subject.attempted_kind(),
            RejectionJoinField::AttemptedKind,
        ),
        (
            attempt.input_commitment == subject.input_commitment(),
            RejectionJoinField::InputCommitment,
        ),
        (
            attempt.established_sealed_source == subject.established_sealed_source(),
            RejectionJoinField::EstablishedSealedSource,
        ),
    ];
    for (matches, field) in checks {
        if !matches {
            return Err(EvidenceError::RejectionGuardMismatch { field });
        }
    }
    Ok(witness)
}

fn sign_rejection(
    attempt: RejectedAttemptV1,
    witness: &super::GuardWitnessV1,
    validator: &ValidatorContext<'_>,
) -> PreAccessRejectionReceipt {
    let schema_id = schema_id(REJECTION_SCHEMA_DOMAIN);
    let key = SigningKey::from_bytes(validator.signing_key_bytes);
    let validator_key = VerifyingKeyBytes::from_bytes(key.verifying_key().to_bytes());
    let validator_id = ValidatorId::from_bytes(crypto::signer_id(validator_key));
    let witness_reference = GuardWitnessReference {
        content_id: witness.content_id(),
        digest: witness.digest(),
        signature: witness.signature(),
    };
    let validator_authentication = ValidatorAuthentication {
        implementation_id: validator.implementation_id,
        trusted_time: validator.trusted_time,
        validator_id,
        validator_key,
    };
    let canonical = canonical_rejection(
        schema_id,
        &attempt,
        witness_reference,
        validator_authentication,
    );
    let content_id =
        ArtifactContentId::from_bytes(*digest(REJECTION_CONTENT_ID_DOMAIN, &canonical).as_bytes());
    let digest = digest(REJECTION_DIGEST_DOMAIN, &canonical);
    let (_, signature) = crypto::sign(REJECTION_SIGNATURE_DOMAIN, &canonical, &key);
    PreAccessRejectionReceipt {
        schema_id,
        attempt,
        witness: witness_reference,
        content_id,
        digest,
        validator: validator_authentication,
        signature,
    }
}

fn sign_custody_failure(
    attempt: &RejectedAttemptV1,
    error: GuardEvidenceError,
    validator: &ValidatorContext<'_>,
) -> PreAccessCustodyFailureRecord {
    let mut record = PreAccessCustodyFailureRecord {
        schema_id: schema_id(CUSTODY_FAILURE_SCHEMA_DOMAIN),
        attempt_id: attempt.attempt_id,
        attempted_kind: attempt.attempted_kind,
        input_commitment: attempt.input_commitment,
        established_identities: attempt.established_identities.clone(),
        stage: attempt.stage,
        error,
        validator_implementation_id: validator.implementation_id,
        trusted_time: validator.trusted_time,
        validator_id: ValidatorId::from_bytes([0; 32]),
        validator_key: VerifyingKeyBytes::from_bytes([0; 32]),
        signature: EvidenceSignature::from_bytes([0; 64]),
    };
    let key = SigningKey::from_bytes(validator.signing_key_bytes);
    record.validator_key = VerifyingKeyBytes::from_bytes(key.verifying_key().to_bytes());
    record.validator_id = ValidatorId::from_bytes(crypto::signer_id(record.validator_key));
    let canonical = canonical_custody_failure(&record);
    let (_, signature) = crypto::sign(CUSTODY_FAILURE_SIGNATURE_DOMAIN, &canonical, &key);
    record.signature = signature;
    record
}

fn canonical_rejection(
    schema_id: SchemaId,
    attempt: &RejectedAttemptV1,
    witness: GuardWitnessReference,
    validator: ValidatorAuthentication,
) -> Vec<u8> {
    let mut encoder = Encoder::new(REJECTION_CANONICAL_DOMAIN);
    encoder.fixed(schema_id.as_bytes());
    encode_attempt(attempt, &mut encoder);
    encoder.fixed(witness.content_id.as_bytes());
    encoder.fixed(witness.digest.as_bytes());
    encoder.fixed(witness.signature.as_bytes());
    encoder.fixed(validator.implementation_id.as_bytes());
    encoder.u64(validator.trusted_time.unix_seconds());
    encoder.fixed(validator.validator_id.as_bytes());
    encoder.fixed(validator.validator_key.as_bytes());
    encoder.finish()
}

fn canonical_custody_failure(record: &PreAccessCustodyFailureRecord) -> Vec<u8> {
    let mut encoder = Encoder::new(CUSTODY_FAILURE_CANONICAL_DOMAIN);
    encoder.fixed(record.schema_id.as_bytes());
    encoder.fixed(record.attempt_id.as_bytes());
    encoder.byte(record.attempted_kind.tag());
    record.input_commitment.encode(&mut encoder);
    encode_identities(&record.established_identities, &mut encoder);
    encoder.byte(record.stage.tag());
    encode_guard_error(record.error, &mut encoder);
    encoder.fixed(record.validator_implementation_id.as_bytes());
    encoder.u64(record.trusted_time.unix_seconds());
    encoder.fixed(record.validator_id.as_bytes());
    encoder.fixed(record.validator_key.as_bytes());
    encoder.finish()
}

fn encode_attempt(attempt: &RejectedAttemptV1, encoder: &mut Encoder) {
    encoder.fixed(attempt.attempt_id.as_bytes());
    encoder.byte(attempt.attempted_kind.tag());
    attempt.input_commitment.encode(encoder);
    encode_identities(&attempt.established_identities, encoder);
    attempt.established_sealed_source.encode(encoder);
    encoder.byte(attempt.stage.tag());
    encoder.byte(attempt.field.tag());
    encoder.byte(attempt.reason.tag());
}

fn encode_identities(identities: &[EstablishedIdentity], encoder: &mut Encoder) {
    encoder.u32(u32::try_from(identities.len()).expect("bounded identity set length fits in u32"));
    for identity in identities {
        identity.encode(encoder);
    }
}

fn encode_guard_error(error: GuardEvidenceError, encoder: &mut Encoder) {
    match error {
        GuardEvidenceError::Missing => encoder.byte(1),
        GuardEvidenceError::Invalid => encoder.byte(2),
        GuardEvidenceError::WrongSubject => encoder.byte(3),
        GuardEvidenceError::RejectionMismatch { field } => {
            encoder.byte(4);
            encoder.byte(rejection_join_tag(field));
        }
        GuardEvidenceError::AdmissionMismatch { field } => {
            encoder.byte(5);
            encoder.byte(admission_join_tag(field));
        }
    }
}

fn rejection_join_tag(field: RejectionJoinField) -> u8 {
    match field {
        RejectionJoinField::AttemptId => 1,
        RejectionJoinField::AttemptedKind => 2,
        RejectionJoinField::InputCommitment => 3,
        RejectionJoinField::EstablishedSealedSource => 4,
    }
}

fn admission_join_tag(field: super::AdmissionJoinField) -> u8 {
    use super::AdmissionJoinField as Field;
    match field {
        Field::AttemptId => 1,
        Field::RunManifestContentId => 2,
        Field::RunManifestDigest => 3,
        Field::RunManifestSignature => 4,
        Field::SealedSourceId => 5,
        Field::ValidationWindowStart => 6,
        Field::ValidationWindowEnd => 7,
        Field::ValidationPrincipalSet => 8,
        Field::AnalysisPrincipalSet => 9,
        Field::OutcomeCapabilityIssuanceState => 10,
        Field::OutcomeAccessLedgerHead => 11,
        Field::OutcomeAccessLedgerTail => 12,
        Field::AnalysisJobLedgerHead => 13,
        Field::AnalysisJobLedgerTail => 14,
    }
}

fn map_guard_error(error: EvidenceError) -> GuardEvidenceError {
    match error {
        EvidenceError::MissingGuardWitness => GuardEvidenceError::Missing,
        EvidenceError::InvalidGuardWitness
        | EvidenceError::InvalidSignature
        | EvidenceError::ReferenceMismatch
        | EvidenceError::UntrustedGuardAuthority => GuardEvidenceError::Invalid,
        EvidenceError::WrongGuardSubject => GuardEvidenceError::WrongSubject,
        EvidenceError::RejectionGuardMismatch { field } => {
            GuardEvidenceError::RejectionMismatch { field }
        }
        EvidenceError::GuardWitnessMismatch { field } => {
            GuardEvidenceError::AdmissionMismatch { field }
        }
        _ => GuardEvidenceError::Invalid,
    }
}

fn is_guard_error(error: &EvidenceError) -> bool {
    matches!(
        error,
        EvidenceError::MissingGuardWitness
            | EvidenceError::InvalidGuardWitness
            | EvidenceError::UntrustedGuardAuthority
            | EvidenceError::WrongGuardSubject
            | EvidenceError::RejectionGuardMismatch { .. }
            | EvidenceError::GuardWitnessMismatch { .. }
    )
}

fn validate_sealed_source(
    state: SealedSourceState,
    identities: &[EstablishedIdentity],
) -> Result<(), EvidenceError> {
    let sealed_sources: Vec<_> = identities
        .iter()
        .filter(|identity| identity.kind() == EstablishedIdentityKind::SealedSource)
        .copied()
        .collect();
    let valid = match state {
        SealedSourceState::Absent => sealed_sources.is_empty(),
        SealedSourceState::Established(identity) => {
            sealed_sources
                == [EstablishedIdentity::new(
                    EstablishedIdentityKind::SealedSource,
                    EvidenceIdentity::from_bytes(*identity.as_bytes()),
                )]
        }
    };
    if !valid {
        return Err(EvidenceError::InvalidEstablishedSealedSource);
    }
    Ok(())
}
