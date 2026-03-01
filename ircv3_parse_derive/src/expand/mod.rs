mod enums;
mod field;
mod field_kind;
mod unit;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Result};

use crate::ast::{Input, Struct};

pub fn derive_from_message(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node, "FromMessage")?;
    input.validate()?;

    match input {
        Input::Struct(input) => Ok(input.expand_de()),
        Input::Enum(input) => Ok(input.expand_de()),
    }
}

pub fn derive_to_message(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node, "ToMessage")?;

    input.validate()?;
    input.validate_ser()?;

    match input {
        Input::Struct(input) => Ok(input.expand_ser()),
        Input::Enum(input) => Err(Error::new_spanned(
            input.ident,
            "ToMessage only supports structs",
        )),
    }
}

impl<'a> Struct<'a> {
    pub fn expand_de(&self) -> TokenStream {
        match self {
            Struct::Unit(input) => input.expand_de(),
            Struct::Fields(input) => input.expand_de(),
        }
    }

    pub fn expand_ser(&self) -> TokenStream {
        match self {
            Struct::Unit(input) => input.expand_ser(),
            Struct::Fields(input) => input.expand_ser(),
        }
    }
}
