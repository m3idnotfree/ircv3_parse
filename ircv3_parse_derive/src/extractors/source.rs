use quote::{quote, ToTokens};
use syn::{Error, Result};
use syn::{Field, Ident, LitStr};

use crate::{error_msg, TypeKind};

pub enum SourceField {
    Name,
    User,
    Host,
}

impl SourceField {
    pub fn parse(s: &LitStr) -> Result<Self> {
        match s.value().as_ref() {
            "name" => Ok(Self::Name),
            "user" => Ok(Self::User),
            "host" => Ok(Self::Host),
            _ => Err(Error::new_spanned(
                s,
                "invalid source field (valid options: `name`, `user`, or `host`)",
            )),
        }
    }

    pub fn expand(
        &self,
        field: &Field,
        field_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let source = self.expand_source();
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            return Ok(quote! { #field_name: #with_fn(#source) });
        }

        use TypeKind::*;

        let source = self.expand_source();
        match self {
            Self::Name => match TypeKind::classify(&field.ty) {
                Str => Ok(quote! { #field_name: #source }),
                String => Ok(quote! { #field_name: #source.to_string() }),
                _ => Err(Error::new_spanned(
                    field,
                    "source `name` field cannot be Option<...> (use &str or String instead)",
                )),
            },
            Self::User | Self::Host => {
                let source_field_str = self.as_str();

                match TypeKind::classify(&field.ty) {
                    Str => Ok(
                        quote! { #field_name: #source.ok_or(ircv3_parse::ExtractError::missing_source_field(stringify!(#field_name), #source_field_str))? },
                    ),
                    String => Ok(
                        quote! { #field_name: #source.ok_or(ircv3_parse::ExtractError::missing_source_field(stringify!(#field_name), #source_field_str))?.to_string() },
                    ),
                    Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                        Ok(quote! { #field_name: #source})
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), String) => {
                        Ok(quote! { #field_name: #source.map(|s| s.to_string()) })
                    }
                    _ => {
                        let component = format!("source {source_field_str}");
                        Err(Error::new_spanned(
                            field,
                            error_msg::unsupported_type(
                                &component,
                                field_name,
                                field.ty.to_token_stream(),
                            ),
                        ))
                    }
                }
            }
        }
    }

    fn expand_source(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Name => {
                quote! { source.name }
            }
            Self::User => {
                quote! { source.user }
            }
            Self::Host => {
                quote! { source.host }
            }
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Name => "name",
            Self::User => "user",
            Self::Host => "host",
        }
    }
}
