use crate::{prelude::*, view::ItemView};

///
/// SetView
///

pub struct SetView<'a>(pub &'a Set);

impl View for SetView<'_> {
    type Node = Set;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for SetView<'_> {
    fn view_part(&self) -> TokenStream {
        let set = self.node();
        let view_ident = self.view_ident();
        let item_view = ItemView(&set.item).view_expr();

        quote! {
            pub type #view_ident = Vec<#item_view>;
        }
    }
}
