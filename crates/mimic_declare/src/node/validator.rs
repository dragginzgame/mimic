use crate::{
    node::{Def, FieldList},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter, SchemaKind},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// Validator
///

#[derive(Debug, FromMeta)]
pub struct Validator {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub fields: FieldList,
}

impl AsMacro for Validator {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().with_default_traits();
        traits.add(Trait::Default);

        traits.list()
    }
}

impl AsSchema for Validator {
    const KIND: SchemaKind = SchemaKind::Full;

    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let fields = self.fields.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Validator(::mimic::schema::node::Validator {
                def: #def,
                fields: #fields,
            })
        }
    }
}

impl AsType for Validator {
    fn as_type(&self) -> Option<TokenStream> {
        let Def { ident, .. } = &self.def;
        let fields = &self.fields;

        Some(quote! {
            pub struct #ident {
                #fields
            }
        })
    }
}

impl ToTokens for Validator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
