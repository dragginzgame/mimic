use crate::{
    node::{Def, Item, Type, Value},
    node_traits::{self, Imp, Trait, Traits},
    traits::{AsMacro, AsSchema, AsType},
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

impl AsMacro for Map {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_extra(&self) -> TokenStream {
        self.view_tokens()
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
        crate::node::imp::map::tokens(self)
    }
}

impl AsSchema for Map {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let key = self.key.schema();
        let value = self.value.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Map(::mimic::schema::node::Map {
                def: #def,
                key: #key,
                value: #value,
                ty: #ty,
            })
        }
    }
}

impl AsType for Map {
    fn ty(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let key = &self.key;
        let value = &self.value;

        quote! {
            pub struct #ident(::std::collections::HashMap<#key, #value>);
        }
    }

    fn view(&self) -> TokenStream {
        let view_ident = self.def.view_ident();
        let key_view = AsType::view(&self.key);
        let value_view = AsType::view(&self.value);

        quote! {
            pub struct #view_ident(
                ::std::collections::HashMap<#key_view, #value_view>
            );
        }
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.type_tokens())
    }
}
