use crate::{
    node::{Arg, Def},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter},
};
use darling::FromMeta;
use mimic_schema::types::ConstantType;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

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
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl AsSchema for Constant {
    const EMIT_SCHEMA: bool = true;

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

impl AsType for Constant {
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();
        let ty = &self.ty.as_type();
        let value = &self.value;

        Some(quote! {
            pub const #ident: #ty = #value;
        })
    }
}

impl ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
