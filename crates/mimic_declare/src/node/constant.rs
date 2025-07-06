use crate::{
    node::{Arg, Def},
    traits::{AsMacro, AsSchema},
};
use darling::FromMeta;
use mimic_schema::types::ConstantType;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

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

impl AsMacro for Constant {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl AsSchema for Constant {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let ty = &self.ty;
        let value = &self.value.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Constant(::mimic::schema::node::Constant {
                def: #def,
                ty: #ty,
                value: #value,
            })
        }
    }
}

impl ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;

        // quote
        let ty = &self.ty.as_type();
        let value = &self.value;

        tokens.extend(quote! {
            pub const #ident: #ty = #value;
        })
    }
}
