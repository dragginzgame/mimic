use crate::{
    helper::{quote_one, quote_slice, to_str_lit},
    traits::AsSchema,
};
use darling::{Error as DarlingError, FromMeta, ast::NestedMeta};
use derive_more::Deref;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::fmt::{self, Display};
use syn::{Lit, LitStr, Path};

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
    String(LitStr),
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
                    Ok(Self::String(lit.clone()))
                }
            }
            _ => Err(DarlingError::custom(format!(
                "Unsupported literal type: {value:?}",
            ))),
        }
    }
}

impl AsSchema for Arg {
    fn schema(&self) -> TokenStream {
        match self {
            Self::Bool(v) => quote!(::mimic::schema::node::Arg::Bool(#v)),
            Self::Char(v) => quote!(::mimic::schema::node::Arg::Char(#v)),
            Self::Number(v) => {
                let num = quote_one(v, ArgNumber::schema);
                quote!(::mimic::schema::node::Arg::Number(#num))
            }
            Self::Path(v) => {
                let path = quote_one(v, to_str_lit);
                quote!(::mimic::schema::node::Arg::Path(#path))
            }
            Self::String(v) => quote!(::mimic::schema::node::Arg::String(#v)),
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
pub struct Args(Vec<Arg>);

impl FromMeta for Args {
    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        let mut args = Vec::new();

        for item in items {
            args.push(Arg::from_nested_meta(item)?);
        }

        Ok(Self(args))
    }
}

impl AsSchema for Args {
    fn schema(&self) -> TokenStream {
        let args = quote_slice(&self.0, Arg::schema);

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
            assert_eq!(num, ArgNumber::Int32(42), "Parsed number does not match");
        } else {
            panic!("Expected Number variant");
        }
    }

    #[test]
    fn test_path_parsing() {
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
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Nat8(u8),
    Nat16(u16),
    Nat32(u32),
    Nat64(u64),
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
    f32 => Float32,
    f64 => Float64,
    i8 => Int8,
    i16 => Int16,
    i32 => Int32,
    i64 => Int64,
    u8 => Nat8,
    u16 => Nat16,
    u32 => Nat32,
    u64 => Nat64
}

impl ArgNumber {
    // parse_numeric_string
    fn parse_numeric_string(s: &str) -> Result<Self, DarlingError> {
        let s = s.replace('_', "");

        let suffixes = [
            "f32", "f64", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64",
        ];

        // 1. Handle suffixed values
        for &suffix in &suffixes {
            if s.ends_with(suffix) {
                let num_part = s.trim_end_matches(suffix);

                let result = if num_part.contains('.') {
                    match suffix {
                        "f32" => num_part.parse::<f32>().map(Self::Float32),
                        "f64" => num_part.parse::<f64>().map(Self::Float64),
                        _ => unreachable!(),
                    }
                    .map_err(|_| {})
                } else {
                    match suffix {
                        "i8" => num_part.parse::<i8>().map(Self::Int8),
                        "i16" => num_part.parse::<i16>().map(Self::Int16),
                        "i32" => num_part.parse::<i32>().map(Self::Int32),
                        "i64" => num_part.parse::<i64>().map(Self::Int64),
                        "u8" => num_part.parse::<u8>().map(Self::Nat8),
                        "u16" => num_part.parse::<u16>().map(Self::Nat16),
                        "u32" => num_part.parse::<u32>().map(Self::Nat32),
                        "u64" => num_part.parse::<u64>().map(Self::Nat64),
                        _ => unreachable!(),
                    }
                    .map_err(|_| {})
                }
                .map_err(|()| DarlingError::custom(format!("invalid numeric literal '{s}'")));

                return result;
            }
        }

        // 2. Unsuffixed: first try integers
        if s.contains('.') {
            // 3. Unsuffixed float, just do f64 as easier
            if let Ok(f) = s.parse::<f64>() {
                return Ok(Self::Float64(f));
            }
        } else {
            macro_rules! try_parse {
                ($($ty:ty => $variant:ident),*) => {
                    $(
                        if let Ok(value) = s.parse::<$ty>() {
                            return Ok(Self::$variant(value));
                        }
                    )*
                };
            }

            // Try smallest fitting signed int
            try_parse!(
                i32 => Int32,
                i64 => Int64
            );

            // Try smallest fitting unsigned int
            try_parse!(
                u32 => Nat32,
                u64 => Nat64
            );
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
            Self::Float32(v) => write!(f, "{v}"),
            Self::Float64(v) => write!(f, "{v}"),
            Self::Int8(v) => write!(f, "{v}"),
            Self::Int16(v) => write!(f, "{v}"),
            Self::Int32(v) => write!(f, "{v}"),
            Self::Int64(v) => write!(f, "{v}"),
            Self::Nat8(v) => write!(f, "{v}"),
            Self::Nat16(v) => write!(f, "{v}"),
            Self::Nat32(v) => write!(f, "{v}"),
            Self::Nat64(v) => write!(f, "{v}"),
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
                .map(Self::Float64)
                .map_err(|_| DarlingError::custom("invalid float literal")),

            _ => Err(DarlingError::custom("expected numeric literal")),
        }
    }
}

impl PartialEq for ArgNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float64(a), Self::Float64(b)) => a.to_bits() == b.to_bits(),
            (Self::Float32(a), Self::Float32(b)) => a.to_bits() == b.to_bits(),
            (Self::Int8(a), Self::Int8(b)) => a == b,
            (Self::Int16(a), Self::Int16(b)) => a == b,
            (Self::Int32(a), Self::Int32(b)) => a == b,
            (Self::Int64(a), Self::Int64(b)) => a == b,
            (Self::Nat8(a), Self::Nat8(b)) => a == b,
            (Self::Nat16(a), Self::Nat16(b)) => a == b,
            (Self::Nat32(a), Self::Nat32(b)) => a == b,
            (Self::Nat64(a), Self::Nat64(b)) => a == b,
            _ => false,
        }
    }
}

impl AsSchema for ArgNumber {
    fn schema(&self) -> TokenStream {
        match self {
            Self::Float32(v) => quote!(::mimic::schema::node::ArgNumber::Float32(#v)),
            Self::Float64(v) => quote!(::mimic::schema::node::ArgNumber::Float64(#v)),
            Self::Int8(v) => quote!(::mimic::schema::node::ArgNumber::Int8(#v)),
            Self::Int16(v) => quote!(::mimic::schema::node::ArgNumber::Int16(#v)),
            Self::Int32(v) => quote!(::mimic::schema::node::ArgNumber::Int32(#v)),
            Self::Int64(v) => quote!(::mimic::schema::node::ArgNumber::Int64(#v)),
            Self::Nat8(v) => quote!(::mimic::schema::node::ArgNumber::Nat8(#v)),
            Self::Nat16(v) => quote!(::mimic::schema::node::ArgNumber::Nat16(#v)),
            Self::Nat32(v) => quote!(::mimic::schema::node::ArgNumber::Nat32(#v)),
            Self::Nat64(v) => quote!(::mimic::schema::node::ArgNumber::Nat64(#v)),
        }
    }
}

impl ToTokens for ArgNumber {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = self.to_string();
        let stream: TokenStream = s
            .parse()
            .expect("Failed to parse ArgNumber into TokenStream");

        tokens.extend(stream);
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
            ArgNumber::Int8(42)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("-100").unwrap(),
            ArgNumber::Int32(-100)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("1000i16").unwrap(),
            ArgNumber::Int16(1000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("-30000i32").unwrap(),
            ArgNumber::Int32(-30000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("500000i64").unwrap(),
            ArgNumber::Int64(500_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("42u8").unwrap(),
            ArgNumber::Nat8(42)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("65535u16").unwrap(),
            ArgNumber::Nat16(65535)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("4000000000u32").unwrap(),
            ArgNumber::Nat32(4_000_000_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("-10_i8").unwrap(),
            ArgNumber::Int8(-10)
        );
    }

    #[test]
    fn test_integer_uscores() {
        assert_eq!(
            ArgNumber::parse_numeric_string("10_000").unwrap(),
            ArgNumber::Int32(10_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("10_000_u64").unwrap(),
            ArgNumber::Nat64(10_000)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("10_000_i64").unwrap(),
            ArgNumber::Int64(10_000)
        );
    }

    #[test]
    fn test_float_literals() {
        assert_eq!(
            ArgNumber::parse_numeric_string("3.12_f32").unwrap(),
            ArgNumber::Float32(3.12)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("3.13_f64").unwrap(),
            ArgNumber::Float64(3.13)
        );
        assert_eq!(
            ArgNumber::parse_numeric_string("3.15").unwrap(),
            ArgNumber::Float64(3.15)
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
    }

    #[test]
    fn test_to_tokens_float() {
        let num = ArgNumber::parse_numeric_string("3.14").unwrap();
        let tokens = quote!(#num);
        assert_eq!(tokens.to_string(), "3.14");

        let num = ArgNumber::parse_numeric_string("3.14_f64").unwrap();
        let tokens = quote!(#num);
        assert_eq!(tokens.to_string(), "3.14");
    }
}
