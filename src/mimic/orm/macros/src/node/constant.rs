use crate::node::{ArgNumber, Def, MacroNode, Node};
use darling::FromMeta;
use orm::types::PrimitiveType;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Constant
///

#[derive(Debug, FromMeta)]
pub struct Constant {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,

    pub ty: PrimitiveType,
    pub value: ArgNumber,
}

impl Node for Constant {
    fn expand(&self) -> TokenStream {
        let Self { ty, value, .. } = self;
        let Def { ident, .. } = &self.def;

        // strip annotations
        let value = value.to_tokens_stripped();

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #[allow(non_camel_case_types)]
            pub const #ident: #ty = #value;
        };

        // debug
        if self.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s));
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
        let def = self.def.schema();
        let ty = self.ty.schema();
        let value = self.value.schema();

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Constant(::mimic::orm::schema::node::Constant{
                def: #def,
                ty: #ty,
                value: #value,
            })
        }
    }
}
