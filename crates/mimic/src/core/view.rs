use crate::core::traits::{CreateView, EditView, FilterView, View as OtherView};

// View
pub type View<T> = <T as OtherView>::ViewType;

// Create
pub type Create<T> = <T as CreateView>::CreateType;

// Edit
pub type Edit<T> = <T as EditView>::EditType;

// Filter
pub type Filter<T> = <T as FilterView>::FilterType;
