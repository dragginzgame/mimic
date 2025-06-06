#![allow(clippy::type_complexity)]
use crate::{
    data::query::{Selector, load::LoadFormat},
    schema::types::SortDirection,
    traits::EntityKind,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{fmt, marker::PhantomData};

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQuery {
    pub selector: Selector,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
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
}

///
/// LoadQueryBuilder
///

#[derive(Debug, Default)]
pub struct LoadQueryBuilder<E>
where
    E: EntityKind,
{
    phantom: PhantomData<E>,
}

impl<E> LoadQueryBuilder<E>
where
    E: EntityKind,
{
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // query
    #[must_use]
    pub fn query(self, query: LoadQuery) -> LoadQueryInternal<E> {
        LoadQueryInternal::new(query)
    }

    // selector
    #[must_use]
    pub fn selector(self, selector: Selector) -> LoadQueryInternal<E> {
        LoadQueryInternal::new(LoadQuery::new(selector))
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryInternal<E> {
        LoadQueryInternal::new(LoadQuery::new(Selector::All))
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryInternal<E> {
        LoadQueryInternal::new(LoadQuery::new(Selector::Only))
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryInternal<E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let selector = Selector::One(ck_str);

        LoadQueryInternal::new(LoadQuery::new(selector))
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryInternal<E> {
        let selector = Selector::Many(cks.to_vec());

        LoadQueryInternal::new(LoadQuery::new(selector))
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQueryInternal<E> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let selector = Selector::Range(start, end);

        LoadQueryInternal::new(LoadQuery::new(selector))
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQueryInternal<E> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let selector = Selector::Prefix(prefix);

        LoadQueryInternal::new(LoadQuery::new(selector))
    }
}

///
/// LoadQueryInternal
///

pub struct LoadQueryInternal<E>
where
    E: EntityKind,
{
    pub inner: LoadQuery,
    pub filters: Vec<Box<dyn Fn(&E) -> bool>>,
}

impl<E: EntityKind> LoadQueryInternal<E> {
    // new
    #[must_use]
    pub fn new(inner: LoadQuery) -> Self {
        Self {
            inner,
            filters: vec![],
        }
    }

    // build
    #[must_use]
    pub fn build(self) -> (LoadQuery, Vec<Box<dyn Fn(&E) -> bool>>) {
        (self.inner, self.filters)
    }

    // query
    #[must_use]
    pub const fn query(&self) -> &LoadQuery {
        &self.inner
    }

    // filters
    #[must_use]
    pub fn filters(&self) -> &[Box<dyn Fn(&E) -> bool>] {
        &self.filters
    }

    // format
    #[must_use]
    pub const fn format(mut self, format: LoadFormat) -> Self {
        self.inner.format = format;
        self
    }

    // offset
    #[must_use]
    pub const fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.inner.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub const fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.inner.limit = limit;
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
        self.inner.search = search
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
        self.inner.sort = sort.into_iter().map(|(f, d)| (f.into(), d)).collect();
        self
    }

    // sort_field
    #[must_use]
    pub fn sort_field<K: Into<String>>(self, field: K, dir: SortDirection) -> Self {
        self.sort(std::iter::once((field, dir)))
    }

    // filter
    #[must_use]
    pub fn filter<F: Fn(&E) -> bool + 'static>(mut self, f: F) -> Self {
        self.filters.push(Box::new(f));
        self
    }

    // filter_eq
    #[must_use]
    pub fn filter_eq<F, T>(self, f: F, expected: T) -> Self
    where
        F: Fn(&E) -> T + 'static,
        T: PartialEq + 'static,
    {
        self.filter(move |e| f(e) == expected)
    }

    // filter_some_eq
    #[must_use]
    pub fn filter_some_eq<F, T>(self, f: F, value: T) -> Self
    where
        F: Fn(&E) -> Option<T> + 'static,
        T: PartialEq + 'static,
    {
        self.filter(move |e| f(e).as_ref() == Some(&value))
    }
}

impl<E> fmt::Debug for LoadQueryInternal<E>
where
    E: EntityKind,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoadQueryInternal")
            .field("inner", &self.inner)
            .field("filters", &format_args!("{} filters", self.filters.len()))
            .finish()
    }
}
