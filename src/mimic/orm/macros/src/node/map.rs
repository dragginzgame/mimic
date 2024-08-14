use crate::{
    helper::quote_vec,
    imp,
    node::{Def, Item, MacroNode, Node, Trait, TraitNode, Traits, TypeValidator, Value},
};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

//
// Map
//

#[derive(Debug, FromMeta)]
pub struct Map {
    #[darling(default, skip)]
    pub def: Def,

    pub key: Item,
    pub value: Value,

    #[darling(multiple, rename = "validator")]
    pub validators: Vec<TypeValidator>,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Map {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            pub struct #ident(#self);
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

impl MacroNode for Map {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for Map {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_db_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::Deref,
            Trait::DerefMut,
            Trait::From,
        ]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::From => imp::from::map(self, t),
            Trait::Visitable => imp::visitable::map(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Map {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let key = self.key.schema();
        let value = self.value.schema();
        let validators = quote_vec(&self.validators, TypeValidator::schema);

        quote! {
                ::mimic::orm::schema::node::SchemaNode::Map(::mimic::orm::schema::node::Map {
                    def: #def,
                    key: #key,
                    value: #value,
                    validators: #validators,
                })
        }
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = &self.key;
        let value = &self.value;

        tokens.extend(quote! {
            ::mimic::orm::collections::HashMap<#key, #value>
        });
    }
}
