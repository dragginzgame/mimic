use crate::db::primitives::filter::{FilterDsl, FilterExpr, FilterKind, IntoScopedFilterExpr};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// TextFilterKind
///

pub struct TextFilterKind;

impl FilterKind for TextFilterKind {
    type Payload = TextFilter;
}

///
/// TextClause
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextClause {
    pub mode: TextMatchMode,
    pub values: Vec<String>,
}

///
/// TextMatchMode
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub enum TextMatchMode {
    #[serde(rename = "cs")]
    #[default]
    CaseSensitive,
    #[serde(rename = "ci")]
    CaseInsensitive,
}

///
/// TextFilterOp
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub enum TextFilterOp {
    Equal,
    NotEqual,
    Contains,
    StartsWith,
    EndsWith,
}

///
/// TextFilter
///

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextFilter {
    pub equal: Option<TextClause>,
    pub not_equal: Option<TextClause>,
    pub contains: Option<TextClause>,
    pub starts_with: Option<TextClause>,
    pub ends_with: Option<TextClause>,
    pub is_empty: Option<bool>,
}

impl TextClause {
    #[must_use]
    pub const fn new(mode: TextMatchMode) -> Self {
        Self {
            mode,
            values: Vec::new(),
        }
    }

    #[must_use]
    pub fn push(mut self, value: impl Into<String>) -> Self {
        self.values.push(value.into());
        self
    }
}

impl TextFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn is_empty(mut self, val: bool) -> Self {
        self.is_empty = Some(val);
        self
    }

    // equality

    #[must_use]
    pub fn equal(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.equal, TextMatchMode::CaseSensitive, value);
        self
    }

    #[must_use]
    pub fn equal_ci(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.equal, TextMatchMode::CaseInsensitive, value);
        self
    }

    #[must_use]
    pub fn not_equal(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.not_equal, TextMatchMode::CaseSensitive, value);
        self
    }

    #[must_use]
    pub fn not_equal_ci(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.not_equal, TextMatchMode::CaseInsensitive, value);
        self
    }

    // contains

    #[must_use]
    pub fn contains(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.contains, TextMatchMode::CaseSensitive, value);
        self
    }

    #[must_use]
    pub fn contains_ci(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.contains, TextMatchMode::CaseInsensitive, value);
        self
    }

    // starts_with

    #[must_use]
    pub fn starts_with(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.starts_with, TextMatchMode::CaseSensitive, value);
        self
    }

    #[must_use]
    pub fn starts_with_ci(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.starts_with, TextMatchMode::CaseInsensitive, value);
        self
    }

    // ends_with

    #[must_use]
    pub fn ends_with(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.ends_with, TextMatchMode::CaseSensitive, value);
        self
    }

    #[must_use]
    pub fn ends_with_ci(mut self, value: impl Into<String>) -> Self {
        Self::push_value(&mut self.ends_with, TextMatchMode::CaseInsensitive, value);
        self
    }

    // internal helper
    fn push_value(slot: &mut Option<TextClause>, mode: TextMatchMode, value: impl Into<String>) {
        let clause = slot.get_or_insert_with(|| TextClause::new(mode));
        // if mode differs from existing, you can decide to overwrite or ignore;
        // for now we just keep the existing mode.
        clause.values.push(value.into());
    }
}

impl IntoScopedFilterExpr for TextFilter {
    fn into_scoped(self, field: &str) -> FilterExpr {
        let dsl = FilterDsl;
        let mut and_exprs = Vec::new();

        if let Some(clause) = self.equal
            && let Some(e) = clause_exprs(dsl, field, clause, TextFilterOp::Equal)
        {
            and_exprs.push(e);
        }
        if let Some(clause) = self.not_equal
            && let Some(e) = clause_exprs(dsl, field, clause, TextFilterOp::NotEqual)
        {
            and_exprs.push(e);
        }
        if let Some(clause) = self.contains
            && let Some(e) = clause_exprs(dsl, field, clause, TextFilterOp::Contains)
        {
            and_exprs.push(e);
        }
        if let Some(clause) = self.starts_with
            && let Some(e) = clause_exprs(dsl, field, clause, TextFilterOp::StartsWith)
        {
            and_exprs.push(e);
        }
        if let Some(clause) = self.ends_with
            && let Some(e) = clause_exprs(dsl, field, clause, TextFilterOp::EndsWith)
        {
            and_exprs.push(e);
        }

        if let Some(is_empty) = self.is_empty {
            and_exprs.push(if is_empty {
                dsl.is_empty(field)
            } else {
                dsl.is_not_empty(field)
            });
        }

        FilterDsl::all(and_exprs)
    }
}

// helper: build OR over values for one clause/op
fn clause_exprs(
    dsl: FilterDsl,
    field: &str,
    clause: TextClause,
    op: TextFilterOp,
) -> Option<FilterExpr> {
    let TextClause { mode, values } = clause;
    if values.is_empty() {
        return None;
    }

    let ci = matches!(mode, TextMatchMode::CaseInsensitive);
    let mut or_exprs = Vec::new();

    for v in values {
        let expr = match op {
            TextFilterOp::Equal => {
                if ci {
                    dsl.eq_ci(field, v)
                } else {
                    dsl.eq(field, v)
                }
            }
            TextFilterOp::NotEqual => {
                if ci {
                    dsl.ne_ci(field, v)
                } else {
                    dsl.ne(field, v)
                }
            }
            TextFilterOp::Contains => {
                if ci {
                    dsl.contains_ci(field, v)
                } else {
                    dsl.contains(field, v)
                }
            }
            TextFilterOp::StartsWith => {
                if ci {
                    dsl.starts_with_ci(field, v)
                } else {
                    dsl.starts_with(field, v)
                }
            }
            TextFilterOp::EndsWith => {
                if ci {
                    dsl.ends_with_ci(field, v)
                } else {
                    dsl.ends_with(field, v)
                }
            }
        };

        or_exprs.push(expr);
    }

    Some(FilterDsl::any(or_exprs))
}
