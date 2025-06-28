use crate::{
    helper::quote_option,
    imp::{self, Imp},
    node::{Arg, Def, Item, MacroNode, Node, Trait, TraitNode, TraitTokens, Type},
    schema::{BPrimitiveType, Schemable},
    traits::Traits,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Newtype
///

#[derive(Debug, FromMeta)]
pub struct Newtype {
    #[darling(default, skip)]
    pub def: Def,

    pub primitive: BPrimitiveType,
    pub item: Item,

    #[darling(default)]
    pub default: Option<Arg>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Newtype {
    fn expand(&self) -> TokenStream {
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #derive
            #self
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

impl MacroNode for Newtype {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for Newtype {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.add_type_traits();
        traits.extend(vec![
            Trait::Default,
            Trait::Deref,
            Trait::DerefMut,
            Trait::From,
            Trait::Inner,
        ]);

        // primitive traits
        if self.primitive.is_displayable() {
            traits.add(Trait::Display);
        }
        if self.primitive.is_numeric() {
            traits.extend(vec![
                Trait::Add,
                Trait::AddAssign,
                Trait::Copy,
                Trait::Mul,
                Trait::MulAssign,
                Trait::NumCast,
                Trait::NumFromPrimitive,
                Trait::NumToPrimitive,
                Trait::Sub,
                Trait::SubAssign,
            ]);
        }
        if self.primitive.is_orderable() {
            traits.extend(vec![Trait::Ord, Trait::PartialOrd]);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default if self.default.is_some() => imp::DefaultTrait::tokens(self, t),
            Trait::FieldValue => imp::FieldValueTrait::tokens(self, t),
            Trait::From => imp::FromTrait::tokens(self, t),
            Trait::Inner => imp::InnerTrait::tokens(self, t),
            Trait::NumCast => imp::NumCastTrait::tokens(self, t),
            Trait::NumToPrimitive => imp::NumToPrimitiveTrait::tokens(self, t),
            Trait::NumFromPrimitive => imp::NumFromPrimitiveTrait::tokens(self, t),
            Trait::ValidateAuto => imp::ValidateAutoTrait::tokens(self, t),
            Trait::Visitable => imp::VisitableTrait::tokens(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Newtype {
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

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let item = &self.item;

        let q = quote! {
            pub struct #ident(#item);
        };

        tokens.extend(q);
    }
}
