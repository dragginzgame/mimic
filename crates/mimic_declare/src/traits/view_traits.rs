use crate::{
    node::{Trait, TraitList},
    traits::HasDef,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

///
/// HasViewTypes
///
/// A node that emits additional derived view representations
/// (e.g., main View, Edit, Filter).
///
pub trait HasViewTypes: HasDef {
    fn view_parts(&self) -> TokenStream {
        quote!()
    }

    // Naming
    fn view_ident(&self) -> Ident {
        format_ident!("{}View", self.def().ident())
    }

    fn edit_ident(&self) -> Ident {
        format_ident!("{}Edit", self.def().ident())
    }

    fn filter_ident(&self) -> Ident {
        format_ident!("{}Filter", self.def().ident())
    }

    // Standard derives
    fn view_derives(&self) -> TraitList {
        TraitList(vec![
            Trait::CandidType,
            Trait::Clone,
            Trait::Debug,
            Trait::Default,
            Trait::Serialize,
            Trait::Deserialize,
        ])
    }
}

///
/// HasViewTypeExpr
///

pub trait HasViewTypeExpr {
    fn view_type_expr(&self) -> TokenStream {
        quote!()
    }

    fn filter_type_expr(&self) -> Option<TokenStream> {
        None
    }
}

///
/// HasViewDefault
/// just because these can't be derived easily
///

pub trait HasViewDefault {
    fn view_default_impl(&self) -> Option<TokenStream> {
        None
    }
}
