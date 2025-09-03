use crate::{
    core::traits::CanisterKind,
    db::Db,
    metrics::{self, EntitySummary, MetricsReport},
};
use std::cmp::Ordering;

///
/// Metrics Interface
/// Library helpers that canister endpoints can delegate to.
///

///
/// metrics_report
/// Build a metrics report by inspecting in-memory counters only.
///
#[must_use]
pub fn metrics_report<C: CanisterKind>(_db: &Db<C>) -> MetricsReport {
    // counters only
    let mut entity_counters: Vec<EntitySummary> = Vec::new();
    let metrics_snapshot = Some(metrics::with_metrics(|m| m.clone()));

    if let Some(snap) = &metrics_snapshot {
        build_entity_counters(&mut entity_counters, snap);
    }

    MetricsReport {
        counters: metrics_snapshot,
        entity_counters,
    }
}

/// Reset in-memory counters.
pub fn metrics_reset() {
    metrics::reset();
}

fn build_entity_counters(out: &mut Vec<EntitySummary>, snap: &metrics::Metrics) {
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

        out.push(EntitySummary {
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
    out.sort_by(|a, b| {
        match b
            .avg_rows_per_load
            .partial_cmp(&a.avg_rows_per_load)
            .unwrap_or(Ordering::Equal)
        {
            Ordering::Equal => match b.rows_loaded.cmp(&a.rows_loaded) {
                Ordering::Equal => a.path.cmp(&b.path),
                other => other,
            },
            other => other,
        }
    });
}
