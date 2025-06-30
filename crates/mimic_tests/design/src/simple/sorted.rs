use crate::prelude::*;

///
/// SortedEnum
///

#[enum_(
    variant(name = "A"),
    variant(name = "B"),
    variant(name = "C"),
    variant(name = "D"),
    traits(add(Sorted))
)]
pub struct SortedEnum {}
