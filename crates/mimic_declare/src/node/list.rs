use crate::{
    node::{Def, Item, Type},
    node_traits::{self, Imp, Trait, Traits},
    traits::{AsMacro, AsSchema},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// List
///

#[derive(Debug, FromMeta)]
pub struct List {
    #[darling(default, skip)]
    pub def: Def,

    pub item: Item,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl AsMacro for List {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_extra(&self) -> TokenStream {
        let item = &self.item;
        let view_ident = &self.def.view_ident();

        quote! {
            #[derive(CandidType)]
            #[allow(non_camel_case_types)]
            pub struct #view_ident(Vec<<#item as ::mimic::core::traits::TypeView>::View>);
        }
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
            Trait::TypeView => node_traits::TypeViewTrait::tokens(self, t),
            Trait::ValidateAuto => node_traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => None,
        }
    }

    fn custom_impl(&self) -> Option<TokenStream> {
        crate::node::imp::list::tokens(self)
    }
}

impl ToTokens for List {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        tokens.extend(quote! {
            pub struct #ident(Vec<#item>);
        });
    }
}

impl AsSchema for List {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let item = self.item.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::List(::mimic::schema::node::List {
                def: #def,
                item: #item,
                ty: #ty,
            })
        }
    }
}
