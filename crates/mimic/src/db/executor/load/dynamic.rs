use crate::{
    Error,
    db::{
        DataError,
        executor::Loader,
        query::{LoadFormat, LoadQueryDyn},
        response::{LoadCollectionDyn, LoadResponse},
        store::{DataStoreRegistry, IndexStoreRegistry},
        types::DataRow,
    },
    debug,
    ops::traits::EntityKind,
};

///
/// LoadExecutorDyn
///

pub struct LoadExecutorDyn {
    data_registry: DataStoreRegistry,
    index_registry: IndexStoreRegistry,
    debug: bool,
}

impl LoadExecutorDyn {
    // new
    #[must_use]
    pub fn new(data_registry: DataStoreRegistry, index_registry: IndexStoreRegistry) -> Self {
        Self {
            data_registry,
            index_registry,
            debug: false,
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug = false;
        self
    }

    // execute
    pub fn execute<E: EntityKind>(self, query: LoadQueryDyn) -> Result<LoadCollectionDyn, Error> {
        let cl = self.execute_internal::<E>(query)?;

        Ok(cl)
    }

    // execute_response
    pub fn execute_response<E: EntityKind>(
        self,
        query: LoadQueryDyn,
    ) -> Result<LoadResponse, Error> {
        let format = query.format;
        let cl = self.execute_internal::<E>(query)?;

        let resp = match format {
            LoadFormat::Rows => LoadResponse::Rows(cl.data_rows()),
            LoadFormat::Keys => LoadResponse::Keys(cl.keys()),
            LoadFormat::Count => LoadResponse::Count(cl.count()),
        };

        Ok(resp)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        self,
        query: LoadQueryDyn,
    ) -> Result<LoadCollectionDyn, DataError> {
        debug!(self.debug, "query.load_dyn: {query:?}");

        // resolver
        // let resolved_entity = resolve_entity::<E>()?;

        // do we include a row?
        fn include_row(row: &DataRow, query: &LoadQueryDyn, path: &str) -> bool {
            if query.include_children {
                row.value.path.starts_with(path)
            } else {
                row.value.path == path
            }
        }

        // loader
        // no where, search, sort
        // but we have to filter by the fn above and paginate
        let loader = Loader::new(self.data_registry, self.index_registry, self.debug);
        let rows = loader
            .load::<E>(&query.selector, None)?
            .into_iter()
            .filter(|row| include_row(row, &query, E::PATH))
            .skip(query.offset as usize)
            .take(query.limit.unwrap_or(u32::MAX) as usize)
            .collect();

        Ok(LoadCollectionDyn(rows))
    }
}
