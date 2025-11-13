use crate::prelude::*;

///
/// FilterableTrait
///

pub struct FilterableTrait;

impl Imp<Newtype> for FilterableTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let item_ty = node.item.type_expr();

        let q = quote! {
            type Filter = <#item_ty as ::mimic::core::traits::Filterable>::Filter;
        };

        let tokens = Implementor::new(node.def(), TraitKind::Filterable)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
