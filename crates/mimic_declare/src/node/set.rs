use crate::{
    node::{Def, Item, Type},
    node_traits::{self, Imp, Trait, Traits},
    traits::{AsMacro, AsSchema, AsType},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Set
///

#[derive(Debug, FromMeta)]
pub struct Set {
    #[darling(default, skip)]
    pub def: Def,

    pub item: Item,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl AsMacro for Set {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_extra(&self) -> TokenStream {
        self.view_tokens()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();
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
        crate::node::imp::set::tokens(self)
    }
}

impl AsSchema for Set {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let item = self.item.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Set(::mimic::schema::node::Set {
                def: #def,
                item: #item,
                ty: #ty,
            })
        }
    }
}

impl AsType for Set {
    fn ty(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        // quote
        quote! {
            pub struct #ident(::std::collections::HashSet<#item>);
        }
    }

    fn view(&self) -> TokenStream {
        let view_ident = &self.def.view_ident();
        let item_view = AsType::view(&self.item);

        quote! {
            pub struct #view_ident(Vec<#item_view>);
        }
    }
}

impl ToTokens for Set {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.type_tokens());
    }
}
