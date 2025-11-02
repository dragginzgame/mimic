use crate::prelude::*;

///
/// EditViewTrait
///

pub struct EditViewTrait {}

impl Imp<Entity> for EditViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        Some(edit_impl(node, |n| {
            n.iter_editable_fields().map(|f| f.ident.clone()).collect()
        }))
    }
}

impl Imp<Record> for EditViewTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        Some(edit_impl(node, |n| {
            n.fields.iter().map(|f| f.ident.clone()).collect()
        }))
    }
}

/// Shared generator
fn edit_impl<N, F>(node: &N, iter_fields: F) -> TraitStrategy
where
    N: HasViews + HasDef,
    F: Fn(&N) -> Vec<syn::Ident>,
{
    let edit_ident = node.edit_ident();
    let ident = node.def().ident();
    let field_idents = iter_fields(node);

    let merge_pairs: Vec<_> = field_idents
        .iter()
        .map(|ident| {
            quote! {
                if let Some(value) = view.#ident {
                    self.#ident = ::mimic::core::traits::View::from_view(value);
                }
            }
        })
        .collect();

    let q = quote! {
        type EditType = #edit_ident;

        fn merge(&mut self, view: Self::EditType) {
            #(#merge_pairs)*
        }
    };

    let edit_impl = Implementor::new(node.def(), Trait::EditView)
        .set_tokens(q)
        .to_token_stream();
    let conversions = edit_into_conversions(&ident, &edit_ident);
    let tokens = quote! {
        #edit_impl
        #conversions
    };

    TraitStrategy::from_impl(tokens)
}

fn edit_into_conversions(ident: &syn::Ident, edit_ident: &syn::Ident) -> TokenStream {
    quote! {
        impl From<#edit_ident> for #ident {
            fn from(edit: #edit_ident) -> Self {
                let mut value = Self::default();
                <Self as ::mimic::core::traits::EditView>::merge(&mut value, edit);
                value
            }
        }
    }
}
