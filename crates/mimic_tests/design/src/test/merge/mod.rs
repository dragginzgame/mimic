use crate::prelude::*;

///
/// MergeEntity
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(ident = "name", value(item(prim = "Text"))),
        field(ident = "score", value(item(prim = "Nat32"))),
        field(ident = "nickname", value(opt, item(prim = "Text"))),
        field(ident = "scores", value(many, item(prim = "Nat32"))),
        field(ident = "tags", value(item(is = "MergeTags"))),
        field(ident = "settings", value(item(is = "MergeSettings"))),
        field(ident = "profile", value(item(is = "MergeProfile"))),
        field(ident = "wrapper", value(item(is = "MergeWrapper"))),
        field(ident = "tuple_field", value(item(is = "MergeTuple"))),
        field(ident = "opt_profile", value(opt, item(is = "MergeProfile")))
    )
)]
pub struct MergeEntity {}

///
/// MergeSettings
///

#[map(key(prim = "Text"), value(item(prim = "Nat32")))]
pub struct MergeSettings {}

///
/// MergeTags
///

#[set(item(prim = "Text"))]
pub struct MergeTags {}

///
/// MergeProfile
///

#[record(fields(
    field(ident = "bio", value(item(prim = "Text"))),
    field(ident = "visits", value(item(prim = "Nat32"))),
    field(ident = "favorite_numbers", value(many, item(prim = "Nat32")))
))]
pub struct MergeProfile {}

///
/// MergeWrapper
///

#[newtype(item(is = "MergeProfile"))]
pub struct MergeWrapper {}

///
/// MergeTuple
///

#[tuple(value(item(prim = "Text")), value(item(prim = "Nat32")))]
pub struct MergeTuple {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use mimic::core::{traits::UpdateView, view::Update};
    use std::collections::{HashMap, HashSet};

    fn profile(bio: &str, visits: u32, favorites: &[u32]) -> MergeProfile {
        MergeProfile {
            bio: bio.into(),
            visits,
            favorite_numbers: favorites.to_vec(),
        }
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn entity_merge_updates_nested_structures() {
        let mut entity = MergeEntity {
            name: "original".into(),
            score: 7,
            nickname: None,
            scores: vec![1, 2, 3],
            tags: MergeTags::from(vec!["red".to_string(), "blue".to_string()]),
            settings: MergeSettings::from(vec![
                ("volume".to_string(), 10u32),
                ("remove".to_string(), 5u32),
            ]),
            profile: profile("quiet", 1, &[10, 11]),
            wrapper: MergeWrapper(profile("nested", 3, &[42])),
            tuple_field: MergeTuple("alpha".into(), 1),
            opt_profile: None,
            ..Default::default()
        };

        let mut update: Update<MergeEntity> = Default::default();
        update.name = Some("updated".into());
        update.nickname = Some(Some("nick".into()));
        update.scores = Some(vec![10, 20]);
        update.tags = Some(vec!["green".into()]);
        update.settings = Some(vec![
            ("volume".to_string(), Some(77u32)),
            ("remove".to_string(), None),
            ("insert".to_string(), Some(9u32)),
        ]);
        update.profile = Some(MergeProfileUpdate {
            visits: Some(10),
            ..Default::default()
        });
        update.wrapper = Some(MergeWrapperUpdate {
            bio: Some("outer".into()),
            ..Default::default()
        });
        update.tuple_field = Some((Some("omega".into()), Some(99)));
        update.opt_profile = Some(Some(MergeProfileUpdate {
            bio: Some("loaded".into()),
            visits: Some(2),
            favorite_numbers: None,
        }));

        entity.merge(update);

        assert_eq!(entity.name, "updated");
        assert_eq!(entity.nickname.as_deref(), Some("nick"));
        assert_eq!(entity.scores, vec![10, 20, 3]);

        let tags: HashSet<_> = entity.tags.iter().cloned().collect();
        let expected_tags: HashSet<_> = vec!["green".to_string()].into_iter().collect();
        assert_eq!(tags, expected_tags);

        let settings: HashMap<_, _> = entity
            .settings
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        assert_eq!(settings.get("volume"), Some(&77));
        assert_eq!(settings.get("insert"), Some(&9));
        assert!(!settings.contains_key("remove"));

        assert_eq!(entity.profile.visits, 10);
        assert_eq!(entity.wrapper.0.bio, "outer");
        assert_eq!(entity.tuple_field.0, "omega");
        assert_eq!(entity.tuple_field.1, 99);
        let opt_profile = entity.opt_profile.unwrap();
        assert_eq!(opt_profile.bio, "loaded");
        assert_eq!(opt_profile.visits, 2);
        assert!(opt_profile.favorite_numbers.is_empty());
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn record_merge_preserves_unset_fields() {
        let mut profile = profile("start", 1, &[1, 2, 3]);
        let mut update: Update<MergeProfile> = Default::default();
        update.bio = Some("updated".into());
        profile.merge(update);

        assert_eq!(profile.bio, "updated");
        assert_eq!(profile.visits, 1);
        assert_eq!(profile.favorite_numbers, vec![1, 2, 3]);
    }

    #[test]
    fn map_and_set_merge_behaviors() {
        let mut tags = MergeTags::from(vec!["old".to_string(), "stale".to_string()]);
        tags.merge(vec!["fresh".to_string(), "new".to_string()]);
        let tag_set: HashSet<_> = tags.iter().cloned().collect();
        let expected: HashSet<_> = vec!["fresh".to_string(), "new".to_string()]
            .into_iter()
            .collect();
        assert_eq!(tag_set, expected);

        let mut settings =
            MergeSettings::from(vec![("keep".to_string(), 1u32), ("drop".to_string(), 2u32)]);
        let patch: Update<MergeSettings> = vec![
            ("keep".to_string(), Some(5u32)),
            ("drop".to_string(), None),
            ("add".to_string(), Some(9u32)),
        ];
        settings.merge(patch);

        let settings_map: HashMap<_, _> = settings.iter().map(|(k, v)| (k.clone(), *v)).collect();
        assert_eq!(settings_map.get("keep"), Some(&5));
        assert_eq!(settings_map.get("add"), Some(&9));
        assert!(!settings_map.contains_key("drop"));
    }
}
