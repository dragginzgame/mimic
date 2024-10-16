use crate::types::DataKey;
use core_schema::get_schema;
use orm_schema::node::Entity;
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
    Schema { source: core_schema::Error },
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
/// based on the entity path knows the store and how to generate
/// a sort key
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

        // Initialize data_key_parts with empty vectors for each part
        let mut data_key_parts: Vec<(String, Vec<String>)> = chain_format
            .iter()
            .map(|(part, _)| (part.clone(), Vec::new()))
            .collect();

        // Fill the parts with keys from ck as available
        let mut index = 0;
        for (part_keys, (_, count)) in data_key_parts.iter_mut().zip(chain_format.iter()) {
            if index + count >= ck.len() {
                break; // not enough keys
            }
            part_keys.1.extend_from_slice(&ck[index..index + count]);
            index += count;
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
