use crate::{prelude::*, view::ValueFilter};

pub struct FilterViewTrait;

impl Imp<Entity> for FilterViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let filter_ident = node.filter_ident();

        // Each field -> Option<FilterExpr>
        let field_exprs = node.fields.iter().filter_map(|f| {
            let ident = &f.ident;
            let constant = &f.const_ident();

            ValueFilter(&f.value).filter_expr().map(|_| {
                quote! {
                    view.#ident
                        .map(|f| ::mimic::db::query::IntoFilterExpr::into_expr(f, Self::#constant))
                }
            })
        });

        // Build the final FilterView impl
        let q = quote! {
            type FilterType = #filter_ident;

            fn into_expr(view: Self::FilterType) -> ::mimic::db::query::FilterExpr {
                let filters = [#(#field_exprs),*];
                let exprs = filters.into_iter().flatten();

                ::mimic::db::query::FilterDsl::all(exprs)
            }
        };

        let tokens = Implementor::new(node.def(), TraitKind::FilterView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
