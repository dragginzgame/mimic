use crate::{node::Enum, prelude::*};

///
/// ListView
///

pub struct ListView<'a>(pub &'a Enum);

impl View for ListView<'_> {
    type Node = Entity;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl HasViews for List {
    fn view_parts(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let item_view = HasViewExpr::view_type_expr(&self.item);

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}
