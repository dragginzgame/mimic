use crate::core::traits::{CreateView, FilterView, UpdateView, View as OtherView};

// View
pub type View<T> = <T as OtherView>::ViewType;

// Create
pub type Create<T> = <T as CreateView>::CreateViewType;

// Update
pub type Update<T> = <T as UpdateView>::UpdateViewType;

// Filter
pub type Filter<T> = <T as FilterView>::FilterViewType;
