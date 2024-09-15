use std::time::SystemTime;

// time_nanos
fn time_nanos() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        return ::ic_cdk::api::time() as u128;
    }

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos()
}

/// Returns the current time in seconds since UNIX_EPOCH.
#[must_use]
pub fn now_secs() -> u64 {
    (time_nanos() / 1_000_000_000) as u64
}

/// Returns the current time in milliseconds since UNIX_EPOCH.
#[must_use]
pub fn now_millis() -> u64 {
    (time_nanos() / 1_000_000) as u64
}

/// Returns the current time in microseconds since UNIX_EPOCH.
#[must_use]
pub fn now_micros() -> u64 {
    (time_nanos() / 1_000) as u64
}

/// Returns the current time in nanoseconds since UNIX_EPOCH.
#[must_use]
pub fn now_nanos() -> u64 {
    time_nanos() as u64
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    /// Helper function to get the current time in nanoseconds.
    fn current_time_nanos() -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
    }

    /// Test that `now_nanos()` returns a value within a reasonable range.
    #[test]
    fn test_now_nanos_within_range() {
        let before = current_time_nanos();
        let nanos = now_nanos() as u128;
        let after = current_time_nanos();

        assert!(
            nanos >= before && nanos <= after,
            "now_nanos() returned {} which is not between {} and {}",
            nanos,
            before,
            after
        );
    }

    /// Test that `now_millis()` returns milliseconds correctly derived from `now_nanos()`.
    #[test]
    fn test_now_millis_accuracy() {
        let nanos = now_nanos() as u128;
        let millis = now_millis() as u128;
        let expected_millis = nanos / 1_000_000;

        assert_eq!(
            millis, expected_millis,
            "now_millis() returned {} but expected {}",
            millis, expected_millis
        );
    }

    /// Test that `now_micros()` returns microseconds correctly derived from `now_nanos()`.
    #[test]
    fn test_now_micros_accuracy() {
        let nanos = now_nanos() as u128;
        let micros = now_micros() as u128;
        let expected_micros = nanos / 1_000;

        assert_eq!(
            micros, expected_micros,
            "now_micros() returned {} but expected {}",
            micros, expected_micros
        );
    }

    /// Test that `now_seconds()` returns seconds correctly derived from `now_nanos()`.
    #[test]
    fn test_now_seconds_accuracy() {
        let nanos = now_nanos() as u128;
        let seconds = now_secs() as u128;
        let expected_seconds = nanos / 1_000_000_000;

        assert_eq!(
            seconds, expected_seconds,
            "now_seconds() returned {} but expected {}",
            seconds, expected_seconds
        );
    }

    /// Test that multiple calls return increasing or equal values, ensuring time moves forward.
    #[test]
    fn test_time_increases() {
        let nanos1 = now_nanos() as u128;
        let micros1 = now_micros() as u128;
        let millis1 = now_millis() as u128;
        let seconds1 = now_secs() as u128;

        // Introduce a small sleep to ensure time advances
        std::thread::sleep(Duration::from_millis(1));

        let nanos2 = now_nanos() as u128;
        let micros2 = now_micros() as u128;
        let millis2 = now_millis() as u128;
        let seconds2 = now_secs() as u128;

        assert!(
            nanos2 >= nanos1,
            "Nanoseconds did not increase: {} >= {}",
            nanos2,
            nanos1
        );
        assert!(
            micros2 >= micros1,
            "Microseconds did not increase: {} >= {}",
            micros2,
            micros1
        );
        assert!(
            millis2 >= millis1,
            "Milliseconds did not increase: {} >= {}",
            millis2,
            millis1
        );
        assert!(
            seconds2 >= seconds1,
            "Seconds did not increase: {} >= {}",
            seconds2,
            seconds1
        );
    }

    /// Test that `time_nanos()` does not panic when the system time is before UNIX_EPOCH.
    /// (This is more of a theoretical test as such a condition is highly unlikely.)
    #[test]
    #[should_panic(expected = "SystemTime before UNIX_EPOCH!")]
    fn test_time_before_epoch() {
        // Mocking SystemTime is non-trivial; this test is illustrative.
        // In practice, you might use a mocking library or dependency injection.
        use std::time::{Duration, UNIX_EPOCH};

        // Simulate a time before UNIX_EPOCH by subtracting duration
        let fake_time = UNIX_EPOCH - Duration::from_secs(1);
        let _ = fake_time
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH!");
    }
}
