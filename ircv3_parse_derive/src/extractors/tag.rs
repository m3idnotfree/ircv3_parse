use quote::quote;
use syn::Result;
use syn::{Field, Ident, LitStr};

use crate::ser::SerializationBuilder;
use crate::TypeKind;

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
                    quote! { #field_name: #tags.ok_or(ircv3_parse::DeError::not_found_tag(#key))?.as_str() },
                ),
                String => Ok(
                    quote! { #field_name: #tags.ok_or(ircv3_parse::DeError::not_found_tag(#key))?.to_string() },
                ),
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    Ok(quote! { #field_name: #tags.map(|s| s.as_str()) })
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    Ok(quote! { #field_name: #tags.map(|s| s.to_string()) })
                }
                Option(inner) => Ok(quote! { #field_name: <#inner>::from_message(&msg).ok() }),
                _ => {
                    let ty = &field.ty;
                    Ok(quote! { #field_name: <#ty>::from_message(&msg)? })
                }
            },
            Self::Flag(key) => match TypeKind::classify(&field.ty) {
                Bool => Ok(quote! { #field_name: tags.get_flag(#key) }),
                _ => match TypeKind::classify(&field.ty) {
                    Option(inner) => Ok(quote! { #field_name: <#inner>::from_message(&msg).ok() }),
                    _ => {
                        let ty = &field.ty;
                        Ok(quote! { #field_name: <#ty>::from_message(&msg)? })
                    }
                },
            },
        }
    }

    pub fn expand_unnamed(
        &self,
        field: &Field,
        _idx: usize,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let tags = self.expand_tag_with();
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            return Ok(quote! { #with_fn(#tags) });
        }

        use TypeKind::*;

        let tags = self.expand_tag();

        match self {
            Self::Value(key) => match TypeKind::classify(&field.ty) {
                Str => {
                    Ok(quote! { #tags.ok_or(ircv3_parse::DeError::not_found_tag(#key))?.as_str() })
                }
                String => Ok(
                    quote! { #tags.ok_or(ircv3_parse::DeError::not_found_tag(#key))?.to_string() },
                ),
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    Ok(quote! { #tags.map(|s| s.as_str()) })
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    Ok(quote! { #tags.map(|s| s.to_string()) })
                }
                Option(inner) => Ok(quote! { <#inner>::from_message(&msg).ok() }),
                _ => {
                    let ty = &field.ty;
                    Ok(quote! { <#ty>::from_message(&msg)? })
                }
            },
            Self::Flag(key) => match TypeKind::classify(&field.ty) {
                Bool => Ok(quote! { tags.get_flag(#key) }),
                _ => match TypeKind::classify(&field.ty) {
                    Option(inner) => Ok(quote! { <#inner>::from_message(&msg).ok() }),
                    _ => {
                        let ty = &field.ty;
                        Ok(quote! { <#ty>::from_message(&msg)? })
                    }
                },
            },
        }
    }

    pub fn expand_struct_unit(
        &self,
        struct_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        if let Some(with_fn) = with {
            let tags = self.expand_tag_with();
            let with_fn = Ident::new(&with_fn.value(), with_fn.span());
            return Ok(quote! { #with_fn(#tags) });
        }

        let tags = self.expand_tag();
        match self {
            Self::Value(key) => Ok(quote! {
                match #tags {
                    Some(v) => Ok(#struct_name),
                    None => Err(ircv3_parse::DeError::not_found_tag(#key))
                }
            }),
            Self::Flag(key) => Ok(quote! {
                if #tags {
                    Ok(#struct_name)
                } else {
                    Err(ircv3_parse::DeError::not_found_tag(#key))
                }
            }),
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
