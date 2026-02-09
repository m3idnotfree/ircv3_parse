use quote::quote;
use syn::{Error, Result};
use syn::{Ident, LitStr};

use crate::ser::SerializationBuilder;
use crate::type_check;
use crate::TypeKind;

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

        use TypeKind::*;

        match TypeKind::classify(&field.ty) {
            Str => Ok(quote! { #field_name: command.as_str() }),
            String => Ok(quote! { #field_name: command.to_string() }),
            Option(_) => Err(Error::new_spanned(
                field,
                "command field cannot be Option<...> (use &str or String instead)",
            )),
            _ => {
                if type_check::is_type(&field.ty, "Commands") {
                    Ok(quote! { #field_name: command })
                } else {
                    let ty = &field.ty;
                    Ok(quote! { #field_name: <#ty>::from_message(&msg)? })
                }
            }
        }
    }

    pub fn expand_unnamed(
        field: &syn::Field,
        _idx: usize,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            return Ok(quote! { #with_fn(command.as_str()) });
        }

        use TypeKind::*;

        match TypeKind::classify(&field.ty) {
            Str => Ok(quote! { command.as_str() }),
            String => Ok(quote! { command.to_string() }),
            Option(_) => Err(Error::new_spanned(
                field,
                "command field cannot be Option<...> (use &str or String instead)",
            )),
            _ => {
                if type_check::is_type(&field.ty, "Commands") {
                    Ok(quote! { command })
                } else {
                    let ty = &field.ty;
                    Ok(quote! { <#ty>::from_message(&msg)? })
                }
            }
        }
    }

    pub fn expand_de(
        self,
        _field: &syn::Field,
        _field_name: &Ident,
        builder: &mut SerializationBuilder,
    ) -> Result<()> {
        builder.field_command(self.0);
        Ok(())
    }
}
