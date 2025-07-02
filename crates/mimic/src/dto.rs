use std::{collections::HashMap, hash::Hash};

///
/// DTO Helpers
///

pub fn try_map<A, B, E>(opt: Option<A>) -> Result<Option<B>, E>
where
    B: TryFrom<A, Error = E>,
{
    opt.map(B::try_from).transpose()
}

pub fn map_option<'a, A, B>(opt: Option<&'a A>) -> Option<B>
where
    B: From<&'a A>,
{
    opt.map(From::from)
}

pub fn try_map_option<'a, A, B, E>(opt: Option<&'a A>) -> Result<Option<B>, E>
where
    B: TryFrom<&'a A, Error = E>,
{
    opt.map(TryFrom::try_from).transpose()
}

#[must_use]
pub fn map_hashmap<K, V, K2, V2>(input: &HashMap<K, V>) -> HashMap<K2, V2>
where
    K: Clone + Eq + Hash,
    K2: From<K> + Eq + Hash,
    for<'a> V2: From<&'a V>,
{
    input
        .iter()
        .map(|(k, v)| (K2::from(k.clone()), V2::from(v)))
        .collect()
}

pub fn try_map_hashmap<K, V, K2, V2, E>(input: &HashMap<K, V>) -> Result<HashMap<K2, V2>, E>
where
    K: Clone + Eq + Hash,
    K2: TryFrom<K, Error = E> + Eq + Hash,
    for<'a> V2: TryFrom<&'a V, Error = E>,
{
    input
        .iter()
        .map(|(k, v)| Ok((K2::try_from(k.clone())?, V2::try_from(v)?)))
        .collect()
}

pub fn try_map_vec<'a, A, B, E, I>(input: I) -> Result<Vec<B>, E>
where
    A: 'a,
    I: IntoIterator<Item = &'a A>,
    B: TryFrom<&'a A, Error = E>,
{
    input.into_iter().map(B::try_from).collect()
}
