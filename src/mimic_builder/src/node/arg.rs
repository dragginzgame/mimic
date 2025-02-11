use crate::{
    helper::{quote_one, quote_vec, to_string},
    traits::Schemable,
};
use darling::{ast::NestedMeta, Error as DarlingError, FromMeta};
use derive_more::Deref;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::{self, Display};
use syn::{Lit, Path};

///
/// Arg
///
/// String has been removed because it can be confused for Path or Number (negative
/// numbers need to be in quotes)
///
/// If arguments need to be strings then we can revisit and maybe have an alternative
///

#[derive(Clone, Debug)]
pub enum Arg {
    Bool(bool),
    Char(char),
    Number(ArgNumber),
    Path(Path),
    String(String),
}

impl FromMeta for Arg {
    fn from_value(value: &Lit) -> Result<Self, DarlingError> {
        match value {
            Lit::Bool(lit) => Ok(Self::Bool(lit.value)),
            Lit::Char(lit) => Ok(Self::Char(lit.value())),

            // Int
            Lit::Int(_) | Lit::Float(_) => ArgNumber::from_value(value)
                .map(Arg::Number)
                .map_err(|_| DarlingError::custom("Invalid integer format")),

            // Str
            // Here, analyze the literal to decide if it's a path or a plain string
            Lit::Str(lit) => {
                if lit.value().contains("::") {
                    // Simplistic check for path-like syntax
                    syn::parse_str::<Path>(&lit.value())
                        .map(Arg::Path)
                        .map_err(|_| DarlingError::custom("Failed to parse path"))
                } else {
                    Ok(Self::String(lit.value()))
                }
            }
            _ => Err(DarlingError::custom(format!(
                "Unsupported literal type: {value:?}",
            ))),
        }
    }
}

