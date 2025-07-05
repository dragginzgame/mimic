use crate::{
    node::{Arg, Def},
    traits::{Macro, Schemable},
};
use darling::FromMeta;
use mimic_schema::types::ConstantType;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Constant
///

#[derive(Debug, FromMeta)]
pub struct Constant {
    #[darling(default, skip)]
    pub def: Def,

    pub ty: ConstantType,
    pub value: Arg,
}

impl Macro for Constant {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_body(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let ty = &self.ty.as_type();
        let value = &self.value;

        quote! {
            pub const #ident: #ty = #value;
        }
    }
}

impl Schemable for Constant {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let ty = &self.ty;
        let value = &self.value.schema();

        quote! {
            ::mimic::schema::node::Schemable::Constant(::mimic::schema::node::Constant {
                def: #def,
                ty: #ty,
                value: #value,
            })
        }
    }
}
