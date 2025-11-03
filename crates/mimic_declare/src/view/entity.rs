use crate::{
    node::Entity,
    prelude::*,
    view::{
        ValueFilter, ValueView,
        traits::{View, ViewType},
    },
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
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let fields = node.fields.iter().map(|f| {
            let fi = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #fi: #ty)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.add(TraitKind::Default);

        quote! {
            #derives
            pub struct #view_ident {
                #(#fields),*
            }
        }
    }
}

impl ToTokens for EntityView<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
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
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let create_ident = node.create_ident();
        let fields = node.iter_editable_fields().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: #ty)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.add(TraitKind::Default);

        quote! {
            #derives
            pub struct #create_ident {
                #(#fields),*
            }
        }
    }
}

impl ToTokens for EntityCreate<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}

///
/// EntityUpdate
///

pub struct EntityUpdate<'a>(pub &'a Entity);

impl View for EntityUpdate<'_> {
    type Node = Entity;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for EntityUpdate<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let update_ident = node.update_ident();
        let fields = node.iter_editable_fields().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: Option<#ty>)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.add(TraitKind::Default);

        quote! {
            #derives
            pub struct #update_ident {
                #(#fields),*
            }
        }
    }
}

impl ToTokens for EntityUpdate<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
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
    fn generate(&self) -> TokenStream {
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
        derives.add(TraitKind::Default);

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

impl ToTokens for EntityFilter<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
