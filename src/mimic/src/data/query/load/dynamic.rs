use crate::{
    data::query::{LoadFormat, Selector},
    traits::EntityKind,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// LoadQueryDyn
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQueryDyn {
    pub selector: Selector,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
    pub include_children: bool,
}

impl LoadQueryDyn {
    #[must_use]
    pub fn new(selector: Selector) -> Self {
        Self {
            selector,
            ..Default::default()
        }
    }

    // format
    #[must_use]
    pub const fn format(mut self, format: LoadFormat) -> Self {
        self.format = format;
        self
    }

    // offset
    #[must_use]
    pub const fn offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub const fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }

    // children
    #[must_use]
    pub const fn children(mut self) -> Self {
        self.include_children = true;
        self
    }
}

///
/// LoadQueryDynBuilder
///

#[derive(Debug, Default)]
pub struct LoadQueryDynBuilder<E>
where
    E: EntityKind,
{
    phantom: PhantomData<E>,
}

impl<E> LoadQueryDynBuilder<E>
where
    E: EntityKind,
{
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // selector
    #[must_use]
    pub fn selector(self, selector: Selector) -> LoadQueryDyn {
        LoadQueryDyn::new(selector)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryDyn {
        LoadQueryDyn::new(Selector::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryDyn {
        LoadQueryDyn::new(Selector::Only)
    }

    // one
    pub fn one<S: ToString>(self, ck: &[S]) -> LoadQueryDyn {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let selector = Selector::One(ck_str);

        LoadQueryDyn::new(selector)
    }

    // many
    #[must_use]
    pub fn many<I, S>(self, cks: I) -> LoadQueryDyn
    where
        I: IntoIterator,
        I::Item: IntoIterator<Item = S>,
        S: ToString,
    {
        let keys: Vec<Vec<String>> = cks
            .into_iter()
            .map(|key| key.into_iter().map(|s| s.to_string()).collect())
            .collect();
        let selector = Selector::Many(keys);

        LoadQueryDyn::new(selector)
    }

    // range
    pub fn range<S: ToString>(self, start: &[S], end: &[S]) -> LoadQueryDyn {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let selector = Selector::Range(start, end);

        LoadQueryDyn::new(selector)
    }

    // prefix
    pub fn prefix<S: ToString>(self, prefix: &[S]) -> LoadQueryDyn {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let selector = Selector::Prefix(prefix);

        LoadQueryDyn::new(selector)
    }
}
