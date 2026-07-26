use ed25519_dalek::SigningKey;

use super::{
    ArtifactContentId, AttemptId, AttemptedArtifactKind, CapabilityIssuanceState, CustodianId,
    EvidenceDigest, EvidenceError, EvidenceSignature, GuardImplementationId, InputCommitment,
    LedgerBoundary, PrincipalId, SealedSourceId, SealedSourceState, SignedRunManifestV1,
    ValidationWindow, VerifyingKeyBytes,
    canonical::{Encoder, digest},
    crypto,
    types::{canonical_principals, encode_principals, schema_id},
};

const WITNESS_SCHEMA_DOMAIN: &[u8] = b"nemosyne.evidence.guard-witness.schema.v1";
const WITNESS_CANONICAL_DOMAIN: &[u8] = b"nemosyne.evidence.guard-witness.canonical.v1";
const WITNESS_CONTENT_ID_DOMAIN: &[u8] = b"nemosyne.evidence.guard-witness.content-id.v1";
const WITNESS_DIGEST_DOMAIN: &[u8] = b"nemosyne.evidence.guard-witness.digest.v1";
const WITNESS_SIGNATURE_DOMAIN: &[u8] = b"nemosyne.evidence.guard-witness.signature.v1";

/// Independently supplied trust anchor for one guard implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GuardAuthorityV1 {
    custodian_id: CustodianId,
    custodian_key: VerifyingKeyBytes,
    guard_implementation_id: GuardImplementationId,
}

impl GuardAuthorityV1 {
    /// Derives the authority from a verifying key and exact guard
    /// implementation identity.
    #[must_use]
    pub fn new(
        custodian_key: VerifyingKeyBytes,
        guard_implementation_id: GuardImplementationId,
    ) -> Self {
        Self {
            custodian_id: CustodianId::from_bytes(crypto::signer_id(custodian_key)),
            custodian_key,
            guard_implementation_id,
        }
    }

    /// Derives the authority from signing-key bytes without retaining them.
    #[must_use]
    pub fn from_signing_key_bytes(
        signing_key_bytes: &[u8; 32],
        guard_implementation_id: GuardImplementationId,
    ) -> Self {
        let key = SigningKey::from_bytes(signing_key_bytes);
        Self::new(
            VerifyingKeyBytes::from_bytes(key.verifying_key().to_bytes()),
            guard_implementation_id,
        )
    }

    /// Returns the derived custodian identity.
    #[must_use]
    pub const fn custodian_id(self) -> CustodianId {
        self.custodian_id
    }

    /// Returns the trusted custodian verifying key.
    #[must_use]
    pub const fn custodian_key(self) -> VerifyingKeyBytes {
        self.custodian_key
    }

    /// Returns the trusted guard implementation identity.
    #[must_use]
    pub const fn guard_implementation_id(self) -> GuardImplementationId {
        self.guard_implementation_id
    }
}

/// Guard subject for one rejected pre-access attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RejectedGuardSubjectV1 {
    attempted_kind: AttemptedArtifactKind,
    input_commitment: InputCommitment,
    established_sealed_source: SealedSourceState,
}

impl RejectedGuardSubjectV1 {
    /// Constructs a rejection guard subject.
    #[must_use]
    pub const fn new(
        attempted_kind: AttemptedArtifactKind,
        input_commitment: InputCommitment,
        established_sealed_source: SealedSourceState,
    ) -> Self {
        Self {
            attempted_kind,
            input_commitment,
            established_sealed_source,
        }
    }

    /// Returns the attempted artifact kind.
    #[must_use]
    pub const fn attempted_kind(self) -> AttemptedArtifactKind {
        self.attempted_kind
    }

    /// Returns the non-retaining input commitment.
    #[must_use]
    pub const fn input_commitment(self) -> InputCommitment {
        self.input_commitment
    }

    /// Returns the explicit sealed-source state.
    #[must_use]
    pub const fn established_sealed_source(self) -> SealedSourceState {
        self.established_sealed_source
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.byte(self.attempted_kind.tag());
        self.input_commitment.encode(encoder);
        self.established_sealed_source.encode(encoder);
    }
}

