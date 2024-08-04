use sha2::{Digest, Sha256};

// get_wasm_hash
#[must_use]
pub fn get_wasm_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);

    hasher.finalize().to_vec()
}
