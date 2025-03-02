use crate::{
    imp::{self, Imp},
    node::{Def, Item, MacroNode, Node, Trait, TraitNode, TraitTokens, Traits, Type},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// List
///

#[derive(Debug, FromMeta)]
pub struct List {
    #[darling(default, skip)]
    pub def: Def,

    pub item: Item,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for List {
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

impl MacroNode for List {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for List {
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
            Trait::From => imp::FromTrait::tokens(self, t),
            Trait::ValidateAuto => imp::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => imp::VisitableTrait::tokens(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for List {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let item = self.item.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::List(::mimic::schema::node::List {
                def: #def,
                item: #item,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for List {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        // cannot skip if hidden as the traits break
        let q = quote! {
            pub struct #ident(Vec<#item>);
        };

        tokens.extend(q);
    }
}
