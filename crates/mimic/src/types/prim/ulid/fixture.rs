use crate::types::Ulid;
use ::ulid::Ulid as WrappedUlid;
use sha2::{Digest, Sha256};

///
/// Fixtures
///
/// MAX = 1.099T ms, 2^40 - 1
///
/// this gives us a large range where the maximum ULID value starts
/// with 00ZZ, so any fixture ULID can be distinguished easily from a present
/// day ULID which would start with 01
///

const FIXTURE_MAX_TIMESTAMP: u128 = 1_099_511_627_775;

impl Ulid {
    /// from_string_digest
    /// a way of turning a string via a hash function into a valid ULID
    #[must_use]
    pub fn from_string_digest(name: &str) -> Self {
        // hash name to u128
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let result = hasher.finalize();

        // Take the first 16 bytes of the SHA-256 hash and convert them to u128
        let bytes = &result[0..16];
        let rand = u128::from_be_bytes(bytes.try_into().expect("slice with incorrect length"));
        let ulid = WrappedUlid::from_parts((rand % FIXTURE_MAX_TIMESTAMP) as u64, rand);

        Self(ulid)
    }
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_keys() {
        let inputs = vec![
            "key1", "key2", "key3", "key4", "Rarity-1", "Rarity-2", "Rarity-3",
        ];
        let mut keys = vec![];

        for input in inputs {
            let ulid = Ulid::from_string_digest(input);
            keys.push(ulid);
        }

        let keys_set: std::collections::HashSet<_> = keys.iter().collect();
        assert_eq!(keys.len(), keys_set.len(), "Keys are not unique");
    }

    #[test]
    fn test_ulid_fixtures_start_with_00() {
        let mut all_start_with_00 = true;

        for i in 0..10_000 {
            let ulid = Ulid::from_string_digest(&format!("input_{i}"));
            let ulid_str = ulid.to_string();

            if !ulid_str.starts_with("00") {
                all_start_with_00 = false;
                break;
            }
        }

        assert!(all_start_with_00, "Not all ULIDs start with '00'");
    }
}
