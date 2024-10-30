use super::{
    helper::{quote_one, quote_vec, to_path, to_string},
    Def, MacroNode, Node, Trait, TraitNode, Traits,
};
use crate::imp;
use darling::FromMeta;
use orm::types::Sorted;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

///
/// EntityKey
///

#[derive(Debug, FromMeta)]
pub struct EntityKey {
    #[darling(default, skip)]
    pub def: Def,

    pub entity: Path,

    #[darling(default)]
    pub sorted: Sorted,

    #[darling(multiple, rename = "key")]
    pub keys: Vec<Ident>,
}

impl Node for EntityKey {
    fn expand(&self) -> TokenStream {
        let Self { sorted, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let keys = self.keys.iter().map(quote::ToTokens::to_token_stream);
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            #sorted
            pub enum #ident {
                #(#keys,)*
            }
            #imp
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for EntityKey {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for EntityKey {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let entity = quote_one(&self.entity, to_path);
        let keys = quote_vec(&self.keys, to_string);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::EntityKey(
                ::mimic::orm::schema::node::EntityKey{
                    def: #def,
                    entity: #entity,
                    keys: #keys,
                }
            )
        }
    }
}

impl TraitNode for EntityKey {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default();
        traits.extend(vec![Trait::Copy, Trait::EnumDisplay, Trait::EnumStaticStr]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
