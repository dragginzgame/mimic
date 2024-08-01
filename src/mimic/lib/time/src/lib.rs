use std::time::SystemTime;

//
// `now`
//
// Returns the current UNIX timestamp in SECONDS as a u64
//
// # Panics
//
// This function will panic if the system's time is before the UNIX_EPOCH
//
#[must_use]
#[allow(unreachable_code)]
pub fn now() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        // divide by 1e9 to convert nanoseconds to seconds
        return ::ic_cdk::api::time() / 1_000_000_000;
    }

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}

///
/// `now_millis`
///
/// Returns the current time since the UNIX epoch in milliseconds as a u64
///
/// # Panics
///
/// This function will panic if the system's time is before the `UNIX_EPOCH` or
/// if the time since the UNIX epoch cannot fit into a u64
///
#[must_use]
#[allow(unreachable_code)]
pub fn now_millis() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        // divide by 1e6 to convert nanoseconds to milliseconds
        return ::ic_cdk::api::time() / 1_000_000;
    }

    let millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis();

    match u64::try_from(millis) {
        Ok(ms) => ms,
        Err(e) => panic!("{e}"),
    }
}
