use crate::{
    node::{
        Canister, Def, Entity, EntityFixture, EntitySource, Enum, EnumHash, EnumValue, Error,
        MacroNode, Map, Newtype, Permission, Primitive, Record, Role, Sanitizer, Store, Tuple,
        ValidateNode, Validator, VisitableNode,
    },
    visit::Visitor,
};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sha2::{Digest, Sha256};
use std::{
    any::{Any, TypeId},
    collections::{BTreeMap, HashSet},
};
use types::{ErrorVec, Timestamp};

///
/// SchemaNode
///

#[remain::sorted]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SchemaNode {
    Canister(Canister),
    Entity(Entity),
    EntityFixture(EntityFixture),
    EntitySource(EntitySource),
    Enum(Enum),
    EnumHash(EnumHash),
    EnumValue(EnumValue),
    Map(Map),
    Newtype(Newtype),
    Permission(Permission),
    Primitive(Primitive),
    Record(Record),
    Role(Role),
    Sanitizer(Sanitizer),
    Store(Store),
    Tuple(Tuple),
    Validator(Validator),
}

impl SchemaNode {
    const fn def(&self) -> &Def {
        match self {
            Self::Canister(n) => &n.def,
            Self::Entity(n) => &n.def,
            Self::EntityFixture(n) => &n.def,
            Self::EntitySource(n) => &n.def,
            Self::Enum(n) => &n.def,
            Self::EnumHash(n) => &n.def,
            Self::EnumValue(n) => &n.def,
            Self::Map(n) => &n.def,
            Self::Newtype(n) => &n.def,
            Self::Permission(n) => &n.def,
            Self::Primitive(n) => &n.def,
            Self::Record(n) => &n.def,
            Self::Role(n) => &n.def,
            Self::Sanitizer(n) => &n.def,
            Self::Store(n) => &n.def,
            Self::Tuple(n) => &n.def,
            Self::Validator(n) => &n.def,
        }
    }
}

impl MacroNode for SchemaNode {
    fn as_any(&self) -> &dyn Any {
        match self {
            Self::Canister(n) => n.as_any(),
            Self::Entity(n) => n.as_any(),
            Self::EntityFixture(n) => n.as_any(),
            Self::EntitySource(n) => n.as_any(),
            Self::Enum(n) => n.as_any(),
            Self::EnumHash(n) => n.as_any(),
            Self::EnumValue(n) => n.as_any(),
            Self::Map(n) => n.as_any(),
            Self::Newtype(n) => n.as_any(),
            Self::Permission(n) => n.as_any(),
            Self::Primitive(n) => n.as_any(),
            Self::Record(n) => n.as_any(),
            Self::Role(n) => n.as_any(),
            Self::Sanitizer(n) => n.as_any(),
            Self::Store(n) => n.as_any(),
            Self::Tuple(n) => n.as_any(),
            Self::Validator(n) => n.as_any(),
        }
    }
}

impl ValidateNode for SchemaNode {}

impl VisitableNode for SchemaNode {
    fn drive<V: Visitor>(&self, v: &mut V) {
        match self {
            Self::Canister(n) => n.accept(v),
            Self::Entity(n) => n.accept(v),
            Self::EntityFixture(n) => n.accept(v),
            Self::EntitySource(n) => n.accept(v),
            Self::Enum(n) => n.accept(v),
            Self::EnumHash(n) => n.accept(v),
            Self::EnumValue(n) => n.accept(v),
            Self::Map(n) => n.accept(v),
            Self::Newtype(n) => n.accept(v),
            Self::Permission(n) => n.accept(v),
            Self::Primitive(n) => n.accept(v),
            Self::Record(n) => n.accept(v),
            Self::Role(n) => n.accept(v),
            Self::Sanitizer(n) => n.accept(v),
            Self::Store(n) => n.accept(v),
            Self::Tuple(n) => n.accept(v),
            Self::Validator(n) => n.accept(v),
        }
    }
}

///
/// Schema
///

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    pub nodes: BTreeMap<String, SchemaNode>,
    pub hash: String,
    pub timestamp: Timestamp,
}

impl Serialize for Schema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize just the data parts to JSON first (exclude the hash)
        let json = serde_json::to_string(&SchemaNodes { nodes: &self.nodes })
            .map_err(serde::ser::Error::custom)?;

        // Compute the hash of the JSON string
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        let hash_result = hasher.finalize();
        let hash_hex = hex::encode(hash_result);

        // Serialize all including the hash
        let mut state = serializer.serialize_struct("Schema", 3)?;
        state.serialize_field("nodes", &self.nodes)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("hash", &hash_hex)?;
        state.end()
    }
}

