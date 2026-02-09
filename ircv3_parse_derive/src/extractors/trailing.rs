use quote::quote;
use syn::Result;
use syn::{Ident, LitStr};

use crate::ser::SerializationBuilder;
use crate::TypeKind;

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

        use TypeKind::*;

        match TypeKind::classify(&field.ty) {
            Str => Ok(quote! { #field_name: params.trailing.as_str() }),
            String => Ok(quote! { #field_name: params.trailing.to_string() }),
            Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                Ok(quote! { #field_name: params.trailing.raw().filter(|s| !s.is_empty()) })
            }
            Option(inner) if matches!(TypeKind::classify(inner), String) => Ok(
                quote! { #field_name: params.trailing.raw().filter(|s| !s.is_empty()).map(|s| s.to_string()) },
            ),
            Option(inner) => Ok(quote! { #field_name: <#inner>::from_message(&msg).ok() }),
            _ => {
                let ty = &field.ty;
                Ok(quote! { #field_name: <#ty>::from_message(&msg)? })
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
            return Ok(quote! { #with_fn(params.trailing.as_str()) });
        }

        use TypeKind::*;

        match TypeKind::classify(&field.ty) {
            Str => Ok(quote! { params.trailing.as_str() }),
            String => Ok(quote! { params.trailing.to_string() }),
            Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                Ok(quote! { params.trailing.raw().filter(|s| !s.is_empty()) })
            }
            Option(inner) if matches!(TypeKind::classify(inner), String) => Ok(
                quote! { params.trailing.raw().filter(|s| !s.is_empty()).map(|s| s.to_string()) },
            ),
            Option(inner) => Ok(quote! { <#inner>::from_message(&msg).ok() }),
            _ => {
                let ty = &field.ty;
                Ok(quote! { <#ty>::from_message(&msg)? })
            }
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
