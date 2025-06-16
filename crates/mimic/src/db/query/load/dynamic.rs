use crate::{
    db::{
        query::LoadFormat,
        types::{Selector, Where},
    },
    types::Key,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadQueryDynBuilder
///

#[derive(Debug, Default)]
pub struct LoadQueryDynBuilder {}

impl LoadQueryDynBuilder {
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
    pub fn one<K: Into<Key>>(self, key: K) -> LoadQueryDyn {
        let selector = Selector::One(key.into());

        LoadQueryDyn::new(selector)
    }

    // many
    #[must_use]
    pub fn many<K>(self, keys: &[K]) -> LoadQueryDyn
    where
        K: Clone + Into<Key>,
    {
        let keys = keys.iter().cloned().map(Into::into).collect();
        let selector = Selector::Many(keys);

        LoadQueryDyn::new(selector)
    }

    // range
    pub fn range<K: Into<Key>>(self, start: K, end: K) -> LoadQueryDyn {
        let selector = Selector::Range(start.into(), end.into());

        LoadQueryDyn::new(selector)
    }

    // prefix
    pub fn prefix<K: Into<Key>>(self, prefix: K) -> LoadQueryDyn {
        let selector = Selector::Prefix(prefix.into());

        LoadQueryDyn::new(selector)
    }
}

///
/// LoadQueryDyn
/// does not filter by there Where clause, is only there for lookup
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoadQueryDyn {
    pub selector: Selector,
    pub format: LoadFormat,
    pub lookup: Option<Where>,
    pub limit: Option<u32>,
    pub offset: u32,
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

    // lookup
    #[must_use]
    pub fn lookup<W: Into<Where>>(mut self, lookup: W) -> Self {
        self.lookup = Some(lookup.into());
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
