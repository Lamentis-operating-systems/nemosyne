use crate::evidence::{
    AttemptedArtifactKind, RunManifestClaimsV1, SignedRunManifestV1, canonical::Encoder,
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
    let mut encoder = Encoder::new(G1_RUN_PAYLOAD_DOMAIN);
    encoder.u32(1);
    envelope.encode_signed(&mut encoder)?;
    execution.encode(&mut encoder);
    SignedRunManifestV1::sign(claims, &encoder.finish(), signing_key_bytes).map_err(Into::into)
}
