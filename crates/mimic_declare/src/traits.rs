use crate::{
    imp::{Implementor, TraitStrategy},
    node::Def,
    schema_traits::{Trait, TraitList},
};
use mimic_common::case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

//
// ──────────────────────────
// CORE MACRO AGGREGATOR
// ──────────────────────────
//

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

///
/// HasMacro
///
/// High-level entrypoint for code generation.
/// Combines schema, type definitions, and trait impls.
///
pub trait HasMacro: HasSchema + HasTraits + HasType {
    fn all_tokens(&self) -> TokenStream {
        let TraitTokens { derive, impls } = self.resolve_trait_tokens();
        let schema = self.schema_tokens();
        let type_part = self.type_part();
        let view_parts = self.view_parts();

        quote! {
            #schema
            #derive
            #type_part
            #impls

            #view_parts
        }
    }

    fn resolve_trait_tokens(&self) -> TraitTokens {
        let mut derive_traits = Vec::new();
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
                        derive_traits.push(path);
                    }
                }
                None => {
                    if let Some(path) = tr.derive_path() {
                        derive_traits.push(path);
                    }
                }
            }

            if let Some(attr_tokens) = attr {
                attrs.push(attr_tokens);
            }
        }

        let mut derive = if derive_traits.is_empty() {
            quote!()
        } else {
            quote!(#[derive(#(#derive_traits),*)])
        };

        derive.extend(attrs);

        TraitTokens { derive, impls }
    }
}

impl<T> HasMacro for T where T: HasDef + HasSchema + HasTraits + HasType {}

//
// ──────────────────────────
// CORE SCHEMA TRAITS
// ──────────────────────────
//

///
/// HasDef
///
pub trait HasDef {
    fn def(&self) -> &Def;
}

///
/// HasSchema
///
/// Anything that can emit a schema constant.
///
pub trait HasSchema: HasSchemaPart + HasDef {
    fn schema_node_kind() -> SchemaNodeKind {
        unreachable!()
    }

    fn schema_const(&self) -> Ident {
        let ident_s = &self.def().ident().to_string().to_case(Case::UpperSnake);
        format_ident!("{ident_s}_CONST")
    }

    fn schema_tokens(&self) -> TokenStream {
        let schema = self.schema_part();
        if schema.is_empty() {
            return quote!();
        }

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

#[derive(Debug)]
pub enum SchemaNodeKind {
    Canister,
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
        format_ident!("{self:?}").to_tokens(tokens);
    }
}

///
/// HasSchemaPart
///
/// Low-level helper for schema fragments.
///
pub trait HasSchemaPart {
    fn schema_part(&self) -> TokenStream {
        quote!()
    }
}

//
// ──────────────────────────
// TYPE GENERATION
// ──────────────────────────
//

///
/// HasType
///
/// A node that emits a Rust type definition.
///
pub trait HasType: HasViewTypes {
    /// Emits the main rust Type for this node
    fn type_part(&self) -> TokenStream {
        quote!()
    }
}

///
/// HasViewTypes
///
pub trait HasViewTypes: HasDef {
    /// Emits only the View types (View, New, Update etc.)
    fn view_parts(&self) -> TokenStream {
        quote!()
    }

    /// Utility: default naming convention for view variant.
    fn view_ident(&self) -> Ident {
        format_ident!("{}View", self.def().ident())
    }

    fn create_ident(&self) -> Ident {
        format_ident!("{}Create", self.def().ident())
    }

    fn update_ident(&self) -> Ident {
        format_ident!("{}Update", self.def().ident())
    }

    /// Utility: standard derives for generated types.
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
/// HasTypeExpr
///
/// For schema nodes that emit *inline* type expressions, not full structs/enums.
/// (e.g. Value, Item, Field)
///
pub trait HasTypeExpr {
    fn type_expr(&self) -> TokenStream {
        quote!()
    }

    fn view_type_expr(&self) -> TokenStream {
        quote!()
    }
}

//
// ──────────────────────────
// TRAIT GENERATION
// ──────────────────────────
//

///
/// HasTraits
///
/// Describes which traits to derive or implement.
///
pub trait HasTraits: HasDef + ToTokens {
    fn traits(&self) -> TraitList {
        TraitList::new()
    }

    fn map_trait(&self, _: Trait) -> Option<TraitStrategy> {
        None
    }

    fn map_attribute(&self, _: Trait) -> Option<TokenStream> {
        None
    }

    fn default_strategy(&self, tr: Trait) -> Option<TraitStrategy> {
        let def = self.def();
        let ident = def.ident();

        match tr {
            // Inline `const PATH` impl
            Trait::Path => {
                let q = quote! {
                    const PATH: &'static str = concat!(module_path!(), "::", stringify!(#ident));
                };
                let tokens = Implementor::new(def, tr).set_tokens(q).to_token_stream();
                Some(TraitStrategy::from_impl(tokens))
            }

            // Marker traits (empty impl)
            Trait::CanisterKind
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

            _ => None,
        }
    }
}
