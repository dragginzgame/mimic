use crate::{
    helper::{quote_one, quote_vec, to_path},
    imp::{self, Imp},
    node::{
        Def, FieldList, Index, MacroNode, Node, SortKey, Trait, TraitNode, TraitTokens, Traits,
        Type,
    },
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

///
/// Entity
///

#[derive(Debug, FromMeta)]
pub struct Entity {
    #[darling(default, skip)]
    pub def: Def,

    pub store: Path,

    #[darling(multiple, rename = "sk")]
    pub sort_keys: Vec<SortKey>,

    #[darling(multiple, rename = "index")]
    pub indexes: Vec<Index>,

    #[darling(default)]
    pub fields: FieldList,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Entity {
    fn expand(&self) -> TokenStream {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #derive
            pub struct #ident {
                #fields
            }
            #impls
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for Entity {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Entity {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let store = quote_one(&self.store, to_path);
        let sort_keys = quote_vec(&self.sort_keys, SortKey::schema);
        let indexes = quote_vec(&self.indexes, Index::schema);
        let fields = &self.fields.schema();
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Entity(::mimic::schema::node::Entity {
                def: #def,
                store: #store,
                sort_keys: #sort_keys,
                indexes: #indexes,
                fields: #fields,
                ty: #ty,
            })
        }
    }
}

impl TraitNode for Entity {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::Entity,
            Trait::EntityDyn,
            Trait::EntityFixture,
            Trait::FieldSort,
            Trait::FieldFilter,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.fields.has_default() => {
                <imp::DefaultTrait as Imp<Self>>::tokens(self, t)
            }
            Trait::Entity => <imp::EntityTrait as Imp<Self>>::tokens(self, t),
            Trait::EntityDyn => <imp::EntityDynTrait as Imp<Self>>::tokens(self, t),
            Trait::FieldFilter => <imp::FieldFilterTrait as Imp<Self>>::tokens(self, t),
            Trait::FieldSort => <imp::FieldSortTrait as Imp<Self>>::tokens(self, t),
            Trait::ValidateAuto => <imp::ValidateAutoTrait as Imp<Self>>::tokens(self, t),
            Trait::Visitable => <imp::VisitableTrait as Imp<Self>>::tokens(self, t),

            _ => imp::any(self, t),
        }
    }

    fn derive_attributes(&self) -> Option<TokenStream> {
        self.traits()
            .contains(&Trait::Default)
            .then(|| quote! { #[serde(default)] })
    }
}
