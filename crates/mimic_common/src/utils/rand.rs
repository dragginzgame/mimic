use crate::utils::time::now_nanos;
use std::sync::{LazyLock, Mutex};
use tinyrand::{Rand, Seeded, StdRand};

///
/// STD_RAND
///

pub static STD_RAND: LazyLock<Mutex<StdRand>> =
    LazyLock::new(|| Mutex::new(StdRand::seed(now_nanos())));

// next_u8
// (uses u16 because there is no next_u8)
#[must_use]
pub fn next_u8() -> u8 {
    (next_u16() & 0xFF) as u8
}

// next_u16
#[must_use]
pub fn next_u16() -> u16 {
    STD_RAND.lock().expect("mutex").next_u16()
}

// next_u32
#[must_use]
pub fn next_u32() -> u32 {
    STD_RAND.lock().expect("mutex").next_u32()
}

// next_64
#[must_use]
pub fn next_u64() -> u64 {
    STD_RAND.lock().expect("mutex").next_u64()
}

// next_u128
#[must_use]
pub fn next_u128() -> u128 {
    STD_RAND.lock().expect("mutex").next_u128()
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_u64s() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        while set.len() < 1000 {
            let random_value = next_u64();
            assert!(set.insert(random_value), "value already in set");
        }
    }

    #[test]
    fn test_rng_reseeding() {
        let mut rng1 = StdRand::seed(now_nanos());
        let mut rng2 = StdRand::seed(now_nanos() + 1);

        let mut matched = false;
        for _ in 0..100 {
            if rng1.next_u64() == rng2.next_u64() {
                matched = true;
                break;
            }
        }
        assert!(
            !matched,
            "RNGs with different seeds unexpectedly produced the same value"
        );
    }

    #[test]
    fn test_determinism_with_fixed_seed() {
        let seed = 42;
        let mut rng1 = StdRand::seed(seed);
        let mut rng2 = StdRand::seed(seed);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_bit_entropy() {
        let mut bits = 0u64;
        for _ in 0..100 {
            bits |= next_u64();
        }

        let bit_count = bits.count_ones();
        assert!(
            bit_count > 8,
            "Low entropy: only {bit_count} bits set in 100 samples",
        );
    }
}
