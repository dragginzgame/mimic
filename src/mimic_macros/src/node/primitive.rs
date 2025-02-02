use crate::{
    helper::quote_one,
    node::{Def, MacroNode, Node, PrimitiveType, Type},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Path;

///
/// Primitive
///

#[derive(Debug, FromMeta)]
pub struct Primitive {
    #[darling(default, skip)]
    pub def: Def,

    pub variant: PrimitiveType,
    pub path: Path,

    #[darling(default)]
    pub ty: Type,
}

impl Node for Primitive {
    fn expand(&self) -> TokenStream {
        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #self
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
        let variant = quote_one(&self.variant, PrimitiveType::schema);
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Primitive(::mimic::schema::node::Primitive {
                def: #def,
                variant: #variant,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for Primitive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.def.ident;
        let path = &self.path;

        tokens.extend(quote! {
            pub type #ident = #path;
        })
    }
}
