use crate::{
    imp::TraitStrategy,
    node::{Def, Item, Type, Value},
    schema_traits::{Trait, TraitList, Traits},
    traits::{
        HasDef, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart, SchemaNodeKind,
    },
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

impl HasDef for Map {
    fn def(&self) -> &Def {
        &self.def
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
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut, Trait::IntoIterator]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for Map {}

impl HasTypePart for Map {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let key = &self.key.type_part();
        let value = &self.value.type_part();

        quote! {
            #[repr(transparent)]
            pub struct #ident(pub ::std::collections::HashMap<#key, #value>);
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
