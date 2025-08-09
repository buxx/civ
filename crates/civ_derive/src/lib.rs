use proc_macro::TokenStream;

#[cfg(feature = "geo")]
mod geo;
// mod task;

#[cfg(feature = "geo")]
#[proc_macro_derive(Geo)]
pub fn derive_geo(item: TokenStream) -> TokenStream {
    geo::derive_geo(item)
}
