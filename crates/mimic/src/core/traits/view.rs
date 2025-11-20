use std::{
    collections::{HashMap, HashSet, hash_map::Entry as HashMapEntry},
    hash::{BuildHasher, Hash},
    iter::IntoIterator,
};

///
/// View
/// Recursive for all field/value nodes
///

pub trait View {
    type ViewType: Default;

    fn to_view(&self) -> Self::ViewType;
    fn from_view(view: Self::ViewType) -> Self;
}

impl View for String {
    type ViewType = Self;

    fn to_view(&self) -> Self::ViewType {
        self.clone()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view
    }
}

impl<T: View> View for Box<T> {
    type ViewType = Box<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        Box::new((**self).to_view())
    }

    fn from_view(view: Self::ViewType) -> Self {
        Self::new(T::from_view(*view))
    }
}

impl<T: View> View for Option<T> {
    type ViewType = Option<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        self.as_ref().map(View::to_view)
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.map(T::from_view)
    }
}

impl<T: View> View for Vec<T> {
    type ViewType = Vec<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        self.iter().map(View::to_view).collect()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.into_iter().map(T::from_view).collect()
    }
}

impl<T, S> View for HashSet<T, S>
where
    T: View + Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type ViewType = Vec<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        self.iter().map(View::to_view).collect()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.into_iter().map(T::from_view).collect()
    }
}

impl<K, V, S> View for HashMap<K, V, S>
where
    K: View + Eq + Hash + Clone,
    V: View,
    S: BuildHasher + Default,
{
    type ViewType = Vec<(K::ViewType, V::ViewType)>;

    fn to_view(&self) -> Self::ViewType {
        self.iter()
            .map(|(k, v)| (k.to_view(), v.to_view()))
            .collect()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.into_iter()
            .map(|(k, v)| (K::from_view(k), V::from_view(v)))
            .collect()
    }
}

#[macro_export]
macro_rules! impl_view {
    ($($type:ty),*) => {
        $(
            impl View for $type {
                type ViewType = Self;

                fn to_view(&self) -> Self::ViewType {
                    *self
                }

                fn from_view(view: Self::ViewType) -> Self {
                    view
                }
            }
        )*
    };
}

