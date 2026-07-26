use sha2::{Digest, Sha256};

use super::{EvidenceDigest, EvidenceError};

pub(super) struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    pub(super) fn new(domain: &[u8]) -> Self {
        let mut bytes = Vec::with_capacity(domain.len() + 128);
        bytes.extend_from_slice(domain);
        bytes.push(0);
        Self { bytes }
    }

    pub(super) fn byte(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub(super) fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(super) fn fixed(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
    }

    pub(super) fn bounded_bytes(
        &mut self,
        value: &[u8],
        maximum: usize,
    ) -> Result<(), EvidenceError> {
        if value.len() > maximum {
            return Err(EvidenceError::PayloadTooLarge {
                actual: value.len(),
                maximum,
            });
        }
        let length = u32::try_from(value.len()).map_err(|_| EvidenceError::PayloadTooLarge {
            actual: value.len(),
            maximum,
        })?;
        self.u32(length);
        self.fixed(value);
        Ok(())
    }

    pub(super) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

pub(super) fn digest(domain: &[u8], bytes: &[u8]) -> EvidenceDigest {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update([0]);
    hasher.update(bytes);
    EvidenceDigest::from_bytes(hasher.finalize().into())
}
