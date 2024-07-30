use crate::{
    helper::{quote_one, to_string},
    node::{Def, MacroNode, Node},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use schema::Schemable;
use syn::Path;

///
/// Primitive
///

#[derive(Debug, FromMeta)]
pub struct Primitive {
    #[darling(default, skip)]
    pub def: Def,

    pub ty: Path,

    #[darling(default)]
    pub debug: bool,
}

impl Node for Primitive {
    fn expand(&self) -> TokenStream {
        let Self { ty, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            pub type #ident = #ty;
        };

        // debug
        assert!(!self.debug, "{q}");

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
        let ty = quote_one(&self.ty, to_string);

        quote! {
            ::mimic::schema::node::SchemaNode::Primitive(::mimic::schema::node::Primitive {
                def: #def,
                ty: #ty,
            })
        }
    }
}
