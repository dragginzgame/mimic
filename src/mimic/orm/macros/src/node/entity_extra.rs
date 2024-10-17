use crate::{
    helper::{quote_one, quote_vec, to_path, to_string},
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

///
/// EntityExtra
///

#[derive(Debug, FromMeta)]
pub struct EntityExtra {
    #[darling(default, skip)]
    pub def: Def,

    pub entity: Path,

    #[darling(multiple, rename = "source")]
    pub sources: Vec<EntityExtraSource>,
}

impl Node for EntityExtra {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            pub enum #ident {
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

impl MacroNode for EntityExtra {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for EntityExtra {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let entity = quote_one(&self.entity, to_path);
        let sources = quote_vec(&self.sources, EntityExtraSource::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::EntityExtra(
                ::mimic::orm::schema::node::EntityExtra{
                def: #def,
                entity: #entity,
                sources: #sources,
            })
        }
    }
}

impl TraitNode for EntityExtra {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default();
        traits.extend(vec![Trait::Copy, Trait::EnumDisplay, Trait::EnumStaticStr]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}

///
/// EntityExtraSource
///

#[derive(Debug, FromMeta)]
pub struct EntityExtraSource {
    pub name: String,
    pub path: Path,
}

impl Node for EntityExtraSource {
    fn expand(&self) -> TokenStream {
        let name = &self.name;
        let path = &self.path;

        let q = quote! {
            #name: #path,
        };

        q
    }
}

impl Schemable for EntityExtraSource {
    fn schema(&self) -> TokenStream {
        // quote
        let name = quote_one(&self.name, to_path);
        let path = quote_one(&self.path, to_string);

        quote! {
            ::mimic::orm::schema::node::EntityExtraSource {
                name: #name,
                path: #path,
            }
        }
    }
}
