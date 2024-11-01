use super::Ulid;
use crate::utils::time::now_millis;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::{LazyLock, Mutex};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("monotonic error - overflow"))]
    Overflow,
}

///
/// GENERATOR is lazily initiated with a Mutex
/// it has to keep state to make sure key order is maintained
///

static GENERATOR: LazyLock<Mutex<Generator>> = LazyLock::new(|| Mutex::new(Generator::new()));

pub fn generate() -> Result<Ulid, Error> {
    let mut generator = GENERATOR.lock().unwrap();
    generator.generate()
}

///
/// Generator
///
/// hacked from https://github.com/dylanhart/ulid-rs/blob/master/src/generator.rs
/// as the ulid crate doesn't support a no-std generator
///

pub struct Generator {
    previous: Ulid,
}

impl Generator {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            previous: Ulid::nil(),
        }
    }

    // generate
    pub fn generate(&mut self) -> Result<Ulid, Error> {
        let last_ts = self.previous.timestamp_ms();
        let ts = now_millis();

        // maybe time went backward, or it is the same ms.
        // increment instead of generating a new random so that it is monotonic
        if ts <= last_ts {
            if let Some(next) = self.previous.increment() {
                let ulid = next;
                self.previous = ulid.into();

                return Ok(self.previous);
            }

            return Err(Error::Overflow);
        }

        // generate
        let rand = crate::utils::rand::next_u128();
        let ulid = Ulid::from_parts(ts, rand);

        self.previous = ulid;

        Ok(ulid)
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
