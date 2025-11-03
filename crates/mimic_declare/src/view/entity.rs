use crate::{
    node::Entity,
    prelude::*,
    view::{ValueFilter, ValueView},
};

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
        let node = self.node();
        let ident = node.view_ident();
        let fields = node.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: #ty)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.push(Trait::Default);

        quote! {
            #derives
            pub struct #ident {
                #(#fields),*
            }
        }
    }
}

///
/// EntityCreate
///

pub struct EntityCreate<'a>(pub &'a Entity);

impl View for EntityCreate<'_> {
    type Node = Entity;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for EntityCreate<'_> {
    fn view_part(&self) -> TokenStream {
        let node = self.node();
        let create_ident = node.create_ident();
        let fields = node.iter_editable_fields().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: #ty)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.push(Trait::Default);

        quote! {
            #derives
            pub struct #create_ident {
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
        let node = self.node();
        let update_ident = node.update_ident();
        let fields = node.iter_editable_fields().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: Option<#ty>)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.push(Trait::Default);

        quote! {
            #derives
            pub struct #update_ident {
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
        let node = self.node();
        let entity_ident = node.def.ident();
        let filter_ident = node.filter_ident();
        let fields = node.fields.iter().filter_map(|f| {
            let ident = &f.ident;
            let ty = ValueFilter(&f.value).filter_expr()?;

            Some(quote!(pub #ident: Option<#ty>))
        });

        // add in default manually
        let mut derives = self.traits();
        derives.push(Trait::Default);

        quote! {
            #derives
            pub struct #filter_ident {
                #(#fields),*
            }

            impl ::mimic::db::query::IntoFilterOpt for #filter_ident {
                #[inline]
                fn into_filter_opt(self) -> Option<::mimic::db::query::FilterExpr> {
                    <#entity_ident as ::mimic::core::traits::FilterView>::into_expr(self)
                }
            }
        }
    }
}