impl Schemable for Arg {
    fn schema(&self) -> TokenStream {
        match self {
            Self::Bool(v) => quote!(::mimic::schema::node::Arg::Bool(#v)),
            Self::Char(v) => quote!(::mimic::schema::node::Arg::Char(#v)),
            Self::Number(v) => {
                let num = quote_one(v, ArgNumber::schema);
                quote!(::mimic::schema::node::Arg::Number(#num))
            }
            Self::Path(v) => {
                let path = quote_one(v, to_string);
                quote!(::mimic::schema::node::Arg::Path(#path.to_string()))
            }
            Self::String(v) => quote!(::mimic::schema::node::Arg::String(#v.to_string())),
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Bool(v) => quote!(#v),
            Self::Char(v) => quote!(#v),
            Self::Number(v) => quote!(#v),
            Self::Path(v) => quote!(#v),
            Self::String(v) => quote!(#v),
        }
        .to_tokens(tokens);
    }
}

///
/// Args
/// Generic re-useable list of arguments
///

#[derive(Clone, Debug, Default, Deref)]
pub struct Args(pub Vec<Arg>);

impl FromMeta for Args {
    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        let mut args = Vec::new();

        for item in items {
            args.push(Arg::from_nested_meta(item)?);
        }

        Ok(Self(args))
    }
}

impl Schemable for Args {
    fn schema(&self) -> TokenStream {
        let args = quote_vec(&self.0, Arg::schema);

        quote! {
            ::mimic::schema::node::Args(#args)
        }
    }
}

///
/// TESTS
///

#[cfg(test)]
mod arg_tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_bool_parsing() {
        let lit = parse_quote!(true);
        if let Ok(Arg::Bool(b)) = Arg::from_value(&lit) {
            assert!(b, "Parsed boolean should be true");
        } else {
            panic!("Expected Bool variant");
        }
    }

    #[test]
    fn test_char_parsing() {
        let lit = parse_quote!('a');
        if let Ok(Arg::Char(c)) = Arg::from_value(&lit) {
            assert_eq!(c, 'a', "Parsed char should be 'a'");
        } else {
            panic!("Expected Char variant");
        }
    }

    #[test]
    fn test_number_parsing() {
        let lit = parse_quote!(42);
        if let Ok(Arg::Number(num)) = Arg::from_value(&lit) {
            assert_eq!(num, ArgNumber::Integer(42), "Parsed number does not match");
        } else {
            panic!("Expected Number variant");
        }
    }

    #[test]
    fn test_string_and_path_parsing() {
        let string_lit = parse_quote!("example");
        if let Ok(Arg::String(s)) = Arg::from_value(&string_lit) {
            assert_eq!(s, "example", "Parsed string does not match.");
        } else {
            panic!("Expected String variant");
        }

        let path_like_string_lit = parse_quote!("crate::module::Type");
        if let Ok(Arg::Path(path)) = Arg::from_value(&path_like_string_lit) {
            assert_eq!(
                path.to_token_stream().to_string(),
                "crate :: module :: Type",
                "Parsed path-like string does not match."
            );
        } else {
            panic!("Expected Path variant");
        }
    }

    #[test]
    fn test_invalid_input() {
        let lit = parse_quote!(b"invalid");
        assert!(
            Arg::from_value(&lit).is_err(),
            "Expected an error for unsupported literal type."
        );
    }
}

///
/// ArgNumber
///

#[derive(Clone, Debug)]
pub enum ArgNumber {
    Float(f64), // unsuffixed
    F32(f32),
    F64(f64),
    Integer(i128), // unsuffixed
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
}

macro_rules! impl_from_for_numeric_value {
    ($($t:ty => $variant:ident),*) => {
        $(
            impl From<$t> for ArgNumber {
                fn from(n: $t) -> Self {
                    Self::$variant(n)
                }
            }
        )*
    }
}

impl_from_for_numeric_value! {
    f32 => F32,
    f64 => F64,
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    i128 => I128,
    isize => Isize,
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    u128 => U128,
    usize => Usize
}

impl ArgNumber {
    // parse_numeric_string
    fn parse_numeric_string(s: &str) -> Result<Self, DarlingError> {
        let s = s.replace('_', "");

        let suffixes = [
            "f32", "f64", "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128",
            "usize", "isize",
        ];

        for &suffix in &suffixes {
            if s.ends_with(suffix) {
                let num_part = s.trim_end_matches(suffix);

                let result = if num_part.contains('.') {
                    match suffix {
                        "f32" => num_part.parse::<f32>().map(ArgNumber::F32),
                        "f64" => num_part.parse::<f64>().map(ArgNumber::F64),
                        _ => unreachable!(),
                    }
                    .map_err(|_| {})
                } else {
                    match suffix {
                        "i8" => num_part.parse::<i8>().map(ArgNumber::I8),
                        "i16" => num_part.parse::<i16>().map(ArgNumber::I16),
                        "i32" => num_part.parse::<i32>().map(ArgNumber::I32),
                        "i64" => num_part.parse::<i64>().map(ArgNumber::I64),
                        "i128" => num_part.parse::<i128>().map(ArgNumber::I128),
                        "u8" => num_part.parse::<u8>().map(ArgNumber::U8),
                        "u16" => num_part.parse::<u16>().map(ArgNumber::U16),
                        "u32" => num_part.parse::<u32>().map(ArgNumber::U32),
                        "u64" => num_part.parse::<u64>().map(ArgNumber::U64),
                        "u128" => num_part.parse::<u128>().map(ArgNumber::U128),
                        "usize" => num_part.parse::<usize>().map(ArgNumber::Usize),
                        "isize" => num_part.parse::<isize>().map(ArgNumber::Isize),
                        _ => unreachable!(),
                    }
                    .map_err(|_| {})
                }
                .map_err(|()| DarlingError::custom(format!("invalid numeric literal '{s}'")));

                return result;
            }
        }

        // Try parsing unsuffixed as Integer
        if let Ok(integer) = s.parse::<i128>() {
            return Ok(Self::Integer(integer));
        }

        // Try parsing unsuffixed as Float
        if let Ok(float) = s.parse::<f64>() {
            return Ok(Self::Float(float));
        }

        // Return error if no match found
        Err(DarlingError::custom(format!(
            "invalid or unsupported numeric literal '{s}'"
        )))
    }
}

impl Display for ArgNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(v) | Self::F64(v) => write!(f, "{v}"),
            Self::Integer(v) | Self::I128(v) => write!(f, "{v}"),
            Self::F32(v) => write!(f, "{v}"),
            Self::I8(v) => write!(f, "{v}"),
            Self::I16(v) => write!(f, "{v}"),
            Self::I32(v) => write!(f, "{v}"),
            Self::I64(v) => write!(f, "{v}"),
            Self::Isize(v) => write!(f, "{v}"),
            Self::U8(v) => write!(f, "{v}"),
            Self::U16(v) => write!(f, "{v}"),
            Self::U32(v) => write!(f, "{v}"),
            Self::U64(v) => write!(f, "{v}"),
            Self::U128(v) => write!(f, "{v}"),
            Self::Usize(v) => write!(f, "{v}"),
        }
    }
}

impl FromMeta for ArgNumber {
    fn from_value(value: &Lit) -> Result<Self, DarlingError> {
        match value {
            // Int
            Lit::Int(lit) => {
                let s = lit.to_string();
                Self::parse_numeric_string(&s)
            }

            // Float
            Lit::Float(lit) => lit
                .base10_parse::<f64>()
                .map(Self::F64)
                .map_err(|_| DarlingError::custom("invalid float literal")),

            _ => Err(DarlingError::custom("expected numeric literal")),
        }
    }
}

impl PartialEq for ArgNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(a), Self::Float(b)) | (Self::F64(a), Self::F64(b)) => {
                a.to_bits() == b.to_bits()
            }
            (Self::F32(a), Self::F32(b)) => a.to_bits() == b.to_bits(),
            (Self::Integer(a), Self::Integer(b)) | (Self::I128(a), Self::I128(b)) => a == b,
            (Self::I8(a), Self::I8(b)) => a == b,
            (Self::I16(a), Self::I16(b)) => a == b,
            (Self::I32(a), Self::I32(b)) => a == b,
            (Self::I64(a), Self::I64(b)) => a == b,
            (Self::Isize(a), Self::Isize(b)) => a == b,
            (Self::U8(a), Self::U8(b)) => a == b,
            (Self::U16(a), Self::U16(b)) => a == b,
            (Self::U32(a), Self::U32(b)) => a == b,
            (Self::U64(a), Self::U64(b)) => a == b,
            (Self::U128(a), Self::U128(b)) => a == b,
            (Self::Usize(a), Self::Usize(b)) => a == b,
            _ => false,
        }
    }
}

impl Schemable for ArgNumber {
    fn schema(&self) -> TokenStream {
        match self {
            Self::Float(v) => quote!(::mimic::schema::node::ArgNumber::Float(#v)),
            Self::F32(v) => quote!(::mimic::schema::node::ArgNumber::F32(#v)),
            Self::F64(v) => quote!(::mimic::schema::node::ArgNumber::F64(#v)),
            Self::Integer(v) => quote!(::mimic::schema::node::ArgNumber::Integer(#v)),
            Self::I8(v) => quote!(::mimic::schema::node::ArgNumber::I8(#v)),
            Self::I16(v) => quote!(::mimic::schema::node::ArgNumber::I16(#v)),
            Self::I32(v) => quote!(::mimic::schema::node::ArgNumber::I32(#v)),
            Self::I64(v) => quote!(::mimic::schema::node::ArgNumber::I64(#v)),
            Self::I128(v) => quote!(::mimic::schema::node::ArgNumber::I128(#v)),
            Self::Isize(v) => quote!(::mimic::schema::node::ArgNumber::Isize(#v)),
            Self::U8(v) => quote!(::mimic::schema::node::ArgNumber::U8(#v)),
            Self::U16(v) => quote!(::mimic::schema::node::ArgNumber::U16(#v)),
            Self::U32(v) => quote!(::mimic::schema::node::ArgNumber::U32(#v)),
            Self::U64(v) => quote!(::mimic::schema::node::ArgNumber::U64(#v)),
            Self::U128(v) => quote!(::mimic::schema::node::ArgNumber::U128(#v)),
            Self::Usize(v) => quote!(::mimic::schema::node::ArgNumber::Usize(#v)),
        }
    }
}

impl ToTokens for ArgNumber {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            Self::Float(v) => {
                let value = format!("{v}");
                value.parse::<TokenStream>().unwrap()
            }
            Self::F32(v) => quote!(#v),
            Self::F64(v) => quote!(#v),
            Self::Integer(v) => {
                let value = format!("{v}");
                value.parse::<TokenStream>().unwrap()
            }
            Self::I8(v) => quote!(#v),
            Self::I16(v) => quote!(#v),
            Self::I32(v) => quote!(#v),
            Self::I64(v) => quote!(#v),
            Self::I128(v) => quote!(#v),
            Self::Isize(v) => quote!(#v),
            Self::U8(v) => quote!(#v),
            Self::U16(v) => quote!(#v),
            Self::U32(v) => quote!(#v),
            Self::U64(v) => quote!(#v),
            Self::U128(v) => quote!(#v),
            Self::Usize(v) => quote!(#v),
        };

        tokens.extend(q);
    }
}

///
/// TESTS
///

#[cfg(test)]
mod number_tests {
    use super::*;

    #[test]
    fn test_integer_literals() {
        assert_eq!(
            ArgNumber::parse_numeric_string("42i8").unwrap(),
            ArgNumber::I8(42)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("-100_isize").unwrap(),
            ArgNumber::Isize(-100)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("-100").unwrap(),
            ArgNumber::Integer(-100)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("1000i16").unwrap(),
            ArgNumber::I16(1000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("-30000i32").unwrap(),
            ArgNumber::I32(-30000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("500000i64").unwrap(),
            ArgNumber::I64(500_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("42u8").unwrap(),
            ArgNumber::U8(42)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("65535u16").unwrap(),
            ArgNumber::U16(65535)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("4000000000u32").unwrap(),
            ArgNumber::U32(4_000_000_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("-10_i8").unwrap(),
            ArgNumber::I8(-10)
        );
    }

    #[test]
    fn test_integer_uscores() {
        assert_eq!(
            ArgNumber::parse_numeric_string("10_000").unwrap(),
            ArgNumber::Integer(10_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("10_000_u64").unwrap(),
            ArgNumber::U64(10_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("10_000_i64").unwrap(),
            ArgNumber::I64(10_000)
        );
    }

    #[test]
    fn test_float_literals() {
        assert_eq!(
            ArgNumber::parse_numeric_string("3.12_f32").unwrap(),
            ArgNumber::F32(3.12)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("3.13_f64").unwrap(),
            ArgNumber::F64(3.13)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("3.15").unwrap(),
            ArgNumber::Float(3.15)
        );
    }

    #[test]
    fn test_invalid_literals() {
        assert!(ArgNumber::parse_numeric_string("hello").is_err());
        assert!(ArgNumber::parse_numeric_string("42x").is_err());
        assert!(ArgNumber::parse_numeric_string("4.2.5_f32").is_err());
    }

    #[test]
    fn test_to_tokens_integer() {
        let num = ArgNumber::parse_numeric_string("10").unwrap();
        let tokens = quote!(#num);
        assert_eq!(tokens.to_string(), "10");

        let num = ArgNumber::parse_numeric_string("10_isize").unwrap();
        let tokens = quote!(#num);
        assert_eq!(tokens.to_string(), "10isize");
    }

    #[test]
    fn test_to_tokens_float() {
        let num = ArgNumber::parse_numeric_string("3.14").unwrap();
        let tokens = quote!(#num);
        assert_eq!(tokens.to_string(), "3.14");

        let num = ArgNumber::parse_numeric_string("3.14_f64").unwrap();
        let tokens = quote!(#num);
        assert_eq!(tokens.to_string(), "3.14f64");
    }
}
