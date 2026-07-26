use ed25519_dalek::SigningKey;

use super::{
    ArtifactContentId, EvaluatorId, EvidenceDigest, EvidenceDisposition, EvidenceError,
    EvidenceSignature, GuardAuthorityV1, GuardSubjectV1, GuardWitnessV1, SchemaId,
    SignedRunManifestV1, ValidForOutcomeAccess, VerifyingKeyBytes,
    canonical::{Encoder, digest},
    crypto,
    manifest::compare_admission,
};

/// Maximum typed post-admission evidence payload length.
pub const MAX_EXPERIMENT_RECEIPT_PAYLOAD_BYTES: usize = 4_194_304;

const RECEIPT_CANONICAL_DOMAIN: &[u8] = b"nemosyne.evidence.experiment-receipt.canonical.v1";
const RECEIPT_CONTENT_ID_DOMAIN: &[u8] = b"nemosyne.evidence.experiment-receipt.content-id.v1";
const RECEIPT_DIGEST_DOMAIN: &[u8] = b"nemosyne.evidence.experiment-receipt.digest.v1";
const RECEIPT_SIGNATURE_DOMAIN: &[u8] = b"nemosyne.evidence.experiment-receipt.signature.v1";

/// A versioned, reconstructible typed payload produced only after admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExperimentReceiptPayloadV1 {
    schema_id: SchemaId,
    disposition: EvidenceDisposition,
    bytes: Box<[u8]>,
}

impl ExperimentReceiptPayloadV1 {
    /// Constructs a bounded nonempty payload under its independently versioned
    /// schema identity.
    pub fn new(
        schema_id: SchemaId,
        disposition: EvidenceDisposition,
        bytes: &[u8],
    ) -> Result<Self, EvidenceError> {
        if bytes.is_empty() {
            return Err(EvidenceError::EmptyPayload);
        }
        if bytes.len() > MAX_EXPERIMENT_RECEIPT_PAYLOAD_BYTES {
            return Err(EvidenceError::PayloadTooLarge {
                actual: bytes.len(),
                maximum: MAX_EXPERIMENT_RECEIPT_PAYLOAD_BYTES,
            });
        }
        Ok(Self {
            schema_id,
            disposition,
            bytes: bytes.into(),
        })
    }

    /// Returns the typed payload schema.
    #[must_use]
    pub const fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    /// Returns the frozen terminal disposition.
    #[must_use]
    pub const fn disposition(&self) -> EvidenceDisposition {
        self.disposition
    }

    /// Returns exact payload bytes for reconstruction.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// A signed, content-identified experiment receipt that can be created only
/// by consuming a successful pre-access admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidExperimentReceiptV1 {
    manifest: SignedRunManifestV1,
    witness: GuardWitnessV1,
    payload: ExperimentReceiptPayloadV1,
    content_id: ArtifactContentId,
    digest: EvidenceDigest,
    evaluator_id: EvaluatorId,
    evaluator_key: VerifyingKeyBytes,
    signature: EvidenceSignature,
}

