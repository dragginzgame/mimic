use crate::core::{Key, traits::EntityKind};
use candid::CandidType;
use icu::{cdk::api::performance_counter, utils::time};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::BTreeMap, marker::PhantomData};

///
/// Metrics
/// Ephemeral, in-memory counters and simple perf totals for operations.
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct Metrics {
    pub ops: Ops,
    pub perf: Perf,
    pub entities: BTreeMap<String, EntityOps>,
    pub since_ms: u64,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            ops: Ops::default(),
            perf: Perf::default(),
            entities: BTreeMap::new(),
            since_ms: time::now_millis(),
        }
    }
}

///
/// Ops
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Ops {
    // Executor entrypoints
    pub load_calls: u64,
    pub save_calls: u64,
    pub delete_calls: u64,

    // Planner kinds
    pub plan_index: u64,
    pub plan_keys: u64,
    pub plan_range: u64,

    // Rows touched
    pub rows_loaded: u64,
    pub rows_deleted: u64,

    // Index maintenance
    pub index_inserts: u64,
    pub index_removes: u64,
    pub unique_violations: u64,
}

///
/// EntityOps
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct EntityOps {
    pub load_calls: u64,
    pub save_calls: u64,
    pub delete_calls: u64,
    pub rows_loaded: u64,
    pub rows_deleted: u64,
    pub index_inserts: u64,
    pub index_removes: u64,
    pub unique_violations: u64,
}

///
/// Perf
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Perf {
    // Instruction totals per executor (ic_cdk::api::performance_counter(1))
    pub load_inst_total: u128,
    pub save_inst_total: u128,
    pub delete_inst_total: u128,

    // Maximum observed instruction deltas
    pub load_inst_max: u64,
    pub save_inst_max: u64,
    pub delete_inst_max: u64,
}

thread_local! {
    static METRICS: RefCell<Metrics> = RefCell::new(Metrics::default());
}

/// Borrow metrics immutably.
pub fn with_metrics<R>(f: impl FnOnce(&Metrics) -> R) -> R {
    METRICS.with(|m| f(&m.borrow()))
}

/// Borrow metrics mutably.
pub fn with_metrics_mut<R>(f: impl FnOnce(&mut Metrics) -> R) -> R {
    METRICS.with(|m| f(&mut m.borrow_mut()))
}

/// Reset all counters (useful in tests).
pub fn reset() {
    with_metrics_mut(|m| *m = Metrics::default());
}

/// Accumulate instruction counts and track a max.
#[allow(clippy::missing_const_for_fn)]
pub fn add_instructions(total: &mut u128, max: &mut u64, delta_inst: u64) {
    *total = total.saturating_add(u128::from(delta_inst));
    if delta_inst > *max {
        *max = delta_inst;
    }
}

///
/// ExecKind
///

#[derive(Clone, Copy, Debug)]
pub enum ExecKind {
    Load,
    Save,
    Delete,
}

/// Begin an executor timing span and increment call counters.
/// Returns the start instruction counter value.
#[must_use]
pub fn exec_start(kind: ExecKind) -> u64 {
    with_metrics_mut(|m| match kind {
        ExecKind::Load => m.ops.load_calls = m.ops.load_calls.saturating_add(1),
        ExecKind::Save => m.ops.save_calls = m.ops.save_calls.saturating_add(1),
        ExecKind::Delete => m.ops.delete_calls = m.ops.delete_calls.saturating_add(1),
    });

    // Instruction counter (counter_type = 1) is per-message and monotonic.
    performance_counter(1)
}

/// Finish an executor timing span and aggregate instruction deltas and row counters.
pub fn exec_finish(kind: ExecKind, start_inst: u64, rows_touched: u64) {
    let now = performance_counter(1);
    let delta = now.saturating_sub(start_inst);

    with_metrics_mut(|m| match kind {
        ExecKind::Load => {
            m.ops.rows_loaded = m.ops.rows_loaded.saturating_add(rows_touched);
            add_instructions(
                &mut m.perf.load_inst_total,
                &mut m.perf.load_inst_max,
                delta,
            );
        }
        ExecKind::Save => {
            add_instructions(
                &mut m.perf.save_inst_total,
                &mut m.perf.save_inst_max,
                delta,
            );
        }
        ExecKind::Delete => {
            m.ops.rows_deleted = m.ops.rows_deleted.saturating_add(rows_touched);
            add_instructions(
                &mut m.perf.delete_inst_total,
                &mut m.perf.delete_inst_max,
                delta,
            );
        }
    });
}

