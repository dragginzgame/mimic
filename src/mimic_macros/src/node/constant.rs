use crate::{
    node::{Arg, ConstantType, Def, MacroNode, Node},
    traits::Schemable,
};
use darling::FromMeta;
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

impl Node for Constant {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let ty = &self.ty;
        let value = &self.value;
        let q = quote! {
            #schema
            pub const #ident: #ty = #value;
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for Constant {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Constant {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let ty = &self.ty.schema();
        let value = &self.value.schema();

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Constant(::mimic::orm::schema::node::Constant {
                def: #def,
                ty: #ty,
                value: #value,
            })
        }
    }
}
