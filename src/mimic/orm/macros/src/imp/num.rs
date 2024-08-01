use crate::{
    imp::Implementor,
    node::{Newtype, Trait},
};
use orm::types::PrimitiveType;
use proc_macro2::TokenStream;
use quote::ToTokens;

///
/// NumCast
///

pub mod cast {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
        let num_fn = node.primitive.expect("has primitive").num_cast_fn();

        let to_method = format_ident!("to_{}", num_fn);
        let from_method = format_ident!("from_{}", num_fn);

        let q = quote! {
            fn from<T: ::mimic::orm::traits::NumToPrimitive>(n: T) -> Option<Self> {
                let num = n.#to_method()?;
                <Self as ::mimic::orm::traits::NumFromPrimitive>::#from_method(num)
            }
        };

        Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream()
    }
}

///
/// NumFromPrimitive
///

pub mod from_primitive {
    use super::*;

    // newtype
    pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
        let value = &node.value;

        let mut q = quote! {
            fn from_i64(n: i64) -> Option<Self> {
                type Ty = #value;
                Ty::from_i64(n).map(Self)
            }

            fn from_u64(n: u64) -> Option<Self> {
                type Ty = #value;
                Ty::from_u64(n).map(Self)
            }
        };

        // Decimal
        if matches!(
            node.primitive,
            Some(PrimitiveType::Decimal | PrimitiveType::F64)
        ) {
            q.extend(quote! {
                fn from_f64(n: f64) -> Option<Self> {
                    type Ty = #value;
                    Ty::from_f64(n).map(Self)
                }
            });
        }

        Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream()
    }
}

///
/// NumToPrimitive
///

pub mod to_primitive {
    use super::*;

    // newtype
    pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
        let q = quote! {
            fn to_i64(&self) -> Option<i64> {
                ::num_traits::NumCast::from(self.0)
            }

            fn to_u64(&self) -> Option<u64> {
                ::num_traits::NumCast::from(self.0)
            }
        };

        Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream()
    }
}
