use crate::{
    helper::format_view_ident,
    imp::{Implementor, TraitStrategy},
    node::Def,
    schema_traits::{Trait, TraitList},
};
use mimic_common::utils::case::{Case, Casing};
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
        let mut impls = TokenStream::new();

        for tr in self.traits() {
            let strat = self.map_trait(tr).or_else(|| self.default_strategy(tr));
            let attr = self.map_attribute(tr);

            match strat {
                Some(strategy) => {
                    if let Some(ts) = strategy.imp {
                        impls.extend(ts);
                    }

                    if let Some(derive_tr) = strategy.derive
                        && let Some(path) = derive_tr.derive_path()
                    {
                        derived_traits.push(path);
                    }
                }
                None => {
                    // No impl strategy; fallback to deriving the trait if it supports it
                    if let Some(path) = tr.derive_path() {
                        derived_traits.push(path);
                    } else if attr.is_none() {
                        panic!("Trait `{tr:?}` has no impl, derive, or attributes.");
                    }
                }
            }

            if let Some(attr_tokens) = attr {
                attrs.push(attr_tokens);
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

impl<T> HasMacro for T where T: HasDef + HasSchema + HasTraits + HasType + HasTypePart {}

///
/// HasDef
///

pub trait HasDef {
    fn def(&self) -> &Def;
}

///
/// HasTraits
/// a schema node that has traits (derives or impls)
///

pub trait HasTraits: HasDef + ToTokens {
    /// Returns a list of traits to implement
    fn traits(&self) -> TraitList {
        TraitList::new()
    }

    /// Maps a trait to its token implementation
    fn map_trait(&self, _: Trait) -> Option<TraitStrategy> {
        None
    }

    /// Maps a trait to its attribute-level implementation
    fn map_attribute(&self, _: Trait) -> Option<TokenStream> {
        None
    }

    /// Provides a default strategy for built-in traits
    fn default_strategy(&self, tr: Trait) -> Option<TraitStrategy> {
        let def = self.def();
        let ident = self.def().ident();

        match tr {
            // Generates a `const PATH` string pointing to the module + type name
            Trait::Path => {
                let q = quote! {
                    const PATH: &'static str = concat!(module_path!(), "::", stringify!(#ident));
                };

                let tokens = Implementor::new(def, tr).set_tokens(q).to_token_stream();

                Some(TraitStrategy::from_impl(tokens))
            }

            // Generate empty impl blocks for marker traits
            Trait::CanisterKind
            | Trait::EntityFixture
            | Trait::EntityIdKind
            | Trait::FieldValue
            | Trait::SanitizeAuto
            | Trait::SanitizeCustom
            | Trait::ValidateAuto
            | Trait::ValidateCustom
            | Trait::Visitable => {
                let tokens = Implementor::new(def, tr).to_token_stream();

                Some(TraitStrategy::from_impl(tokens))
            }

            // All others fallback to None
            _ => None,
        }
    }
}

///
/// HasSchema
/// an element that can generate schema tokens
///

pub trait HasSchema: HasSchemaPart + HasDef {
    fn schema_node_kind() -> SchemaNodeKind {
        unreachable!();
    }

    fn schema_const(&self) -> Ident {
        let ident_s = &self.def().ident().to_string().to_case(Case::UpperSnake);

        format_ident!("{ident_s}_CONST")
    }

    // schema_tokens
    // generates the structure passed via ctor to the static schema
    fn schema_tokens(&self) -> TokenStream {
        let schema = self.schema_part();
        if schema.is_empty() {
            return quote!();
        }

        // insert statement
        let const_var = self.schema_const();
        let kind = Self::schema_node_kind();

        quote! {
            const #const_var: ::mimic::schema::node::#kind = #schema;

            #[cfg(not(target_arch = "wasm32"))]
            #[::mimic::export::ctor::ctor(anonymous, crate_path = ::mimic::export::ctor)]
            fn __ctor() {
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
    Canister,
    Constant,
    Entity,
    Enum,
    EnumValue,
    List,
    Map,
    Newtype,
    Record,
    Sanitizer,
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

pub trait HasType: HasTypePart + HasDef {
    fn view_ident(&self) -> Ident {
        format_view_ident(&self.def().ident())
    }

    fn view_derives(&self) -> TraitList {
        TraitList(vec![
            Trait::CandidType,
            Trait::Clone,
            Trait::Debug,
            Trait::Serialize,
            Trait::Deserialize,
        ])
    }
}

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
