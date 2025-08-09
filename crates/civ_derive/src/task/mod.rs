extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn derive_task_box(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    TokenStream::from(quote! {
        impl common::geo::Geo for #name {
            fn geo(&self) -> &GeoContext {
                &self.geo
            }

            fn geo_mut(&mut self) -> &mut GeoContext {
                &mut self.geo
            }
        }
    })
}
