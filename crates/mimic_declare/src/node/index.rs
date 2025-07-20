use crate::{
    helper::{quote_one, quote_slice, split_idents, to_path, to_str_lit},
    node::Def,
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter},
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

impl AsMacro for Index {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().with_path_trait();
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

impl AsSchema for Index {
    const EMIT_SCHEMA: bool = false;

    fn schema(&self) -> TokenStream {
        let entity = quote_one(&self.entity, to_path);
        let fields = quote_slice(&self.fields, to_str_lit);
        let store = quote_one(&self.store, to_path);
        let unique = &self.unique;

        quote! {
            ::mimic::schema::node::Index {
                store: #store,
                entity: #entity,
                fields: #fields,
                unique: #unique,
            }
        }
    }
}

impl AsType for Index {
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();

        Some(quote! {
            pub struct #ident {}
        })
    }
}

impl ToTokens for Index {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens())
    }
}
