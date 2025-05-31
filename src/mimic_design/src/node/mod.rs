mod arg;
mod canister;
mod constant;
mod def;
mod entity;
mod entity_id;
mod r#enum;
mod enum_value;
mod field;
mod item;
mod list;
mod map;
mod newtype;
mod primitive;
mod record;
mod selector;
mod set;
mod sort_key;
mod store;
mod traits;
mod tuple;
mod r#type;
mod validator;
mod value;

// mostly just one or two types in each file so wildcard should be ok
pub use self::arg::*;
pub use self::canister::*;
pub use self::constant::*;
pub use self::def::*;
pub use self::entity::*;
pub use self::entity_id::*;
pub use self::r#enum::*;
pub use self::enum_value::*;
pub use self::field::*;
pub use self::item::*;
pub use self::list::*;
pub use self::map::*;
pub use self::newtype::*;
pub use self::primitive::*;
pub use self::record::*;
pub use self::selector::*;
pub use self::set::*;
pub use self::sort_key::*;
pub use self::store::*;
pub use self::traits::*;
pub use self::tuple::*;
pub use self::r#type::*;
pub use self::validator::*;
pub use self::value::*;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// NODE TRAITS
///

///
/// Node
///

pub trait Node {
    fn expand(&self) -> TokenStream;
}

///
/// MacroNode
///

pub trait MacroNode {
    fn def(&self) -> &Def;
}

///
/// TraitNode
///

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

pub trait TraitNode: MacroNode {
    // traits
    // returns the list of traits for this type
    fn traits(&self) -> Vec<Trait>;

    // trait_tokens
    fn trait_tokens(&self) -> TraitTokens {
        let mut impls = quote!();
        let traits = self.traits();

        // we only derive traits that have no map_imp tokens
        let mut derived_traits = Vec::new();
        for t in traits {
            match self.map_trait(t) {
                Some(tokens) => impls.extend(tokens),
                None => derived_traits.push(
                    t.derive_path()
                        .unwrap_or_else(|| panic!("trait '{t}' cannot be derived")),
                ),
            }
        }

        // derive
        let mut derive = if derived_traits.is_empty() {
            quote!()
        } else {
            quote! {
                #[derive(#(#derived_traits),*)]
            }
        };

        // derive attributes
        if let Some(attrs) = self.derive_attributes() {
            derive.extend(attrs);
        }

        TraitTokens { derive, impls }
    }

    // map_trait
    // if None is returned it means that this trait should be derived
    // otherwise it's the code for the implementation
    fn map_trait(&self, t: Trait) -> Option<TokenStream>;

    // derive_attributes
    // extra attributes for the derive
    fn derive_attributes(&self) -> Option<TokenStream> {
        None
    }
}

///
/// NODE HELPERS
///

///
/// Sorted
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct Sorted(bool);

impl ToTokens for Sorted {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.0 {
            tokens.extend(quote!(#[::mimic::export::remain::sorted]));
        }
    }
}
