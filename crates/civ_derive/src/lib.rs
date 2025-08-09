use proc_macro::TokenStream;

#[cfg(feature = "geo")]
mod geo;
#[cfg(feature = "task")]
mod task;

#[cfg(feature = "geo")]
#[proc_macro_derive(Geo)]
pub fn derive_geo(item: TokenStream) -> TokenStream {
    geo::derive_geo(item)
}

#[cfg(feature = "task")]
#[proc_macro_derive(TaskBox)]
pub fn derive_task_box(item: TokenStream) -> TokenStream {
    task::derive_task_box(item)
}
