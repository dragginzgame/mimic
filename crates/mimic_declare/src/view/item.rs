use crate::prelude::*;

///
/// ItemView
///

pub struct ItemView<'a>(pub &'a Item);

impl View for ItemView<'_> {
    type Node = Item;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ItemView<'_> {
    pub fn view_expr(&self) -> TokenStream {
        let item = self.node();
        let view_ident = self.view_ident();
        let target_view = item.target.type_expr();

        quote! {
            pub type #view_ident = #target_view;
        }
    }
}
