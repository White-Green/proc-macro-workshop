use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {};
    TokenStream::from(expanded)
}
