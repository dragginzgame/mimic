use crate::{
    imp::{self, Imp},
    node::{Def, FieldList, MacroNode, Node, Trait, TraitNode, TraitTokens, Traits, Type},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Record
///

#[derive(Debug, FromMeta)]
pub struct Record {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub fields: FieldList,

    #[darling(default)]
    pub traits: Traits,

    #[darling(default)]
    pub ty: Type,
}

impl Node for Record {
    fn expand(&self) -> TokenStream {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #derive
            pub struct #ident {
                #fields
            }
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

impl MacroNode for Record {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Record {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let fields = self.fields.schema();
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Record(::mimic::schema::node::Record {
                def: #def,
                fields: #fields,
                ty: #ty,
            })
        }
    }
}

impl TraitNode for Record {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.fields.has_default() => imp::DefaultTrait::tokens(self, t),
            Trait::FieldSearch => imp::FieldSearchTrait::tokens(self, t),
            Trait::FieldSort => imp::FieldSortTrait::tokens(self, t),
            Trait::ValidateAuto => imp::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => imp::VisitableTrait::tokens(self, t),

            _ => imp::any(self, t),
        }
    }

    fn derive_attributes(&self) -> Option<TokenStream> {
        self.traits()
            .contains(&Trait::Default)
            .then(|| quote! { #[serde(default)] })
    }
}
