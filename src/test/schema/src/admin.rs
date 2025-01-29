use mimic::orm::{base::types, prelude::*};

///
/// ComplexEntity
///

#[entity(
    sk(entity = "ComplexEntity", field = "id"),
    fields(
        field(
            name = "id",
            value(item(is = "types::Ulid"), default = "types::Ulid::generate")
        ),
        field(name = "string_test", value(item(is = "types::String"))),
        field(name = "principal_test", value(item(is = "types::Principal"))),
        field(name = "blob_test", value(item(is = "types::Blob"))),
        field(name = "u_8", value(item(is = "types::U8"))),
        field(name = "u_16", value(item(is = "types::U16"))),
        field(name = "u_32", value(item(is = "types::U32"))),
        field(name = "u_64", value(item(is = "types::U64"))),
        field(name = "u_128", value(item(is = "types::U128"))),
        field(name = "i_8", value(item(is = "types::I8"))),
        field(name = "i_16", value(item(is = "types::I16"))),
        field(name = "i_32", value(item(is = "types::I32"))),
        field(name = "i_64", value(item(is = "types::I64"))),
        field(name = "i_128", value(item(is = "types::I128"))),
        field(name = "f_32", value(item(is = "types::F32"))),
        field(name = "f_64", value(item(is = "types::F64"))),
        field(name = "bool_test", value(item(is = "types::Bool"))),
        field(name = "decimal_test", value(item(is = "types::Decimal"))),
        field(name = "bytes_test", value(item(is = "types::bytes::Bytes<20>"))),
        field(name = "utf8_test", value(item(is = "types::bytes::Utf8<20>"))),
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
        field(name = "map_int_string", value(item(is = "MapIntString"))),
        field(name = "map_int_string_many", value(many, item(is = "MapIntString"))),
        field(name = "map_string_string", value(item(is = "MapStringString"))),
        field(
            name = "map_string_string_many",
            value(many, item(is = "MapStringString"))
        ),
        field(name = "map_string_record", value(item(is = "MapStringRecord"))),
        field(
            name = "map_string_record_many",
            value(many, item(is = "MapStringRecord"))
        ),
    ),
    traits(remove(Eq))
)]
pub struct ComplexEntity {}

///
/// AdminEntity
///

#[entity(
    sk(entity = "AdminEntity", field = "id"),
    fields(
        field(
            name = "id",
            value(item(is = "types::Ulid"), default = "types::Ulid::generate")
        ),
        field(name = "simple_text", value(item(is = "types::String"))),
        field(name = "tuple_test", value(item(is = "Tuple"))),
        field(name = "text_many", value(many, item(is = "types::String"))),
        field(name = "text_opt", value(opt, item(is = "types::String"))),
        field(name = "number_32", value(item(is = "types::U32"))),
        field(name = "record_a", value(item(is = "RecordA"))),
        field(name = "record_opt", value(opt, item(is = "RecordB"))),
        field(name = "record_many", value(many, item(is = "RecordB"))),
        field(name = "variant_complex", value(item(is = "EnumA"))),
        field(name = "variant_complex_opt", value(opt, item(is = "EnumA"))),
        field(name = "variant_complex_many", value(many, item(is = "EnumA"))),
        field(name = "variant_simple", value(item(is = "EnumB"))),
        field(name = "variant_simple_many", value(many, item(is = "EnumB"))),
        field(name = "variant_simple_opt", value(opt, item(is = "EnumB"))),
        field(name = "map_int_string", value(item(is = "MapIntString"))),
        field(name = "map_int_string_many", value(many, item(is = "MapIntString"))),
        field(name = "map_string_string", value(item(is = "MapStringString"))),
        field(
            name = "map_string_string_many",
            value(many, item(is = "MapStringString"))
        ),
    )
)]
pub struct AdminEntity {}

///
/// SimpleEntity
///

#[entity(
    sk(entity = "SimpleEntity", field = "id"),
    fields(
        field(
            name = "id",
            value(item(is = "types::Ulid"), default = "types::Ulid::generate")
        ),
        field(name = "name", value(item(is = "types::String"))),
    )
)]
pub struct SimpleEntity {}

///
/// RecordA
///

#[record(
    fields(
        field(
            name = "name",
            value(item(is = "types::Ulid"), default = "types::Ulid::generate")
        ),
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
    variant(name = "G", value(item(is = "MapStringString"))),
    variant(name = "H", value(item(is = "MapStringRecord"))),
    variant(name = "I", value(item(is = "RecordB")))
)]
pub struct EnumC {}

///
/// MapIntString
///

#[map(key(is = "types::I32"), value(item(is = "types::String")))]
pub struct MapIntString {}

///
/// MapStringRecord
///

#[map(key(is = "types::String"), value(item(is = "RecordA")))]
pub struct MapStringRecord {}

///
/// MapStringString
///

#[map(key(is = "types::String"), value(item(is = "types::String")))]
pub struct MapStringString {}

///
/// Newtype
///

#[newtype(primitive = "String", value(item(is = "types::String")))]
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
