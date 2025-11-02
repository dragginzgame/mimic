use crate::{prelude::*, view::NewtypeView};

///
/// Newtype
///

#[derive(Debug, FromMeta)]
pub struct Newtype {
    #[darling(default, skip)]
    pub def: Def,

    pub primitive: Option<Primitive>,
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
        // panic on invalid primitive/item combinations
        match (self.primitive, self.item.primitive) {
            (Some(a), Some(b)) if a != b => {
                panic!("invalid #[newtype] config: conflicting primitive ({a:?}) and item({b:?})");
            }
            (None, Some(_)) => {
                panic!(
                    "invalid #[newtype] config: item has a primitive but outer 'primitive' is not set"
                );
            }
            _ => {}
        }

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
        traits.extend(vec![Trait::Deref, Trait::DerefMut, Trait::Inner]);

        // primitive traits
        if let Some(primitive) = self.primitive {
            if primitive.supports_arithmetic() {
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
            if primitive.supports_copy() {
                traits.add(Trait::Copy);
            }
            if primitive.supports_display() {
                traits.add(Trait::Display);
            }
            if primitive.supports_hash() {
                traits.add(Trait::Hash);
            }
            if primitive.supports_num_cast() {
                traits.extend(vec![
                    Trait::NumCast,
                    Trait::NumFromPrimitive,
                    Trait::NumToPrimitive,
                ]);
            }
            if primitive.supports_ord() {
                traits.add(Trait::Ord);
                traits.add(Trait::PartialOrd);
            }
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
            Trait::Inner => InnerTrait::strategy(self),
            Trait::NumCast => NumCastTrait::strategy(self),
            Trait::NumToPrimitive => NumToPrimitiveTrait::strategy(self),
            Trait::NumFromPrimitive => NumFromPrimitiveTrait::strategy(self),
            Trait::SanitizeAuto => SanitizeAutoTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),
            Trait::View => ViewTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for Newtype {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let item = &self.item.type_expr();

        quote! {
            #[repr(transparent)]
            pub struct #ident(pub #item);
        }
    }
}

impl HasViews for Newtype {
    fn view_parts(&self) -> Vec<TokenStream> {
        vec![NewtypeView(self).view_part()]
    }
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
