use crate::{
    node::{Def, FieldList},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

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
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().with_default_traits();
        traits.add(Trait::Default);

        traits.list()
    }
}

impl AsSchema for Validator {
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

impl ToTokens for Validator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        tokens.extend(quote! {
            pub struct #ident {
                #fields
            }
        });
    }
}
