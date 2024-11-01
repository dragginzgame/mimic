use crate::imp;
use crate::{
    helper::{quote_one, quote_vec, to_path, to_string},
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm_schema::traits::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

///
/// EntitySource
///

#[derive(Debug, FromMeta)]
pub struct EntitySource {
    #[darling(default, skip)]
    pub def: Def,

    pub entity: Path,

    #[darling(multiple, rename = "source")]
    pub sources: Vec<EntitySourceEntry>,
}

impl Node for EntitySource {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();
        let sources = self.sources.iter().map(Node::expand);
        let q = quote! {
            #schema
            #derive
            pub struct #ident {
                #(#sources,)*
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

impl MacroNode for EntitySource {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for EntitySource {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let entity = quote_one(&self.entity, to_path);
        let sources = quote_vec(&self.sources, EntitySourceEntry::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::EntitySource(
                ::mimic::orm::schema::node::EntitySource{
                def: #def,
                entity: #entity,
                sources: #sources,
            })
        }
    }
}

impl TraitNode for EntitySource {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}

///
/// EntitySourceEntry
///

#[derive(Debug, FromMeta)]
pub struct EntitySourceEntry {
    pub name: Ident,
    pub path: Path,
}

impl Node for EntitySourceEntry {
    fn expand(&self) -> TokenStream {
        let name = &self.name;
        let path = &self.path;

        let q = quote! {
            pub #name: #path
        };

        q
    }
}

impl Schemable for EntitySourceEntry {
    fn schema(&self) -> TokenStream {
        // quote
        let name = quote_one(&self.name, to_string);
        let path = quote_one(&self.path, to_path);

        quote! {
            ::mimic::orm::schema::node::EntitySourceEntry {
                name: #name,
                path: #path,
            }
        }
    }
}
