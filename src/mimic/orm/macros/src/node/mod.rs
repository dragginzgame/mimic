mod arg;
mod canister;
mod constant;
mod def;
mod entity;
mod r#enum;
mod enum_hash;
mod field;
mod fixture;
mod guide;
mod item;
mod map;
mod newtype;
mod permission;
mod primitive;
mod record;
mod role;
mod sanitizer;
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
pub use self::enum_hash::*;
pub use self::field::*;
pub use self::fixture::*;
pub use self::guide::*;
pub use self::item::*;
pub use self::map::*;
pub use self::newtype::*;
pub use self::permission::*;
pub use self::primitive::*;
pub use self::r#enum::*;
pub use self::r#type::*;
pub use self::record::*;
pub use self::role::*;
pub use self::sanitizer::*;
pub use self::sort_key::*;
pub use self::store::*;
pub use self::traits::*;
pub use self::tuple::*;
pub use self::validator::*;
pub use self::value::*;

pub const PRIM_ULID: &str = "base::types::Ulid";

use crate::helper::{quote_one, to_path};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use syn::Path;

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

pub trait MacroNode: Schemable {
    // def
    fn def(&self) -> &Def;

    // ctor_schema
    // formats the code needed to send something via ctor to the schema
    #[must_use]
    fn ctor_schema(&self) -> TokenStream {
        let ctor_fn = format_ident!("ctor_{}", lib_rand::next_u64());
        let schema = self.schema();

        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[::ctor::ctor]
            fn #ctor_fn() {
                ::mimic::orm::schema::build::schema_write().add_node(
                    #schema
                );
            }
        }
    }
}

///
/// TraitNode
///

pub trait TraitNode: MacroNode {
    // traits
    // for each type this should return the list of traits it requires
    // want to make the function explicit to make it less confusing
    fn traits(&self) -> Vec<Trait>;

    // derive
    fn derive(&self) -> TokenStream {
        let mut derives = Vec::new();

        // map_derive checks if we should derive it
        for t in self.traits() {
            if let Some(path) = t.derive_path() {
                if self.map_derive(t) {
                    derives.push(path);
                }
            }
        }

        if derives.is_empty() {
            quote!()
        } else {
            quote! {
                #[derive(#(#derives),*)]
            }
        }
    }

    // derive_struct
    // includes the extra attributes that a struct needs
    fn derive_struct(&self) -> TokenStream {
        let mut q = self.derive();

        // attributes
        if self.traits().contains(&Trait::Default) {
            q.extend(quote! {
                #[serde(default)]
            });
        }

        q
    }

    // map_derive
    // should a deriveable trait be derived?
    fn map_derive(&self, _: Trait) -> bool {
        true
    }

    /// imp
    /// every trait that returns Some(tokens) is an impl block
    fn imp(&self) -> TokenStream {
        let mut output = quote!();

        for t in self.traits() {
            output.extend(self.map_imp(t));
        }

        output
    }

    // map_imp
    // passes through the trait to the impl generator function
    fn map_imp(&self, t: Trait) -> TokenStream;
}

///
/// NODES
///

///
/// AccessPolicy
///

#[derive(Clone, Debug, Default, FromMeta)]
pub enum AccessPolicy {
    #[default]
    Deny,

    Allow,
    Permission(Path),
}

impl Schemable for AccessPolicy {
    fn schema(&self) -> TokenStream {
        match &self {
            Self::Allow => quote!(::mimic::orm::schema::node::AccessPolicy::Allow),
            Self::Deny => quote!(::mimic::orm::schema::node::AccessPolicy::Deny),
            Self::Permission(path) => {
                let path = quote_one(path, to_path);
                quote!(::mimic::orm::schema::node::AccessPolicy::Permission(#path))
            }
        }
    }
}

///
/// Crud
///

#[derive(Debug, Default, FromMeta)]
pub struct Crud {
    #[darling(default)]
    pub load: AccessPolicy,

    #[darling(default)]
    pub save: AccessPolicy,

    #[darling(default)]
    pub delete: AccessPolicy,
}

impl Schemable for Crud {
    fn schema(&self) -> TokenStream {
        let load = &self.load.schema();
        let save = &self.save.schema();
        let delete = &self.delete.schema();

        quote! {
            ::mimic::orm::schema::node::Crud {
                load: #load,
                save: #save,
                delete: #delete,
            }
        }
    }
}
