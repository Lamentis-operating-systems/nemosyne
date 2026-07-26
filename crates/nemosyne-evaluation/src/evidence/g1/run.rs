use crate::evidence::{
    ArtifactContentId, AttemptedArtifactKind, RunManifestClaimsV1, SignedRunManifestV1,
    ValidForOutcomeAccess, canonical::Encoder,
};

use super::{G1EnvelopeError, G1ExecutionBindingV1, SignedG1EvaluationEnvelopeV1};

const G1_RUN_PAYLOAD_DOMAIN: &[u8] = b"nemosyne.evidence.g1-run-payload.v1";

/// Finalizes one signed G1 execution manifest from a verified frozen envelope.
///
/// The returned EVD-01 manifest still requires a matching independently
/// authenticated guard witness before `ValidForOutcomeAccess` can exist.
pub fn finalize_g1_run_manifest(
    envelope: &SignedG1EvaluationEnvelopeV1,
    execution: &G1ExecutionBindingV1,
    claims: RunManifestClaimsV1,
    signing_key_bytes: &[u8; 32],
) -> Result<SignedRunManifestV1, G1EnvelopeError> {
    envelope.verify()?;
    if claims.kind() != AttemptedArtifactKind::G1RunManifest {
        return Err(crate::evidence::EvidenceError::ExpectedRunManifest.into());
    }
    let payload = g1_run_payload(envelope, execution)?;
    SignedRunManifestV1::sign(claims, &payload, signing_key_bytes).map_err(Into::into)
}

/// A successful EVD-01 admission bound to one exact signed G1 design and
/// execution instance.
///
/// This value contains no outcomes and makes no empirical or product claim.
/// It is the only intended input boundary for a later EVD-02 analyzer.
#[derive(Debug, Eq, PartialEq)]
pub struct AdmittedG1RunV1 {
    admission: ValidForOutcomeAccess,
    envelope: SignedG1EvaluationEnvelopeV1,
    execution: G1ExecutionBindingV1,
}

impl AdmittedG1RunV1 {
    /// Verifies that a successful EVD-01 admission contains exactly the
    /// supplied signed G1 envelope and execution binding.
    pub fn bind(
        admission: ValidForOutcomeAccess,
        envelope: &SignedG1EvaluationEnvelopeV1,
        execution: &G1ExecutionBindingV1,
    ) -> Result<Self, G1EnvelopeError> {
        envelope.verify()?;
        if admission.manifest().claims().kind() != AttemptedArtifactKind::G1RunManifest {
            return Err(crate::evidence::EvidenceError::ExpectedRunManifest.into());
        }
        let expected_payload = g1_run_payload(envelope, execution)?;
        if admission.manifest().payload() != expected_payload {
            return Err(G1EnvelopeError::RunBindingMismatch);
        }
        Ok(Self {
            admission,
            envelope: envelope.clone(),
            execution: execution.clone(),
        })
    }

    /// Returns the complete admitted EVD-01 value.
    #[must_use]
    pub const fn admission(&self) -> &ValidForOutcomeAccess {
        &self.admission
    }

    /// Returns the exact verified signed G1 envelope bound into the run.
    #[must_use]
    pub const fn envelope(&self) -> &SignedG1EvaluationEnvelopeV1 {
        &self.envelope
    }

    /// Returns the exact G1 execution binding bound into the run.
    #[must_use]
    pub const fn execution(&self) -> &G1ExecutionBindingV1 {
        &self.execution
    }

    /// Returns the exact signed G1 envelope identity bound into the run.
    #[must_use]
    pub const fn envelope_content_id(&self) -> ArtifactContentId {
        self.envelope.content_id()
    }

    /// Returns the exact G1 execution-instance identity bound into the run.
    #[must_use]
    pub const fn execution_identity(&self) -> super::G1ExecutionIdentity {
        self.execution.execution_identity()
    }
}

fn g1_run_payload(
    envelope: &SignedG1EvaluationEnvelopeV1,
    execution: &G1ExecutionBindingV1,
) -> Result<Vec<u8>, G1EnvelopeError> {
    let mut encoder = Encoder::new(G1_RUN_PAYLOAD_DOMAIN);
    encoder.u32(1);
    envelope.encode_signed(&mut encoder)?;
    execution.encode(&mut encoder);
    Ok(encoder.finish())
}
