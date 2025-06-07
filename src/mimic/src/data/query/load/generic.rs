#![allow(clippy::type_complexity)]
use crate::{
    data::query::{LoadFormat, Selector, Where},
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
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQuery {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let selector = Selector::One(ck_str);

        LoadQuery::new(selector)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQuery {
        let selector = Selector::Many(cks.to_vec());

        LoadQuery::new(selector)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQuery {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let selector = Selector::Range(start, end);

        LoadQuery::new(selector)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQuery {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let selector = Selector::Prefix(prefix);

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
