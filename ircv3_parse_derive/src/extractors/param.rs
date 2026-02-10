use quote::quote;
use syn::Result;
use syn::{Field, Ident, LitInt, LitStr};

use crate::ser::SerializationBuilder;
use crate::TypeKind;

pub struct ParamField(usize);

impl ParamField {
    pub fn new(lit: &LitInt) -> Result<Self> {
        let idx: usize = lit.base10_parse()?;
        Ok(Self(idx))
    }

    pub fn expand(
        &self,
        field: &Field,
        field_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            let params = self.expand_param();
            return Ok(quote! { #field_name: #with_fn(#params) });
        }

        use TypeKind::*;

        let idx = self.0;
        let params = self.expand_param();

        match TypeKind::classify(&field.ty) {
            Str => Ok(
                quote! { #field_name: #params.ok_or(ircv3_parse::DeError::missing_param_field(stringify!(#field_name), #idx))? },
            ),
            String => Ok(
                quote! { #field_name: #params.ok_or(ircv3_parse::DeError::missing_param_field(stringify!(#field_name), #idx))?.to_string() },
            ),
            Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                Ok(quote! { #field_name: #params })
            }
            Option(inner) if matches!(TypeKind::classify(inner), String) => {
                Ok(quote! { #field_name: #params.map(|s| s.to_string()) })
            }
            Option(inner) => Ok(quote! { #field_name: <#inner>::from_message(&msg).ok() }),
            _ => {
                let ty = &field.ty;
                Ok(quote! { #field_name: <#ty>::from_message(&msg)? })
            }
        }
    }

    pub fn expand_unnamed(
        &self,
        field: &Field,
        _field_idx: usize,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            let params = self.expand_param();
            return Ok(quote! { #with_fn(#params) });
        }

        use TypeKind::*;

        let idx = self.0;
        let params = self.expand_param();

        match TypeKind::classify(&field.ty) {
            Str => Ok(
                quote! { #params.ok_or(ircv3_parse::DeError::missing_param_field(stringify!(#idx), #idx))? },
            ),
            String => Ok(
                quote! { #params.ok_or(ircv3_parse::DeError::missing_param_field(stringify!(#idx), #idx))?.to_string() },
            ),
            Option(inner) if matches!(TypeKind::classify(inner), Str) => Ok(quote! { #params }),
            Option(inner) if matches!(TypeKind::classify(inner), String) => {
                Ok(quote! { #params.map(|s| s.to_string()) })
            }
            Option(inner) => Ok(quote! { <#inner>::from_message(&msg).ok() }),
            _ => {
                let ty = &field.ty;
                Ok(quote! { <#ty>::from_message(&msg)? })
            }
        }
    }

    pub fn expand_struct_unit(
        &self,
        struct_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            let params = self.expand_param();
            return Ok(quote! { #with_fn(#params) });
        }

        let idx = self.0;
        let params = self.expand_param();
        Ok(quote! {
            if #params.is_some() {
                Ok(#struct_name)
            } else {
                Err(ircv3_parse::DeError::missing_param_field(stringify!(#struct_name), #idx))
            }
        })
    }

    pub fn expand_de(
        &self,
        field: &Field,
        field_name: &Ident,
        builder: &mut SerializationBuilder,
    ) -> Result<()> {
        use TypeKind::*;

        match TypeKind::classify(&field.ty) {
            Str => {
                builder.params_push(quote! { params.push(self.#field_name)?; });
                Ok(())
            }
            String => {
                builder.params_push(quote! { params.push(self.#field_name.as_ref())?; });
                Ok(())
            }
            Option(inner) => match TypeKind::classify(inner) {
                Str => {
                    builder.params_push(quote! {
                        if let Some(p) = self.#field_name {
                            params.push(p)?;
                        }
                    });
                    Ok(())
                }
                String => {
                    builder.params_push(quote! {
                        if let Some(p) = self.#field_name {
                            params.push(p.as_ref())?;
                        }
                    });
                    Ok(())
                }
                _ => {
                    builder.custom_params(quote! {
                        if let Some(p) = self.#field_name {
                            p.to_message(serialize)?;
                        }
                    });
                    Ok(())
                }
            },
            _ => {
                builder.custom_params(quote! {
                    self.#field_name.to_message(serialize)?;
                });
                Ok(())
            }
        }
    }

    fn expand_param(&self) -> proc_macro2::TokenStream {
        if self.0 == 0 {
            quote! { params.middles.first() }
        } else if self.0 == 1 {
            quote! { params.middles.second() }
        } else {
            let idx = self.0;
            quote! { params.middles.iter().nth(#idx) }
        }
    }
}
