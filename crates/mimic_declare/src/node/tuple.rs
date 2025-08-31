use crate::{
    helper::quote_slice,
    imp::TraitStrategy,
    node::{Def, Type, Value},
    schema_traits::{Trait, TraitList, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart,
        SchemaNodeKind,
    },
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// Tuple
///

#[derive(Debug, Default, FromMeta)]
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

impl HasIdent for Tuple {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for Tuple {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Tuple
    }
}

impl HasSchemaPart for Tuple {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let values = quote_slice(&self.values, Value::schema_part);
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Tuple {
                def: #def,
                values: #values,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for Tuple {
    fn traits(&self) -> TraitList {
        self.traits.clone().with_type_traits().list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for Tuple {}

impl HasTypePart for Tuple {
    fn type_part(&self) -> TokenStream {
        let ident = self.ident();
        let values = self.values.iter().map(HasTypePart::type_part);

        quote! {
            pub struct #ident(pub #(#values),*);
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let view_ident = &self.view_ident();
        let view_values = self.values.iter().map(HasTypePart::view_type_part);

        quote! {
            pub type #view_ident = (#(#view_values),*);
        }
    }
}

impl ToTokens for Tuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
