use mimic::core::traits::EditView;
use test_design::test::view_into::{
    ViewIntoRoundTrip, ViewIntoRoundTripEdit, ViewIntoRoundTripView,
};

///
/// ViewIntoSuite
/// Validates that generated View and Edit types participate in `Into` conversions.
///

pub struct ViewIntoSuite;

impl ViewIntoSuite {
    pub fn test() {
        Self::view_into_round_trip();
        Self::edit_into_round_trip();
    }

    fn view_into_round_trip() {
        let mut entity = ViewIntoRoundTrip {
            name: "primary".into(),
            score: 42,
            tags: vec!["alpha".into(), "beta".into()],
            nickname: Some("prime".into()),
            ..Default::default()
        };

        let view: ViewIntoRoundTripView = entity.clone().into();
        assert_eq!(view.name, "primary");
        assert_eq!(view.score, 42);
        assert_eq!(view.tags, vec!["alpha", "beta"]);
        assert_eq!(view.nickname.as_deref(), Some("prime"));

        entity.name = "updated".into();
        let from_view: ViewIntoRoundTrip = view.into();
        assert_eq!(from_view.name, "primary");
        assert_eq!(from_view.score, 42);
        assert_eq!(from_view.tags, vec!["alpha", "beta"]);
        assert_eq!(from_view.nickname.as_deref(), Some("prime"));
    }

    fn edit_into_round_trip() {
        let mut target = ViewIntoRoundTrip::default();

        let edit = ViewIntoRoundTripEdit {
            name: Some("patched".into()),
            score: Some(99),
            tags: Some(vec!["fresh".into(), "mint".into()]),
            nickname: Some(Some("pulse".into())),
        };

        <ViewIntoRoundTrip as EditView>::merge(&mut target, edit.clone());
        assert_eq!(target.name, "patched");
        assert_eq!(target.score, 99);
        assert_eq!(target.tags, vec!["fresh", "mint"]);
        assert_eq!(target.nickname.as_deref(), Some("pulse"));

        let from_edit: ViewIntoRoundTrip = edit.into();
        assert_eq!(from_edit.name, "patched");
        assert_eq!(from_edit.score, 99);
        assert_eq!(from_edit.tags, vec!["fresh", "mint"]);
        assert_eq!(from_edit.nickname.as_deref(), Some("pulse"));
    }
}
