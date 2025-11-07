use crate::prelude::*;

///
/// AddTrait
///

pub struct AddTrait {}

impl Imp<Newtype> for AddTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let primitive = node.primitive.as_ref()?; // bail early if no primitive

        let ident = &node.def.ident();
        let prim = &primitive.as_type();

        // Quote the implementation of Add for the newtype
        let tokens = quote! {
            impl ::mimic::core::traits::Add<#prim> for #ident {
                type Output = #ident;

                fn add(self, other: #prim) -> Self::Output {
                    Self(self.0 + other)
                }
            }

            impl ::mimic::core::traits::Add<#ident> for #prim {
                type Output = #ident;

                fn add(self, other: #ident) -> Self::Output {
                    #ident(self + other.0)
                }
            }

            // Optionally: also implement Add<#ident> for #ident itself
            impl ::mimic::core::traits::Add<#ident> for #ident {
                type Output = #ident;

                fn add(self, other: #ident) -> Self::Output {
                    Self(self.0 + other.0)
                }
            }
        };

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// AddAssignTrait
///

pub struct AddAssignTrait {}

impl Imp<Newtype> for AddAssignTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let primitive = node.primitive.as_ref()?; // bail early if no primitive

        let ident = &node.def.ident();
        let prim = &primitive.as_type();

        // Quote the implementation of AddAssign for the newtype
        let tokens = quote! {
            impl ::mimic::core::traits::AddAssign<#prim> for #ident {
                fn add_assign(&mut self, other: #prim) {
                    self.0 += other;
                }
            }

            impl ::mimic::core::traits::AddAssign<#ident> for #ident {
                fn add_assign(&mut self, other: #ident) {
                    self.0 += other.0;
                }
            }
        };

        Some(TraitStrategy::from_impl(tokens))
    }
}
