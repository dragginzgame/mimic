use crate::{
    helper::{quote_one, quote_vec, to_path, to_string},
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm::types::Sorted;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use syn::{Ident, Path};

///
/// Fixture
///

#[derive(Debug, FromMeta)]
pub struct Fixture {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,

    pub entity: Path,

    #[darling(default)]
    pub sorted: Sorted,

    #[darling(multiple, rename = "key")]
    pub keys: Vec<Ident>,
}

impl Node for Fixture {
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
        assert!(!self.debug, "{q}");

        q
    }
}

impl MacroNode for Fixture {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Fixture {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let entity = quote_one(&self.entity, to_path);
        let keys = quote_vec(&self.keys, to_string);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Fixture(::mimic::orm::schema::node::Fixture{
                def: #def,
                entity: #entity,
                keys: #keys,
            })
        }
    }
}

impl TraitNode for Fixture {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default();
        traits.extend(vec![Trait::Copy, Trait::EnumDisplay, Trait::EnumStaticStr]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
