use ed25519_dalek::SigningKey;

use super::{
    AdmissionJoinField, ArtifactContentId, AttemptId, AttemptedArtifactKind,
    CapabilityIssuanceState, EvidenceDigest, EvidenceError, EvidenceSignature, GuardSubjectV1,
    GuardWitnessEvidence, GuardWitnessV1, LedgerBoundary, PrincipalId, SealedSourceId,
    ValidationWindow, VerifyingKeyBytes,
    canonical::{Encoder, digest},
    crypto,
    types::{canonical_principals, encode_principals, schema_id},
};

/// Maximum opaque run-manifest payload length accepted by the offline
/// evidence envelope.
pub const MAX_RUN_MANIFEST_PAYLOAD_BYTES: usize = 1_048_576;
const MAX_SIGNED_MANIFEST_CANONICAL_BYTES: usize = MAX_RUN_MANIFEST_PAYLOAD_BYTES + 20_000;

const MANIFEST_SCHEMA_DOMAIN: &[u8] = b"nemosyne.evidence.run-manifest.schema.v1";
const MANIFEST_CANONICAL_DOMAIN: &[u8] = b"nemosyne.evidence.run-manifest.canonical.v1";
const MANIFEST_CONTENT_ID_DOMAIN: &[u8] = b"nemosyne.evidence.run-manifest.content-id.v1";
const MANIFEST_DIGEST_DOMAIN: &[u8] = b"nemosyne.evidence.run-manifest.digest.v1";
const MANIFEST_SIGNATURE_DOMAIN: &[u8] = b"nemosyne.evidence.run-manifest.signature.v1";

/// Validated, canonical fields shared by a G1 or G9 run manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunManifestClaimsV1 {
    attempt_id: AttemptId,
    kind: AttemptedArtifactKind,
    sealed_source_id: SealedSourceId,
    validation_window: ValidationWindow,
    validation_principals: Box<[PrincipalId]>,
    analysis_principals: Box<[PrincipalId]>,
    capability_state: CapabilityIssuanceState,
    outcome_access_ledger: LedgerBoundary,
    analysis_job_ledger: LedgerBoundary,
}

impl RunManifestClaimsV1 {
    /// Constructs canonical claims for a complete G1 or G9 run manifest.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        attempt_id: AttemptId,
        kind: AttemptedArtifactKind,
        sealed_source_id: SealedSourceId,
        validation_window: ValidationWindow,
        validation_principals: Vec<PrincipalId>,
        analysis_principals: Vec<PrincipalId>,
        capability_state: CapabilityIssuanceState,
        outcome_access_ledger: LedgerBoundary,
        analysis_job_ledger: LedgerBoundary,
    ) -> Result<Self, EvidenceError> {
        if !kind.is_run_manifest() {
            return Err(EvidenceError::ExpectedRunManifest);
        }
        Ok(Self {
            attempt_id,
            kind,
            sealed_source_id,
            validation_window,
            validation_principals: canonical_principals(validation_principals)?,
            analysis_principals: canonical_principals(analysis_principals)?,
            capability_state,
            outcome_access_ledger,
            analysis_job_ledger,
        })
    }

    /// Returns the opaque attempt identity.
    #[must_use]
    pub const fn attempt_id(&self) -> AttemptId {
        self.attempt_id
    }

    /// Returns the run-manifest kind.
    #[must_use]
    pub const fn kind(&self) -> AttemptedArtifactKind {
        self.kind
    }

    /// Returns the sealed outcome-source identity.
    #[must_use]
    pub const fn sealed_source_id(&self) -> SealedSourceId {
        self.sealed_source_id
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

    fn encode(&self, encoder: &mut Encoder) {
        encoder.fixed(schema_id(MANIFEST_SCHEMA_DOMAIN).as_bytes());
        encoder.fixed(self.attempt_id.as_bytes());
        encoder.byte(self.kind.tag());
        encoder.fixed(self.sealed_source_id.as_bytes());
        self.validation_window.encode(encoder);
        encode_principals(&self.validation_principals, encoder);
        encode_principals(&self.analysis_principals, encoder);
        encoder.byte(self.capability_state.tag());
        self.outcome_access_ledger.encode(encoder);
        self.analysis_job_ledger.encode(encoder);
    }
}

/// A canonical, content-identified, signed G1 or G9 run manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedRunManifestV1 {
    claims: RunManifestClaimsV1,
    payload: Box<[u8]>,
    content_id: ArtifactContentId,
    digest: EvidenceDigest,
    verifying_key: VerifyingKeyBytes,
    signature: EvidenceSignature,
}

