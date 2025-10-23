use crate::core::traits::{CreateView, TypeView, UpdateView};

// View
pub type View<T> = <T as TypeView>::View;

// Create
pub type Create<T> = <T as CreateView>::View;

// Update
pub type Update<T> = <T as UpdateView>::View;