/// Guard subject for one validated complete run manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedRunGuardSubjectV1 {
    run_manifest_content_id: ArtifactContentId,
    run_manifest_digest: EvidenceDigest,
    run_manifest_signature: EvidenceSignature,
    sealed_source_id: SealedSourceId,
}

impl ValidatedRunGuardSubjectV1 {
    /// Constructs a manifest-bound subject from exact signed-manifest
    /// references. Admission still verifies every field against the supplied
    /// manifest.
    #[must_use]
    pub const fn new(
        run_manifest_content_id: ArtifactContentId,
        run_manifest_digest: EvidenceDigest,
        run_manifest_signature: EvidenceSignature,
        sealed_source_id: SealedSourceId,
    ) -> Self {
        Self {
            run_manifest_content_id,
            run_manifest_digest,
            run_manifest_signature,
            sealed_source_id,
        }
    }

    /// Derives the exact guard subject from a signed run manifest.
    #[must_use]
    pub fn from_manifest(manifest: &SignedRunManifestV1) -> Self {
        Self::new(
            manifest.content_id(),
            manifest.digest(),
            manifest.signature(),
            manifest.claims().sealed_source_id(),
        )
    }

    /// Returns the run-manifest content identity.
    #[must_use]
    pub const fn run_manifest_content_id(self) -> ArtifactContentId {
        self.run_manifest_content_id
    }

    /// Returns the run-manifest digest.
    #[must_use]
    pub const fn run_manifest_digest(self) -> EvidenceDigest {
        self.run_manifest_digest
    }

    /// Returns the run-manifest signature.
    #[must_use]
    pub const fn run_manifest_signature(self) -> EvidenceSignature {
        self.run_manifest_signature
    }

    /// Returns the sealed outcome-source identity.
    #[must_use]
    pub const fn sealed_source_id(self) -> SealedSourceId {
        self.sealed_source_id
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.fixed(self.run_manifest_content_id.as_bytes());
        encoder.fixed(self.run_manifest_digest.as_bytes());
        encoder.fixed(self.run_manifest_signature.as_bytes());
        encoder.fixed(self.sealed_source_id.as_bytes());
    }
}

/// The two closed guard-witness subjects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardSubjectV1 {
    /// A commitment-bound rejected attempt.
    RejectedAttempt(RejectedGuardSubjectV1),
    /// A manifest-bound validated run.
    ValidatedRun(ValidatedRunGuardSubjectV1),
}

impl GuardSubjectV1 {
    fn encode(&self, encoder: &mut Encoder) {
        match self {
            Self::RejectedAttempt(subject) => {
                encoder.byte(1);
                subject.encode(encoder);
            }
            Self::ValidatedRun(subject) => {
                encoder.byte(2);
                subject.encode(encoder);
            }
        }
    }
}

/// Canonical claims authenticated by a pre-access guard witness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardWitnessClaimsV1 {
    attempt_id: AttemptId,
    subject: GuardSubjectV1,
    validation_window: ValidationWindow,
    validation_principals: Box<[PrincipalId]>,
    analysis_principals: Box<[PrincipalId]>,
    capability_state: CapabilityIssuanceState,
    outcome_access_ledger: LedgerBoundary,
    analysis_job_ledger: LedgerBoundary,
    guard_implementation_id: GuardImplementationId,
}

