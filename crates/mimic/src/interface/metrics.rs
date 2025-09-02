use crate::{
    core::traits::CanisterKind,
    db::Db,
    metrics::{self, EntitySummary, IndexMetrics, MetricsReport, MetricsSelect, StoreMetrics},
};

///
/// Metrics Interface
/// Library helpers that canister endpoints can delegate to.
///

/// Build a metrics report by inspecting the provided `db` registries
/// and the in-memory counters. Selection controls which sections to include.
#[must_use]
pub fn metrics_report<C: CanisterKind>(db: &Db<C>, select: MetricsSelect) -> MetricsReport {
    let mut data = Vec::new();
    let mut index = Vec::new();
    let mut entity_stats: Vec<EntitySummary> = Vec::new();
    let mut metrics_snapshot = None;

    if select.data {
        db.with_data(|reg| {
            reg.for_each(|path, store| {
                data.push(StoreMetrics {
                    path: path.to_string(),
                    entries: store.len(),
                    min_key: store.first_key_value().map(|(k, _)| k.into()),
                    max_key: store.last_key_value().map(|(k, _)| k.into()),
                    memory_bytes: store.memory_bytes(),
                });
            });
        });
    }

    if select.index {
        db.with_index(|reg| {
            reg.for_each(|path, store| {
                index.push(IndexMetrics {
                    path: path.to_string(),
                    entries: store.len(),
                    memory_bytes: store.memory_bytes(),
                });
            });
        });
    }

    if select.counters || select.entities {
        let snap = metrics::with_metrics(|m| m.clone());
        if select.counters {
            metrics_snapshot = Some(snap.clone());
        }
        if select.entities {
            for (path, ops) in snap.entities.iter() {
                let avg_load = if ops.load_calls > 0 {
                    ops.rows_loaded as f64 / ops.load_calls as f64
                } else {
                    0.0
                };
                let avg_delete = if ops.delete_calls > 0 {
                    ops.rows_deleted as f64 / ops.delete_calls as f64
                } else {
                    0.0
                };
                entity_stats.push(EntitySummary {
                    path: path.clone(),
                    load_calls: ops.load_calls,
                    delete_calls: ops.delete_calls,
                    rows_loaded: ops.rows_loaded,
                    rows_deleted: ops.rows_deleted,
                    avg_rows_per_load: avg_load,
                    avg_rows_per_delete: avg_delete,
                    index_inserts: ops.index_inserts,
                    index_removes: ops.index_removes,
                    unique_violations: ops.unique_violations,
                });
            }
            entity_stats.sort_by(|a, b| {
                match b
                    .avg_rows_per_load
                    .partial_cmp(&a.avg_rows_per_load)
                    .unwrap_or(core::cmp::Ordering::Equal)
                {
                    core::cmp::Ordering::Equal => match b.rows_loaded.cmp(&a.rows_loaded) {
                        core::cmp::Ordering::Equal => a.path.cmp(&b.path),
                        other => other,
                    },
                    other => other,
                }
            });
        }
    }

    MetricsReport {
        data_stores: data,
        index_stores: index,
        metrics: metrics_snapshot,
        entity_stats,
    }
}

/// Reset in-memory counters.
pub fn metrics_reset() {
    metrics::reset();
}
