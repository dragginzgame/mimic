use crate::{
    helper::format_view_ident,
    node_traits::{Implementor, Trait, TraitList},
};
use mimic_common::utils::{
    case::{Case, Casing},
    hash::hash_u64,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

///
/// TraitTokens
///

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

///
/// HasMacro
/// for any schema node that has a derive macro
///

pub trait HasMacro: HasSchema + HasTraits + HasType {
    fn all_tokens(&self) -> TokenStream {
        let schema = self.schema_tokens();
        let main_type = self.type_part();
        let view_type = self.view_type_part();

        let TraitTokens { derive, impls } = self.resolve_trait_tokens();

        quote! {
            #schema
            #derive
            #main_type
            #view_type
            #impls
        }
    }

    fn resolve_trait_tokens(&self) -> TraitTokens {
        let mut derived_traits = Vec::new();
        let mut attrs = Vec::new();
        let mut impls = quote!();

        for tr in self.traits() {
            let impl_block = self.resolve_trait(tr);
            let attr = self.map_attribute(tr);

            match (impl_block, attr) {
                (Some(t), Some(a)) => {
                    impls.extend(t);
                    attrs.push(a);
                }
                (Some(t), None) => {
                    impls.extend(t);
                }
                (None, Some(a)) => {
                    if let Some(path) = tr.derive_path() {
                        derived_traits.push(path);
                    }
                    attrs.push(a);
                }
                (None, None) => {
                    // Enforce that at least one strategy is defined
                    derived_traits.push(tr.derive_path().unwrap_or_else(|| {
                        panic!("trait '{tr}' has no derive, impl, or attributes")
                    }));
                }
            }
        }

        let mut derive = if derived_traits.is_empty() {
            quote!()
        } else {
            quote! {
                #[derive(#(#derived_traits),*)]
            }
        };
        derive.extend(attrs);

        TraitTokens { derive, impls }
    }
}

impl<T> HasMacro for T where T: HasIdent + HasSchema + HasTraits + HasType + HasTypePart {}

///
/// HasIdent
///

pub trait HasIdent {
    /// Returns the primary identifier for the item
    fn ident(&self) -> Ident;
}

///
/// HasTraits
/// a schema node that has traits (derives or impls)
///

pub trait HasTraits: HasIdent + ToTokens {
    /// Returns a list of traits to implement
    fn traits(&self) -> Vec<Trait> {
        vec![]
    }

    /// Maps a trait to its token implementation
    fn map_trait(&self, _: Trait) -> Option<TokenStream> {
        None
    }

    /// Maps a trait to its attribute-level implementation
    fn map_attribute(&self, _: Trait) -> Option<TokenStream> {
        None
    }

    /// Resolves a trait using map_trait or a default implementation
    fn resolve_trait(&self, tr: Trait) -> Option<TokenStream> {
        self.map_trait(tr).or_else(|| self.default_trait(tr))
    }

    /// Provides a default implementation for built-in traits
    fn default_trait(&self, tr: Trait) -> Option<TokenStream> {
        let ident = self.ident();

        match tr {
            // Generates a `const PATH` string pointing to the module + type name
            Trait::Path => {
                let q = quote! {
                    const PATH: &'static str = concat!(module_path!(), "::", stringify!(#ident));
                };

                Some(Implementor::new(ident, tr).set_tokens(q).to_token_stream())
            }

            // Generate empty impl blocks for marker traits
            Trait::EntityFixture
            | Trait::EntityIdKind
            | Trait::FieldValue
            | Trait::ValidateAuto
            | Trait::ValidateCustom
            | Trait::Visitable => Some(Implementor::new(ident, tr).to_token_stream()),

            // All others fallback to None
            _ => None,
        }
    }
}

///
/// HasSchema
/// an element that can generate schema tokens
///

pub trait HasSchema: HasSchemaPart + HasIdent {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::None
    }

    fn schema_const(&self) -> Ident {
        let ident_s = &self.ident().to_string().to_case(Case::UpperSnake);

        format_ident!("{ident_s}_CONST")
    }

    // schema_tokens
    // generates the structure passed via ctor to the static schema
    fn schema_tokens(&self) -> TokenStream {
        let schema = self.schema_part();
        if schema.is_empty() {
            return quote!();
        }

        // randomly generate fn name
        let ident = self.ident();
        let hash = hash_u64(ident.to_string().as_bytes());
        let ctor_fn = format_ident!("ctor_{hash}");

        // insert statement
        let const_var = self.schema_const();
        let kind = Self::schema_node_kind();

        quote! {
            const #const_var: ::mimic::schema::node::#kind = #schema;

            #[cfg(not(target_arch = "wasm32"))]
            #[::mimic::export::ctor::ctor]
            fn #ctor_fn() {
                ::mimic::schema::build::schema_write().insert_node(
                    ::mimic::schema::node::SchemaNode::#kind(#const_var)
                );
            }
        }
    }
}

///
/// SchemaNodeKind
///

#[derive(Debug)]
pub enum SchemaNodeKind {
    None,
    Canister,
    Constant,
    Entity,
    Enum,
    EnumValue,
    Index,
    List,
    Map,
    Newtype,
    Record,
    Selector,
    Set,
    Store,
    Tuple,
    Validator,
}

impl ToTokens for SchemaNodeKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{self:?}");
        ident.to_tokens(tokens);
    }
}

///
/// HasSchemaPart
/// for types that only emit parts of a schema
///

pub trait HasSchemaPart {
    fn schema_part(&self) -> TokenStream {
        quote!()
    }
}

///
/// HasType
/// an element that can define a rust type
///

pub trait HasType: HasTypePart + HasIdent {
    fn view_ident(&self) -> Ident {
        format_view_ident(&self.ident())
    }

    fn view_derives() -> TokenStream {
        TraitList(vec![
            Trait::CandidType,
            Trait::Clone,
            Trait::Debug,
            Trait::Serialize,
            Trait::Deserialize,
        ])
        .to_derive_tokens()
    }
}

impl<T> HasType for T where T: HasTypePart + HasIdent {}

///
/// HasTypePart
///

pub trait HasTypePart {
    fn type_part(&self) -> TokenStream {
        quote!()
    }

    fn view_type_part(&self) -> TokenStream {
        quote!()
    }
}
