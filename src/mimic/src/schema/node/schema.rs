use crate::{
    schema::{
        node::{
            Constant, Def, Entity, Enum, EnumValue, MacroNode, Map, Newtype, Primitive, Record,
            Selector, Tuple, TypeNode, ValidateNode, Validator, VisitableNode,
        },
        visit::Visitor,
        SchemaError,
    },
    Error,
};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sha2::{Digest, Sha256};
use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
};

///
/// SchemaNode
///

#[remain::sorted]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SchemaNode {
    Constant(Constant),
    Entity(Entity),
    Enum(Enum),
    EnumValue(EnumValue),
    Map(Map),
    Newtype(Newtype),
    Primitive(Primitive),
    Record(Record),
    Selector(Selector),
    Tuple(Tuple),
    Validator(Validator),
}

impl SchemaNode {
    #[must_use]
    pub fn get_type(&self) -> Option<Box<dyn TypeNode>> {
        match self {
            SchemaNode::Entity(ref n) => Some(Box::new(n.clone())),
            SchemaNode::Enum(ref n) => Some(Box::new(n.clone())),
            SchemaNode::EnumValue(ref n) => Some(Box::new(n.clone())),
            SchemaNode::Map(ref n) => Some(Box::new(n.clone())),
            SchemaNode::Newtype(ref n) => Some(Box::new(n.clone())),
            SchemaNode::Primitive(ref n) => Some(Box::new(n.clone())),
            SchemaNode::Record(ref n) => Some(Box::new(n.clone())),
            SchemaNode::Tuple(ref n) => Some(Box::new(n.clone())),
            _ => None,
        }
    }
}

impl SchemaNode {
    const fn def(&self) -> &Def {
        match self {
            Self::Constant(n) => &n.def,
            Self::Entity(n) => &n.def,
            Self::Enum(n) => &n.def,
            Self::EnumValue(n) => &n.def,
            Self::Map(n) => &n.def,
            Self::Newtype(n) => &n.def,
            Self::Primitive(n) => &n.def,
            Self::Record(n) => &n.def,
            Self::Selector(n) => &n.def,
            Self::Tuple(n) => &n.def,
            Self::Validator(n) => &n.def,
        }
    }
}

impl MacroNode for SchemaNode {
    fn as_any(&self) -> &dyn Any {
        match self {
            Self::Constant(n) => n.as_any(),
            Self::Entity(n) => n.as_any(),
            Self::Enum(n) => n.as_any(),
            Self::EnumValue(n) => n.as_any(),
            Self::Map(n) => n.as_any(),
            Self::Newtype(n) => n.as_any(),
            Self::Primitive(n) => n.as_any(),
            Self::Record(n) => n.as_any(),
            Self::Selector(n) => n.as_any(),
            Self::Tuple(n) => n.as_any(),
            Self::Validator(n) => n.as_any(),
        }
    }
}

impl ValidateNode for SchemaNode {}

impl VisitableNode for SchemaNode {
    fn drive<V: Visitor>(&self, v: &mut V) {
        match self {
            Self::Constant(n) => n.accept(v),
            Self::Entity(n) => n.accept(v),
            Self::Enum(n) => n.accept(v),
            Self::EnumValue(n) => n.accept(v),
            Self::Map(n) => n.accept(v),
            Self::Newtype(n) => n.accept(v),
            Self::Primitive(n) => n.accept(v),
            Self::Record(n) => n.accept(v),
            Self::Selector(n) => n.accept(v),
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
    pub timestamp: u64,
}

impl Serialize for Schema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize just the nodes part to JSON
        let nodes_json = serde_json::to_string(&self.nodes).map_err(serde::ser::Error::custom)?;

        // Compute the hash of the nodes JSON string
        let mut hasher = Sha256::new();
        hasher.update(nodes_json.as_bytes());
        let hash_result = hasher.finalize();
        let hash_hex = hex::encode(hash_result);

        // Serialize the Schema struct, including the hash
        let mut state = serializer.serialize_struct("Schema", 3)?;
        state.serialize_field("nodes", &self.nodes)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("hash", &hash_hex)?;
        state.end()
    }
}

impl Schema {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            hash: String::new(),
            timestamp: crate::utils::time::now_secs(),
        }
    }

    // insert_node
    pub fn insert_node(&mut self, node: SchemaNode) {
        self.nodes.insert(node.def().path(), node);
    }

    // get_node
    #[must_use]
    pub fn get_node<'a>(&'a self, path: &str) -> Option<&'a SchemaNode> {
        self.nodes.get(path)
    }

    // try_get_node
    pub fn try_get_node<'a>(&'a self, path: &str) -> Result<&'a SchemaNode, Error> {
        let node = self
            .nodes
            .get(path)
            .ok_or_else(|| SchemaError::PathNotFound(path.to_string()))?;

        Ok(node)
    }

    // get_node_as
    #[must_use]
    pub fn get_node_as<'a, T: 'static>(&'a self, path: &str) -> Option<&'a T> {
        self.nodes
            .get(path)
            .and_then(|node| node.as_any().downcast_ref::<T>())
    }

    // check_node_as
    pub fn check_node_as<T: 'static>(&self, path: &str) -> Result<(), Error> {
        self.try_cast_node::<T>(path).map(|_| ())
    }

    // try_cast_node
    // attempts to downcast the node to the specified type `T` and returns Result
    pub fn try_cast_node<'a, T: 'static>(&'a self, path: &str) -> Result<&'a T, Error> {
        let node = self
            .nodes
            .get(path)
            .and_then(|node| node.as_any().downcast_ref::<T>())
            .ok_or_else(|| {
                let path = path.to_string();

                if let Some(node) = self.nodes.get(&path) {
                    if node.as_any().type_id() == TypeId::of::<T>() {
                        SchemaError::DowncastFail(path)
                    } else {
                        SchemaError::IncorrectNodeType(path)
                    }
                } else {
                    SchemaError::PathNotFound(path)
                }
            })?;

        Ok(node)
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

impl ValidateNode for Schema {}

impl VisitableNode for Schema {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in self.nodes.values() {
            node.accept(v);
        }
    }
}
