use crate::{
    helper::{quote_one, to_path},
    node::PRIM_ULID,
};
use darling::{Error as DarlingError, FromMeta};
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Lit, Path};

///
/// Item
///

#[derive(Clone, Debug, FromMeta)]
pub enum Item {
    Id,
    Is(ItemIs),

    #[darling(rename = "rel")]
    Relation(ItemRelation),
}

impl Item {
    // is_relation
    pub const fn is_relation(&self) -> bool {
        matches!(self, Self::Relation(_))
    }
}

impl Schemable for Item {
    fn schema(&self) -> TokenStream {
        match self {
            Self::Id => {
                quote!(::mimic::orm::schema::node::Item::Id)
            }
            Self::Is(node) => node.schema(),
            Self::Relation(node) => node.schema(),
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Id => {
                let ty: Path = parse_str(PRIM_ULID).unwrap();
                tokens.extend(quote!(#ty));
            }
            Self::Is(node) => node.to_tokens(tokens),
            Self::Relation(node) => node.to_tokens(tokens),
        }
    }
}

///
/// ItemIs
/// type path and generics
///

#[derive(Clone, Debug)]
pub struct ItemIs {
    pub path: Path,
}

impl FromMeta for ItemIs {
    fn from_value(value: &Lit) -> Result<Self, DarlingError> {
        if let Lit::Str(s) = value {
            // parse the entire darling value into the path
            // is = "path::to::thing"
            let path = parse_str::<Path>(&s.value())?;

            Ok(Self { path })
        } else {
            Err(DarlingError::unexpected_lit_type(value))
        }
    }
}

impl Schemable for ItemIs {
    fn schema(&self) -> TokenStream {
        let path = quote_one(&self.path, to_path);

        quote!(
            ::mimic::orm::schema::node::Item::Is(::mimic::orm::schema::node::ItemIs {
                path: #path,
            }
        ))
    }
}

impl ToTokens for ItemIs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = &self.path;
        tokens.extend(quote!(#path));
    }
}

///
/// ItemRelation
///

#[derive(Clone, Debug)]
pub struct ItemRelation {
    pub path: Path,
}

impl FromMeta for ItemRelation {
    fn from_value(value: &syn::Lit) -> Result<Self, DarlingError> {
        match value {
            syn::Lit::Str(s) => {
                // parse the entire darling value into the path
                // rel = "path::to::thing"
                let path: Path = syn::parse_str(&s.value())?;

                Ok(Self { path })
            }
            _ => Err(DarlingError::unexpected_type("not string")),
        }
    }
}

impl Schemable for ItemRelation {
    fn schema(&self) -> TokenStream {
        let path = quote_one(&self.path, to_path);

        quote!(
            ::mimic::orm::schema::node::Item::Relation(::mimic::orm::schema::node::ItemRelation {
                path: #path,
            })
        )
    }
}

impl ToTokens for ItemRelation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty: Path = parse_str(PRIM_ULID).unwrap();
        tokens.extend(quote!(#ty));
    }
}