impl ValidExperimentReceiptV1 {
    /// Consumes a successful admission and signs its exact typed evidence
    /// payload.
    pub fn sign(
        admission: ValidForOutcomeAccess,
        payload: ExperimentReceiptPayloadV1,
        signing_key_bytes: &[u8; 32],
    ) -> Result<Self, EvidenceError> {
        let canonical = canonical_receipt(&admission, &payload)?;
        let content_id = ArtifactContentId::from_bytes(
            *digest(RECEIPT_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let digest = digest(RECEIPT_DIGEST_DOMAIN, &canonical);
        let signing_key = SigningKey::from_bytes(signing_key_bytes);
        let (evaluator_key, signature) =
            crypto::sign(RECEIPT_SIGNATURE_DOMAIN, &canonical, &signing_key);
        let evaluator_id = EvaluatorId::from_bytes(crypto::signer_id(evaluator_key));
        let (manifest, witness) = admission.into_parts();
        Ok(Self {
            manifest,
            witness,
            payload,
            content_id,
            digest,
            evaluator_id,
            evaluator_key,
            signature,
        })
    }

    /// Returns the complete admitted signed run manifest.
    #[must_use]
    pub const fn manifest(&self) -> &SignedRunManifestV1 {
        &self.manifest
    }

    /// Returns the complete authenticated pre-access guard witness.
    #[must_use]
    pub const fn witness(&self) -> &GuardWitnessV1 {
        &self.witness
    }

    /// Returns the exact admitted run-manifest content identity.
    #[must_use]
    pub const fn manifest_content_id(&self) -> ArtifactContentId {
        self.manifest.content_id()
    }

    /// Returns the admitted run-manifest digest.
    #[must_use]
    pub const fn manifest_digest(&self) -> EvidenceDigest {
        self.manifest.digest()
    }

    /// Returns the admitted run-manifest signature.
    #[must_use]
    pub const fn manifest_signature(&self) -> EvidenceSignature {
        self.manifest.signature()
    }

    /// Returns the admitted run-manifest verifying key.
    #[must_use]
    pub const fn manifest_verifying_key(&self) -> VerifyingKeyBytes {
        self.manifest.verifying_key()
    }

    /// Returns the exact joined guard-witness content identity.
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

    /// Returns the versioned reconstructible evidence payload.
    #[must_use]
    pub const fn payload(&self) -> &ExperimentReceiptPayloadV1 {
        &self.payload
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

    /// Returns the evaluator identity derived from its verifying key.
    #[must_use]
    pub const fn evaluator_id(&self) -> EvaluatorId {
        self.evaluator_id
    }

    /// Returns the evaluator verifying key.
    #[must_use]
    pub const fn evaluator_key(&self) -> VerifyingKeyBytes {
        self.evaluator_key
    }

    /// Returns the evaluator signature.
    #[must_use]
    pub const fn signature(&self) -> EvidenceSignature {
        self.signature
    }

    /// Revalidates the complete manifest/witness admission against an
    /// independently supplied guard authority and verifies the receipt
    /// signature.
    pub fn verify(&self, authority: GuardAuthorityV1) -> Result<(), EvidenceError> {
        self.manifest.verify()?;
        self.witness.verify_with(authority)?;
        let GuardSubjectV1::ValidatedRun(subject) = self.witness.claims().subject() else {
            return Err(EvidenceError::WrongGuardSubject);
        };
        compare_admission(&self.manifest, &self.witness, subject)?;
        let canonical = canonical_receipt_fields(&self.manifest, &self.witness, &self.payload)?;
        let content_id = ArtifactContentId::from_bytes(
            *digest(RECEIPT_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let digest = digest(RECEIPT_DIGEST_DOMAIN, &canonical);
        if content_id != self.content_id || digest != self.digest {
            return Err(EvidenceError::ReferenceMismatch);
        }
        if self.evaluator_id != EvaluatorId::from_bytes(crypto::signer_id(self.evaluator_key)) {
            return Err(EvidenceError::ReferenceMismatch);
        }
        crypto::verify(
            RECEIPT_SIGNATURE_DOMAIN,
            &canonical,
            self.evaluator_key,
            self.signature,
        )
    }
}

fn canonical_receipt(
    admission: &ValidForOutcomeAccess,
    payload: &ExperimentReceiptPayloadV1,
) -> Result<Vec<u8>, EvidenceError> {
    canonical_receipt_fields(admission.manifest(), admission.witness(), payload)
}

fn canonical_receipt_fields(
    manifest: &SignedRunManifestV1,
    witness: &GuardWitnessV1,
    payload: &ExperimentReceiptPayloadV1,
) -> Result<Vec<u8>, EvidenceError> {
    let mut encoder = Encoder::new(RECEIPT_CANONICAL_DOMAIN);
    manifest.encode_signed(&mut encoder)?;
    witness.encode_signed(&mut encoder);
    encoder.fixed(payload.schema_id.as_bytes());
    encoder.byte(payload.disposition.tag());
    encoder.bounded_bytes(&payload.bytes, MAX_EXPERIMENT_RECEIPT_PAYLOAD_BYTES)?;
    Ok(encoder.finish())
}
