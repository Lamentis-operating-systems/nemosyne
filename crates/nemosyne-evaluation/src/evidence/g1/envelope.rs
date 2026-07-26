use ed25519_dalek::SigningKey;

use crate::evidence::{
    ArtifactContentId, EvidenceDigest, EvidenceSignature, VerifyingKeyBytes,
    canonical::{Encoder, digest},
    crypto,
    manifest::MAX_RUN_MANIFEST_PAYLOAD_BYTES,
};

use super::{
    G1ArtifactBindingV1, G1AttentionMatchingV1, G1ConditionArtifactV1, G1CriticalFailureBoundV1,
    G1EnvelopeError, G1PopulationV1, G1ThresholdV1,
    model::{
        canonical_artifacts, canonical_conditions, canonical_critical_failures,
        canonical_thresholds,
    },
};

const ENVELOPE_CANONICAL_DOMAIN: &[u8] = b"nemosyne.evidence.g1-envelope.canonical.v1";
const ENVELOPE_CONTENT_ID_DOMAIN: &[u8] = b"nemosyne.evidence.g1-envelope.content-id.v1";
const ENVELOPE_DIGEST_DOMAIN: &[u8] = b"nemosyne.evidence.g1-envelope.digest.v1";
const ENVELOPE_SIGNATURE_DOMAIN: &[u8] = b"nemosyne.evidence.g1-envelope.signature.v1";
const MAX_G1_ENVELOPE_CANONICAL_BYTES: usize = MAX_RUN_MANIFEST_PAYLOAD_BYTES - 4_096;

/// Complete validated pre-outcome G1 design.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G1DesignV1 {
    conditions: Box<[G1ConditionArtifactV1]>,
    matching: G1AttentionMatchingV1,
    population: G1PopulationV1,
    thresholds: Box<[G1ThresholdV1]>,
    critical_failures: Box<[G1CriticalFailureBoundV1]>,
    artifacts: Box<[G1ArtifactBindingV1]>,
}

impl G1DesignV1 {
    /// Constructs the complete canonical design without inspecting outcomes.
    pub fn new(
        conditions: Vec<G1ConditionArtifactV1>,
        matching: G1AttentionMatchingV1,
        population: G1PopulationV1,
        thresholds: Vec<G1ThresholdV1>,
        critical_failures: Vec<G1CriticalFailureBoundV1>,
        artifacts: Vec<G1ArtifactBindingV1>,
    ) -> Result<Self, G1EnvelopeError> {
        Ok(Self {
            conditions: canonical_conditions(conditions)?,
            matching,
            population,
            thresholds: canonical_thresholds(thresholds)?,
            critical_failures: canonical_critical_failures(critical_failures)?,
            artifacts: canonical_artifacts(artifacts)?,
        })
    }

    /// Returns all seven conditions in closed condition order.
    #[must_use]
    pub fn conditions(&self) -> &[G1ConditionArtifactV1] {
        &self.conditions
    }

    /// Returns the shared attention matching contract.
    #[must_use]
    pub const fn matching(&self) -> G1AttentionMatchingV1 {
        self.matching
    }

    /// Returns the complete prospective population design.
    #[must_use]
    pub const fn population(&self) -> &G1PopulationV1 {
        &self.population
    }

    /// Returns every proof-owned threshold in canonical key order.
    #[must_use]
    pub fn thresholds(&self) -> &[G1ThresholdV1] {
        &self.thresholds
    }

    /// Returns all closed critical-failure bounds in canonical class order.
    #[must_use]
    pub fn critical_failures(&self) -> &[G1CriticalFailureBoundV1] {
        &self.critical_failures
    }

