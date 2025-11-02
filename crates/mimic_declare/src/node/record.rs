use crate::{
    prelude::*,
    view::{RecordEdit, RecordView},
};

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
            Trait::SanitizeAuto => SanitizeAutoTrait::strategy(self),
            Trait::ValidateAuto => ValidateAutoTrait::strategy(self),
            Trait::View => ViewTrait::strategy(self),
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

impl HasViews for Record {
    fn view_parts(&self) -> Vec<TokenStream> {
        vec![RecordView(self).view_part(), RecordEdit(self).view_part()]
    }
}

impl ToTokens for Record {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
