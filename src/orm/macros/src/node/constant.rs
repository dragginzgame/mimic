use crate::node::{Def, MacroNode, Node};
use darling::FromMeta;
use proc_macro2::TokenStream;
use schema::Schemable;
use syn::{Expr, Type};

///
/// Constant
///

#[derive(Debug, FromMeta)]
pub struct Constant {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,

    pub ty: Type,
    pub value: Expr,
}

impl Node for Constant {
    fn expand(&self) -> TokenStream {
        let Self { ty, value, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #[allow(non_camel_case_types)]
            pub const #ident: #ty = #value;
        };

        // debug
        assert!(!self.debug, "{q}");

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

        quote! {
            ::mimic::schema::node::SchemaNode::Constant(::mimic::schema::node::Constant{
                def: #def,
            })
        }
    }
}
