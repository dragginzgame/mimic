use crate::orm::traits::Orderable;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::{collections::HashMap, fmt, hash::Hash, marker::PhantomData};

///
/// StrHashMap
///
/// A wrapper around HashMap that enforces FromStr on keys, so we can deserialize Strings
/// into a StrHashMap<u32, u32> for instance
///

#[derive(CandidType, Clone, Debug, Deref, DerefMut, Eq, PartialEq, Serialize)]
pub struct StrHashMap<K, V>(HashMap<K, V>)
where
    K: Eq + Hash;

impl<K, V> Default for StrHashMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(HashMap::<K, V>::new())
    }
}

impl<'de, K, V> Deserialize<'de> for StrHashMap<K, V>
where
    K: Eq + Hash + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StrHashMapVisitor {
            marker: PhantomData,
        })
    }
}

impl<K, V, IK, IV> From<Vec<(IK, IV)>> for StrHashMap<K, V>
where
    K: Eq + Hash,
    IK: Into<K>,
    IV: Into<V>,
{
    fn from(vec: Vec<(IK, IV)>) -> Self {
        let map = vec
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();

        Self(map)
    }
}

impl<K, V> FromIterator<(K, V)> for StrHashMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(iter.into_iter().collect::<HashMap<K, V>>())
    }
}

impl<K, V> Orderable for StrHashMap<K, V> where K: Eq + Hash {}

///
/// StrHashMapVisitor
///

struct StrHashMapVisitor<K, V> {
    marker: PhantomData<fn() -> (K, V)>,
}

impl<'de, K, V> Visitor<'de> for StrHashMapVisitor<K, V>
where
    K: Eq + Hash + Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = StrHashMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<StrHashMap<K, V>, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = HashMap::new();
        while let Some((key, value)) = access.next_entry::<K, V>()? {
            map.insert(key, value);
        }

        Ok(StrHashMap(map))
    }
}
