use quote::{quote, ToTokens};
use syn::{Error, Result};
use syn::{Field, Ident, LitInt, LitStr};

use crate::PARAM;
use crate::{error_msg, TypeKind};

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
            _ => Err(Error::new_spanned(
                field,
                error_msg::unsupported_type(PARAM, field_name, field.ty.to_token_stream()),
            )),
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
