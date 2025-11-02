mod type_view;

pub use type_view::*;

use crate::prelude::*;

///
/// EditViewTrait
///

pub struct EditViewTrait {}

/// Entity
impl Imp<Entity> for EditViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        Some(update_impl(node, |n| {
            n.iter_editable_fields().map(|f| f.ident.clone()).collect()
        }))
    }
}

/// Record
impl Imp<Record> for EditViewTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        Some(update_impl(node, |n| {
            n.fields.iter().map(|f| f.ident.clone()).collect()
        }))
    }
}

/// Shared generator
fn update_impl<N, F>(node: &N, iter_fields: F) -> TraitStrategy
where
    N: HasDef + HasView,
    F: Fn(&N) -> Vec<syn::Ident>,
{
    let def = node.def();
    let edit_ident = node.edit_ident();
    let field_idents = iter_fields(node);

    let merge_pairs: Vec<_> = field_idents
        .iter()
        .map(|ident| {
            quote! {
                if let Some(value) = view.#ident {
                    self.#ident = ::mimic::core::traits::TypeView::from_view(value);
                }
            }
        })
        .collect();

    let q = quote! {
        type View = #edit_ident;

        fn merge(&mut self, view: Self::View) {
            #(#merge_pairs)*
        }
    };

    let tokens = Implementor::new(def, Trait::EditView)
        .set_tokens(q)
        .to_token_stream();

    TraitStrategy::from_impl(tokens)
}

///
/// FilterViewTrait
///

pub struct FilterViewTrait {}

/// Entity
impl Imp<Entity> for FilterViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let filter_ident = node.filter_ident();

        let field_exprs = node.fields.iter().filter_map(|f| {
            let ident = &f.ident;
            let constant = &f.const_ident();

            f.value
                .filter_type_expr()
                .map(|_| quote! {
                    view.#ident.and_then(|f| ::mimic::db::query::IntoFilterExpr::into_expr(f, Self::#constant))
                })
        });

        let q = quote! {
            type View = #filter_ident;

            fn into_expr(view: Self::View) -> Option<::mimic::db::query::FilterExpr> {
                ::mimic::db::query::FilterDsl::all([#(#field_exprs),*])
            }
        };

        let tokens = Implementor::new(node.def(), Trait::FilterView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
