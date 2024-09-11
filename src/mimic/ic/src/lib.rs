///
/// IMPORT IC CRATES
///
pub use ic_cdk::*;
pub use ic_cdk_timers as timers;

// re-exports
pub mod helper;
pub mod structures;
pub use helper::get_wasm_hash;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// CYCLES
///

// Cycle Constants
pub const KC: u128 = 1_000;
pub const MC: u128 = 1_000_000;
pub const BC: u128 = 1_000_000_000;
pub const TC: u128 = 1_000_000_000_000;
pub const QC: u128 = 1_000_000_000_000_000;

// format_tc
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn format_cycles(cycles: u128) -> String {
    format!("{:.6} TC", cycles as f64 / TC as f64)
}

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Cell { source: structures::cell::CellError },
}

///
/// Logging
///

pub enum Log {
    Ok,
    Perf,
    Info,
    Warn,
    Error,
}

#[macro_export]
macro_rules! log {
    // Match when only the format string is provided (no additional args)
    ($level:expr, $fmt:expr) => {{
        // Pass an empty set of arguments to @inner
        log!(@inner $level, $fmt,);
    }};

    // Match when additional arguments are provided
    ($level:expr, $fmt:expr, $($arg:tt)*) => {{
        log!(@inner $level, $fmt, $($arg)*);
    }};

    // Inner macro for actual logging logic to avoid code duplication
    (@inner $level:expr, $fmt:expr, $($arg:tt)*) => {{
        let level: Log = $level;
        let formatted_message = format!($fmt, $($arg)*);  // Apply formatting with args

        let msg = match level {
            Log::Ok => format!("\x1b[32mOK\x1b[0m: {}", formatted_message),
            Log::Perf => format!("\x1b[35mPERF\x1b[0m: {}", formatted_message),
            Log::Info => format!("\x1b[34mINFO\x1b[0m: {}", formatted_message),
            Log::Warn => format!("\x1b[33mWARN\x1b[0m: {}", formatted_message),
            Log::Error => format!("\x1b[31mERROR\x1b[0m: {}", formatted_message),

            _ => formatted_message,
        };

        $crate::println!("{}", msg);
    }};
}