impl SignedRunManifestV1 {
    /// Signs a complete bounded run-manifest payload and its admission claims.
    pub fn sign(
        claims: RunManifestClaimsV1,
        payload: &[u8],
        signing_key_bytes: &[u8; 32],
    ) -> Result<Self, EvidenceError> {
        if payload.is_empty() {
            return Err(EvidenceError::EmptyPayload);
        }
        if payload.len() > MAX_RUN_MANIFEST_PAYLOAD_BYTES {
            return Err(EvidenceError::PayloadTooLarge {
                actual: payload.len(),
                maximum: MAX_RUN_MANIFEST_PAYLOAD_BYTES,
            });
        }
        let canonical = canonical_manifest(&claims, payload)?;
        let content_id = ArtifactContentId::from_bytes(
            *digest(MANIFEST_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let digest = digest(MANIFEST_DIGEST_DOMAIN, &canonical);
        let signing_key = SigningKey::from_bytes(signing_key_bytes);
        let (verifying_key, signature) =
            crypto::sign(MANIFEST_SIGNATURE_DOMAIN, &canonical, &signing_key);
        Ok(Self {
            claims,
            payload: payload.into(),
            content_id,
            digest,
            verifying_key,
            signature,
        })
    }

    /// Returns the manifest claims.
    #[must_use]
    pub const fn claims(&self) -> &RunManifestClaimsV1 {
        &self.claims
    }

    /// Returns the exact reconstructible manifest payload.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Returns the domain-separated manifest content identity.
    #[must_use]
    pub const fn content_id(&self) -> ArtifactContentId {
        self.content_id
    }

    /// Returns the domain-separated manifest digest.
    #[must_use]
    pub const fn digest(&self) -> EvidenceDigest {
        self.digest
    }

    /// Returns the manifest verifying key.
    #[must_use]
    pub const fn verifying_key(&self) -> VerifyingKeyBytes {
        self.verifying_key
    }

    /// Returns the manifest signature.
    #[must_use]
    pub const fn signature(&self) -> EvidenceSignature {
        self.signature
    }

    pub(super) fn verify(&self) -> Result<(), EvidenceError> {
        let canonical = canonical_manifest(&self.claims, &self.payload)?;
        let expected_content_id = ArtifactContentId::from_bytes(
            *digest(MANIFEST_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let expected_digest = digest(MANIFEST_DIGEST_DOMAIN, &canonical);
        if self.content_id != expected_content_id || self.digest != expected_digest {
            return Err(EvidenceError::ReferenceMismatch);
        }
        crypto::verify(
            MANIFEST_SIGNATURE_DOMAIN,
            &canonical,
            self.verifying_key,
            self.signature,
        )
    }

    pub(super) fn input_commitment(&self) -> Result<super::InputCommitment, EvidenceError> {
        let mut encoder = Encoder::new(b"nemosyne.evidence.signed-run-manifest.v1");
        self.encode_signed(&mut encoder)?;
        super::InputCommitment::complete(&encoder.finish())
    }

    pub(super) fn encode_signed(&self, encoder: &mut Encoder) -> Result<(), EvidenceError> {
        let canonical = canonical_manifest(&self.claims, &self.payload)?;
        encoder.bounded_bytes(&canonical, MAX_SIGNED_MANIFEST_CANONICAL_BYTES)?;
        encoder.fixed(self.content_id.as_bytes());
        encoder.fixed(self.digest.as_bytes());
        encoder.fixed(self.verifying_key.as_bytes());
        encoder.fixed(self.signature.as_bytes());
        Ok(())
    }
}

/// A sealed admission value that exists only after the exact run manifest and
/// guard witness pass the complete fixed-precedence join.
#[derive(Debug, Eq, PartialEq)]
pub struct ValidForOutcomeAccess {
    manifest: SignedRunManifestV1,
    witness: GuardWitnessV1,
}

impl ValidForOutcomeAccess {
    /// Validates and joins one complete signed run manifest with one guard
    /// witness.
    pub fn new(
        manifest: &SignedRunManifestV1,
        witness_evidence: &GuardWitnessEvidence,
    ) -> Result<Self, EvidenceError> {
        manifest.verify()?;
        let witness = witness_evidence.require_valid()?;
        witness.verify()?;
        let GuardSubjectV1::ValidatedRun(subject) = witness.claims().subject() else {
            return Err(EvidenceError::WrongGuardSubject);
        };

        compare_admission(manifest, witness, subject)?;

        Ok(Self {
            manifest: manifest.clone(),
            witness: witness.clone(),
        })
    }

    /// Returns the complete admitted signed run manifest.
    #[must_use]
    pub const fn manifest(&self) -> &SignedRunManifestV1 {
        &self.manifest
    }

    /// Returns the complete authenticated guard witness.
    #[must_use]
    pub const fn witness(&self) -> &GuardWitnessV1 {
        &self.witness
    }

    /// Returns the admitted run-manifest content identity.
    #[must_use]
    pub const fn manifest_content_id(&self) -> ArtifactContentId {
        self.manifest.content_id
    }

    /// Returns the admitted run-manifest digest.
    #[must_use]
    pub const fn manifest_digest(&self) -> EvidenceDigest {
        self.manifest.digest
    }

    /// Returns the admitted run-manifest signature.
    #[must_use]
    pub const fn manifest_signature(&self) -> EvidenceSignature {
        self.manifest.signature
    }

    /// Returns the admitted run-manifest verifying key.
    #[must_use]
    pub const fn manifest_verifying_key(&self) -> VerifyingKeyBytes {
        self.manifest.verifying_key
    }

    /// Returns the joined guard-witness content identity.
    #[must_use]
    pub const fn witness_content_id(&self) -> ArtifactContentId {
        self.witness.content_id()
    }

    /// Returns the joined guard-witness digest.
    #[must_use]
    pub const fn witness_digest(&self) -> EvidenceDigest {
        self.witness.digest()
    }

    /// Returns the joined guard-witness signature.
    #[must_use]
    pub const fn witness_signature(&self) -> EvidenceSignature {
        self.witness.signature()
    }

    /// Returns the joined guard-witness custodian key.
    #[must_use]
    pub const fn witness_custodian_key(&self) -> VerifyingKeyBytes {
        self.witness.custodian_key()
    }

    pub(super) fn into_parts(self) -> (SignedRunManifestV1, GuardWitnessV1) {
        (self.manifest, self.witness)
    }
}

fn canonical_manifest(
    claims: &RunManifestClaimsV1,
    payload: &[u8],
) -> Result<Vec<u8>, EvidenceError> {
    let mut encoder = Encoder::new(MANIFEST_CANONICAL_DOMAIN);
    claims.encode(&mut encoder);
    encoder.bounded_bytes(payload, MAX_RUN_MANIFEST_PAYLOAD_BYTES)?;
    Ok(encoder.finish())
}

pub(super) fn compare_admission(
    manifest: &SignedRunManifestV1,
    witness: &GuardWitnessV1,
    subject: &super::ValidatedRunGuardSubjectV1,
) -> Result<(), EvidenceError> {
    let claims = manifest.claims();
    let witness_claims = witness.claims();
    let checks = [
        (
            claims.attempt_id() == witness_claims.attempt_id(),
            AdmissionJoinField::AttemptId,
        ),
        (
            manifest.content_id() == subject.run_manifest_content_id(),
            AdmissionJoinField::RunManifestContentId,
        ),
        (
            manifest.digest() == subject.run_manifest_digest(),
            AdmissionJoinField::RunManifestDigest,
        ),
        (
            manifest.signature() == subject.run_manifest_signature(),
            AdmissionJoinField::RunManifestSignature,
        ),
        (
            claims.sealed_source_id() == subject.sealed_source_id(),
            AdmissionJoinField::SealedSourceId,
        ),
        (
            claims.validation_window().start() == witness_claims.validation_window().start(),
            AdmissionJoinField::ValidationWindowStart,
        ),
        (
            claims.validation_window().end() == witness_claims.validation_window().end(),
            AdmissionJoinField::ValidationWindowEnd,
        ),
        (
            claims.validation_principals() == witness_claims.validation_principals(),
            AdmissionJoinField::ValidationPrincipalSet,
        ),
        (
            claims.analysis_principals() == witness_claims.analysis_principals(),
            AdmissionJoinField::AnalysisPrincipalSet,
        ),
        (
            claims.capability_state() == witness_claims.capability_state(),
            AdmissionJoinField::OutcomeCapabilityIssuanceState,
        ),
        (
            claims.outcome_access_ledger().head() == witness_claims.outcome_access_ledger().head(),
            AdmissionJoinField::OutcomeAccessLedgerHead,
        ),
        (
            claims.outcome_access_ledger().tail() == witness_claims.outcome_access_ledger().tail(),
            AdmissionJoinField::OutcomeAccessLedgerTail,
        ),
        (
            claims.analysis_job_ledger().head() == witness_claims.analysis_job_ledger().head(),
            AdmissionJoinField::AnalysisJobLedgerHead,
        ),
        (
            claims.analysis_job_ledger().tail() == witness_claims.analysis_job_ledger().tail(),
            AdmissionJoinField::AnalysisJobLedgerTail,
        ),
    ];
    for (matches, field) in checks {
        if !matches {
            return Err(EvidenceError::GuardWitnessMismatch { field });
        }
    }
    Ok(())
}
