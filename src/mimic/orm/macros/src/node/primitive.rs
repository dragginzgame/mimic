use crate::{
    helper::{quote_one, to_string},
    node::{Def, MacroNode, Node},
};
use darling::FromMeta;
use orm::types::PrimitiveType;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
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

    #[darling(default)]
    pub debug: bool,
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
        let ty = quote_one(&self.ty, PrimitiveType::schema);
        let path = quote_one(&self.path, to_string);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Primitive(::mimic::orm::schema::node::Primitive {
                def: #def,
                ty: #ty,
                path: #path,
            })
        }
    }
}
