use crate::prelude::*;
use base::types;

///
/// ComplexEntity
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "ComplexEntity", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "string_test", value(item(prim = "Text"))),
    field(name = "principal_test", value(item(prim = "Principal"))),
    field(name = "blob_test", value(item(prim = "Blob"))),
    field(name = "int_8", value(item(prim = "Int8"))),
    field(name = "int_16", value(item(prim = "Int16"))),
    field(name = "int_32", value(item(prim = "Int32"))),
    field(name = "int_64", value(item(prim = "Int64"))),
    field(name = "int_128", value(item(prim = "Int128"))),
    field(name = "nat_8", value(item(prim = "Nat8"))),
    field(name = "nat_16", value(item(prim = "Nat16"))),
    field(name = "nat_32", value(item(prim = "Nat32"))),
    field(name = "nat_64", value(item(prim = "Nat64"))),
    field(name = "nat_128", value(item(prim = "Nat128"))),
    field(name = "float_32", value(item(prim = "Float32"))),
    field(name = "float_64", value(item(prim = "Float64"))),
    field(name = "bool_test", value(item(prim = "Bool"))),
    field(name = "decimal_test", value(item(prim = "Decimal"))),
    field(name = "utf8_test", value(item(is = "types::bytes::Utf8"))),
    field(name = "timestamp", value(item(is = "types::time::Timestamp"))),
    field(name = "tuple_test", value(item(is = "Tuple"))),
    field(name = "name_many", value(many, item(prim = "Text"))),
    field(name = "name_opt", value(opt, item(prim = "Text"))),
    field(name = "record_a", value(item(is = "RecordA"))),
    field(name = "record_opt", value(opt, item(is = "RecordB"))),
    field(name = "record_many", value(many, item(is = "RecordB"))),
    field(name = "variant_complex", value(item(is = "EnumA"))),
    field(name = "variant_complex_opt", value(opt, item(is = "EnumA"))),
    field(name = "variant_complex_many", value(many, item(is = "EnumA"))),
    field(name = "variant_simple", value(item(is = "EnumB"))),
    field(name = "variant_simple_many", value(many, item(is = "EnumB"))),
    field(name = "variant_simple_opt", value(opt, item(is = "EnumB"))),
    traits(remove(Eq))
)]
pub struct ComplexEntity {}

///
/// AdminEntity
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "AdminEntity", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "simple_text", value(item(prim = "Text"))),
    field(name = "tuple_test", value(item(is = "Tuple"))),
    field(name = "text_many", value(many, item(prim = "Text"))),
    field(name = "text_opt", value(opt, item(prim = "Text"))),
    field(name = "nat_32", value(item(prim = "Nat32"))),
    field(name = "record_a", value(item(is = "RecordA"))),
    field(name = "record_opt", value(opt, item(is = "RecordB"))),
    field(name = "record_many", value(many, item(is = "RecordB"))),
    field(name = "variant_complex", value(item(is = "EnumA"))),
    field(name = "variant_complex_opt", value(opt, item(is = "EnumA"))),
    field(name = "variant_complex_many", value(many, item(is = "EnumA"))),
    field(name = "variant_simple", value(item(is = "EnumB"))),
    field(name = "variant_simple_many", value(many, item(is = "EnumB"))),
    field(name = "variant_simple_opt", value(opt, item(is = "EnumB")))
)]
pub struct AdminEntity {}

///
/// RelatedEntity
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "RelatedEntity", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "simple_key", value(item(rel = "SimpleEntity"))),
    field(name = "opt_simple_key", value(opt, item(rel = "SimpleEntity"))),
    field(name = "simple_keys", value(many, item(rel = "SimpleEntity")))
)]
pub struct RelatedEntity {}

///
/// SimpleEntity
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "SimpleEntity", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "name", value(item(prim = "Text")))
)]
pub struct SimpleEntity {}

///
/// RecordA
///

#[record(
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "variant_a", value(item(is = "EnumA"))),
    field(name = "description", value(item(prim = "Text"))),
    traits(add(Default))
)]
pub struct RecordA {}

///
/// RecordB
///

#[record(
    field(name = "name", value(item(prim = "Text"))),
    field(name = "name_opt", value(opt, item(prim = "Text")))
)]
pub struct RecordB {}

///
/// RecordC
///

#[record(field(name = "prim", value(item(prim = "Text"))))]
pub struct RecordC {}

///
/// EnumA
///

#[enum_(
    variant(name = "A", default),
    variant(name = "B", value(item(prim = "Text"))),
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
    variant(name = "F", value(item(prim = "Text"))),
    variant(name = "I", value(item(is = "RecordB")))
)]
pub struct EnumC {}

///
/// EnumValue
///

#[enum_value(
    variant(name = "M6", value = 1),
    variant(name = "Y1", value = 5),
    variant(name = "Y2", value = 10),
    variant(name = "Y3", value = 10)
)]
pub struct EnumValue {}

///
/// Newtype
///

#[newtype(primitive = "Text", item(prim = "Text"))]
pub struct Newtype {}

///
/// Tuple
///

#[tuple(
    value(item(prim = "Text")),
    value(item(prim = "Text")),
    traits(add(Default))
)]
pub struct Tuple {}
