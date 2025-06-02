#![allow(clippy::type_complexity)]
use crate::{
    db::types::{DataRow, EntityRow, EntityValue, SortKey},
    query::{
        Selector,
        load::{LoadCollectionDyn, LoadFormat, LoadMap, LoadResponse},
    },
    schema::types::SortDirection,
    traits::Entity,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

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
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> LoadQueryBuilder<E>
where
    E: Entity,
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
        LoadQueryInternal::from_selector(selector)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryInternal<E> {
        LoadQueryInternal::from_selector(Selector::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryInternal<E> {
        LoadQueryInternal::from_selector(Selector::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryInternal<E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let selector = Selector::One(ck_str);

        LoadQueryInternal::from_selector(selector)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryInternal<E> {
        let selector = Selector::Many(cks.to_vec());

        LoadQueryInternal::from_selector(selector)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQueryInternal<E> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let selector = Selector::Range(start, end);

        LoadQueryInternal::from_selector(selector)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQueryInternal<E> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let selector = Selector::Prefix(prefix);

        LoadQueryInternal::from_selector(selector)
    }
}

///
/// LoadQueryInternal
///

pub struct LoadQueryInternal<E>
where
    E: Entity,
{
    pub inner: LoadQuery,
    pub filters: Vec<Box<dyn Fn(&E) -> bool>>,
}

impl<E: Entity> LoadQueryInternal<E> {
    // new
    #[must_use]
    pub fn new(inner: LoadQuery) -> Self {
        Self {
            inner,
            filters: vec![],
        }
    }

    // from_selector
    #[must_use]
    pub fn from_selector(selector: Selector) -> Self {
        let inner = LoadQuery::new(selector);

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
    pub fn query(&self) -> &LoadQuery {
        &self.inner
    }

    // filters
    #[must_use]
    pub fn filters(&self) -> &[Box<dyn Fn(&E) -> bool>] {
        &self.filters
    }

    // format
    #[must_use]
    pub fn format(mut self, format: LoadFormat) -> Self {
        self.inner.format = format;
        self
    }

    // offset
    #[must_use]
    pub fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.inner.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub fn limit_option(mut self, limit: Option<u32>) -> Self {
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

///
/// LoadCollection
///

#[derive(Debug)]
pub struct LoadCollection<E: Entity>(pub Vec<EntityRow<E>>);

impl<E> LoadCollection<E>
where
    E: Entity,
{
    // response
    #[must_use]
    pub fn response(self, format: LoadFormat) -> LoadResponse {
        match format {
            LoadFormat::Rows => LoadResponse::Rows(self.data_rows()),
            LoadFormat::Keys => LoadResponse::Keys(self.keys()),
            LoadFormat::Count => LoadResponse::Count(self.count()),
        }
    }

    // as_dyn
    #[must_use]
    pub fn as_dyn(self) -> LoadCollectionDyn {
        let data_rows: Vec<DataRow> = self
            .0
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect();

        LoadCollectionDyn(data_rows)
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
        self.as_dyn().data_row()
    }

    // data_rows
    #[must_use]
    pub fn data_rows(self) -> Vec<DataRow> {
        self.as_dyn().data_rows()
    }

    // blob
    #[must_use]
    pub fn blob(self) -> Option<Vec<u8>> {
        self.as_dyn().blob()
    }

    // blobs
    #[must_use]
    pub fn blobs(self) -> Vec<Vec<u8>> {
        self.as_dyn().blobs()
    }

    // map
    #[must_use]
    pub fn map(self) -> LoadMap<EntityValue<E>> {
        let pairs = self
            .0
            .into_iter()
            .map(|row| (row.key.into(), row.value))
            .collect::<Vec<_>>();

        LoadMap::from_pairs(pairs)
    }

    // entity
    #[must_use]
    pub fn entity(self) -> Option<E> {
        self.0.first().map(|row| row.value.entity.clone())
    }

    // entities
    #[must_use]
    pub fn entities(self) -> Vec<E> {
        self.0.iter().map(|row| row.value.entity.clone()).collect()
    }

    // entity_row
    #[must_use]
    pub fn entity_row(self) -> Option<EntityRow<E>> {
        self.0.first().cloned()
    }

    // entity_rows
    #[must_use]
    pub fn entity_rows(self) -> Vec<EntityRow<E>> {
        self.0
    }
}
