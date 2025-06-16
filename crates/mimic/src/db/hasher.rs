use xxhash_rust::xxh3::xxh3_64;

// xx_hash_u64
#[must_use]
pub fn xx_hash_u64(path: &str) -> u64 {
    xxh3_64(path.as_bytes())
}
