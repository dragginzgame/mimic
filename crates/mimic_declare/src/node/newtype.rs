use crate::{
    helper::quote_option,
    node::{Arg, Def, Item, Type},
    node_traits::{self, Imp, Trait, Traits},
    traits::{AsMacro, AsSchema, AsType},
};
use darling::FromMeta;
use mimic_schema::types::Primitive;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Newtype
///

#[derive(Debug, FromMeta)]
pub struct Newtype {
    #[darling(default, skip)]
    pub def: Def,

    pub primitive: Primitive,
    pub item: Item,

    #[darling(default)]
    pub default: Option<Arg>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl AsMacro for Newtype {
    fn def(&self) -> &Def {
        &self.def
    }

    fn macro_extra(&self) -> TokenStream {
        self.view_tokens()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::Deref,
            Trait::DerefMut,
            Trait::Inner,
        ]);

        // primitive traits
        if self.primitive.supports_arithmetic() {
            traits.extend(vec![
                Trait::Add,
                Trait::AddAssign,
                Trait::Mul,
                Trait::MulAssign,
                Trait::Sub,
                Trait::SubAssign,
            ]);
        }
        if self.primitive.supports_copy() {
            traits.add(Trait::Copy);
        }
        if self.primitive.supports_display() {
            traits.add(Trait::Display);
        }
        if self.primitive.supports_eq() {
            traits.add(Trait::Eq);
        }
        if self.primitive.supports_hash() {
            traits.add(Trait::Hash);
        }
        if self.primitive.supports_num_cast() {
            traits.extend(vec![
                Trait::NumCast,
                Trait::NumFromPrimitive,
                Trait::NumToPrimitive,
            ]);
        }
        if self.primitive.supports_total_ord() {
            traits.add(Trait::Ord);
        }
        if self.primitive.supports_partial_ord() {
            traits.add(Trait::PartialOrd);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.default.is_some() => node_traits::DefaultTrait::tokens(self, t),
            Trait::FieldValue => node_traits::FieldValueTrait::tokens(self, t),
            Trait::Inner => node_traits::InnerTrait::tokens(self, t),
            Trait::NumCast => node_traits::NumCastTrait::tokens(self, t),
            Trait::NumToPrimitive => node_traits::NumToPrimitiveTrait::tokens(self, t),
            Trait::NumFromPrimitive => node_traits::NumFromPrimitiveTrait::tokens(self, t),
            Trait::TypeView => node_traits::TypeViewTrait::tokens(self, t),
            Trait::ValidateAuto => node_traits::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => node_traits::VisitableTrait::tokens(self, t),

            _ => None,
        }
    }

    fn custom_impl(&self) -> Option<TokenStream> {
        crate::node::imp::newtype::tokens(self)
    }
}

impl AsSchema for Newtype {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let item = self.item.schema();
        let default = quote_option(self.default.as_ref(), Arg::schema);
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Newtype(::mimic::schema::node::Newtype {
                def: #def,
                item: #item,
                default: #default,
                ty: #ty,
            })
        }
    }
}

impl AsType for Newtype {
    fn view(&self) -> TokenStream {
        let view_ident = self.def.view_ident();
        let view_type = self.primitive.as_type();

        quote! {
            pub struct #view_ident(#view_type);
        }
    }
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        tokens.extend(quote! {
            pub struct #ident(#item);
        });
    }
}
