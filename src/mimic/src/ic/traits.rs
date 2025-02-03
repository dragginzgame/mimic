use crate::ic::{
    serialize::{deserialize, serialize},
    structures::storable::Bound,
};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

///
/// UnboundedStorable
///

pub trait UnboundedStorable: Serialize + DeserializeOwned {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serialize(self).unwrap())
    }

    #[must_use]
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        deserialize(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
