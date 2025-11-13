use crate::{prelude::*, view::traits::ViewExpr};

///
/// ItemView
///

pub struct ItemView<'a>(pub &'a Item);

impl ViewExpr for ItemView<'_> {
    fn expr(&self) -> Option<TokenStream> {
        let node = self.0;
        let ty = node.target().type_expr();

        quote!(<#ty as ::mimic::core::traits::View>::ViewType).into()
    }
}

///
/// ItemFilter
///

pub struct ItemFilter<'a>(pub &'a Item);

impl ViewExpr for ItemFilter<'_> {
    fn expr(&self) -> Option<TokenStream> {
        let node = self.0;
        let ty = node.target().type_expr();

        quote!(<#ty as ::mimic::core::traits::FilterView>::FilterViewType).into()
    }
}
