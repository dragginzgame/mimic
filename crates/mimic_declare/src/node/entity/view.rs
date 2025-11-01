use crate::{node::Entity, prelude::*};

///
/// EntityView
///
/// Responsible for generating all view structs (View, Edit, Filter)
/// for a given `Entity`.
///
pub struct EntityView<'a>(pub &'a Entity);

impl EntityView<'_> {
    /// Emits all view types (View + Edit + Filter).
    pub fn view_parts(&self) -> TokenStream {
        let entity = self.0;
        let derives = entity.view_derives();
        let view_ident = entity.view_ident();

        // Main View struct
        let fields = entity.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_type_expr();
            quote!(pub #ident: #ty)
        });

        let view = quote! {
            #derives
            pub struct #view_ident {
                #(#fields),*
            }
        };

        // Other generated variants
        let edit = self.view_type(ViewKind::Edit);
        let filter = self.view_type(ViewKind::Filter);

        quote! {
            #view
            #edit
            #filter
        }
    }

    /// Generates one specific view type (View, Edit, Filter).
    pub fn view_type(&self, kind: ViewKind) -> TokenStream {
        let entity = self.0;
        let derives = entity.view_derives();
        let ident = self.ident_for(kind);
        let fields = self.view_fields_for(kind);

        quote! {
            #derives
            pub struct #ident {
                #fields
            }
        }
    }

    /// Determines the struct name for a given view kind.
    fn ident_for(&self, kind: ViewKind) -> Ident {
        let entity = self.0;
        match kind {
            ViewKind::View => entity.view_ident(),
            ViewKind::Edit => entity.edit_ident(),
            ViewKind::Filter => entity.filter_ident(),
        }
    }

    /// Chooses which fields to include depending on the view kind.
    fn iter_fields_for(&self, kind: ViewKind) -> Box<dyn Iterator<Item = &Field> + '_> {
        let entity = self.0;
        match kind {
            ViewKind::View | ViewKind::Filter => Box::new(entity.fields.iter()),
            ViewKind::Edit => Box::new(entity.iter_editable_fields()),
        }
    }

    /// Emits the field definitions for a specific view type.
    fn view_fields_for(&self, kind: ViewKind) -> TokenStream {
        let fields = self.iter_fields_for(kind);

        let field_tokens = fields.filter_map(|f| {
            let ident = &f.ident;
            match kind {
                ViewKind::View => {
                    let ty = f.value.view_type_expr();
                    Some(quote!(pub #ident: #ty))
                }
                ViewKind::Edit => {
                    let ty = f.value.view_type_expr();
                    Some(quote!(pub #ident: Option<#ty>))
                }
                ViewKind::Filter => f
                    .value
                    .filter_type_expr()
                    .map(|fty| quote!(pub #ident: Option<#fty>)),
            }
        });

        quote!(#(#field_tokens),*)
    }
}
