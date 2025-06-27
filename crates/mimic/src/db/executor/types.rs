use crate::db::store::DataKey;

///
/// ResolvedSelector
///

#[derive(Debug)]
pub enum ResolvedSelector {
    One(DataKey),
    Many(Vec<DataKey>),
    Range(DataKey, DataKey),
}
