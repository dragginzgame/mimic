#![allow(clippy::type_complexity)]
use crate::{
    data::{
        query::LoadFormat,
        types::{CompositeKey, Selector, Where},
    },
    schema::types::SortDirection,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadQueryBuilder
///

#[derive(Debug, Default)]
pub struct LoadQueryBuilder {}

impl LoadQueryBuilder {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // selector
    #[must_use]
    pub fn selector(self, selector: Selector) -> LoadQuery {
        LoadQuery::new(selector)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQuery {
        LoadQuery::new(Selector::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQuery {
        LoadQuery::new(Selector::Only)
    }

    // one
    pub fn one<K: Into<CompositeKey>>(self, ck: K) -> LoadQuery {
        let selector = Selector::One(ck.into());

        LoadQuery::new(selector)
    }

    // many
    #[must_use]
    pub fn many<K>(self, cks: &[K]) -> LoadQuery
    where
        K: Clone + Into<CompositeKey>,
    {
        let cks = cks.iter().cloned().map(Into::into).collect();
        let selector = Selector::Many(cks);

        LoadQuery::new(selector)
    }

    // range
    pub fn range<K: Into<CompositeKey>>(self, start: K, end: K) -> LoadQuery {
        let selector = Selector::Range(start.into(), end.into());

        LoadQuery::new(selector)
    }

    // prefix
    pub fn prefix<K: Into<CompositeKey>>(self, prefix: K) -> LoadQuery {
        let selector = Selector::Prefix(prefix.into());

        LoadQuery::new(selector)
    }
}

///
/// LoadQuery
/// fluent methods are handled in LoadQueryInternal
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQuery {
    pub selector: Selector,
    pub format: LoadFormat,
    pub r#where: Option<Where>,
    pub limit: Option<u32>,
    pub offset: u32,
    pub search: Vec<(String, String)>,
    pub sort: Vec<(String, SortDirection)>,
}

impl LoadQuery {
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

    // where
    #[must_use]
    pub fn r#where<W: Into<Where>>(mut self, r#where: W) -> Self {
        self.r#where = Some(r#where.into());
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

    // search
    #[must_use]
    pub fn search<K, V, I>(mut self, search: I) -> Self
    where
        K: Into<String>,
        V: Into<String>,
        I: IntoIterator<Item = (K, V)>,
    {
        self.search = search
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        self
    }

    // search_field
    #[must_use]
    pub fn search_field<K: Into<String>, V: Into<String>>(self, field: K, value: V) -> Self {
        self.search(std::iter::once((field, value)))
    }

    // sort
    #[must_use]
    pub fn sort<T, I>(mut self, sort: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = (T, SortDirection)>,
    {
        self.sort = sort.into_iter().map(|(f, d)| (f.into(), d)).collect();
        self
    }

    // sort_field
    #[must_use]
    pub fn sort_field<K: Into<String>>(self, field: K, dir: SortDirection) -> Self {
        self.sort(std::iter::once((field, dir)))
    }
}
