use crate::{
    helper::{quote_one, quote_slice, split_idents, to_path, to_str_lit},
    node::{Def, FieldList, Type},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter},
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
    pub indexes: Vec<EntityIndex>,

    #[darling(default)]
    pub fields: FieldList,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl AsMacro for Entity {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![
            Trait::EntityAccessor,
            Trait::EntityKind,
            Trait::EntityFixture,
            Trait::HasStore,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::Default if self.fields.has_default() => DefaultTrait::tokens(self),
            Trait::From => FromTrait::tokens(self),
            Trait::EntityAccessor => EntityAccessorTrait::tokens(self),
            Trait::EntityKind => EntityKindTrait::tokens(self),
            Trait::HasStore => HasStoreTrait::tokens(self),
            Trait::TypeView => TypeViewTrait::tokens(self),
            Trait::ValidateAuto => ValidateAutoTrait::tokens(self),
            Trait::Visitable => VisitableTrait::tokens(self),

            _ => None,
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default => Trait::Default.derive_attribute(),
            _ => None,
        }
    }

    fn macro_children(&self) -> Vec<TokenStream> {
        self.indexes.iter().map(|ix| ix.all_tokens()).collect()
    }
}

impl AsSchema for Entity {
    const EMIT_SCHEMA: bool = true;

    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let store = quote_one(&self.store, to_path);
        let primary_key = quote_one(&self.primary_key, to_str_lit);
        let indexes = quote_slice(&self.indexes, EntityIndex::schema);
        let fields = &self.fields.schema();
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Entity(::mimic::schema::node::Entity {
                def: #def,
                store: #store,
                primary_key: #primary_key,
                indexes: #indexes,
                fields: #fields,
                ty: #ty,
            })
        }
    }
}

impl AsType for Entity {
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();
        let Self { fields, .. } = self;

        Some(quote! {
            pub struct #ident {
                #fields
            }
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let derives = Self::view_derives();
        let ident = self.ident();
        let view_ident = self.view_ident();
        let view_field_list = AsType::as_view_type(&self.fields);

        Some(quote! {
            #derives
            pub struct #view_ident {
                #view_field_list
            }

            impl Default for #view_ident {
                fn default() -> Self {
                    #ident::default().to_view()
                }
            }
        })
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}

///
/// EntityIndex
///

#[derive(Debug, FromMeta)]
pub struct EntityIndex {
    #[darling(default, map = "split_idents")]
    pub fields: Vec<Ident>,

    #[darling(default)]
    pub unique: bool,

    pub store: Path,
}

impl AsMacro for EntityIndex {
    fn ident(&self) -> Ident {
        generate_index_ident(self)
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default();
        traits.extend(vec![Trait::HasStore]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::HasStore => HasStoreTrait::tokens(self),
            _ => None,
        }
    }
}

impl AsSchema for EntityIndex {
    const EMIT_SCHEMA: bool = false;

    fn schema(&self) -> TokenStream {
        let fields = quote_slice(&self.fields, to_str_lit);
        let unique = &self.unique;
        let store = quote_one(&self.store, to_path);

        quote! {
            ::mimic::schema::node::EntityIndex {
                fields: #fields,
                unique: #unique,
                store: #store,
            }
        }
    }
}

impl AsType for EntityIndex {
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();

        Some(quote! {
            pub struct #ident {}
        })
    }
}

impl ToTokens for EntityIndex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens())
    }
}
