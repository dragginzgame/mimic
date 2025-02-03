use crate::{
    helper::quote_option,
    imp,
    node::{
        Cardinality, Def, MacroNode, Node, PrimitiveGroup, PrimitiveType, Trait, TraitNode, Traits,
        Type, Value,
    },
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

///
/// Newtype
///

#[derive(Debug, FromMeta)]
pub struct Newtype {
    #[darling(default, skip)]
    pub def: Def,

    pub value: Value,

    #[darling(default)]
    pub primitive: Option<PrimitiveType>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Node for Newtype {
    fn expand(&self) -> TokenStream {
        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();
        let q = quote! {
            #schema
            #derive
            #self
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
            Trait::AsRef,
            Trait::Default,
            Trait::Deref,
            Trait::DerefMut,
            Trait::From,
        ]);

        // primitive traits
        if let Some(primitive) = &self.primitive {
            // inner
            traits.add(Trait::Inner);

            // ord
            if primitive.is_orderable() {
                match &self.value.cardinality() {
                    Cardinality::One | Cardinality::Opt => {
                        traits.extend(vec![Trait::Ord, Trait::PartialOrd]);
                    }
                    Cardinality::Many => {}
                }
            }
        }

        // group traits
        if self.value.cardinality() == Cardinality::One {
            match self.primitive.map(PrimitiveType::group) {
                Some(PrimitiveGroup::Integer | PrimitiveGroup::Decimal) => {
                    traits.extend(vec![
                        Trait::Add,
                        Trait::AddAssign,
                        Trait::Copy,
                        Trait::Display,
                        Trait::FromStr,
                        Trait::Mul,
                        Trait::MulAssign,
                        Trait::NumCast,
                        Trait::NumFromPrimitive,
                        Trait::NumToPrimitive,
                        Trait::Sub,
                        Trait::SubAssign,
                    ]);
                }
                Some(PrimitiveGroup::String | PrimitiveGroup::Ulid) => {
                    traits.extend(vec![Trait::Display, Trait::FromStr]);
                }
                _ => {}
            }
        }

        traits.list()
    }

    fn map_derive(&self, t: Trait) -> bool {
        match t {
            // derive default if no default value
            Trait::Default => self.value.default.is_none(),
            _ => true,
        }
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::Default if self.value.default.is_some() => imp::default::newtype(self, t),
            Trait::Display => imp::display::newtype(self, t),
            Trait::Filterable => imp::filterable::newtype(self, t),
            Trait::From => imp::from::newtype(self, t),
            Trait::Inner => imp::inner::newtype(self, t),
            Trait::NumCast => imp::num::cast::newtype(self, t),
            Trait::NumToPrimitive => imp::num::to_primitive::newtype(self, t),
            Trait::NumFromPrimitive => imp::num::from_primitive::newtype(self, t),
            Trait::Orderable => imp::orderable::newtype(self, t),
            Trait::SortKey => imp::sort_key::newtype(self, t),
            Trait::ValidateAuto => imp::validate_auto::newtype(self, t),
            Trait::Visitable => imp::visitable::newtype(self, t),

            _ => imp::any(self, t),
        }
    }
}

impl Schemable for Newtype {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let value = self.value.schema();
        let primitive = quote_option(self.primitive.as_ref(), PrimitiveType::schema);
        let ty = self.ty.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Newtype(::mimic::schema::node::Newtype {
                def: #def,
                value: #value,
                primitive: #primitive,
                ty: #ty,
            })
        }
    }
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def {
            ident, generics, ..
        } = &self.def;
        let value = &self.value;

        // cannot skip if hidden as the traits break
        let q = quote! {
            pub struct #ident #generics(#value);
        };

        tokens.extend(q);
    }
}