impl_view!(bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

///
/// CreateView
///

pub trait CreateView {
    type CreateViewType: Default;
}

///
/// UpdateView
///

pub trait UpdateView {
    type UpdateViewType: Default;

    /// merge the updateview into self
    fn merge(&mut self, _: Self::UpdateViewType) {}
}

impl<T> UpdateView for Option<T>
where
    T: UpdateView + Default,
{
    type UpdateViewType = Option<T::UpdateViewType>;

    fn merge(&mut self, update: Self::UpdateViewType) {
        if let Some(inner_update) = update {
            if let Some(inner_value) = self {
                inner_value.merge(inner_update);
            } else {
                let mut new_value = T::default();
                new_value.merge(inner_update);
                *self = Some(new_value);
            }
        }
    }
}

impl<T> UpdateView for Vec<T>
where
    T: UpdateView,
{
    type UpdateViewType = Vec<T::UpdateViewType>;

    fn merge(&mut self, updates: Self::UpdateViewType) {
        for (elem, update) in self.iter_mut().zip(updates.into_iter()) {
            elem.merge(update);
        }
        // ignore trailing updates
        // do not append or truncate
    }
}

impl<T, S> UpdateView for HashSet<T, S>
where
    T: UpdateView + Default + Eq + Hash,
    S: BuildHasher + Default,
{
    type UpdateViewType = Vec<T::UpdateViewType>;

    fn merge(&mut self, updates: Self::UpdateViewType) {
        self.clear();

        for patch in updates {
            let mut value = T::default();
            value.merge(patch);
            self.insert(value);
        }
    }
}

impl<K, V, S> UpdateView for HashMap<K, V, S>
where
    K: UpdateView + Default + Eq + Hash + Clone,
    V: UpdateView + Default,
    S: BuildHasher + Default,
{
    type UpdateViewType = Vec<(K::UpdateViewType, Option<V::UpdateViewType>)>;

    fn merge(&mut self, updates: Self::UpdateViewType) {
        for (k_patch, v_patch) in updates {
            let mut key = K::default();
            key.merge(k_patch);

            match v_patch {
                Some(vu) => match self.entry(key) {
                    HashMapEntry::Occupied(mut e) => {
                        e.get_mut().merge(vu);
                    }
                    HashMapEntry::Vacant(e) => {
                        let mut v = V::default();
                        v.merge(vu);
                        e.insert(v);
                    }
                },
                None => {
                    self.remove(&key);
                }
            }
        }
    }
}

macro_rules! impl_update_view {
    ($($type:ty),*) => {
        $(
            impl UpdateView for $type {
                type UpdateViewType = Self;

                fn merge(&mut self, update: Self::UpdateViewType) {
                    *self = update;
                }
            }
        )*
    };
}

impl_update_view!(bool, i8, i16, i32, i64, u8, u16, u32, u64, String);

///
/// FilterView
///

pub trait FilterView {
    type FilterViewType: Default + ::mimic::db::primitives::IntoFilterExpr;
}

///
/// TESTS
///

#[cfg(test)]
mod test {
    use mimic::core::traits::UpdateView;
    use std::collections::{HashMap, HashSet};

    //
    // Vec<T>
    //

    #[test]
    fn vec_merge_updates_existing_elements_only() {
        let mut v = vec![1u8, 2, 3];

        // Vec<T>::UpdateViewType = Vec<T::UpdateViewType> = Vec<u8>
        let patch = vec![10u8, 20];

        v.merge(patch);

        // first two positions updated, tail untouched
        assert_eq!(v, vec![10, 20, 3]);
    }

    #[test]
    fn vec_merge_ignores_extra_updates() {
        let mut v = vec![1u8, 2];

        // patch is longer than vec; extra element should be ignored
        let patch = vec![10u8, 20, 30];

        v.merge(patch);

        assert_eq!(v, vec![10, 20]);
    }

    //
    // HashSet<T>
    //

    #[test]
    fn hashset_merge_replaces_entire_set() {
        let mut set: HashSet<u8> = [1u8, 2, 3].into_iter().collect();

        // HashSet<T>::UpdateViewType = Vec<T::UpdateViewType> = Vec<u8>
        let patch = vec![5u8, 6];

        set.merge(patch);

        let result: HashSet<u8> = [5u8, 6].into_iter().collect();
        assert_eq!(set, result);
    }

    #[test]
    fn hashset_merge_can_clear_via_empty_patch() {
        let mut set: HashSet<u8> = [1u8, 2, 3].into_iter().collect();

        let patch: Vec<u8> = Vec::new();
        set.merge(patch);

        assert!(set.is_empty());
    }

    //
    // HashMap<K, V>
    //

    #[test]
    fn hashmap_merge_updates_inserts_and_removes_keys() {
        let mut map: HashMap<String, u8> = [
            ("a".to_string(), 1u8),
            ("b".to_string(), 2u8),
            ("keep".to_string(), 9u8),
        ]
        .into_iter()
        .collect();

        // HashMap<K,V>::UpdateViewType = Vec<(K, Option<V::UpdateViewType>)>
        let patch = vec![
            // update existing key "a"
            ("a".to_string(), Some(10u8)),
            // remove key "b"
            ("b".to_string(), None),
            // insert new key "c"
            ("c".to_string(), Some(30u8)),
        ];

        map.merge(patch);

        assert_eq!(map.get("a"), Some(&10));
        assert_eq!(map.get("c"), Some(&30));
        assert!(!map.contains_key("b"), "b should be removed");
        assert_eq!(map.get("keep"), Some(&9), "untouched keys must remain");
    }

    #[test]
    #[allow(clippy::iter_on_single_items)]
    fn hashmap_merge_ignores_none_for_absent_keys() {
        let mut map: HashMap<String, u8> = [("x".to_string(), 1u8)].into_iter().collect();

        let patch = vec![
            // None for a key that doesn't exist should be a no-op
            ("y".to_string(), None),
        ];

        map.merge(patch);

        assert_eq!(map.len(), 1);
        assert_eq!(map.get("x"), Some(&1));
        assert!(!map.contains_key("y"));
    }
}
