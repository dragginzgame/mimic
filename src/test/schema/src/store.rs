use base::types;
use mimic::orm::prelude::*;

///
/// CreateBasic
///

#[entity(
    store = "crate::Store",
    pks = "id",
    fields(field(name = "id", value(item(is = "types::Ulid"))))
)]
pub struct CreateBasic {}

///
/// Filterable
///

#[entity(
    store = "crate::Store",
    pks = "id",
    fields(
        field(name = "id", value(item(is = "types::Ulid"))),
        field(name = "name", value(item(is = "types::String"))),
        field(name = "description", value(item(is = "types::String"))),
    )
)]
pub struct Filterable {}

///
/// Limit
///

#[entity(
    store = "crate::Store",
    pks = "value",
    fields(field(name = "value", value(item(is = "types::U32"))))
)]
pub struct Limit {}

///
/// SortKeyOrder
///

#[cfg(not(target_arch = "wasm32"))]
#[::mimic::export::ctor::ctor]
fn ctor_8985972516518343339() {
    ::mimic::orm::schema::build::schema_write().add_node(
        ::mimic::orm::schema::node::SchemaNode::Entity(::mimic::orm::schema::node::Entity {
            def: ::mimic::orm::schema::node::Def {
                module_path: module_path!().to_string(),
                comments: "SortKeyOrder".to_string(),
                ident: "SortKeyOrder".to_string(),
            },
            store: <crate::Store as ::mimic::orm::traits::Path>::path().to_string(),
            sort_keys: vec![],
            fields: ::mimic::orm::schema::node::FieldList {
                fields: vec![::mimic::orm::schema::node::Field {
                    name: "id".to_string(),
                    value: ::mimic::orm::schema::node::Value {
                        cardinality: ::mimic::orm::schema::types::Cardinality::One,
                        item: ::mimic::orm::schema::node::Item {
                            indirect: true,
                            is: Some(
                                <types::Ulid as ::mimic::orm::traits::Path>::path().to_string(),
                            ),
                            relation: None,
                        },
                        default: None,
                    },
                }],
                order: vec![],
            },
            primary_keys: vec!["id".to_string()],
            crud: None,
        }),
    );
}
#[derive(
    Clone,
    Debug,
    Default,
    :: serde :: Deserialize,
    Eq,
    PartialEq,
    ::
