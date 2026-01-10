use quote::{quote, ToTokens};
use syn::{Error, Result};
use syn::{Ident, LitStr};

use crate::ser::SerializationBuilder;
use crate::TRAILING;
use crate::{error_msg, TypeKind};

pub struct TrailingField;

impl TrailingField {
    pub fn expand(
        field: &syn::Field,
        field_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            return Ok(quote! { #field_name: #with_fn(params.trailing.as_str()) });
        }

        match TypeKind::classify(&field.ty) {
            TypeKind::Str => Ok(quote! { #field_name: params.trailing.as_str() }),
            TypeKind::String => Ok(quote! { #field_name: params.trailing.to_string() }),
            _ => Err(Error::new_spanned(
                field,
                error_msg::unsupported_type(TRAILING, field_name, field.ty.to_token_stream()),
            )),
        }
    }

    pub fn expand_de(
        field: &syn::Field,
        field_name: &Ident,
        builder: &mut SerializationBuilder,
    ) -> Result<()> {
        use TypeKind::*;

        match TypeKind::classify(&field.ty) {
            Str => {
                builder.set_trailing(quote! { serialize.trailing(self.#field_name)?; });
                Ok(())
            }
            String => {
                builder.set_trailing(quote! { serialize.trailing(self.#field_name.as_ref())?; });
                Ok(())
            }
            Option(inner) => match TypeKind::classify(inner) {
                Str => {
                    builder.set_trailing(quote! {
                        if let Some(t) = self.#field_name{
                            serialize.trailing(t)?;
                        }
                    });
                    Ok(())
                }
                String => {
                    builder.set_trailing(quote! {
                        if let Some(t) = self.#field_name{
                            serialize.trailing(t.as_ref())?;
                        }
                    });
                    Ok(())
                }
                _ => {
                    builder.custom_trailing(quote! {
                        if let Some(t) = self.#field_name{
                            t.to_message(serialize)?;
                        }
                    });
                    Ok(())
                }
            },
            _ => {
                builder.custom_trailing(quote! { self.#field_name.to_message(serialize)?; });
                Ok(())
            }
        }
    }
}
