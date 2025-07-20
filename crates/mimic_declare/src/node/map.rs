use crate::{
    node::{Def, Item, Type, Value},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

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
    fn ident(&self) -> Ident {
        self.def.ident.clone()
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

impl AsSchema for Map {
    const EMIT_SCHEMA: bool = true;

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
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();
        let key = &self.key;
        let value = &self.value;

        Some(quote! {
            pub struct #ident(::std::collections::HashMap<#key, #value>);
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let view_ident = self.view_ident();
        let key_view = AsType::as_view_type(&self.key);
        let value_view = AsType::as_view_type(&self.value);

        Some(quote! {
            pub type #view_ident = Vec<(#key_view, #value_view)>;
        })
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
