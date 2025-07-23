use xxhash_rust::xxh3::{xxh3_64, xxh3_128};

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
