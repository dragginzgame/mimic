use crate::prelude::*;

///
/// EnumValue
///

#[derive(Debug, FromMeta)]
pub struct EnumValue {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "variant")]
    pub variants: Vec<EnumValueVariant>,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl EnumValue {
    pub fn default_variant(&self) -> Option<&EnumValueVariant> {
        self.variants.iter().find(|v| v.default)
    }
}

impl HasDef for EnumValue {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for EnumValue {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::EnumValue
    }
}

impl HasSchemaPart for EnumValue {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let variants = quote_slice(&self.variants, EnumValueVariant::schema_part);
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::EnumValue{
                def: #def,
                variants: #variants,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for EnumValue {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::Copy, Trait::EnumValueKind, Trait::Hash]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::Default => DefaultTrait::strategy(self),
            Trait::EnumValueKind => EnumValueKindTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for EnumValue {
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

impl HasViewTypes for EnumValue {
    fn view_parts(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let view_variants = self.variants.iter().map(HasViewTypeExpr::view_type_expr);
        let derives = self.view_derives();

        quote! {
            #derives
            pub enum #view_ident {
                #(#view_variants),*
            }
        }
    }
}

impl ToTokens for EnumValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}

///
/// EnumValueVariant
///

#[derive(Debug, FromMeta)]
pub struct EnumValueVariant {
    #[darling(default = EnumValueVariant::unspecified_ident)]
    pub ident: Ident,

    pub value: ArgNumber,

    #[darling(default)]
    pub default: bool,

    #[darling(default)]
    pub unspecified: bool,
}

impl EnumValueVariant {
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

impl HasSchemaPart for EnumValueVariant {
    fn schema_part(&self) -> TokenStream {
        let Self {
            default,
            unspecified,
            ..
        } = self;

        // quote
        let ident = quote_one(&self.ident, to_str_lit);
        let value = self.value.schema_part();

        quote! {
            ::mimic::schema::node::EnumValueVariant {
                ident: #ident,
                value : #value,
                default: #default,
                unspecified: #unspecified,
            }
        }
    }
}

impl HasTypeExpr for EnumValueVariant {
    fn type_expr(&self) -> TokenStream {
        let ident = self.effective_ident();

        quote! {
            #ident
        }
    }
}

impl HasViewTypeExpr for EnumValueVariant {
    fn view_type_expr(&self) -> TokenStream {
        let ident = self.effective_ident();

        quote! {
            #ident
        }
    }
}
