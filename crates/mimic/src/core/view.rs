use crate::core::traits::{CreateView, FilterView, SortView, TypeView, UpdateView};

// View
pub type View<T> = <T as TypeView>::View;

// Create
pub type Create<T> = <T as CreateView>::View;

// Update
pub type Update<T> = <T as UpdateView>::View;

// Filter
pub type Filter<T> = <T as FilterView>::View;

// Sort
pub type Sort<T> = <T as SortView>::View;
