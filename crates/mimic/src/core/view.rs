use crate::core::traits::{CreateView, FilterView, UpdateView, View as OtherView};

// View
pub type View<T> = <T as OtherView>::ViewType;

// Create
pub type Create<T> = <T as CreateView>::CreateType;

// Update
pub type Update<T> = <T as UpdateView>::UpdateType;

// Filter
pub type Filter<T> = <T as FilterView>::FilterType;
