use crate::{
    helper::{quote_one, to_string},
    imp,
    node::{Def, Item, MacroNode, Node, Trait, TraitNode, Traits, Type},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// Map
///

#[derive(Debug, FromMeta)]
pub struct Map {
    #[darling(default, skip)]
    pub def: Def,

    pub item: Item,
    pub key: Ident,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Map {
    fn expand(&self) -> TokenStream {
        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            #self
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

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::ValidateAuto => imp::validate_auto::map(self, t),
            Trait::Visitable => imp::visitable::map(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Map {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let item = self.item.schema();
        let key = quote_one(&self.key, to_string);
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Map(::mimic::schema::node::Map {
                def: #def,
                item: #item,
                key: #key,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        // quote
        let q = quote! {
            pub struct #ident(Vec<#item>);
        };

        tokens.extend(q);
    }
}
