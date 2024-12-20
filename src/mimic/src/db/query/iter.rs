use crate::db::{
    query::{
        types::{EntityRow, Filter, Order, QueryRow},
        Error as QueryError,
    },
    types::{DataKey, DataRow},
};
use crate::orm::traits::Entity;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::iter;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("no results found"))]
    NoResultsFound,
}

///
/// IterFilter
///

type IterFilter<T> = Box<dyn Fn(&T) -> bool>;

///
/// RowIterator
///

pub struct RowIterator<E>
where
    E: Entity,
{
    iter: Box<dyn Iterator<Item = EntityRow<E>>>,
    manager: IterManager<EntityRow<E>>,
}

impl<E> RowIterator<E>
where
    E: Entity + 'static,
{
    // new
    #[must_use]
    pub fn new(
        iter: Box<dyn Iterator<Item = EntityRow<E>>>,
        limit: Option<u32>,
        offset: u32,
        filter: Option<Filter>,
        order: Option<Order>,
    ) -> Self {
        // sorting?
        // if we have a specific order we need to collect and rebuild the iter
        let iter = if let Some(order) = order {
            let mut rows: Vec<EntityRow<E>> = iter.collect();
            let sorter = E::sort(&order);

            rows.sort_by(|a, b| sorter(&a.value.entity, &b.value.entity));

            Box::new(rows.into_iter()) as Box<dyn Iterator<Item = EntityRow<E>>>
        } else {
            iter
        };

        // Map the optional Filter struct to an optional closure
        let filter_closure = filter.map(|filter| {
            Box::new(move |row: &EntityRow<E>| {
                // Apply the captured filter criteria to each EntityRow<E>
                match &filter {
                    Filter::All(text) => row.value.entity.filter_all(text),
                    Filter::Fields(fields) => row.value.entity.filter_fields(fields.clone()),
                }
            }) as Box<dyn Fn(&EntityRow<E>) -> bool>
        });

        // Build IterManager
        let manager = IterManager::new(limit, offset, filter_closure);

        Self { iter, manager }
    }

    // move_next
    // Move to the next row, applying the filter
    fn move_next(&mut self) -> Option<EntityRow<E>> {
        self.iter
            .by_ref()
            .find(|row| self.manager.should_return(row))
    }

    // key
    pub fn key(mut self) -> Result<DataKey, QueryError> {
        let row = self.move_next().ok_or(Error::NoResultsFound)?;

        Ok(row.key)
    }

    // keys
    pub fn keys(mut self) -> impl Iterator<Item = DataKey> {
        iter::from_fn(move || self.move_next().map(|row| row.key))
    }

    // entity
    pub fn entity(mut self) -> Result<E, QueryError> {
        let res = self
            .move_next()
            .as_ref()
            .map(|row| row.value.entity.clone())
            .ok_or(Error::NoResultsFound)
            .map_err(Error::from)?;

        Ok(res)
    }

    // entities
    pub fn entities(mut self) -> impl Iterator<Item = E> {
        iter::from_fn(move || {
            self.move_next()
                .as_ref()
                .map(|row| row.value.entity.clone())
        })
    }

    // entity_row
    pub fn entity_row(mut self) -> Result<EntityRow<E>, QueryError> {
        let res = self
            .move_next()
            .ok_or(Error::NoResultsFound)
            .map_err(Error::from)?;

        Ok(res)
    }

    // entity_rows
    pub fn entity_rows(mut self) -> impl Iterator<Item = EntityRow<E>> {
        iter::from_fn(move || self.move_next())
    }
}

impl<E> Iterator for RowIterator<E>
where
    E: Entity + 'static,
{
    type Item = EntityRow<E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.move_next()
    }
}

///
/// RowIteratorDyn
///
/// complex logic is handled better with iter::from_fn and move_next()
/// all iterator methods (for now) are consuming
///

pub struct RowIteratorDyn {
    iter: Box<dyn Iterator<Item = DataRow>>,
    manager: IterManager<DataRow>,
}

impl RowIteratorDyn {
    #[must_use]
    pub fn new(iter: Box<dyn Iterator<Item = DataRow>>, limit: Option<u32>, offset: u32) -> Self {
        Self {
            iter,
            manager: IterManager::new(limit, offset, None),
        }
    }

    // move_next
    fn move_next(&mut self) -> Option<DataRow> {
        self.iter
            .by_ref()
            .find(|row| self.manager.should_return(row))
    }

    // results
    pub fn results(mut self) -> impl Iterator<Item = DataRow> {
        iter::from_fn(move || self.move_next())
    }

    // key
    pub fn key(mut self) -> Result<String, QueryError> {
        let row = self.move_next().ok_or(Error::NoResultsFound)?;

        Ok(row.key.to_string())
    }

    // keys
    pub fn keys(mut self) -> impl Iterator<Item = String> {
        iter::from_fn(move || self.move_next().map(|row| row.key.to_string()))
    }

    // query_row
    pub fn query_row(mut self) -> Result<QueryRow, QueryError> {
        let row = self.move_next().ok_or(Error::NoResultsFound)?;

        Ok(row.into())
    }

    // query_rows
    pub fn query_rows(mut self) -> impl Iterator<Item = QueryRow> {
        iter::from_fn(move || self.move_next().map(Into::into))
    }

    // blob
    pub fn blob(mut self) -> Result<Vec<u8>, QueryError> {
        let blob = self
            .iter
            .next()
            .map(|row| row.value.data)
            .ok_or(Error::NoResultsFound)?;

        Ok(blob)
    }

    // blobs
    pub fn blobs(mut self) -> impl Iterator<Item = Vec<u8>> {
        iter::from_fn(move || self.move_next().map(|row| row.value.data))
    }
}

impl Iterator for RowIteratorDyn {
    type Item = DataRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.move_next()
    }
}

///
/// IterManager
///

struct IterManager<T> {
    limit: Option<u32>,
    offset: u32,
    filter: Option<IterFilter<T>>,
    rows_offset: u32,
    rows_processed: u32,
}

impl<T> IterManager<T> {
    pub const fn new(limit: Option<u32>, offset: u32, filter: Option<IterFilter<T>>) -> Self {
        Self {
            limit,
            offset,
            filter,
            rows_offset: 0,
            rows_processed: 0,
        }
    }

    pub fn should_return(&mut self, item: &T) -> bool {
        // First, check if the item passes the filter (if any filter is set)
        // Skip the item if it doesn't pass the filter
        if let Some(ref filter) = self.filter {
            if !filter(item) {
                return false;
            }
        }

        // Apply offset: skip processing for a number of rows equal to the offset
        if self.rows_offset < self.offset {
            self.rows_offset += 1;
            return false;
        }

        // Apply limit: only process up to the limit of items after the offset
        if self.limit.map_or(false, |lim| self.rows_processed >= lim) {
            return false;
        }

        // If the item passes the filter and is within the offset and limit
        self.rows_processed += 1;

        true
    }
}
