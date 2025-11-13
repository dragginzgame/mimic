use std::iter::IntoIterator;

///
/// View
/// Recursive for all field/value nodes
///

pub trait View {
    type ViewType: Default;

    fn to_view(&self) -> Self::ViewType;
    fn from_view(view: Self::ViewType) -> Self;
}

impl<T: View> View for Box<T> {
    type ViewType = Box<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        Box::new((**self).to_view())
    }

    fn from_view(view: Self::ViewType) -> Self {
        Self::new(T::from_view(*view))
    }
}

impl<T: View> View for Option<T> {
    type ViewType = Option<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        self.as_ref().map(View::to_view)
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.map(T::from_view)
    }
}

impl<T: View> View for Vec<T> {
    type ViewType = Vec<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        self.iter().map(View::to_view).collect()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.into_iter().map(T::from_view).collect()
    }
}

impl View for String {
    type ViewType = Self;

    fn to_view(&self) -> Self::ViewType {
        self.clone()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view
    }
}

// impl_view
#[macro_export]
macro_rules! impl_view {
    ($($type:ty),*) => {
        $(
            impl View for $type {
                type ViewType = Self;

                fn to_view(&self) -> Self::ViewType {
                    *self
                }

                fn from_view(view: Self::ViewType) -> Self {
                    view
                }
            }
        )*
    };
}

impl_view!(bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

///
/// CreateView
///

pub trait CreateView {
    type CreateViewType: Default;
}

///
/// UpdateView
///

pub trait UpdateView {
    type UpdateViewType: Default;

    /// Merge `view` into `self`, skipping `None` fields.
    fn merge(&mut self, _: Self::UpdateViewType) {}
}

///
/// FilterView
///

pub trait FilterView {
    type FilterViewType: Default;
}