// SchemaNodes is a serialization helper
#[derive(Serialize)]
struct SchemaNodes<'a> {
    nodes: &'a BTreeMap<String, SchemaNode>,
}

impl Schema {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            hash: String::new(),
            timestamp: Timestamp::now(),
        }
    }

    // add_node
    pub fn add_node(&mut self, node: SchemaNode) {
        self.nodes.insert(node.def().path(), node);
    }

    // check_node
    pub fn check_node<T: 'static>(&self, path: &str) -> Result<(), Error> {
        self.nodes
            .get(path)
            .ok_or_else(|| Error::path_not_found(path))
            .and_then(|node| {
                if node.as_any().type_id() == TypeId::of::<T>() {
                    Ok(())
                } else {
                    Err(Error::incorrect_node_type(path))
                }
            })
    }

    // check_node_types
    // allows you to check to see if the type is within a set
    pub fn check_node_types(
        &self,
        path: &str,
        acceptable_types: &HashSet<TypeId>,
    ) -> Result<(), Error> {
        self.nodes.get(path).map_or_else(
            || Err(Error::path_not_found(path)),
            |node| {
                if acceptable_types.contains(&node.as_any().type_id()) {
                    Ok(())
                } else {
                    Err(Error::incorrect_node_type(path))
                }
            },
        )
    }

    // get_node
    #[must_use]
    pub fn get_node<'a, T: 'static>(&'a self, path: &str) -> Option<&'a T> {
        self.nodes
            .get(path) // This returns Option<&SchemaNode>
            .and_then(|node| node.as_any().downcast_ref::<T>())
    }

    // try_get_node
    // function to retrieve a node of type T, if exists and matches the type
    pub fn try_get_node<'a, T: 'static>(&'a self, path: &str) -> Result<&'a T, Error> {
        self.nodes.get(path).map_or_else(
            || Err(Error::path_not_found(path)),
            |node| {
                node.as_any().downcast_ref::<T>().ok_or_else(|| {
                    if node.as_any().type_id() == TypeId::of::<T>() {
                        Error::downcast_fail(path)
                    } else {
                        Error::incorrect_node_type(path)
                    }
                })
            },
        )
    }

    // get_nodes
    #[must_use]
    pub fn get_nodes<'a, T: 'static>(&'a self) -> Box<dyn Iterator<Item = (&'a str, &'a T)> + 'a> {
        Box::new(self.nodes.iter().filter_map(|(key, node)| {
            node.as_any()
                .downcast_ref::<T>()
                .map(|node| (key.as_str(), node))
        }))
    }

    // get_node_values
    #[must_use]
    pub fn get_node_values<'a, T: 'static>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        Box::new(
            self.nodes
                .values()
                .filter_map(|node| node.as_any().downcast_ref::<T>()),
        )
    }

    // filter_nodes
    // Generic method to filter key, and nodes of any type with a predicate
    pub fn filter_nodes<'a, T: 'static, F>(
        &'a self,
        predicate: F,
    ) -> Box<dyn Iterator<Item = (&'a str, &'a T)> + 'a>
    where
        F: Fn(&T) -> bool + 'a,
    {
        Box::new(self.nodes.iter().filter_map(move |(key, node)| {
            if let Some(target) = node.as_any().downcast_ref::<T>() {
                if predicate(target) {
                    return Some((key.as_str(), target));
                }
            }

            None
        }))
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidateNode for Schema {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // duplicate fixtures for the same entity
        let mut set = HashSet::new();
        for fixture in self.get_node_values::<EntityFixture>() {
            for key in &fixture.keys {
                let map_key = format!("{}-{}", fixture.entity, key);

                if !set.insert(map_key) {
                    errs.add(format!(
                        "entity '{}' has duplicate fixture: {}",
                        fixture.entity, key,
                    ));
                }
            }
        }

        // no two stores can use the same memory_id
        for store in self.get_node_values::<Store>() {
            let mut memory_values = HashSet::new();
            if !memory_values.insert(store.memory_id) {
                errs.add(format!(
                    "duplicate store memory_id value: {}",
                    store.memory_id
                ));
            }
        }

        // canister dir
        let mut dirs_seen = HashSet::new();
        for canister in self.get_node_values::<Canister>() {
            // Check for duplicate names
            if !dirs_seen.insert(canister.name().clone()) {
                errs.push(format!(
                    "duplicate canister name found: {}",
                    canister.name()
                ));
            }
        }

        errs.result()
    }
}

impl VisitableNode for Schema {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in self.nodes.values() {
            node.accept(v);
        }
    }
}
