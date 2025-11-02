use crate::{node::Entity, prelude::*};

///
/// EntityView
///

pub struct EntityView<'a>(pub &'a Entity);

impl View for EntityView<'_> {
    type Node = Entity;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for EntityView<'_> {
    fn view_part(&self) -> TokenStream {
        let entity = self.node();
        let derives = self.view_derives();
        let ident = self.view_ident();

        let fields = entity.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_expr();

            quote!(pub #ident: #ty)
        });

        quote! {
            #derives
            pub struct #ident {
                #(#fields),*
            }
        }
    }
}

///
/// EntityEdit
///

pub struct EntityEdit<'a>(pub &'a Entity);

impl View for EntityEdit<'_> {
    type Node = Entity;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for EntityEdit<'_> {
    fn view_part(&self) -> TokenStream {
        let entity = self.node();
        let derives = self.view_derives();
        let ident = self.edit_ident();

        let fields = entity.iter_editable_fields().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_expr();

            quote!(pub #ident: Option<#ty>)
        });

        quote! {
            #derives
            pub struct #ident {
                #(#fields),*
            }
        }
    }
}

///
/// EntityFilter
///

pub struct EntityFilter<'a>(pub &'a Entity);

impl View for EntityFilter<'_> {
    type Node = Entity;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for EntityFilter<'_> {
    fn view_part(&self) -> TokenStream {
        let entity = self.node();
        let derives = self.view_derives();
        let ident = self.filter_ident();

        let fields = entity.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = f.value.view_expr();

            quote!(pub #ident: Option<#ty>)
        });

        quote! {
            #derives
            pub struct #ident {
                #(#fields),*
            }
        }
    }
}
