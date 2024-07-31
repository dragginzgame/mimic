use crate::traits::Orderable;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::{collections::HashMap as StdHashMap, fmt, hash::Hash, marker::PhantomData};

///
/// Key (Trait)
///

pub trait Key: Eq + Hash {}
impl<T: Eq + Hash> Key for T {}

///
/// HashMap
///
/// A wrapper around HashMap that enforces FromStr on keys, so we can deserialize Strings
/// into a StrHashMap<u32, u32> for instance
///

#[derive(CandidType, Clone, Debug, Deref, DerefMut, Eq, PartialEq, Serialize)]
pub struct HashMap<K, V>(StdHashMap<K, V>)
where
    K: Key;

impl<K, V> Default for HashMap<K, V>
where
    K: Key,
{
    fn default() -> Self {
        Self(StdHashMap::<K, V>::new())
    }
}

impl<'de, K, V> Deserialize<'de> for HashMap<K, V>
where
    K: Deserialize<'de> + Key,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(HashMapVisitor {
            marker: PhantomData,
        })
    }
}

///
/// HashMapVisitor
///

struct HashMapVisitor<K, V> {
    marker: PhantomData<fn() -> (K, V)>,
}

impl<'de, K, V> Visitor<'de> for HashMapVisitor<K, V>
where
    K: Key + Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = HashMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<HashMap<K, V>, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = StdHashMap::new();
        while let Some((key, value)) = access.next_entry::<K, V>()? {
            map.insert(key, value);
        }
        Ok(HashMap(map))
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Key,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<StdHashMap<K, V>>())
    }
}

impl<K, V> Orderable for HashMap<K, V> where K: Key {}
