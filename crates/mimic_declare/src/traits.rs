use crate::{
    helper::format_view_ident,
    node_traits::{Implementor, Trait, TraitList},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::{
    sync::{LazyLock, Mutex},
    time::SystemTime,
};
use syn::Ident;
use tinyrand::{Rand, Seeded, StdRand};

///
/// TraitTokens
///

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

///
/// RNG
///
/// Create a static, lazily-initialized StdRng instance wrapped in a Mutex
///

static RNG: LazyLock<Mutex<StdRand>> = LazyLock::new(|| {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let now_u64 = u64::try_from(now).unwrap();

    Mutex::new(StdRand::seed(now_u64))
});

///
/// AsMacro
/// any schema element that's invoked from a crate macro
///

pub trait AsMacro: AsSchema + AsType + quote::ToTokens {
    fn ident(&self) -> Ident;
    fn view_ident(&self) -> Ident {
        format_view_ident(&self.ident())
    }
    fn macro_children(&self) -> Vec<TokenStream> {
        vec![]
    }
    fn traits(&self) -> Vec<Trait> {
        vec![]
    }
    fn map_trait(&self, _: Trait) -> Option<TokenStream> {
        None
    }
    fn map_attribute(&self, _: Trait) -> Option<TokenStream> {
        None
    }
    fn resolve_trait(&self, tr: Trait) -> Option<TokenStream> {
        self.map_trait(tr).or_else(|| self.default_trait(tr))
    }

    // default_trait
    fn default_trait(&self, tr: Trait) -> Option<TokenStream> {
        let ident = self.ident();

        match tr {
            Trait::Path => {
                let ident_str = format!("{ident}");
                let q = quote! {
                    const PATH: &'static str = concat!(module_path!(), "::", #ident_str);
                };

                Some(Implementor::new(ident, tr).set_tokens(q).to_token_stream())
            }

            // empty implementations are generated for these traits
            Trait::EntityFixture
            | Trait::EntityIdKind
            | Trait::FieldSearchable
            | Trait::FieldSortable
            | Trait::FieldValue
            | Trait::ValidateAuto
            | Trait::ValidateCustom
            | Trait::Visitable => Some(Implementor::new(ident, tr).to_token_stream()),

            _ => None,
        }
    }
}

///
/// MacroEmitter
///

pub trait MacroEmitter: AsMacro {
    fn all_tokens(&self) -> TokenStream {
        let schema = self.schema_tokens();
        let main_type = self.as_type();
        let view_type = self.as_view_type();
        let children = self.macro_children();

        let TraitTokens { derive, impls } = self.resolve_trait_tokens();

        quote! {
            #schema
            #derive
            #main_type
            #view_type
            #impls
            #(#children)*
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

impl<T> MacroEmitter for T where T: AsMacro {}

///
/// AsSchema
/// an element that can generate schema tokens
///

pub trait AsSchema {
    const EMIT_SCHEMA: bool;

    // returns the schema fragment
    fn schema(&self) -> TokenStream {
        quote!()
    }

    // schema_tokens
    // generates the structure passed via ctor to the static schema
    fn schema_tokens(&self) -> Option<TokenStream> {
        if !Self::EMIT_SCHEMA {
            return None;
        }

        let schema = self.schema();
        let mut rng = RNG.lock().expect("Failed to lock RNG");
        let ctor_fn = format_ident!("ctor_{}", rng.next_u32());

        Some(quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[::mimic::export::ctor::ctor]
            fn #ctor_fn() {
                ::mimic::schema::build::schema_write().insert_node(
                    #schema
                );
            }
        })
    }
}

///
/// AsType
/// an element that can define a rust type
///

pub trait AsType {
    fn as_type(&self) -> Option<TokenStream> {
        None
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        None
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
