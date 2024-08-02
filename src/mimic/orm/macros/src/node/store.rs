use crate::{
    helper::{quote_one, to_path},
    imp,
    node::{Crud, Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use syn::Path;

///
/// Store
///

#[derive(Debug, FromMeta)]
pub struct Store {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,

    pub canister: Path,
    pub memory_id: u8,

    #[darling(default)]
    pub crud: Crud,
}

impl Node for Store {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let imp = self.imp();
        let q = quote! {
            #schema
            pub struct #ident {}
            #imp
        };

        // debug
        assert!(!self.debug, "{q}");

        q
    }
}

impl MacroNode for Store {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Store {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let canister = quote_one(&self.canister, to_path);
        let memory_id = &self.memory_id;
        let crud = self.crud.schema();

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Store(::mimic::orm::schema::node::Store{
                def: #def,
                canister: #canister,
                memory_id: #memory_id,
                crud: #crud,
            })
        }
    }
}

impl TraitNode for Store {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
