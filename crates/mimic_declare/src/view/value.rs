use crate::{
    prelude::*,
    view::{ItemFilter, ItemView, traits::ViewExpr},
};

///
/// ValueView
///

pub struct ValueView<'a>(pub &'a Value);

impl ViewExpr for ValueView<'_> {
    fn expr(&self) -> Option<TokenStream> {
        let node = self.0;
        let item = ItemView(&node.item).expr();

        match node.cardinality() {
            Cardinality::One => quote!(#item),
            Cardinality::Opt => quote!(Option<#item>),
            Cardinality::Many => quote!(Vec<#item>),
        }
        .into()
    }
}

///
/// ValueFilter
///

pub struct ValueFilter<'a>(pub &'a Value);

impl ViewExpr for ValueFilter<'_> {
    fn expr(&self) -> Option<TokenStream> {
        let node = self.0;

        match node.cardinality() {
            Cardinality::Many => {
                // For lists, we’ll usually use ContainsFilter
                Some(quote!(::mimic::db::query::ContainsFilter))
            }
            Cardinality::Opt | Cardinality::One => {
                // Delegate to ItemFilter for the actual type
                let item = ItemFilter(&node.item);
                item.expr()
            }
        }
    }
}