serde :: Serialize,
    :: candid :: CandidType,
)]
#[serde(default)]
pub struct SortKeyOrder {
    pub id: Box<types::Ulid>,
}
impl ::mimic::orm::traits::Orderable for SortKeyOrder {}
impl ::mimic::orm::traits::Entity for SortKeyOrder {
    fn composite_key(values: &[String]) -> Result<Vec<::std::string::String>, ::mimic::orm::Error> {
        let mut this = Self::default();
        if let Some(value) = values.get(0usize) {
            *this.id = value
                .parse()
                .map_err(|_| ::mimic::orm::Error::parse_field("id"))?;
        }
        let keys = vec![::mimic::orm::traits::PrimaryKey::format(&*this.id)];
        let limited_keys = keys.into_iter().take(values.len()).collect::<Vec<_>>();
        Ok(limited_keys)
    }
}
impl ::mimic::orm::traits::EntityDynamic for SortKeyOrder {
    fn on_create(&mut self) {
        *self.id = ::mimic::orm::traits::PrimaryKey::on_create(&*self.id);
    }
    fn composite_key_dyn(&self) -> Vec<::std::string::String> {
        vec![::mimic::orm::traits::PrimaryKey::format(&*self.id)]
    }
    fn path_dyn(&self) -> String {
        <Self as ::mimic::orm::traits::Path>::path()
    }
    fn serialize_dyn(&self) -> Result<Vec<u8>, ::mimic::orm::Error> {
        ::mimic::orm::serialize(&self)
    }
}
impl ::mimic::orm::traits::FieldFilter for SortKeyOrder {
    fn list_fields(&self) -> &'static [&'static str] {
        static FIELDS: [&str; 1usize] = ["id"];
        &FIELDS
    }
    fn filter_field(&self, field: &str, text: &str) -> bool {
        match field {
            "id" => {
                if ::mimic::orm::traits::Filterable::contains_text(&*self.id, text) {
                    return true;
                }
            }
            _ => {}
        }
        false
    }
}
impl ::mimic::orm::traits::FieldSort for SortKeyOrder {
    fn default_order() -> Vec<(String, ::mimic::orm::types::SortDirection)> {
        vec![]
    }
    fn generate_sorter(
        order: &[(String, ::mimic::orm::types::SortDirection)],
    ) -> Box<dyn Fn(&Self, &Self) -> ::std::cmp::Ordering> {
        let mut funcs: Vec<Box<dyn Fn(&Self, &Self) -> ::std::cmp::Ordering>> = Vec::new();
        for (field, direction) in order {
            match field.as_str() {
                "id" => {
                    if matches!(direction, ::mimic::orm::types::SortDirection::Asc) {
                        funcs.push(Box::new(|a, b| {
                            ::mimic::orm::traits::Orderable::cmp(&*a.id, &*b.id)
                        }));
                    } else {
                        funcs.push(Box::new(|a, b| {
                            ::mimic::orm::traits::Orderable::cmp(&*b.id, &*a.id)
                        }));
                    }
                }
                _ => (),
            }
        }
        Box::new(move |a, b| {
            for func in &funcs {
                let result = func(a, b);
                if result != ::std::cmp::Ordering::Equal {
                    return result;
                }
            }
            ::std::cmp::Ordering::Equal
        })
    }
}
impl ::mimic::orm::traits::Filterable for SortKeyOrder {}
impl ::mimic::orm::traits::Path for SortKeyOrder {
    const IDENT: &'static str = "SortKeyOrder";
    const PATH: &'static str = concat!(module_path!(), "::", "SortKeyOrder");
}
impl ::mimic::orm::traits::SanitizeManual for SortKeyOrder {}
impl ::mimic::orm::traits::SanitizeAuto for SortKeyOrder {}
impl ::mimic::orm::traits::ValidateManual for SortKeyOrder {}
impl ::mimic::orm::traits::ValidateAuto for SortKeyOrder {}
impl ::mimic::orm::traits::Visitable for SortKeyOrder {
    fn drive(&self, visitor: &mut dyn ::mimic::orm::visit::Visitor) {
        ::mimic::orm::visit::perform_visit(visitor, &*self.id, "id");
    }
    fn drive_mut(&mut self, visitor: &mut dyn ::mimic::orm::visit::Visitor) {
        ::mimic::orm::visit::perform_visit_mut(visitor, &mut *self.id, "id");
    }
}

///
/// SortKeyA
///

#[entity(
    store = "crate::Store",
    pks = "a_id",
    fields(field(name = "a_id", value(item(is = "types::Ulid"))))
)]
pub struct SortKeyA {}

///
/// SortKeyB
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", fields = "a_id"),
    pks = "b_id, c_id",
    fields(
        field(name = "a_id", value(item(rel = "SortKeyA"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
        field(name = "c_id", value(item(is = "types::Ulid"))),
    )
)]
pub struct SortKeyB {}

///
/// SortKeyC
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", fields = "a_id"),
    sk(entity = "SortKeyB", fields = "b_id, c_id"),
    pks = "d_id, e_id, f_id",
    fields(
        field(name = "a_id", value(item(rel = "SortKeyA"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
        field(name = "c_id", value(item(is = "types::Ulid"))),
        field(name = "d_id", value(item(is = "types::Ulid"))),
        field(name = "e_id", value(item(is = "types::Ulid"))),
        field(name = "f_id", value(item(is = "types::Ulid"))),
    )
)]
pub struct SortKeyC {}

///
/// MissingFieldSmall
///

#[record(
    fields(
        field(name = "a_id", value(item(is = "types::Ulid"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
    ),
    traits(add(Default))
)]
pub struct MissingFieldSmall {}

///
/// MissingFieldLarge
///

#[record(
    fields(
        field(name = "a_id", value(item(is = "types::Ulid"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
        field(name = "c_id", value(item(is = "types::Ulid"))),
    ),
    traits(add(Default))
)]
pub struct MissingFieldLarge {}
