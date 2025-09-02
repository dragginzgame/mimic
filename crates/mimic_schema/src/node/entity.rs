use crate::{
    build::schema_read,
    node::{
        Def, Field, FieldList, Index, MacroNode, Store, Type, TypeNode, ValidateNode, VisitableNode,
    },
    types::{Cardinality, StoreType},
    visit::Visitor,
};
use mimic_common::{err, error::ErrorTree};
use serde::Serialize;

///
/// Entity
///

#[derive(Clone, Debug, Serialize)]
pub struct Entity {
    pub def: Def,
    pub store: &'static str,
    pub primary_key: &'static str,

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub indexes: &'static [Index],

    pub fields: FieldList,
    pub ty: Type,
}

impl Entity {
    #[must_use]
    pub fn get_pk_field(&self) -> &Field {
        self.fields
            .get(self.primary_key)
            .unwrap_or_else(|| panic!("missing primary key field '{}'", self.primary_key))
    }
}

impl MacroNode for Entity {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for Entity {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for Entity {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let schema = schema_read();

        // store
        match schema.try_get_node_as::<Store>(self.store) {
            Ok(store) if !matches!(store.ty, StoreType::Data) => {
                err!(errs, "store is not type Data");
            }
            Ok(_) => {}
            Err(e) => errs.add(e),
        }

        // Load and validate index references
        let mut resolved_indexes = Vec::new();

        // check indexes have proper fields
        for index in self.indexes {
            // Check all fields in the index exist on the entity
            for field in index.fields {
                if let Some(field) = self.fields.get(field) {
                    if field.value.cardinality == Cardinality::Many {
                        err!(errs, "cannot add an index field with many cardinality");
                    }
                } else {
                    err!(errs, "index field '{field}' not found");
                }
            }
            resolved_indexes.push(index);
        }

        // Check for redundant indexes (prefix relationships)
        for (i, a) in resolved_indexes.iter().enumerate() {
            for b in resolved_indexes.iter().skip(i + 1) {
                if a.unique == b.unique {
                    if a.is_prefix_of(b) {
                        err!(
                            errs,
                            "index {:?} is redundant (prefix of {:?})",
                            a.fields,
                            b.fields
                        );
                    } else if b.is_prefix_of(a) {
                        err!(
                            errs,
                            "index {:?} is redundant (prefix of {:?})",
                            b.fields,
                            a.fields
                        );
                    }
                }
            }
        }

        errs.result()
    }
}

impl VisitableNode for Entity {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.fields.accept(v);
        self.ty.accept(v);
    }
}
