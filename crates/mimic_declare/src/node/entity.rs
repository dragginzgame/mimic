use crate::{
    helper::{quote_one, quote_slice, to_path, to_str_lit},
    imp::TraitStrategy,
    node::{Def, Field, FieldList, Index, Type},
    schema_traits::{Trait, TraitList, Traits},
    traits::{
        HasDef, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart, SchemaNodeKind,
    },
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

///
/// Entity
///

#[derive(Debug, FromMeta)]
pub struct Entity {
    #[darling(default, skip)]
    pub def: Def,

    pub store: Path,

    #[darling(rename = "pk")]
    pub primary_key: Ident,

    #[darling(multiple, rename = "index")]
    pub indexes: Vec<Index>,

    #[darling(default, map = "Entity::add_metadata")]
    pub fields: FieldList,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Entity {
    fn add_metadata(mut fields: FieldList) -> FieldList {
        fields.push(Field::created_at());
        fields.push(Field::updated_at());

        fields
    }
}

impl HasDef for Entity {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Entity {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Entity
    }
}

impl HasSchemaPart for Entity {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let store = quote_one(&self.store, to_path);
        let primary_key = quote_one(&self.primary_key, to_str_lit);
        let indexes = quote_slice(&self.indexes, Index::schema_part);
        let fields = &self.fields.schema_part();
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Entity {
                def: #def,
                store: #store,
                primary_key: #primary_key,
                indexes: #indexes,
                fields: #fields,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for Entity {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![
            Trait::Inherent,
            Trait::EntityKind,
            Trait::EntityFixture,
            Trait::EntityLifecycle,
            Trait::FieldValues,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::Inherent => InherentTrait::strategy(self),

            Trait::Default => DefaultTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::EntityKind => EntityKindTrait::strategy(self),
            Trait::EntityLifecycle => EntityLifecycleTrait::strategy(self),
            Trait::FieldValues => FieldValuesTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default => Trait::Default.derive_attribute(),
            _ => None,
        }
    }
}

impl HasType for Entity {}

impl HasTypePart for Entity {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let fields = self.fields.type_part();

        quote! {
            pub struct #ident {
                #fields
            }
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let derives = self.view_derives();
        let ident = self.def.ident();
        let view_ident = self.view_ident();
        let view_field_list = HasTypePart::view_type_part(&self.fields);

        quote! {
            #derives
            pub struct #view_ident {
                #view_field_list
            }

            impl Default for #view_ident {
                fn default() -> Self {
                    #ident::default().to_view()
                }
            }
        }
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
