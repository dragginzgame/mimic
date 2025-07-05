use crate::{
    helper::quote_slice,
    node::{Def, Type, Value},
    node_traits::{self, Imp, Trait, Traits},
    traits::{Macro, Schemable},
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

impl Macro for Tuple {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_body(&self) -> TokenStream {
        quote! { self }
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => node_traits::any(self, t),
        }
    }
}

impl Schemable for Tuple {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let values = quote_slice(&self.values, Value::schema);
        let ty = &self.ty.schema();

        quote! {
            ::mimic::schema::node::Schemable::Tuple(::mimic::schema::node::Tuple {
                def: #def,
                values: #values,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for Tuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let values = &self.values;

        // quote
        tokens.extend(quote! {
            pub struct #ident(pub (#(#values,)*));
        });
    }
}
