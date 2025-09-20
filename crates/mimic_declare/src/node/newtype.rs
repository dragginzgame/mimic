use crate::{
    helper::quote_option,
    imp::TraitStrategy,
    node::{Arg, Def, Item, Type},
    schema_traits::{Trait, TraitList, Traits},
    traits::{
        HasDef, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasType, HasTypePart, SchemaNodeKind,
    },
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

impl HasDef for Newtype {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Newtype {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Newtype
    }
}

impl HasSchemaPart for Newtype {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let item = self.item.schema_part();
        let default = quote_option(self.default.as_ref(), Arg::schema_part);
        let ty = self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Newtype {
                def: #def,
                item: #item,
                default: #default,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for Newtype {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Deref, Trait::DerefMut]);

        // primitive traits
        if self.primitive.supports_arithmetic() {
            traits.extend(vec![
                Trait::Add,
                Trait::AddAssign,
                Trait::Mul,
                Trait::MulAssign,
                Trait::Sub,
                Trait::SubAssign,
                Trait::Sum,
            ]);
        }
        if self.primitive.supports_copy() {
            traits.add(Trait::Copy);
        }
        if self.primitive.supports_display() {
            traits.add(Trait::Display);
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
        if self.primitive.supports_ord() {
            traits.add(Trait::Ord);
            traits.add(Trait::PartialOrd);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::PartialEq => PartialEqTrait::strategy(self).map(|s| s.with_derive(t)),
            Trait::PartialOrd => PartialOrdTrait::strategy(self).map(|s| s.with_derive(t)),

            Trait::FieldValue => FieldValueTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::NumCast => NumCastTrait::strategy(self),
            Trait::NumToPrimitive => NumToPrimitiveTrait::strategy(self),
            Trait::NumFromPrimitive => NumFromPrimitiveTrait::strategy(self),
            Trait::SanitizeAuto => SanitizeAutoTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for Newtype {}

impl HasTypePart for Newtype {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let item = &self.item.type_part();

        quote! {
            #[repr(transparent)]
            pub struct #ident(pub #item);
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let view_type = self.primitive.as_type();

        quote! {
            pub type #view_ident = #view_type;
        }
    }
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
