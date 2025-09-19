use crate::{
    imp::TraitStrategy,
    node::{Def, Item, Type},
    schema_traits::{Trait, TraitList, Traits},
    traits::{
        HasDef, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart, SchemaNodeKind,
    },
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

impl HasDef for List {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for List {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::List
    }
}

impl HasSchemaPart for List {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let item = self.item.schema_part();
        let ty = self.ty.schema_part();

        quote! {
            ::mimic::schema::node::List {
                def: #def,
                item: #item,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for List {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut, Trait::IntoIterator]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::FieldValue => FieldValueTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for List {}

impl HasTypePart for List {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let item = &self.item.type_part();

        quote! {
            #[repr(transparent)]
            pub struct #ident(pub Vec<#item>);
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let item_view = HasTypePart::view_type_part(&self.item);

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}

impl ToTokens for List {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
