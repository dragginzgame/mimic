use crate::{
    node::{Def, FieldList},
    schema_traits::{Trait, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasTypePart, SchemaNodeKind,
    },
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

impl HasIdent for Validator {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for Validator {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Validator
    }
}

impl HasSchemaPart for Validator {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let fields = self.fields.schema_part();

        quote! {
            ::mimic::schema::node::Validator {
                def: #def,
                fields: #fields,
            }
        }
    }
}

impl HasTraits for Validator {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().with_default_traits();
        traits.add(Trait::Default);

        traits.list()
    }
}

impl HasTypePart for Validator {
    fn type_part(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let fields = self.fields.type_part();

        quote! {
            pub struct #ident {
                #fields
            }
        }
    }
}

impl ToTokens for Validator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
