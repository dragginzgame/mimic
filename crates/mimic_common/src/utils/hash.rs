use xxhash_rust::xxh3::{xxh3_64, xxh3_128};

pub use xxhash_rust::xxh3::Xxh3;

//
// hashing algorithms that are fast and safe, taken
// from the xxhash_rust crate
//

// hash_u64
#[must_use]
pub fn hash_u64(bytes: &[u8]) -> u64 {
    xxh3_64(bytes)
}

// hash_u128
#[must_use]
pub fn hash_u128(bytes: &[u8]) -> u128 {
    xxh3_128(bytes)
}

// fnv1a_64
// const hashing
#[must_use]
#[allow(clippy::unreadable_literal)]
pub const fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    let mut i = 0;

    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }

    hash
}
