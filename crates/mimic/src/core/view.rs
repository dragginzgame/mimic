use crate::core::traits::{EditView, FilterView, View as OtherView};

// View
pub type View<T> = <T as OtherView>::ViewType;

// Edit
pub type Edit<T> = <T as EditView>::EditType;

// Filter
pub type Filter<T> = <T as FilterView>::FilterType;
