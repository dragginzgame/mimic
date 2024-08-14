use crate::{
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm::types::Cycles;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Canister
/// regardless of the path, the name is used to uniquely identify each canister
///

#[derive(Debug, FromMeta)]
pub struct Canister {
    #[darling(skip, default)]
    pub def: Def,

    pub initial_cycles: Cycles,
    pub min_cycles: Cycles,
    pub build: CanisterBuild,
}

impl Node for Canister {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def();

        // quote
        let schema = self.ctor_schema();
        let imp = &self.imp();
        let q = quote! {
            #schema
            pub struct #ident {}
            #imp
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s));
        }

        q
    }
}

impl MacroNode for Canister {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Canister {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let initial_cycles = &self.initial_cycles.schema();
        let min_cycles = &self.min_cycles.schema();
        let build = self.build.schema();

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Canister(::mimic::orm::schema::node::Canister{
                def: #def,
                initial_cycles: #initial_cycles,
                min_cycles: #min_cycles,
                build: #build,
            })
        }
    }
}

impl TraitNode for Canister {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}

///
/// CanisterBuild
/// canister logic is just v1 for now, we may have more overlaps between
///

#[derive(Debug, FromMeta)]
pub enum CanisterBuild {
    Basic(CanisterBuildBasic),
    Root,
    Test,
    User,
}

impl Schemable for CanisterBuild {
    fn schema(&self) -> TokenStream {
        match &self {
            Self::Basic(canister) => {
                let canister = canister.schema();
                quote!(::mimic::orm::schema::node::CanisterBuild::Basic(#canister))
            }
            Self::Root => quote!(::mimic::orm::schema::node::CanisterBuild::Root),
            Self::Test => quote!(::mimic::orm::schema::node::CanisterBuild::Test),
            Self::User => quote!(::mimic::orm::schema::node::CanisterBuild::User),
        }
    }
}

///
/// CanisterBuildBasic
///

#[derive(Debug, FromMeta)]
pub struct CanisterBuildBasic {
    #[darling(default)]
    pub replicated: bool,
}

impl Schemable for CanisterBuildBasic {
    fn schema(&self) -> TokenStream {
        let replicated = self.replicated;

        quote! {
            ::mimic::orm::schema::node::CanisterBuildBasic{
                replicated: #replicated,
            }
        }
    }
}
