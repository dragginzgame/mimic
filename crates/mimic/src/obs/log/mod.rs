// Event telemetry (resettable counters, perf, and logs).
// Re-export existing metrics counters for compatibility, and provide a simple
// in-memory log buffer for lightweight event logging.

use std::{cell::RefCell, collections::VecDeque};

thread_local! {
    static LOG_BUFFER: RefCell<VecDeque<String>> = const { RefCell::new(VecDeque::new()) };
}

/// Append a log line to the event buffer (fixed-size ring).
pub fn log_push(line: impl Into<String>) {
    const CAP: usize = 256;

    LOG_BUFFER.with(|b| {
        let mut buf = b.borrow_mut();
        if buf.len() >= CAP {
            buf.pop_front();
        }
        buf.push_back(line.into());
    });
}

/// Snapshot current logs (oldest → newest).
#[must_use]
pub fn logs_snapshot() -> Vec<String> {
    LOG_BUFFER.with(|b| b.borrow().iter().cloned().collect())
}

/// Clear the log buffer.
pub fn logs_reset() {
    LOG_BUFFER.with(|b| b.borrow_mut().clear());
}
