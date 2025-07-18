use crate::{
    node::{Def, Item, Type},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType},
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
        self.as_view_type()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut, Trait::IntoIterator]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::From => FromTrait::tokens(self),
            Trait::TypeView => TypeViewTrait::tokens(self),
            Trait::ValidateAuto => ValidateAutoTrait::tokens(self),
            Trait::Visitable => VisitableTrait::tokens(self),

            _ => None,
        }
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

impl AsType for List {
    fn as_type(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        quote! {
            pub struct #ident(Vec<#item>);
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let view_ident = &self.def.view_ident();
        let item_view = AsType::as_view_type(&self.item);

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}

impl ToTokens for List {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
