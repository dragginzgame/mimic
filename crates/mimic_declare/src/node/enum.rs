use crate::prelude::*;

///
/// Enum
///

#[derive(Debug, FromMeta)]
pub struct Enum {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<EnumVariant>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Enum {
    pub fn is_unit_enum(&self) -> bool {
        self.variants.iter().all(|v| v.value.is_none())
    }
}

impl HasDef for Enum {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasTraits for Enum {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();

        // extra traits
        if self.is_unit_enum() {
            traits.extend(vec![Trait::Copy, Trait::Hash, Trait::PartialOrd]);
        }

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::FieldValue => FieldValueTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::SanitizeAuto => SanitizeAutoTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Sorted => Trait::Sorted.derive_attribute(),
            _ => None,
        }
    }
}

impl HasSchemaPart for Enum {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let variants = quote_slice(&self.variants, EnumVariant::schema_part);
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Enum {
                def: #def,
                variants: #variants,
                ty: #ty,
            }
        }
    }
}

impl HasType for Enum {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let variants = self.variants.iter().map(HasTypeExpr::type_expr);

        quote! {
            pub enum #ident {
                #(#variants),*
            }
        }
    }
}

impl HasViewTypes for Enum {
    fn view_parts(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let view_variants = self.variants.iter().map(HasViewTypeExpr::view_type_expr);

        // extra traits
        let mut derives = self.view_derives();
        if self.is_unit_enum() {
            derives.extend(vec![
                Trait::Copy,
                Trait::Hash,
                Trait::Eq,
                Trait::Ord,
                Trait::PartialEq,
                Trait::PartialOrd,
            ]);
        }

        quote! {
            #derives
            pub enum #view_ident {
                #(#view_variants),*
            }
        }
    }
}

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}

///
/// EnumVariant
///

#[derive(Clone, Debug, FromMeta)]
pub struct EnumVariant {
    #[darling(default = EnumVariant::unspecified_ident)]
    pub ident: Ident,

    #[darling(default)]
    pub value: Option<Value>,

    #[darling(default)]
    pub default: bool,

    #[darling(default)]
    pub unspecified: bool,
}

impl EnumVariant {
    fn unspecified_ident() -> Ident {
        format_ident!("Unspecified")
    }

    /// Pick the effective identifier for codegen
    pub fn effective_ident(&self) -> Ident {
        if self.unspecified {
            Self::unspecified_ident()
        } else {
            self.ident.clone()
        }
    }
}

impl HasSchema for Enum {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Enum
    }
}

impl HasSchemaPart for EnumVariant {
    fn schema_part(&self) -> TokenStream {
        let Self {
            default,
            unspecified,
            ..
        } = self;
        let ident = quote_one(&self.ident, to_str_lit);
        let value = quote_option(self.value.as_ref(), Value::schema_part);

        quote! {
            ::mimic::schema::node::EnumVariant {
                ident: #ident,
                value : #value,
                default: #default,
                unspecified: #unspecified,
            }
        }
    }
}

impl HasTypeExpr for EnumVariant {
    fn type_expr(&self) -> TokenStream {
        let ident = self.effective_ident();
        let default_attr = self.default.then(|| quote!(#[default]));

        let body = if let Some(value) = &self.value {
            let value = value.type_expr();
            quote!(#ident(#value))
        } else {
            quote!(#ident)
        };

        quote! {
            #default_attr
            #body
        }
    }
}

impl HasViewTypeExpr for EnumVariant {
    fn view_type_expr(&self) -> TokenStream {
        let ident = &self.ident;
        let default_attr = self.default.then(|| quote!(#[default]));

        if let Some(value) = &self.value {
            let value_view = HasViewTypeExpr::view_type_expr(value);

            quote! {
                #ident(#value_view)
            }
        } else {
            quote! {
                #default_attr
                #ident
            }
        }
    }
}
