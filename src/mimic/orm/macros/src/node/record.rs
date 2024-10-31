use crate::{
    imp,
    node::{Def, FieldList, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm_schema::Schemable;
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
}

impl Node for Record {
    fn expand(&self) -> TokenStream {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive_struct();
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            pub struct #ident {
                #fields
            }
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

impl MacroNode for Record {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Record {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let fields = self.fields.schema();

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Record(::mimic::orm::schema::node::Record {
                def: #def,
                fields: #fields,
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

    fn map_derive(&self, t: Trait) -> bool {
        match t {
            Trait::Default => !self.fields.has_default(),
            _ => true,
        }
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::Default if self.fields.has_default() => imp::default::record(self, t),
            Trait::FieldFilter => imp::record_filter::record(self, t),
            Trait::FieldSort => imp::record_sort::record(self, t),
            Trait::Visitable => imp::visitable::record(self, t),

            _ => imp::any(self, t),
        }
    }
}
