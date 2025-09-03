//! Observability: runtime event telemetry (metrics + logs) and storage snapshots.
//! Import via `mimic::obs::*` or re-exports below.

pub mod log;
pub mod metrics;
pub mod snapshot;

// Convenient re-exports
pub use log::{log_push, logs_reset, logs_snapshot};
pub use metrics::{
    EntityCounters, EntitySummary, EventOps, EventPerf, EventReport, EventSelect, EventState, Span,
    report as event_report, reset_all as event_reset_all,
};
pub use snapshot::{
    DataStoreSnapshot, EntitySnapshot, IndexStoreSnapshot, StorageReport, storage_report,
};
