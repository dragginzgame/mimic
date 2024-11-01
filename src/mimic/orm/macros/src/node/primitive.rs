use crate::{
    helper::quote_one,
    node::{Def, MacroNode, Node, PrimitiveType},
};
use darling::FromMeta;
use orm_schema::traits::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

///
/// Primitive
///

#[derive(Debug, FromMeta)]
pub struct Primitive {
    #[darling(default, skip)]
    pub def: Def,

    pub ty: PrimitiveType,
    pub path: Path,
}

impl Node for Primitive {
    fn expand(&self) -> TokenStream {
        let Self { path, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            pub type #ident = #path;
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for Primitive {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Primitive {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let ty = quote_one(&self.ty, PrimitiveType::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Primitive(::mimic::orm::schema::node::Primitive {
                def: #def,
                ty: #ty,
            })
        }
    }
}
