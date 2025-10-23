use crate::prelude::*;

///
/// Entity
///

#[derive(Debug, FromMeta)]
pub struct Entity {
    #[darling(default, skip)]
    pub def: Def,

    pub store: Path,

    #[darling(rename = "pk")]
    pub primary_key: Ident,

    #[darling(multiple, rename = "index")]
    pub indexes: Vec<Index>,

    #[darling(default, map = "Entity::add_metadata")]
    pub fields: FieldList,

    #[darling(default)]
    pub ty: Type,

    #[darling(default)]
    pub traits: Traits,
}

impl Entity {
    pub fn iter_without_pk(&self) -> impl Iterator<Item = &Field> {
        self.fields
            .iter()
            .filter(move |f| f.ident != self.primary_key)
    }

    pub fn create_ident(&self) -> Ident {
        format_ident!("{}Create", self.def.ident())
    }

    pub fn update_ident(&self) -> Ident {
        format_ident!("{}Update", self.def.ident())
    }

    /// Generates the `EntityCreate` struct (excluding PK)
    pub fn create_type_part(&self) -> TokenStream {
        let derives = self.view_derives();
        let create_ident = self.create_ident();

        let field_tokens = self.iter_without_pk().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_type_expr();

            quote!(pub #ident: #ty,)
        });

        quote! {
            #derives
            pub struct #create_ident {
                #(#field_tokens)*
            }
        }
    }

    /// Generates the `EntityUpdate` struct (excluding PK, all Option<>)
    pub fn update_type_part(&self) -> TokenStream {
        /*
        let derives = self.view_derives();
        let update_ident = self.update_ident();

        let field_tokens = self.iter_without_pk().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_type_expr();

            quote!(pub #ident: Option<#ty>,)
        });

        quote! {
            #derives
            pub struct #update_ident {
                #(#field_tokens)*
            }
        }*/

        quote!()
    }

    fn add_metadata(mut fields: FieldList) -> FieldList {
        fields.push(Field::created_at());
        fields.push(Field::updated_at());

        fields
    }
}

//
// ──────────────────────────
// TRAIT IMPLEMENTATIONS
// ──────────────────────────
//

impl HasDef for Entity {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl HasSchema for Entity {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Entity
    }
}

impl HasSchemaPart for Entity {
    fn schema_part(&self) -> TokenStream {
        let def = &self.def.schema_part();
        let store = quote_one(&self.store, to_path);
        let primary_key = quote_one(&self.primary_key, to_str_lit);
        let indexes = quote_slice(&self.indexes, Index::schema_part);
        let fields = &self.fields.schema_part();
        let ty = &self.ty.schema_part();

        quote! {
            ::mimic::schema::node::Entity {
                def: #def,
                store: #store,
                primary_key: #primary_key,
                indexes: #indexes,
                fields: #fields,
                ty: #ty,
            }
        }
    }
}

impl HasTraits for Entity {
    fn traits(&self) -> TraitList {
        let mut traits = self.traits.clone().with_type_traits();
        traits.extend(vec![
            Trait::Inherent,
            Trait::CreateView,
            Trait::EntityKind,
            Trait::FieldValues,
            Trait::UpdateView,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TraitStrategy> {
        use crate::imp::*;

        match t {
            Trait::Inherent => InherentTrait::strategy(self),

            Trait::CreateView => CreateViewTrait::strategy(self),
            Trait::Default => DefaultTrait::strategy(self),
            Trait::From => FromTrait::strategy(self),
            Trait::EntityKind => EntityKindTrait::strategy(self),
            Trait::FieldValues => FieldValuesTrait::strategy(self),
            Trait::TypeView => TypeViewTrait::strategy(self),
            Trait::SanitizeAuto => SanitizeAutoTrait::strategy(self),
            Trait::UpdateView => UpdateViewTrait::strategy(self),
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

impl HasType for Entity {
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

impl HasViewTypes for Entity {
    fn view_parts(&self) -> TokenStream {
        let derives = self.view_derives();
        let ident = self.def.ident();
        let view_ident = self.view_ident();

        let fields = self.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_type_expr();
            quote!(pub #ident: #ty,)
        });

        // other types
        let create = self.create_type_part();
        let update = self.update_type_part();

        quote! {
            #derives
            pub struct #view_ident {
                #(#fields)*
            }

            impl Default for #view_ident {
                fn default() -> Self {
                    #ident::default().to_view()
                }
            }

            #create
            #update
        }
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
