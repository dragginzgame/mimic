use crate::{
    node::{Def, Item, Type},
    node_traits::{self, Imp, Trait, Traits},
    traits::{MacroNode, SchemaNode, TypeNode},
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

impl ToTokens for List {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schema = self.schema_tokens();
        let ty = self.type_tokens();

        tokens.extend(quote! {
            #schema
            #ty
        });
    }
}

impl TypeNode for List {
    fn main_tokens(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        quote! {
            pub struct #ident(Vec<#item>);
        }
    }

    fn view_tokens(&self) -> TokenStream {
        let item = &self.item;
        let view_ident = &self.def.view_ident();

        quote! {
            pub struct #view_ident(Vec<<#item as ::mimic::core::traits::TypeView>::View>);
        }
    }
}

impl MacroNode for List {
    fn def(&self) -> &Def {
        &self.def
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
            //    Trait::TypeView => node_traits::TypeViewTrait::tokens(self, t),
            Trait::ValidateAuto => node_traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => node_traits::any(self, t),
        }
    }

    fn custom_impl(&self) -> Option<TokenStream> {
        crate::node::imp::list::tokens(self)
    }
}

impl SchemaNode for List {
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
