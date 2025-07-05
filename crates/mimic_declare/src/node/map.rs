use crate::{
    node::{Def, Item, Type, Value},
    node_traits::{self, Imp, Trait, Traits},
    traits::{Macro, Schemable},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Map
///

#[derive(Debug, FromMeta)]
pub struct Map {
    #[darling(default, skip)]
    pub def: Def,

    pub key: Item,
    pub value: Value,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Macro for Map {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_body(&self) -> TokenStream {
        quote! { self }
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::Deref,
            Trait::DerefMut,
            Trait::IntoIterator,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::ValidateAuto => node_traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => node_traits::any(self, t),
        }
    }

    fn custom_impl(&self) -> Option<TokenStream> {
        crate::node::imp::map::tokens(self)
    }
}

impl Schemable for Map {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let key = self.key.schema();
        let value = self.value.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::Schemable::Map(::mimic::schema::node::Map {
                def: #def,
                key: #key,
                value: #value,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let key = &self.key;
        let value = &self.value;

        tokens.extend(quote! {
            pub struct #ident(::std::collections::HashMap<#key, #value>);
        });
    }
}
