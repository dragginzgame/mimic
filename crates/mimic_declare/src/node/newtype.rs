use crate::{
    helper::quote_option,
    node::{Arg, Def, Item, Type},
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter, SchemaKind},
};
use darling::FromMeta;
use mimic_schema::types::Primitive;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

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
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut, Trait::Inner]);

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
        use crate::node_traits::*;

        match t {
            Trait::FieldValue => FieldValueTrait::tokens(self),
            Trait::From => FromTrait::tokens(self),
            Trait::Inner => InnerTrait::tokens(self),
            Trait::NumCast => NumCastTrait::tokens(self),
            Trait::NumToPrimitive => NumToPrimitiveTrait::tokens(self),
            Trait::NumFromPrimitive => NumFromPrimitiveTrait::tokens(self),
            Trait::TypeView => TypeViewTrait::tokens(self),
            Trait::ValidateAuto => ValidateAutoTrait::tokens(self),
            Trait::Visitable => VisitableTrait::tokens(self),

            _ => None,
        }
    }
}

impl AsSchema for Newtype {
    const KIND: SchemaKind = SchemaKind::Full;

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
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();
        let item = &self.item;

        Some(quote! {
            #[repr(transparent)]
            pub struct #ident(#item);
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let view_ident = self.view_ident();
        let view_type = self.primitive.as_type();

        Some(quote! {
            pub type #view_ident = #view_type;
        })
    }
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
