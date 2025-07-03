use crate::{
    helper::quote_slice,
    node::{Def, MacroNode, Node, TraitNode, TraitTokens, Type, Value},
    schema::Schemable,
    traits::{self, Imp, Trait, Traits},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Tuple
///

#[derive(Debug, FromMeta)]
pub struct Tuple {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "value")]
    pub values: Vec<Value>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Tuple {
    fn expand(&self) -> TokenStream {
        let TraitTokens { derive, impls } = self.trait_tokens();

        // vars
        let Def { ident, .. } = &self.def;
        let schema = self.ctor_schema();

        // quote
        let q = quote! {
            #schema
            #derive
            pub struct #ident(pub #self);
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

impl MacroNode for Tuple {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Tuple {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let values = quote_slice(&self.values, Value::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Tuple(::mimic::schema::node::Tuple {
                def: #def,
                values: #values,
                ty: #ty,
            })
        }
    }
}

impl TraitNode for Tuple {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Visitable => traits::VisitableTrait::tokens(self, t),

            _ => traits::any(self, t),
        }
    }
}

impl ToTokens for Tuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let values = &self.values;

        tokens.extend(quote! {
            ( #( #values ,)* )
        });
    }
}
