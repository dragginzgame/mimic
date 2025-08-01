use crate::core::types::{Ulid, UlidError};
use icu::utils;
use std::sync::{LazyLock, Mutex};

///
/// GENERATOR is lazily initiated with a Mutex
/// it has to keep state to make sure key order is maintained
///

static GENERATOR: LazyLock<Mutex<Generator>> = LazyLock::new(|| Mutex::new(Generator::default()));

pub fn generate() -> Result<Ulid, UlidError> {
    let mut generator = GENERATOR.lock().unwrap();

    generator.generate()
}

///
/// Generator
///
/// hacked from https://github.com/dylanhart/ulid-rs/blob/master/src/generator.rs
/// as the ulid crate doesn't support a no-std generator
///

#[derive(Default)]
pub struct Generator {
    previous: Ulid,
}

impl Generator {
    // generate
    pub fn generate(&mut self) -> Result<Ulid, UlidError> {
        let last_ts = self.previous.timestamp_ms();
        let ts = utils::time::now_millis();

        // maybe time went backward, or it is the same ms.
        // increment instead of generating a new random so that it is monotonic
        if ts <= last_ts {
            if let Some(next) = self.previous.increment() {
                let ulid = next;
                self.previous = ulid.into();

                return Ok(self.previous);
            }

            return Err(UlidError::GeneratorOverflow);
        }

        // generate
        let rand = utils::rand::next_u128();
        let ulid = Ulid::from_parts(ts, rand);

        self.previous = ulid;

        Ok(ulid)
    }
}

mod test {
    #[allow(unused_imports)] // weird
    use super::*;

    #[test]
    fn test_monotonic_generation() {
        let mut g = Generator::default();
        let a = g.generate().unwrap();
        let b = g.generate().unwrap();

        assert!(a < b);
    }
}
