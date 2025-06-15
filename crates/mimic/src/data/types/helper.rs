use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

// hash_path_to_u64
// 10-50x faster than doing it via Sha256
pub fn hash_path_to_u64(path: &str) -> u64 {
    let mut hasher = XxHash64::with_seed(0);
    path.hash(&mut hasher);

    hasher.finish()
}
