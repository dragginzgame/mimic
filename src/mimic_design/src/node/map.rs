use crate::{
    imp,
    node::{Def, Item, MacroNode, Node, Trait, TraitNode, TraitTokens, Traits, Type, Value},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Map
///

#[derive(Debug, FromMeta)]
pub struct Map {
    #[darling(default, skip)]
    pub def: Def,

    pub key: Item,
    pub value: Value,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Map {
    fn expand(&self) -> TokenStream {
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #derive
            #self
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

impl MacroNode for Map {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for Map {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::AsRef,
            Trait::Default,
            Trait::Deref,
            Trait::DerefMut,
            Trait::From,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
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
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Map(::mimic::schema::node::Map {
                def: #def,
                key: #key,
                value: #value,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let key = &self.key;
        let value = &self.value;

        // quote
        let q = quote! {
            pub struct #ident(::std::collections::HashMap<#key, #value>);
        };

        tokens.extend(q);
    }
}
