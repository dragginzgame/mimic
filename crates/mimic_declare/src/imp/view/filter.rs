use crate::{prelude::*, view::ValueFilter};

///
/// FilterViewTrait
///

pub struct FilterViewTrait {}

impl Imp<Entity> for FilterViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let filter_ident = node.filter_ident();

        let field_exprs = node.fields.iter().filter_map(|f| {
            let ident = &f.ident;
            let constant = &f.const_ident();

            ValueFilter(&f.value)
                .filter_expr()
                .map(|_| quote! {
                    view.#ident.and_then(|f| ::mimic::db::query::IntoFilterExpr::into_expr(f, Self::#constant))
                })
        });

        let q = quote! {
            type FilterType = #filter_ident;

            fn into_expr(view: Self::FilterType) -> Option<::mimic::db::query::FilterExpr> {
                ::mimic::db::query::FilterDsl::all([#(#field_exprs),*])
            }
        };

        let tokens = Implementor::new(node.def(), TraitKind::FilterView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
