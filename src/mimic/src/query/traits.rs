use crate::{
    db::types::{DataRow, SortKey},
    query::{
        QueryError,
        load::LoadFormat,
        types::{Order, Search},
    },
};

///
/// LoadQueryBuilderTrait
///

pub trait LoadQueryBuilderTrait {
    fn debug(self) -> Self;

    fn format(self, format: LoadFormat) -> Self;
    fn offset(self, offset: u32) -> Self;
    fn limit(self, limit: u32) -> Self;
    fn limit_option(self, limit: Option<u32>) -> Self;

    fn search<T: Into<Search>>(self, search: T) -> Self;
    fn search_option(self, search: Option<Search>) -> Self;
    fn search_all(self, text: &str) -> Self;
    fn search_fields<I, K, V>(self, fields: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>;

    fn order<T: Into<Order>>(self, order: T) -> Self;
    fn order_option<T: Into<Order>>(self, order: Option<T>) -> Self;
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
