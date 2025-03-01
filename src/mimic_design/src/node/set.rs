use crate::{
    imp,
    node::{Def, Item, MacroNode, Node, Trait, TraitNode, TraitTokens, Traits, Type},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Set
///

#[derive(Debug, FromMeta)]
pub struct Set {
    #[darling(default, skip)]
    pub def: Def,

    pub item: Item,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Set {
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

impl MacroNode for Set {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for Set {
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
            Trait::From => imp::from::set(self, t),
            Trait::Visitable => imp::visitable::set(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Set {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let item = self.item.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Set(::mimic::schema::node::Set {
                def: #def,
                item: #item,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for Set {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        // quote
        let q = quote! {
            pub struct #ident(::std::collections::HashSet<#item>);
        };

        tokens.extend(q);
    }
}
