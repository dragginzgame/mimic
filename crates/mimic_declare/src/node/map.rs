use crate::{
    node::{Def, Item, Type, Value},
    node_traits::{Trait, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart,
        SchemaNodeKind,
    },
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

impl HasIdent for Map {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for Map {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Map
    }
}

impl HasSchemaPart for Map {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let key = self.key.schema_part();
        let value = self.value.schema_part();
        let ty = self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Map {
                def: #def,
                key: #key,
                value: #value,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for Map {
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

impl HasTypePart for Map {
    fn type_part(&self) -> TokenStream {
        let ident = self.ident();
        let key = &self.key.type_part();
        let value = &self.value.type_part();

        quote! {
            pub struct #ident(::std::collections::HashMap<#key, #value>);
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let key_view = HasTypePart::view_type_part(&self.key);
        let value_view = HasTypePart::view_type_part(&self.value);

        quote! {
            pub type #view_ident = Vec<(#key_view, #value_view)>;
        }
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
