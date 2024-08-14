use crate::{
    helper::quote_vec,
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits, Value},
};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

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
    pub traits: Traits,
}

impl Node for Tuple {
    fn expand(&self) -> TokenStream {
        // vars
        let Def {
            ident, generics, ..
        } = &self.def;
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();

        // quote
        let q = quote! {
            #schema
            #derive
            pub struct #ident #generics(pub #self);
            #imp
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s));
        }

        q
    }
}

impl MacroNode for Tuple {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for Tuple {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_db_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut, Trait::From]);

        traits.list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::From => imp::from::tuple(self, t),
            Trait::Visitable => imp::visitable::tuple(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Tuple {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let values = quote_vec(&self.values, Value::schema);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Tuple(::mimic::orm::schema::node::Tuple {
                def: #def,
                values: #values,
            })
        }
    }
}

impl ToTokens for Tuple {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let values = &self.values;
        tokens.extend(quote! {
            ( #( #values ,)* )
        });
    }
}
