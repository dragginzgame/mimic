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
        let item = ItemFilter(&node.item).expr()?;

        let ty = if node.cardinality() == Cardinality::Many {
            quote!(::mimic::db::primitives::NoFilter)
        } else {
            quote!(#item)
        };

        Some(ty)
    }
}
