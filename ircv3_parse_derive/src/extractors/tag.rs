use quote::{quote, ToTokens};
use syn::{Error, Result};
use syn::{Field, Ident, LitStr};

use crate::ser::SerializationBuilder;
use crate::TAG;
use crate::{error_msg, TypeKind};

pub enum Tag {
    Value(LitStr),
    Flag(LitStr),
}

impl Tag {
    pub fn expand(
        &self,
        field: &Field,
        field_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let tags = self.expand_tag_with();
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            return Ok(quote! { #field_name: #with_fn(#tags) });
        }

        use TypeKind::*;

        let tags = self.expand_tag();
        match self {
            Self::Value(key) => match TypeKind::classify(&field.ty) {
                Str => Ok(
                    quote! { #field_name: #tags.ok_or(ircv3_parse::DeError::missing_tag(stringify!(#field_name), #key))?.as_str() },
                ),
                String => Ok(
                    quote! { #field_name: #tags.ok_or(ircv3_parse::DeError::missing_tag(stringify!(#field_name), #key))?.to_string() },
                ),
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    Ok(quote! { #field_name: #tags.map(|s| s.as_str()) })
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    Ok(quote! { #field_name: #tags.map(|s| s.to_string()) })
                }
                _ => Err(Error::new_spanned(
                    field,
                    error_msg::unsupported_type(TAG, field_name, field.ty.to_token_stream()),
                )),
            },
            Self::Flag(key) => match TypeKind::classify(&field.ty) {
                Bool => Ok(quote! { #field_name: tags.get_flag(#key) }),
                _ => Err(Error::new_spanned(
                    &field.ty,
                    "tag_flag field must be of type bool",
                )),
            },
        }
    }

    pub fn expand_de(
        &self,
        field: &Field,
        field_name: &Ident,
        builder: &mut SerializationBuilder,
    ) -> Result<()> {
        use TypeKind::*;

        match self {
            Self::Value(key) => match TypeKind::classify(&field.ty) {
                Str => {
                    builder.tag(quote! { tags.tag(&#key, Some(self.#field_name))?; });
                    Ok(())
                }
                String => {
                    builder.tag(quote! { tags.tag(&#key, Some(self.#field_name.as_ref()))?; });
                    Ok(())
                }
                Option(inner) => match TypeKind::classify(inner) {
                    Str => {
                        builder.tag(quote! { tags.tag(&#key, self.#field_name)?; });
                        Ok(())
                    }
                    String => {
                        builder.tag(quote! { tags.tag(&#key, self.#field_name.as_deref())?; });
                        Ok(())
                    }
                    _ => {
                        builder.custom_tag(quote! {
                            if let Some(field) = self.#field_name {
                                field.to_message(serialize)?;
                            }
                        });
                        Ok(())
                    }
                },
                _ => {
                    builder.custom_tag(quote! {
                            self.#field_name.to_message(serialize)?;
                    });
                    Ok(())
                }
            },
            Self::Flag(key) => match TypeKind::classify(&field.ty) {
                Bool => {
                    builder.tag(quote! {
                        if self.#field_name {
                            tags.flag(&#key)?;
                        }
                    });
                    Ok(())
                }
                _ => {
                    builder.custom_tag(quote! {
                        self.#field_name.to_message(serialize)?;
                    });
                    Ok(())
                }
            },
        }
    }

    fn expand_tag(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Value(key) => quote! { tags.get(#key) },
            Self::Flag(key) => quote! { tags.get_flag(#key) },
        }
    }

    fn expand_tag_with(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Value(key) => quote! { tags.get(#key).map(|s| s.as_str()) },
            Self::Flag(key) => quote! { tags.get_flag(#key) },
        }
    }
}
