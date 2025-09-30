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
            Trait::EnumValueKind => EnumValueKindTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),

            _ => None,
        }
    }
}

impl HasType for EnumValue {}

impl HasTypePart for EnumValue {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let variants = self.variants.iter().map(HasTypePart::type_part);

        quote! {
            pub enum #ident {
                #(#variants,)*
            }
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let view_ident = self.view_ident();
        let view_variants = self.variants.iter().map(HasTypePart::view_type_part);
        let derives = self.view_derives();

        quote! {
            #derives
            pub enum #view_ident {
                #(#view_variants,)*
            }

            impl Default for #view_ident {
                fn default() -> Self {
                    #ident::default().to_view()
                }
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

impl HasTypePart for EnumValueVariant {
    fn type_part(&self) -> TokenStream {
        let ident = self.effective_ident();
        let default_attr = if self.default {
            quote!(#[default])
        } else {
            quote!()
        };

        quote! {
            #default_attr
            #ident
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let ident = self.effective_ident();

        quote! {
            #ident
        }
    }
}
