use crate::node::Def;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
    sync::{LazyLock, Mutex},
    time::SystemTime,
};
use tinyrand::{Rand, Seeded, StdRand};

///
/// TRAITS
/// (Node Traits)
///

///
/// RNG
///
/// Create a static, lazily-initialized StdRng instance wrapped in a Mutex
///

pub(crate) static RNG: LazyLock<Mutex<StdRand>> = LazyLock::new(|| {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let now_u64 = u64::try_from(now).unwrap();

    Mutex::new(StdRand::seed(now_u64))
});

///
/// Macro
/// any schema element that's invoked from a crate macro
///

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

pub trait Macro {
    fn def(&self) -> &Def;

    /// Returns the primary item to be wrapped by macro expansion —
    /// this will be the struct definition that receives the derived traits
    /// and additional `impl` blocks.
    fn macro_body(&self) -> TokenStream {
        let Def { ident, .. } = &self.def();

        quote! {
            pub struct #ident {}
        }
    }

    /// Returns any extra tokens to emit *after* the main item and its impls.
    fn macro_extra(&self) -> TokenStream {
        quote!()
    }

    // emit_macro
    fn emit_macro(&self) -> TokenStream {
        let TraitTokens { derive, impls } = self.trait_tokens();
        let body = self.macro_body();
        let extra = self.macro_extra();

        let q = quote! {
            #derive
            #body
            #impls
            #extra
        };

        // debug
        if self.def().debug {
            quote! {
                compile_error!(stringify! { #q });
            }
        } else {
            quote!(#q)
        }
    }

    // trait_tokens
    fn trait_tokens(&self) -> TraitTokens {
        let mut derived_traits = Vec::new();
        let mut attrs = Vec::new();
        let mut impls = quote!();

        // we only derive traits that have no map_imp tokens
        for tr in self.traits() {
            match (self.map_trait(tr), self.map_attribute(tr)) {
                (Some(t), Some(a)) => {
                    impls.extend(t);
                    attrs.push(a);
                }
                (Some(t), None) => {
                    impls.extend(t);
                }
                (None, Some(a)) => {
                    if let Some(derive) = tr.derive_path() {
                        derived_traits.push(derive);
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

        // custom impls
        if let Some(custom) = self.custom_impl() {
            impls.extend(custom);
        }

        // derive
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

    // traits
    // returns the list of traits for this type
    fn traits(&self) -> Vec<crate::node_traits::Trait> {
        Vec::new()
    }

    // custom_impl
    fn custom_impl(&self) -> Option<TokenStream> {
        None
    }

    // map_trait
    // if None is returned it means that this trait should be derived
    // otherwise it's the code for the implementation
    fn map_trait(&self, _: crate::node_traits::Trait) -> Option<TokenStream> {
        None
    }

    // map_attribute
    // extra attributes for the derive
    fn map_attribute(&self, _: crate::node_traits::Trait) -> Option<TokenStream> {
        None
    }
}

///
/// Renderable
/// how this type is rendered within the code, ie a newtype within a struct
///

pub trait Renderable {
    fn render(&self) -> TokenStream;
}

///
/// Schemable
/// an element that can generate schema tokens
///

pub trait Schemable {
    // schema_tokens
    // generates the structure passed via ctor to the static schema
    #[must_use]
    fn schema_tokens(&self) -> TokenStream {
        let mut rng = RNG.lock().expect("Failed to lock RNG");
        let ctor_fn = format_ident!("ctor_{}", rng.next_u32());
        let schema = self.schema();

        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[::mimic::export::ctor::ctor]
            fn #ctor_fn() {
                ::mimic::schema::build::schema_write().insert_node(
                    #schema
                );
            }
        }
    }

    // schema
    fn schema(&self) -> TokenStream;
}
