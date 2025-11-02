use crate::{node::Enum, prelude::*};

///
/// EnumView
///

pub struct EnumView<'a>(pub &'a Enum);

impl View for EnumView<'_> {
    type Node = Enum;

    fn node(&self) -> &Self::Node {
        self.0
    }
}

impl ViewType for EnumView<'_> {
    fn view_part(&self) -> TokenStream {
        let node = self.node();
        let view_ident = node.view_ident();
        let view_variants = node.variants.iter().map(HasTypeExpr::type_expr);

        // extra traits
        let mut derives = self.traits();
        if node.is_unit_enum() {
            derives.extend(vec![
                Trait::Copy,
                Trait::Hash,
                Trait::Eq,
                Trait::Ord,
                Trait::PartialEq,
                Trait::PartialOrd,
            ]);
        }

        // handle default manually
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

impl EnumView<'_> {
    fn view_default_impl(&self) -> Option<TokenStream> {
        let node = self.node();
        let view_ident = node.view_ident();
        let default_variant = node.default_variant()?;
        let variant_ident = default_variant.effective_ident();

        // Handle payloads
        let value_expr = if default_variant.value.is_some() {
            quote!((Default::default()))
        } else {
            quote!()
        };

        Some(quote! {
            impl Default for #view_ident {
                fn default() -> Self {
                    Self::#variant_ident #value_expr
                }
            }
        })
    }
}
