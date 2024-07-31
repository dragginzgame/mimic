use std::sync::{LazyLock, Mutex};
use tinyrand::{Rand, Seeded, StdRand};
use types::Timestamp;

///
/// STD_RAND
///

pub static STD_RAND: LazyLock<Mutex<StdRand>> =
    LazyLock::new(|| Mutex::new(StdRand::seed(*Timestamp::now_millis())));

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
        let mut rng1 = StdRand::seed(*Timestamp::now_millis());
        let mut rng2 = StdRand::seed(*Timestamp::now_millis() + 1);

        let mut matched = false;
        for _ in 0..100 {
            if rng1.next_u64() == rng2.next_u64() {
                matched = true;
                break;
            }
        }
        assert!(
            !matched,
            "RNGs with different seeds produced different values"
        );
    }
}
