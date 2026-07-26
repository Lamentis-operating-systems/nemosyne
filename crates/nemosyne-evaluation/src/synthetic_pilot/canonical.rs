use sha2::{Digest, Sha256};

use super::PilotRoot;

pub(super) struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    pub(super) fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub(super) fn byte(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub(super) fn bytes(&mut self, value: &[u8]) {
        self.u64(value.len() as u64);
        self.bytes.extend_from_slice(value);
    }

    pub(super) fn text(&mut self, value: &str) {
        self.bytes(value.as_bytes());
    }

    pub(super) fn root(self, domain: &[u8]) -> PilotRoot {
        let mut hasher = Sha256::new();
        hasher.update((domain.len() as u64).to_be_bytes());
        hasher.update(domain);
        hasher.update((self.bytes.len() as u64).to_be_bytes());
        hasher.update(self.bytes);
        PilotRoot::from_digest(hasher.finalize().into())
    }
}