impl GuardWitnessClaimsV1 {
    /// Constructs canonical guard claims.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        attempt_id: AttemptId,
        subject: GuardSubjectV1,
        validation_window: ValidationWindow,
        validation_principals: Vec<PrincipalId>,
        analysis_principals: Vec<PrincipalId>,
        capability_state: CapabilityIssuanceState,
        outcome_access_ledger: LedgerBoundary,
        analysis_job_ledger: LedgerBoundary,
        guard_implementation_id: GuardImplementationId,
    ) -> Result<Self, EvidenceError> {
        Ok(Self {
            attempt_id,
            subject,
            validation_window,
            validation_principals: canonical_principals(validation_principals)?,
            analysis_principals: canonical_principals(analysis_principals)?,
            capability_state,
            outcome_access_ledger,
            analysis_job_ledger,
            guard_implementation_id,
        })
    }

    /// Returns the opaque attempt identity.
    #[must_use]
    pub const fn attempt_id(&self) -> AttemptId {
        self.attempt_id
    }

    /// Returns the closed guard subject.
    #[must_use]
    pub const fn subject(&self) -> &GuardSubjectV1 {
        &self.subject
    }

    /// Returns the trusted validation window.
    #[must_use]
    pub const fn validation_window(&self) -> ValidationWindow {
        self.validation_window
    }

    /// Returns validation principals in canonical order.
    #[must_use]
    pub fn validation_principals(&self) -> &[PrincipalId] {
        &self.validation_principals
    }

    /// Returns analysis principals in canonical order.
    #[must_use]
    pub fn analysis_principals(&self) -> &[PrincipalId] {
        &self.analysis_principals
    }

    /// Returns the outcome-capability issuance state.
    #[must_use]
    pub const fn capability_state(&self) -> CapabilityIssuanceState {
        self.capability_state
    }

    /// Returns outcome-access ledger boundaries.
    #[must_use]
    pub const fn outcome_access_ledger(&self) -> LedgerBoundary {
        self.outcome_access_ledger
    }

    /// Returns analysis-job ledger boundaries.
    #[must_use]
    pub const fn analysis_job_ledger(&self) -> LedgerBoundary {
        self.analysis_job_ledger
    }

    /// Returns the guard implementation identity.
    #[must_use]
    pub const fn guard_implementation_id(&self) -> GuardImplementationId {
        self.guard_implementation_id
    }

    fn encode(&self, encoder: &mut Encoder) {
        encoder.fixed(schema_id(WITNESS_SCHEMA_DOMAIN).as_bytes());
        encoder.fixed(self.attempt_id.as_bytes());
        self.subject.encode(encoder);
        self.validation_window.encode(encoder);
        encode_principals(&self.validation_principals, encoder);
        encode_principals(&self.analysis_principals, encoder);
        encoder.byte(self.capability_state.tag());
        self.outcome_access_ledger.encode(encoder);
        self.analysis_job_ledger.encode(encoder);
        encoder.fixed(self.guard_implementation_id.as_bytes());
    }
}

/// A canonical guard witness signed by its derived Ed25519 custodian identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardWitnessV1 {
    claims: GuardWitnessClaimsV1,
    custodian_id: CustodianId,
    content_id: ArtifactContentId,
    digest: EvidenceDigest,
    custodian_key: VerifyingKeyBytes,
    signature: EvidenceSignature,
}

