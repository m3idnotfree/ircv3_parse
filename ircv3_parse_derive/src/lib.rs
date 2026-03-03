#![doc = include_str!("../README.md")]

mod ast;
mod attr;
mod component_set;
mod error_msg;
mod expand;
mod type_check;
mod valid;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(FromMessage, attributes(irc))]
pub fn derive_from_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive_from_message(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(ToMessage, attributes(irc))]
pub fn derive_to_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    expand::derive_to_message(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
