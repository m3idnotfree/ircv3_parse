use quote::{quote, ToTokens};
use syn::{Error, Result};
use syn::{Ident, LitStr};

use crate::type_check;
use crate::COMMAND;
use crate::{error_msg, TypeKind};

#[derive(Clone)]
pub struct CommandField(pub Option<LitStr>);

impl CommandField {
    pub fn new(value: Option<LitStr>) -> Self {
        Self(value)
    }

    pub fn expand(
        field: &syn::Field,
        field_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            return Ok(quote! { #field_name: #with_fn(command.as_str()) });
        }

        match TypeKind::classify(&field.ty) {
            TypeKind::Str => Ok(quote! { #field_name: command.as_str() }),
            TypeKind::String => Ok(quote! { #field_name: command.to_string() }),
            _ => {
                if type_check::is_type(&field.ty, "Commands") {
                    Ok(quote! { #field_name: command })
                } else {
                    Err(Error::new_spanned(
                        field,
                        error_msg::unsupported_type(
                            COMMAND,
                            field_name,
                            field.ty.to_token_stream(),
                        ),
                    ))
                }
            }
        }
    }
}
