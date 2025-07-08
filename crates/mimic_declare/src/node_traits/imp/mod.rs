mod default;
mod entity;
mod enum_value;
mod field;
mod from;
mod implementor;
mod inner;
mod into;
mod num;
mod type_view;
mod validate;
mod visitable;

pub use default::*;
pub use entity::*;
pub use enum_value::*;
pub use field::*;
pub use from::*;
pub use implementor::*;
pub use inner::*;
pub use into::*;
pub use num::*;
pub use type_view::*;
pub use validate::*;
pub use visitable::*;

use crate::{node_traits::Trait, traits::AsMacro};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// TraitTokens
///

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

///
/// MacroHandler
///

pub struct MacroHandler<'a, T: AsMacro> {
    pub item: &'a T,
}

impl<'a, T: AsMacro> MacroHandler<'a, T> {
    pub const fn new(item: &'a T) -> Self {
        Self { item }
    }

    // macro_tokens
    pub fn macro_tokens(&self) -> TokenStream {
        let TraitTokens { derive, impls } = self.trait_tokens();
        let schema = self.item.schema_tokens();
        let extra = self.item.macro_extra();
        let item = self.item;

        let q = quote! {
            #schema
            #derive
            #item
            #impls
            #extra
        };

        if self.item.def().debug {
            quote! {
                compile_error!(stringify! { #q });
            }
        } else {
            quote!(#q)
        }
    }

    pub fn trait_tokens(&self) -> TraitTokens {
        let mut derived_traits = Vec::new();
        let mut attrs = Vec::new();
        let mut impls = quote!();

        for tr in self.item.traits() {
            match (self.resolve_trait(tr), self.resolve_attribute(tr)) {
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
                    derived_traits.push(tr.derive_path().unwrap_or_else(|| {
                        panic!("trait '{tr}' has no derive, impl or attributes")
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

    // resolve_attribute
    fn resolve_attribute(&self, tr: Trait) -> Option<TokenStream> {
        self.item.map_attribute(tr)
    }

    // resolve_trait
    fn resolve_trait(&self, tr: Trait) -> Option<TokenStream> {
        self.item.map_trait(tr).or_else(|| self.default_trait(tr))
    }

    // default_trait
    pub fn default_trait(&self, tr: Trait) -> Option<TokenStream> {
        let def = &self.item.def();

        match tr {
            Trait::Path => {
                let ident_str = format!("{}", def.ident);
                let q = quote! {
                    const PATH: &'static str = concat!(module_path!(), "::", #ident_str);
                };

                Some(Implementor::new(def, tr).set_tokens(q).to_token_stream())
            }

            // empty implementations are generated for these traits
            Trait::EntityFixture
            | Trait::EntityIdKind
            | Trait::FieldSearchable
            | Trait::FieldSortable
            | Trait::FieldValue
            | Trait::ValidateAuto
            | Trait::ValidateCustom
            | Trait::Visitable => Some(Implementor::new(def, tr).to_token_stream()),

            _ => None,
        }
    }
}

///
/// Imp
///

pub trait Imp<N> {
    fn tokens(node: &N) -> Option<TokenStream>;
}

///
/// ImpFn
/// for breaking down traits even further
///

pub trait ImpFn<N> {
    fn tokens(node: &N) -> TokenStream;
}
