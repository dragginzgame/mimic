use crate::{
    node::{Def, FieldList, Type},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

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

impl AsMacro for Record {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_extra(&self) -> TokenStream {
        self.as_view_type()
    }

    fn traits(&self) -> Vec<Trait> {
        let traits = self.traits.clone().with_type_traits();

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::Default if self.fields.has_default() => DefaultTrait::tokens(self),
            Trait::From => FromTrait::tokens(self),
            Trait::TypeView => TypeViewTrait::tokens(self),
            Trait::ValidateAuto => ValidateAutoTrait::tokens(self),
            Trait::Visitable => VisitableTrait::tokens(self),

            _ => None,
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default => Trait::Default.derive_attribute(),

            _ => None,
        }
    }
}

impl AsSchema for Record {
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

impl AsType for Record {
    fn as_type(&self) -> TokenStream {
        let Self { fields, .. } = self;
        let Def { ident, .. } = &self.def;

        // quote
        quote! {
            pub struct #ident {
                #fields
            }
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let view_ident = &self.def.view_ident();
        let view_field_list = AsType::as_view_type(&self.fields);
        let derives = Self::basic_derives();

        // quote
        quote! {
            #derives
            pub struct #view_ident {
                #view_field_list
            }
        }
    }
}

impl ToTokens for Record {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
