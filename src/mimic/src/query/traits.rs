use crate::{
    db::types::{DataRow, SortKey},
    query::{QueryError, load::LoadFormat},
};

///
/// LoadQueryBuilderTrait
///

pub trait LoadQueryBuilderTrait {
    #[must_use]
    fn debug(self) -> Self;
    #[must_use]
    fn format(self, format: LoadFormat) -> Self;
    #[must_use]
    fn offset(self, offset: u32) -> Self;
    #[must_use]
    fn limit(self, limit: u32) -> Self;
    #[must_use]
    fn limit_option(self, limit: Option<u32>) -> Self;
}

///
/// LoadCollectionTrait
///

pub trait LoadCollectionTrait {
    fn count(self) -> usize;

    fn key(self) -> Option<SortKey>;
    fn try_key(self) -> Result<SortKey, QueryError>;
    fn keys(self) -> Vec<SortKey>;

    fn data_row(self) -> Option<DataRow>;
    fn try_data_row(self) -> Result<DataRow, QueryError>;
    fn data_rows(self) -> Vec<DataRow>;

    fn blob(self) -> Option<Vec<u8>>;
    fn try_blob(self) -> Result<Vec<u8>, QueryError>;
    fn blobs(self) -> Vec<Vec<u8>>;
}
