use mimic::orm::{base::types, prelude::*};

///
/// ComplexEntity
///

#[entity(
    store = "crate::Store",
    sk(entity = "ComplexEntity", field = "id"),
    fields(
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "string_test", value(item(is = "types::String"))),
        field(name = "principal_test", value(item(is = "types::Principal"))),
        field(name = "blob_test", value(item(is = "types::Blob"))),
        field(name = "int_8", value(item(is = "types::Int8"))),
        field(name = "int_16", value(item(is = "types::Int16"))),
        field(name = "int_32", value(item(is = "types::Int32"))),
        field(name = "int_64", value(item(is = "types::Int64"))),
        field(name = "int_128", value(item(is = "types::Int128"))),
        field(name = "nat_8", value(item(is = "types::Nat8"))),
        field(name = "nat_16", value(item(is = "types::Nat16"))),
        field(name = "nat_32", value(item(is = "types::Nat32"))),
        field(name = "nat_64", value(item(is = "types::Nat64"))),
        field(name = "nat_128", value(item(is = "types::Nat128"))),
        field(name = "float_32", value(item(is = "types::Float32"))),
        field(name = "float_64", value(item(is = "types::Float64"))),
        field(name = "bool_test", value(item(is = "types::Bool"))),
        field(name = "decimal_test", value(item(is = "types::Decimal"))),
        field(name = "utf8_test", value(item(is = "types::bytes::Utf8"))),
        field(name = "timestamp", value(item(is = "types::time::Timestamp"))),
        field(name = "tuple_test", value(item(is = "Tuple"))),
        field(name = "name_many", value(many, item(is = "types::String"))),
        field(name = "name_opt", value(opt, item(is = "types::String"))),
        field(name = "record_a", value(item(is = "RecordA"))),
        field(name = "record_opt", value(opt, item(is = "RecordB"))),
        field(name = "record_many", value(many, item(is = "RecordB"))),
        field(name = "variant_complex", value(item(is = "EnumA"))),
        field(name = "variant_complex_opt", value(opt, item(is = "EnumA"))),
        field(name = "variant_complex_many", value(many, item(is = "EnumA"))),
        field(name = "variant_simple", value(item(is = "EnumB"))),
        field(name = "variant_simple_many", value(many, item(is = "EnumB"))),
        field(name = "variant_simple_opt", value(opt, item(is = "EnumB"))),
    ),
    traits(remove(Eq))
)]
pub struct ComplexEntity {}

///
/// AdminEntity
///

#[entity(
    store = "crate::Store",
    sk(entity = "AdminEntity", field = "id"),
    fields(
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "simple_text", value(item(is = "types::String"))),
        field(name = "tuple_test", value(item(is = "Tuple"))),
        field(name = "text_many", value(many, item(is = "types::String"))),
        field(name = "text_opt", value(opt, item(is = "types::String"))),
        field(name = "nat_32", value(item(is = "types::Nat32"))),
        field(name = "record_a", value(item(is = "RecordA"))),
        field(name = "record_opt", value(opt, item(is = "RecordB"))),
        field(name = "record_many", value(many, item(is = "RecordB"))),
        field(name = "variant_complex", value(item(is = "EnumA"))),
        field(name = "variant_complex_opt", value(opt, item(is = "EnumA"))),
        field(name = "variant_complex_many", value(many, item(is = "EnumA"))),
        field(name = "variant_simple", value(item(is = "EnumB"))),
        field(name = "variant_simple_many", value(many, item(is = "EnumB"))),
        field(name = "variant_simple_opt", value(opt, item(is = "EnumB"))),
    )
)]
pub struct AdminEntity {}

///
/// RelatedEntity
///

#[entity(
    store = "crate::Store",
    sk(entity = "RelatedEntity", field = "id"),
    fields(
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "simple_rel", value(item(rel = "SimpleEntity"))),
        field(name = "opt_simple_rel", value(opt, item(rel = "SimpleEntity"))),
        field(name = "simple_rels", value(many, item(rel = "SimpleEntity"))),
    )
)]
pub struct RelatedEntity {}

///
/// SimpleEntity
///

#[entity(
    store = "crate::Store",
    sk(entity = "SimpleEntity", field = "id"),
    fields(
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(is = "types::String"))),
    )
)]
pub struct SimpleEntity {}

///
/// RecordA
///

#[record(
    fields(
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "variant_a", value(item(is = "EnumA"))),
        field(name = "description", value(item(is = "types::String"))),
    ),
    traits(add(Default))
)]
pub struct RecordA {}

///
/// RecordB
///

#[record(fields(
    field(name = "name", value(item(is = "types::String"))),
    field(name = "name_opt", value(opt, item(is = "types::String")))
))]
pub struct RecordB {}

///
/// RecordC
///

#[record(fields(field(name = "prim", value(item(is = "types::String")))))]
pub struct RecordC {}

///
/// EnumA
///

#[enum_(
    variant(name = "A", default),
    variant(name = "B", value(item(is = "types::String"))),
    variant(name = "C", value(item(is = "RecordB"))),
    variant(name = "D", value(item(is = "RecordC")))
)]
pub struct EnumA {}

///
/// EnumB
///

#[enum_(variant(name = "F", default), variant(name = "G"))]
pub struct EnumB {}

///
/// EnumC
///

#[enum_(
    variant(name = "F", value(item(is = "types::String"))),
    variant(name = "I", value(item(is = "RecordB")))
)]
pub struct EnumC {}

///
/// Newtype
///

#[newtype(primitive = "String", item(is = "types::String"))]
pub struct Newtype {}

///
/// Tuple
///

#[tuple(
    value(item(is = "types::String")),
    value(item(is = "types::String")),
    traits(add(Default))
)]
pub struct Tuple {}
