#![feature(test)]

extern crate test;
use test::Bencher;

use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SortKeyPart {
    pub path_id: u64,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SortKey(pub Vec<SortKeyPart>);

fn hash_path_to_u64(path: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    let result = hasher.finalize();
    u64::from_be_bytes([
        result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7],
    ])
}

#[bench]
fn compare_sort_key_u64(b: &mut Bencher) {
    let path_id = hash_path_to_u64("::long::path::to::field");
    let k1 = SortKey(vec![
        SortKeyPart {
            path_id,
            value: Some("Alpha".to_string()),
        },
        SortKeyPart {
            path_id,
            value: Some("Bravo".to_string()),
        },
    ]);
    let k2 = SortKey(vec![
        SortKeyPart {
            path_id,
            value: Some("Alpha".to_string()),
        },
        SortKeyPart {
            path_id,
            value: Some("Charlie".to_string()),
        },
    ]);

    b.iter(|| {
        let _ = k1.cmp(&k2);
    });
}

#[bench]
fn compare_sort_key_string(b: &mut Bencher) {
    let k1 = vec![
        (
            "::long::path::to::field".to_string(),
            Some("Alpha".to_string()),
        ),
        (
            "::long::path::to::field".to_string(),
            Some("Bravo".to_string()),
        ),
    ];
    let k2 = vec![
        (
            "::long::path::to::field".to_string(),
            Some("Alpha".to_string()),
        ),
        (
            "::long::path::to::field".to_string(),
            Some("Charlie".to_string()),
        ),
    ];

    b.iter(|| {
        let _ = k1.cmp(&k2);
    });
}
