use crate::{
    helper::{quote_one, quote_slice, split_idents, to_path, to_str_lit},
    node::Def,
    node_traits::{Trait, TraitStrategy, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasTypePart, SchemaNodeKind,
    },
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

///
/// Index
///

#[derive(Debug, FromMeta)]
pub struct Index {
    #[darling(default, skip)]
    pub def: Def,

    pub store: Path,
    pub entity: Path,

    #[darling(default, map = "split_idents")]
    pub fields: Vec<Ident>,

    #[darling(default)]
    pub unique: bool,
}

impl HasIdent for Index {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for Index {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Index
    }
}

impl HasSchemaPart for Index {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let store = quote_one(&self.store, to_path);
        let entity = quote_one(&self.entity, to_path);
        let fields = quote_slice(&self.fields, to_str_lit);
        let unique = &self.unique;

        quote! {
            ::mimic::schema::node::Index {
                def: #def,
                store: #store,
                entity: #entity,
                fields: #fields,
                unique: #unique,
            }
        }
    }
}

impl HasTraits for Index {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().with_path_trait();
        traits.extend(vec![Trait::IndexKind]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::node_traits::*;

        match t {
            Trait::IndexKind => IndexKindTrait::strategy(self),
            _ => None,
        }
    }
}

impl HasTypePart for Index {
    fn type_part(&self) -> TokenStream {
        let ident = self.ident();

        quote! {
            pub struct #ident {}
        }
    }
}

impl ToTokens for Index {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
