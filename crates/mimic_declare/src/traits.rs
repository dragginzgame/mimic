use crate::{
    node::Def,
    node_traits::{Trait, TraitList},
};
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

pub trait AsMacro: AsSchema + quote::ToTokens {
    fn def(&self) -> &Def;

    /// Returns any extra tokens to emit *after* the main item and its impls.
    fn macro_extra(&self) -> TokenStream {
        quote!()
    }

    // traits
    // returns the list of traits for this type
    fn traits(&self) -> Vec<Trait> {
        Vec::new()
    }

    // map_attribute
    // extra attributes for the derive
    fn map_attribute(&self, _: Trait) -> Option<TokenStream> {
        None
    }

    // map_trait
    // if None is returned it means that this trait should be derived
    // otherwise it's the code for the implementation
    fn map_trait(&self, _: Trait) -> Option<TokenStream> {
        None
    }
}

///
/// AsSchema
/// an element that can generate schema tokens
///

pub trait AsSchema {
    // schema
    fn schema(&self) -> TokenStream {
        quote!()
    }

    // schema_tokens
    // generates the structure passed via ctor to the static schema
    #[must_use]
    fn schema_tokens(&self) -> TokenStream {
        let schema = self.schema();
        if schema.is_empty() {
            return quote!();
        }

        let mut rng = RNG.lock().expect("Failed to lock RNG");
        let ctor_fn = format_ident!("ctor_{}", rng.next_u32());

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
}

///
/// AsType
/// an element that can define a rust type
///

pub trait AsType {
    fn as_type(&self) -> TokenStream;

    fn as_view_type(&self) -> TokenStream;

    fn view_default(&self) -> TokenStream;

    fn basic_derives() -> TokenStream {
        TraitList(vec![
            Trait::CandidType,
            Trait::Debug,
            Trait::Serialize,
            Trait::Deserialize,
        ])
        .to_derive_tokens()
    }
}
