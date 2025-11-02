use crate::{node::Enum, prelude::*};

///
/// EnumView
///

pub struct EnumView<'a>(pub &'a Enum);

impl View for EnumView<'_> {
    type Node = Entity;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for EnumView<'_> {
    fn view_part(&self) -> TokenStream {
        let view_ident = self.view_ident();
        let view_variants = self.variants.iter().map(HasTypeExpr::type_expr);

        // extra traits
        let mut derives = self.derives();
        if self.is_unit_enum() {
            derives.extend(vec![
                Trait::Copy,
                Trait::Hash,
                Trait::Eq,
                Trait::Ord,
                Trait::PartialEq,
                Trait::PartialOrd,
            ]);
        }

        let default_impl = self.view_default_impl();

        quote! {
            #derives
            pub enum #view_ident {
                #(#view_variants),*
            }

            #default_impl
        }
    }
}

/*
impl HasViewDefault for Enum {
    fn view_default_impl(&self) -> Option<TokenStream> {
        let default_variant = self.default_variant()?;
        let variant_ident = default_variant.effective_ident();

        // Handle payloads
        let value_expr = if default_variant.value.is_some() {
            quote!((Default::default()))
        } else {
            quote!()
        };

        let view_ident = self.view_ident();

        Some(quote! {
            impl Default for #view_ident {
                fn default() -> Self {
                    Self::#variant_ident #value_expr
                }
            }
        })
    }
}
*/
