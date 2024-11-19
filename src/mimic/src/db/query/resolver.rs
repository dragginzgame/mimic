use crate::{core::schema::get_schema, db::types::DataKey, orm::schema::node::Entity};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("entity not found: {path}"))]
    EntityNotFound { path: String },

    #[snafu(transparent)]
    Schema { source: crate::core::schema::Error },
}

impl Error {
    #[must_use]
    pub fn entity_not_found(path: &str) -> Self {
        Self::EntityNotFound {
            path: path.to_string(),
        }
    }
}

///
/// Resolver
///
/// generates a sort key using the Entity's path
///

pub struct Resolver {
    pub entity: String,
}

impl Resolver {
    #[must_use]
    pub fn new(entity: &str) -> Self {
        Self {
            entity: entity.to_string(),
        }
    }

    // store
    pub fn store(&self) -> Result<String, Error> {
        let schema = get_schema().map_err(Error::from)?;
        let entity = schema
            .get_node::<Entity>(&self.entity)
            .ok_or_else(|| Error::entity_not_found(&self.entity))?;

        Ok(entity.store.clone())
    }

    // data_key
    pub fn data_key(&self, ck: &[String]) -> Result<DataKey, Error> {
        let chain_format = self.chain_format()?;

        // Initialize an empty vector to store parts that have keys
        let mut data_key_parts: Vec<(String, Vec<String>)> = Vec::new();
        let mut index = 0;

        for (i, (part, count)) in chain_format.iter().enumerate() {
            let available_count = if index < ck.len() {
                usize::min(*count, ck.len() - index)
            } else {
                0
            };

            // Add the new part if it's the root part of the chain, or if
            // it isn't and we have keys
            if i == 0 || available_count > 0 {
                let part_keys = ck[index..index + available_count].to_vec();

                data_key_parts.push((part.clone(), part_keys));
            };

            // Stop processing after consuming all keys
            index += available_count;
        }

        Ok(DataKey::new(data_key_parts))
    }

    // chain_format
    // returns the data used to format the sort key
    fn chain_format(&self) -> Result<Vec<(String, usize)>, Error> {
        let schema = get_schema().map_err(Error::from)?;

        //
        // Create the chain from the Schema
        //
        let entity = schema
            .get_node::<Entity>(&self.entity)
            .ok_or_else(|| Error::entity_not_found(&self.entity))?;

        // create an ordered vec from the parents
        let mut chain = Vec::new();
        for sk in &entity.sort_keys {
            let sk_entity = schema
                .get_node::<Entity>(&sk.entity)
                .ok_or_else(|| Error::entity_not_found(&sk.entity))?;

            chain.push(sk_entity);
        }
        chain.push(entity);

        //
        // format the chain
        //

        let mut format = Vec::new();
        for (i, entity) in chain.into_iter().enumerate() {
            let num_keys = entity.primary_keys.len();
            let part = if i == 0 {
                entity.def.path()
            } else {
                entity.def.ident.to_string()
            };

            format.push((part, num_keys));
        }

        Ok(format)
    }
}
