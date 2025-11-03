use crate::{
    node::Record,
    prelude::*,
    view::{
        ValueView,
        traits::{View, ViewType},
    },
};

///
/// RecordView
///

pub struct RecordView<'a>(pub &'a Record);

impl View for RecordView<'_> {
    type Node = Record;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for RecordView<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let ident = node.view_ident();
        let fields = node.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: #ty)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.add(TraitKind::Default);

        quote! {
            #derives
            pub struct #ident {
                #(#fields),*
            }
        }
    }
}

impl ToTokens for RecordView<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}

///
/// RecordUpdate
///

pub struct RecordUpdate<'a>(pub &'a Record);

impl View for RecordUpdate<'_> {
    type Node = Record;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for RecordUpdate<'_> {
    fn generate(&self) -> TokenStream {
        let node = self.node();
        let ident = node.update_ident();
        let fields = node.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: Option<#ty>)
        });

        // add in default manually
        let mut derives = self.traits();
        derives.add(TraitKind::Default);

        quote! {
            #derives
            pub struct #ident {
                #(#fields),*
            }
        }
    }
}

impl ToTokens for RecordUpdate<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generate());
    }
}
