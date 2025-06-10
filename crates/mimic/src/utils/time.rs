use std::time::SystemTime;

// time_nanos
#[allow(unreachable_code)]
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

/// now_secs
/// returns the current time in seconds since UNIX_EPOCH
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn now_secs() -> u64 {
    (time_nanos() / 1_000_000_000) as u64
}

/// now_millis
/// returns the current time in milliseconds since UNIX_EPOCH.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn now_millis() -> u64 {
    (time_nanos() / 1_000_000) as u64
}

/// now_micros
/// returns the current time in microseconds since UNIX_EPOCH
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn now_micros() -> u64 {
    (time_nanos() / 1_000) as u64
}

/// now_nanos
/// returns the current time in nanoseconds since UNIX_EPOCH
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn now_nanos() -> u64 {
    time_nanos() as u64
}
