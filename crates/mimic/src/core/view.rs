use crate::core::traits::{EditView, FilterView, TypeView};

// View
pub type View<T> = <T as TypeView>::View;

// Edit
pub type Edit<T> = <T as EditView>::View;

// Filter
pub type Filter<T> = <T as FilterView>::View;
