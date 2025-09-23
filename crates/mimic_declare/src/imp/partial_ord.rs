use crate::{
    imp::{Imp, TraitStrategy},
    node::Newtype,
};
use quote::quote;

///
/// PartialOrdTrait
///

pub struct PartialOrdTrait {}

///
/// Newtype
///

impl Imp<Newtype> for PartialOrdTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let ident = &node.def.ident();
        let prim = &node.primitive.as_type();

        let tokens = quote! {
            impl PartialOrd<#prim> for #ident {
                fn partial_cmp(&self, other: &#prim) -> Option<::std::cmp::Ordering> {
                    self.0.partial_cmp(other)
                }
            }

            impl PartialOrd<#ident> for #prim {
                fn partial_cmp(&self, other: &#ident) -> Option<::std::cmp::Ordering> {
                    self.partial_cmp(&other.0)
                }
            }

            impl PartialOrd<#prim> for &#ident {
                fn partial_cmp(&self, other: &#prim) -> Option<::std::cmp::Ordering> {
                    <#ident as PartialOrd<#prim>>::partial_cmp(*self, other)
                }
            }

            impl PartialOrd<&#ident> for #prim {
                fn partial_cmp(&self, other: &&#ident) -> Option<::std::cmp::Ordering> {
                    <#prim as PartialOrd<#ident>>::partial_cmp(self, *other)
                }
            }
        };

        Some(TraitStrategy::from_impl(tokens))
    }
}
