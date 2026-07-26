use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use super::{
    EvidenceError, EvidenceSignature, VerifyingKeyBytes,
    canonical::{Encoder, digest},
};

const SIGNATURE_MESSAGE_DOMAIN: &[u8] = b"nemosyne.evidence.signature-message.v1";
const SIGNER_ID_DOMAIN: &[u8] = b"nemosyne.evidence.signer-id.v1";

pub(super) fn sign(
    domain: &[u8],
    canonical_bytes: &[u8],
    signing_key: &SigningKey,
) -> (VerifyingKeyBytes, EvidenceSignature) {
    let message = signature_message(domain, canonical_bytes);
    let signature = signing_key.sign(&message);
    (
        VerifyingKeyBytes::from_bytes(signing_key.verifying_key().to_bytes()),
        EvidenceSignature::from_bytes(signature.to_bytes()),
    )
}

pub(super) fn verify(
    domain: &[u8],
    canonical_bytes: &[u8],
    verifying_key: VerifyingKeyBytes,
    signature: EvidenceSignature,
) -> Result<(), EvidenceError> {
    let key = VerifyingKey::from_bytes(verifying_key.as_bytes())
        .map_err(|_| EvidenceError::InvalidSignature)?;
    let signature = Signature::from_bytes(signature.as_bytes());
    key.verify(&signature_message(domain, canonical_bytes), &signature)
        .map_err(|_| EvidenceError::InvalidSignature)
}

pub(super) fn signer_id(verifying_key: VerifyingKeyBytes) -> [u8; 32] {
    *digest(SIGNER_ID_DOMAIN, verifying_key.as_bytes()).as_bytes()
}

fn signature_message(domain: &[u8], canonical_bytes: &[u8]) -> Vec<u8> {
    let mut encoder = Encoder::new(SIGNATURE_MESSAGE_DOMAIN);
    encoder
        .bounded_bytes(domain, u16::MAX as usize)
        .expect("fixed signature domain is bounded");
    encoder
        .bounded_bytes(canonical_bytes, u32::MAX as usize)
        .expect("canonical evidence artifact is bounded before signing");
    encoder.finish()
}