impl GuardWitnessV1 {
    /// Signs canonical guard claims.
    #[must_use]
    pub fn sign(claims: GuardWitnessClaimsV1, signing_key_bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(signing_key_bytes);
        let custodian_key = VerifyingKeyBytes::from_bytes(signing_key.verifying_key().to_bytes());
        let custodian_id = CustodianId::from_bytes(crypto::signer_id(custodian_key));
        let canonical = canonical_witness(&claims, custodian_id);
        let content_id = ArtifactContentId::from_bytes(
            *digest(WITNESS_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let digest = digest(WITNESS_DIGEST_DOMAIN, &canonical);
        let (_, signature) = crypto::sign(WITNESS_SIGNATURE_DOMAIN, &canonical, &signing_key);
        Self {
            claims,
            custodian_id,
            content_id,
            digest,
            custodian_key,
            signature,
        }
    }

    /// Returns canonical witness claims.
    #[must_use]
    pub const fn claims(&self) -> &GuardWitnessClaimsV1 {
        &self.claims
    }

    /// Returns the custodian identity derived from the verifying key.
    #[must_use]
    pub const fn custodian_id(&self) -> CustodianId {
        self.custodian_id
    }

    /// Returns the witness content identity.
    #[must_use]
    pub const fn content_id(&self) -> ArtifactContentId {
        self.content_id
    }

    /// Returns the witness digest.
    #[must_use]
    pub const fn digest(&self) -> EvidenceDigest {
        self.digest
    }

    /// Returns the custodian verifying key.
    #[must_use]
    pub const fn custodian_key(&self) -> VerifyingKeyBytes {
        self.custodian_key
    }

    /// Returns the witness signature.
    #[must_use]
    pub const fn signature(&self) -> EvidenceSignature {
        self.signature
    }

    pub(super) fn verify(&self) -> Result<(), EvidenceError> {
        if self.custodian_id != CustodianId::from_bytes(crypto::signer_id(self.custodian_key)) {
            return Err(EvidenceError::ReferenceMismatch);
        }
        let canonical = canonical_witness(&self.claims, self.custodian_id);
        let expected_content_id = ArtifactContentId::from_bytes(
            *digest(WITNESS_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let expected_digest = digest(WITNESS_DIGEST_DOMAIN, &canonical);
        if self.content_id != expected_content_id || self.digest != expected_digest {
            return Err(EvidenceError::ReferenceMismatch);
        }
        crypto::verify(
            WITNESS_SIGNATURE_DOMAIN,
            &canonical,
            self.custodian_key,
            self.signature,
        )
    }

    pub(super) fn encode_signed(&self, encoder: &mut Encoder) {
        self.claims.encode(encoder);
        encoder.fixed(self.custodian_id.as_bytes());
        encoder.fixed(self.content_id.as_bytes());
        encoder.fixed(self.digest.as_bytes());
        encoder.fixed(self.custodian_key.as_bytes());
        encoder.fixed(self.signature.as_bytes());
    }

    pub(super) fn verify_with(&self, authority: GuardAuthorityV1) -> Result<(), EvidenceError> {
        self.verify()?;
        if self.custodian_id != authority.custodian_id
            || self.custodian_key != authority.custodian_key
            || self.claims.guard_implementation_id != authority.guard_implementation_id
        {
            return Err(EvidenceError::UntrustedGuardAuthority);
        }
        Ok(())
    }
}

/// Checked status of untrusted witness input.
///
/// Invalid raw witness material is discarded before this value is returned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardWitnessEvidence(GuardWitnessEvidenceState);

#[derive(Clone, Debug, Eq, PartialEq)]
enum GuardWitnessEvidenceState {
    Missing,
    Invalid,
    Valid(Box<GuardWitnessV1>),
}

impl GuardWitnessEvidence {
    /// Constructs explicit missing-witness evidence.
    #[must_use]
    pub const fn missing() -> Self {
        Self(GuardWitnessEvidenceState::Missing)
    }

    /// Authenticates a witness against an independently supplied authority.
    ///
    /// Invalid or untrusted witness material is discarded.
    #[must_use]
    pub fn authenticate(witness: GuardWitnessV1, authority: GuardAuthorityV1) -> Self {
        if witness.verify_with(authority).is_ok() {
            Self(GuardWitnessEvidenceState::Valid(Box::new(witness)))
        } else {
            Self(GuardWitnessEvidenceState::Invalid)
        }
    }

    /// Checks untrusted signed witness parts and discards them on any failure.
    #[must_use]
    pub fn from_signed_parts(
        claims: GuardWitnessClaimsV1,
        claimed_content_id: ArtifactContentId,
        claimed_digest: EvidenceDigest,
        claimed_custodian_id: CustodianId,
        custodian_key: VerifyingKeyBytes,
        signature: EvidenceSignature,
        authority: GuardAuthorityV1,
    ) -> Self {
        let witness = GuardWitnessV1 {
            claims,
            custodian_id: claimed_custodian_id,
            content_id: claimed_content_id,
            digest: claimed_digest,
            custodian_key,
            signature,
        };
        Self::authenticate(witness, authority)
    }

    pub(super) fn require_valid(&self) -> Result<&GuardWitnessV1, EvidenceError> {
        match &self.0 {
            GuardWitnessEvidenceState::Missing => Err(EvidenceError::MissingGuardWitness),
            GuardWitnessEvidenceState::Invalid => Err(EvidenceError::InvalidGuardWitness),
            GuardWitnessEvidenceState::Valid(witness) => Ok(witness),
        }
    }
}

fn canonical_witness(claims: &GuardWitnessClaimsV1, custodian_id: CustodianId) -> Vec<u8> {
    let mut encoder = Encoder::new(WITNESS_CANONICAL_DOMAIN);
    claims.encode(&mut encoder);
    encoder.fixed(custodian_id.as_bytes());
    encoder.finish()
}