/// Per-entity variants using EntityKind::PATH
#[must_use]
pub fn exec_start_for<E>(kind: ExecKind) -> u64
where
    E: EntityKind,
{
    let start = exec_start(kind);
    with_metrics_mut(|m| {
        let entry = m.entities.entry(E::PATH.to_string()).or_default();
        match kind {
            ExecKind::Load => entry.load_calls = entry.load_calls.saturating_add(1),
            ExecKind::Save => entry.save_calls = entry.save_calls.saturating_add(1),
            ExecKind::Delete => entry.delete_calls = entry.delete_calls.saturating_add(1),
        }
    });
    start
}

pub fn exec_finish_for<E>(kind: ExecKind, start_inst: u64, rows_touched: u64)
where
    E: EntityKind,
{
    exec_finish(kind, start_inst, rows_touched);
    with_metrics_mut(|m| {
        let entry = m.entities.entry(E::PATH.to_string()).or_default();
        match kind {
            ExecKind::Load => entry.rows_loaded = entry.rows_loaded.saturating_add(rows_touched),
            ExecKind::Delete => {
                entry.rows_deleted = entry.rows_deleted.saturating_add(rows_touched);
            }
            ExecKind::Save => {}
        }
    });
}

///
/// RAII span guard to simplify metrics instrumentation
///

pub struct Span<E: EntityKind> {
    kind: ExecKind,
    start: u64,
    rows: u64,
    finished: bool,
    _marker: PhantomData<E>,
}

impl<E: EntityKind> Span<E> {
    #[must_use]
    pub fn new(kind: ExecKind) -> Self {
        Self {
            kind,
            start: exec_start_for::<E>(kind),
            rows: 0,
            finished: false,
            _marker: PhantomData,
        }
    }

    pub const fn set_rows(&mut self, rows: u64) {
        self.rows = rows;
    }

    pub const fn add_rows(&mut self, rows: u64) {
        self.rows = self.rows.saturating_add(rows);
    }

    pub fn finish(mut self) {
        if !self.finished {
            exec_finish_for::<E>(self.kind, self.start, self.rows);
            self.finished = true;
        }
    }
}

impl<E: EntityKind> Drop for Span<E> {
    fn drop(&mut self) {
        if !self.finished {
            exec_finish_for::<E>(self.kind, self.start, self.rows);
            self.finished = true;
        }
    }
}

//
// Snapshot types (formerly metrics::db)
// Kept here under metrics to consolidate naming: everything is "metrics".
//

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct MetricsReport {
    /// Ephemeral runtime counters since `since_ms`.
    pub counters: Option<Metrics>,
    /// Per-entity ephemeral counters and averages.
    pub entity_counters: Vec<EntitySummary>,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct StoreMetrics {
    pub path: String,
    pub entries: u64,
    pub min_key: Option<Key>,
    pub max_key: Option<Key>,
    pub memory_bytes: u64,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct IndexMetrics {
    pub path: String,
    pub entries: u64,
    pub memory_bytes: u64,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct EntitySummary {
    pub path: String,
    pub load_calls: u64,
    pub delete_calls: u64,
    pub rows_loaded: u64,
    pub rows_deleted: u64,
    pub avg_rows_per_load: f64,
    pub avg_rows_per_delete: f64,
    pub index_inserts: u64,
    pub index_removes: u64,
    pub unique_violations: u64,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct EntityStorage {
    /// Store path (e.g., test_design::schema::TestDataStore)
    pub store: String,
    /// Entity path (e.g., test_design::canister::db::Index)
    pub path: String,
    /// Number of rows for this entity in the store
    pub entries: u64,
    /// Approximate bytes used (key + value)
    pub memory_bytes: u64,
}

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct StorageReport {
    /// Live storage inventory for data stores.
    pub storage_data: Vec<StoreMetrics>,
    /// Live storage inventory for index stores.
    pub storage_index: Vec<IndexMetrics>,
    /// Live per-entity storage breakdown by store and entity path.
    pub entity_storage: Vec<EntityStorage>,
}

/// Increment unique-violation counters globally and for a specific entity type.
pub fn record_unique_violation_for<E>(m: &mut Metrics)
where
    E: crate::core::traits::EntityKind,
{
    m.ops.unique_violations = m.ops.unique_violations.saturating_add(1);
    let entry = m.entities.entry(E::PATH.to_string()).or_default();
    entry.unique_violations = entry.unique_violations.saturating_add(1);
}

/// Select which parts of the metrics report to include.
#[derive(CandidType, Clone, Copy, Debug, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct MetricsSelect {
    pub data: bool,
    pub index: bool,
    pub counters: bool,
    pub entities: bool,
}

impl MetricsSelect {
    #[must_use]
    pub const fn all() -> Self {
        Self {
            data: true,
            index: true,
            counters: true,
            entities: true,
        }
    }
}

impl Default for MetricsSelect {
    fn default() -> Self {
        Self::all()
    }
}

// Note: no backward-compat type aliases to avoid tech debt.
