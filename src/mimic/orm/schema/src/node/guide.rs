use crate::{
    node::{ArgNumber, ValidateNode, VisitableNode},
    visit::Visitor,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use types::ErrorVec;

///
/// Guide
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Guide {
    pub entries: Vec<GuideEntry>,
}

impl ValidateNode for Guide {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // check actor
        if self.entries.is_empty() {
            errs.add("guide has no entries");
        }

        // check for duplicate names
        let dupe_names: Vec<_> = self
            .entries
            .iter()
            .filter_map(|e| e.name.as_ref())
            .duplicates()
            .collect();

        if let Some(dupe) = dupe_names.first() {
            errs.add(format!("duplicate guide entry name: {dupe}"));
        }

        errs.result()
    }
}

impl VisitableNode for Guide {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in &self.entries {
            node.accept(v);
        }
    }
}

///
/// GuideEntry
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuideEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub value: ArgNumber,
}

impl ValidateNode for GuideEntry {}

impl VisitableNode for GuideEntry {}
