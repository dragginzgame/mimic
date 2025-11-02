use crate::prelude::*;

///
/// Record
///

#[derive(Debug, FromMeta)]
pub struct Record {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub fields: FieldList,

    #[darling(default)]
    pub traits: Traits,

    #[darling(default)]
    pub ty: Type,
}

impl HasDef for Record {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Record {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Record
    }
}

impl HasSchemaPart for Record {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();
        let fields = self.fields.schema_part();
        let ty = self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Record {
                def: #def,
                fields: #fields,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for Record {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![Trait::EditView]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::Default => DefaultTrait::strategy(self),
            Trait::EditView => EditViewTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::SanitizeAuto => SanitizeAutoTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),
            Trait::Visitable => VisitableTrait::strategy(self),

            _ => None,
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Default => Trait::Default.derive_attribute(),

            _ => None,
        }
    }
}

impl HasType for Record {
    fn type_part(&self) -> TokenStream {
        let ident = self.def.ident();
        let fields = self.fields.type_expr();

        quote! {
            pub struct #ident {
                #fields
            }
        }
    }
}

impl HasView for Record {
    fn view_part(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let derives = self.view_derives();
        let view_field_list = &self.fields.view_type_expr();

        quote! {
            #derives
            pub struct #view_ident {
                #view_field_list
            }
        }
    }

    /// Generates the `EntityUpdate` struct (excluding PK, all Option<>)
    fn edit_part(&self) -> TokenStream {
        let derives = self.view_derives();
        let edit_ident = self.edit_ident();

        let field_tokens = self.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_type_expr();

            quote!(pub #ident: Option<#ty>)
        });

        quote! {
            #derives
            pub struct #edit_ident {
                #(#field_tokens),*
            }
        }
    }
}

impl HasViewTraits for Record {}

impl ToTokens for Record {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
