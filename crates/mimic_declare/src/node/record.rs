use crate::{
    node::{Def, FieldList, Type},
    node_traits::{self, Imp, Trait, Traits},
    traits::{Macro, Schemable},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Record
///

#[derive(Debug, FromMeta)]
pub struct Record {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub fields: FieldList,

    #[darling(default)]
    pub traits: Traits,

    #[darling(default)]
    pub ty: Type,
}

impl ToTokens for Record {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;

        // view
        let view_ident = &self.def.view_ident();
        let view = self.fields.type_view_fields(view_ident);

        // quote
        tokens.extend(quote! {
            pub struct #ident {
                #fields
            }

            #view
        });
    }
}

impl Macro for Record {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.fields.has_default() => {
                node_traits::DefaultTrait::tokens(self, t)
            }
            Trait::TypeView => node_traits::TypeViewTrait::tokens(self, t),
            Trait::ValidateAuto => node_traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => node_traits::any(self, t),
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default => Trait::Default.derive_attribute(),
            _ => None,
        }
    }
}

impl Schemable for Record {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let fields = self.fields.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::Schemable::Record(::mimic::schema::node::Record {
                def: #def,
                fields: #fields,
                ty: #ty,
            })
        }
    }
}
