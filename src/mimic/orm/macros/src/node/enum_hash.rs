use crate::{
    helper::{quote_vec, to_string},
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm::types::Sorted;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

///
/// EnumHash
///

#[derive(Debug, FromMeta)]
pub struct EnumHash {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub sorted: Sorted,

    #[darling(multiple, rename = "key")]
    pub keys: Vec<Ident>,
}

impl Node for EnumHash {
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
            return quote!(compile_error!(#s));
        }

        q
    }
}

impl MacroNode for EnumHash {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for EnumHash {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let keys = quote_vec(&self.keys, to_string);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::EnumHash(::mimic::orm::schema::node::EnumHash{
                def: #def,
                keys: #keys,
            })
        }
    }
}

impl TraitNode for EnumHash {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default();
        traits.extend(vec![Trait::Copy, Trait::EnumHash]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::EnumHash => imp::enum_hash::enum_hash(self, t),

            _ => imp::any(self, t),
        }
    }
}
