use crate::{node::Record, prelude::*, view::ValueView};

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
/// RecordEdit
///

pub struct RecordEdit<'a>(pub &'a Record);

impl View for RecordEdit<'_> {
    type Node = Record;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for RecordEdit<'_> {
    fn view_part(&self) -> TokenStream {
        let node = self.node();
        let ident = node.edit_ident();
        let fields = node.fields.iter().map(|f| {
            let ident = &f.ident;
            let ty = ValueView(&f.value).view_expr();

            quote!(pub #ident: Option<#ty>)
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
