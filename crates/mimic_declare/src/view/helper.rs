use crate::{
    prelude::*,
    view::{FieldFilter, traits::ViewExpr},
};

/// Shared generator for any node with a field list (Entity, Record).
pub fn generate_field_list_filter(
    filter_ident: &Ident,
    fields: &FieldList,
    derives: &TraitSet,
) -> TokenStream {
    // Build payload types for each field
    let field_defs = fields.iter().map(|f| FieldFilter(f).expr());

    // Build each field’s scoped filter expression
    let field_exprs = fields.iter().map(|f| {
        let ident = &f.ident;
        let field_name = f.ident.to_string();

        quote! {
            self.#ident.map(|f| {
                ::mimic::db::primitives::filter::IntoScopedFilterExpr::into_scoped(f, #field_name)
            })
        }
    });

    quote! {
        #derives
        pub struct #filter_ident {
            #(#field_defs),*
        }

        impl ::mimic::db::primitives::filter::IntoFilterExpr for #filter_ident {
            fn into_expr(self) -> ::mimic::db::primitives::filter::FilterExpr {
                let filters = [#(#field_exprs),*];
                let exprs = filters.into_iter().flatten();
                ::mimic::db::primitives::filter::FilterDsl::all(exprs)
            }
        }
    }
}
