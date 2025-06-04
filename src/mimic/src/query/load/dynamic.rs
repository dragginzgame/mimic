use crate::{
    db::types::{DataRow, SortKey},
    query::{LoadFormat, LoadResponse, Selector},
    traits::EntityKind,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// LoadQueryDyn
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQueryDyn {
    pub selector: Selector,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
    pub include_children: bool,
}

impl LoadQueryDyn {
    #[must_use]
    pub fn new(selector: Selector) -> Self {
        Self {
            selector,
            ..Default::default()
        }
    }

    // format
    #[must_use]
    pub const fn format(mut self, format: LoadFormat) -> Self {
        self.format = format;
        self
    }

    // offset
    #[must_use]
    pub const fn offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub const fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }

    // children
    #[must_use]
    pub const fn children(mut self) -> Self {
        self.include_children = true;
        self
    }
}

///
/// LoadQueryDynBuilder
///

#[derive(Debug, Default)]
pub struct LoadQueryDynBuilder<E>
where
    E: EntityKind,
{
    phantom: PhantomData<E>,
}

impl<E> LoadQueryDynBuilder<E>
where
    E: EntityKind,
{
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // selector
    #[must_use]
    pub fn selector(self, selector: Selector) -> LoadQueryDyn {
        LoadQueryDyn::new(selector)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryDyn {
        LoadQueryDyn::new(Selector::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryDyn {
        LoadQueryDyn::new(Selector::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryDyn {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let selector = Selector::One(ck_str);

        LoadQueryDyn::new(selector)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryDyn {
        let selector = Selector::Many(cks.to_vec());

        LoadQueryDyn::new(selector)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQueryDyn {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let selector = Selector::Range(start, end);

        LoadQueryDyn::new(selector)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQueryDyn {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let selector = Selector::Prefix(prefix);

        LoadQueryDyn::new(selector)
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