    /// Returns all required design artifacts in canonical kind order.
    #[must_use]
    pub fn artifacts(&self) -> &[G1ArtifactBindingV1] {
        &self.artifacts
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoder = Encoder::new(ENVELOPE_CANONICAL_DOMAIN);
        encoder.u32(1);
        encoder
            .u32(u32::try_from(self.conditions.len()).expect("fixed condition count fits in u32"));
        for condition in &self.conditions {
            condition.encode(&mut encoder);
        }
        self.matching.encode(&mut encoder);
        self.population.encode(&mut encoder);
        encoder
            .u32(u32::try_from(self.thresholds.len()).expect("fixed threshold count fits in u32"));
        for threshold in &self.thresholds {
            threshold.encode(&mut encoder);
        }
        encoder.u32(
            u32::try_from(self.critical_failures.len())
                .expect("fixed critical-failure count fits in u32"),
        );
        for bound in &self.critical_failures {
            bound.encode(&mut encoder);
        }
        encoder.u32(u32::try_from(self.artifacts.len()).expect("fixed artifact count fits in u32"));
        for artifact in &self.artifacts {
            artifact.encode(&mut encoder);
        }
        encoder.finish()
    }
}

/// Signed, content-identified G1 design envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedG1EvaluationEnvelopeV1 {
    design: G1DesignV1,
    content_id: ArtifactContentId,
    digest: EvidenceDigest,
    verifying_key: VerifyingKeyBytes,
    signature: EvidenceSignature,
}

impl SignedG1EvaluationEnvelopeV1 {
    /// Signs a complete validated G1 design.
    pub fn sign(design: G1DesignV1, signing_key_bytes: &[u8; 32]) -> Result<Self, G1EnvelopeError> {
        let canonical = design.canonical_bytes();
        if canonical.len() > MAX_G1_ENVELOPE_CANONICAL_BYTES {
            return Err(crate::evidence::EvidenceError::PayloadTooLarge {
                actual: canonical.len(),
                maximum: MAX_G1_ENVELOPE_CANONICAL_BYTES,
            }
            .into());
        }
        let content_id = ArtifactContentId::from_bytes(
            *digest(ENVELOPE_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        let digest = digest(ENVELOPE_DIGEST_DOMAIN, &canonical);
        let signing_key = SigningKey::from_bytes(signing_key_bytes);
        let (verifying_key, signature) =
            crypto::sign(ENVELOPE_SIGNATURE_DOMAIN, &canonical, &signing_key);
        Ok(Self {
            design,
            content_id,
            digest,
            verifying_key,
            signature,
        })
    }

    /// Verifies content identity, digest, and signature.
    pub fn verify(&self) -> Result<(), G1EnvelopeError> {
        let canonical = self.design.canonical_bytes();
        let expected_content_id = ArtifactContentId::from_bytes(
            *digest(ENVELOPE_CONTENT_ID_DOMAIN, &canonical).as_bytes(),
        );
        if self.content_id != expected_content_id
            || self.digest != digest(ENVELOPE_DIGEST_DOMAIN, &canonical)
        {
            return Err(crate::evidence::EvidenceError::ReferenceMismatch.into());
        }
        crypto::verify(
            ENVELOPE_SIGNATURE_DOMAIN,
            &canonical,
            self.verifying_key,
            self.signature,
        )?;
        Ok(())
    }

    /// Returns the frozen design.
    #[must_use]
    pub const fn design(&self) -> &G1DesignV1 {
        &self.design
    }

    /// Returns the envelope content identity.
    #[must_use]
    pub const fn content_id(&self) -> ArtifactContentId {
        self.content_id
    }

    /// Returns the envelope digest.
    #[must_use]
    pub const fn digest(&self) -> EvidenceDigest {
        self.digest
    }

    /// Returns the signing-key identity.
    #[must_use]
    pub const fn verifying_key(&self) -> VerifyingKeyBytes {
        self.verifying_key
    }

    /// Returns the envelope signature.
    #[must_use]
    pub const fn signature(&self) -> EvidenceSignature {
        self.signature
    }

    pub(super) fn encode_signed(&self, encoder: &mut Encoder) -> Result<(), G1EnvelopeError> {
        let canonical = self.design.canonical_bytes();
        encoder
            .bounded_bytes(&canonical, MAX_G1_ENVELOPE_CANONICAL_BYTES)
            .map_err(G1EnvelopeError::from)?;
        encoder.fixed(self.content_id.as_bytes());
        encoder.fixed(self.digest.as_bytes());
        encoder.fixed(self.verifying_key.as_bytes());
        encoder.fixed(self.signature.as_bytes());
        Ok(())
    }
}
