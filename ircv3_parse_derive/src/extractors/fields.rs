use syn::{punctuated::Punctuated, token::Comma};
use syn::{Data, Field, Fields};
use syn::{DeriveInput, Error, Result};

pub fn extract_named_fields(input: &DeriveInput) -> Result<&Punctuated<Field, Comma>> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Ok(&fields.named),
            _ => Err(Error::new_spanned(
                &input.ident,
                "FromMessage only supports structs with named fields",
            )),
        },
        _ => Err(Error::new_spanned(
            &input.ident,
            "FromMessage only supports structs",
        )),
    }
}
