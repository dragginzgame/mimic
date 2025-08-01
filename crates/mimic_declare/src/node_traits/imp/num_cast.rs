use crate::{
    node::Newtype,
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::{HasIdent, HasTypePart},
};
use quote::{ToTokens, format_ident, quote};

///
/// NumCastTrait
///

pub struct NumCastTrait {}

///
/// Newtype
///

impl Imp<Newtype> for NumCastTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let num_fn = node.primitive.num_cast_fn();
        let to_method = format_ident!("to_{}", num_fn);
        let from_method = format_ident!("from_{}", num_fn);

        let q = quote! {
            fn from<T: ::mimic::core::traits::NumToPrimitive>(n: T) -> Option<Self> {
                let num = n.#to_method()?;
                <Self as ::mimic::core::traits::NumFromPrimitive>::#from_method(num)
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::NumCast)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// NumFromPrimitiveTrait
///

pub struct NumFromPrimitiveTrait {}

///
/// Newtype
///

impl Imp<Newtype> for NumFromPrimitiveTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let item = &node.item.type_part();

        let mut q = quote! {
            fn from_i64(n: i64) -> Option<Self> {
                #item::from_i64(n).map(Self)
            }

            fn from_u64(n: u64) -> Option<Self> {
                #item::from_u64(n).map(Self)
            }
        };

        // Decimal
        if node.primitive.is_float() {
            q.extend(quote! {
                fn from_f64(n: f64) -> Option<Self> {
                    #item::from_f64(n).map(Self)
                }
            });
        }

        let tokens = Implementor::new(node.ident(), Trait::NumFromPrimitive)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// NumToPrimitiveTrait
///

pub struct NumToPrimitiveTrait {}

///
/// Newtype
///

impl Imp<Newtype> for NumToPrimitiveTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let q = quote! {
            fn to_i64(&self) -> Option<i64> {
                ::mimic::export::num_traits::NumCast::from(self.0)
            }

            fn to_u64(&self) -> Option<u64> {
                ::mimic::export::num_traits::NumCast::from(self.0)
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::NumToPrimitive)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
