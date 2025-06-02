use crate::{
    db::types::{DataRow, SortKey},
    query::{LoadFormat, LoadResponse, Selector},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadQueryDynBuilder
///

#[derive(Debug, Default)]
pub struct LoadQueryDynBuilder {}

impl LoadQueryDynBuilder {
    // new
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    // method
    #[must_use]
    pub fn method(self, path: &str, selector: Selector) -> LoadQueryDyn {
        LoadQueryDyn::new(path, selector)
    }

    // all
    #[must_use]
    pub fn all(self, path: &str) -> LoadQueryDyn {
        LoadQueryDyn::new(path, Selector::All)
    }

    // only
    #[must_use]
    pub fn only(self, path: &str) -> LoadQueryDyn {
        LoadQueryDyn::new(path, Selector::Only)
    }

    // one
    pub fn one<T: ToString>(self, path: &str, ck: &[T]) -> LoadQueryDyn {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = Selector::One(ck_str);

        LoadQueryDyn::new(path, method)
    }

    // many
    #[must_use]
    pub fn many(self, path: &str, cks: &[Vec<String>]) -> LoadQueryDyn {
        let method = Selector::Many(cks.to_vec());

        LoadQueryDyn::new(path, method)
    }

    // range
    pub fn range<T: ToString>(self, path: &str, start: &[T], end: &[T]) -> LoadQueryDyn {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = Selector::Range(start, end);

        LoadQueryDyn::new(path, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, path: &str, prefix: &[T]) -> LoadQueryDyn {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = Selector::Prefix(prefix);

        LoadQueryDyn::new(path, method)
    }
}

///
/// LoadQueryDyn
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQueryDyn {
    pub path: String,
    pub selector: Selector,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
}

impl LoadQueryDyn {
    #[must_use]
    pub fn new(path: &str, selector: Selector) -> Self {
        Self {
            path: path.to_string(),
            selector,
            ..Default::default()
        }
    }

    // format
    #[must_use]
    pub fn format(mut self, format: LoadFormat) -> Self {
        self.format = format;
        self
    }

    // offset
    #[must_use]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }
}

///
/// LoadCollectionDyn
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct LoadCollectionDyn(pub Vec<DataRow>);

impl LoadCollectionDyn {
    // response
    #[must_use]
    pub fn response(self, format: LoadFormat) -> LoadResponse {
        match format {
            LoadFormat::Rows => LoadResponse::Rows(self.data_rows()),
            LoadFormat::Keys => LoadResponse::Keys(self.keys()),
            LoadFormat::Count => LoadResponse::Count(self.count()),
        }
    }

    // count
    #[must_use]
    pub fn count(self) -> usize {
        self.0.len()
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<SortKey> {
        self.0.first().map(|row| row.key.clone())
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<SortKey> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    // data_row
    #[must_use]
    pub fn data_row(self) -> Option<DataRow> {
        self.0.first().cloned()
    }

    // data_rows
    #[must_use]
    pub fn data_rows(self) -> Vec<DataRow> {
        self.0
    }

    // blob
    #[must_use]
    pub fn blob(self) -> Option<Vec<u8>> {
        self.0.first().map(|row| row.value.data.clone())
    }

    // blobs
    #[must_use]
    pub fn blobs(self) -> Vec<Vec<u8>> {
        self.0.into_iter().map(|row| row.value.data).collect()
    }
}
