use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Type};

use crate::{
    attr::{FieldDefault, FieldKind, Source},
    type_check::{self, TypeKind},
};

impl FieldKind {
    pub fn expand_de(
        &self,
        field: &syn::Field,
        with: Option<&LitStr>,
        default: Option<&FieldDefault>,
    ) -> TokenStream {
        if let Some(with_fn) = with {
            return self.expand_with(field, with_fn);
        }

        let value = self.expand_value(&field.ty, default);
        if let Some(field_name) = &field.ident {
            quote! { #field_name: #value }
        } else {
            value
        }
    }

    fn expand_with(&self, field: &syn::Field, with_fn: &LitStr) -> TokenStream {
        let with_ident = Ident::new(&with_fn.value(), with_fn.span());
        let input = self.with_input();
        if let Some(field_name) = &field.ident {
            quote! { #field_name: #with_ident(#input) }
        } else {
            quote! { #with_ident(#input) }
        }
    }

    pub fn expand_with_accessor(&self, ty: &Type, accessor: &TokenStream) -> TokenStream {
        match self {
            Self::Tag(key) => expand_tag_ser(key, ty, accessor),
            Self::TagFlag(key) => expand_tag_flag_ser(key, ty, accessor),
            Self::Source(inner) => expand_source_ser(inner, ty, accessor),
            Self::Param(_) => expand_param_ser(ty, accessor),
            Self::Params => expand_params_ser(ty, accessor),
            Self::Trailing => expand_trailing_ser(ty, accessor),
            Self::Command => expand_command_ser(ty, accessor),
        }
    }

    fn with_input(&self) -> TokenStream {
        match self {
            Self::Tag(key) => quote! { tags.get(#key).map(|s| s.as_str()) },
            Self::TagFlag(key) => quote! { tags.get_flag(#key) },
            Self::Source(inner) => match inner {
                Source::Name => quote! { source.name },
                Source::User => quote! { source.user },
                Source::Host => quote! { source.host },
            },
            Self::Param(idx) => quote! { params.middles.iter().nth(#idx) },
            Self::Params => quote! { params.middles.to_vec() },
            Self::Trailing => quote! { params.trailing.raw() },
            Self::Command => quote! { command.as_str() },
        }
    }

    fn expand_value(&self, ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
        match self {
            Self::Tag(key) => expand_tag_value(key, ty, default),
            Self::TagFlag(key) => expand_tag_flag_value(key, ty, default),
            Self::Source(inner) => expand_source_value(inner, ty, default),
            Self::Param(idx) => expand_param_value(idx, ty, default),
            Self::Params => expand_params_value(ty, default),
            Self::Trailing => expand_trailing_value(ty, default),
            Self::Command => expand_command_value(ty, default),
        }
    }

    pub fn expand_unit_de(&self, value: &LitStr) -> TokenStream {
        match self {
            Self::Tag(key) => quote! {
                match tags.get(#key) {
                    None => return Err(ircv3_parse::DeError::not_found_tag(#key)),
                    Some(v) if v.as_str() != #value => {
                        return Err(ircv3_parse::DeError::not_found_with_context(
                            "tag",
                            #key,
                            format!("expected `{}`, got `{}`", #value, v.as_str())
                        ))
                    }
                    _ => {}
                }
            },
            Self::TagFlag(key) => quote! {
                if !tags.get_flag(#key) {
                    return Err(ircv3_parse::DeError::not_found_tag(#key));
                }
            },
            Self::Source(inner) => match inner {
                Source::Name => {
                    let name = inner.name();
                    quote! {
                        if source.name != #value {
                            return Err(ircv3_parse::DeError::not_found_with_context(
                                "source",
                                #name,
                                format!("expected `{}`, got `{}`", #value, source.name)
                            ))
                        }
                    }
                }
                Source::User => {
                    let name = inner.name();
                    quote! {
                        match source.user {
                            None => return Err(ircv3_parse::DeError::not_found_source(#name)),
                            Some(v) if v != #value => {
                                return Err(ircv3_parse::DeError::not_found_with_context(
                                    "source",
                                    #name,
                                    format!("expected `{}`, got `{}`", #value, v)
                                ))
                            }
                            _ => {}
                        }
                    }
                }
                Source::Host => {
                    let name = inner.name();
                    quote! {
                        match source.host {
                            None => return Err(ircv3_parse::DeError::not_found_source(#name)),
                            Some(v) if v != #value => {
                                return Err(ircv3_parse::DeError::not_found_with_context(
                                    "source",
                                    #name,
                                    format!("expected `{}`, got `{}`", #value, v)
                                ))
                            }
                            _ => {}
                        }
                    }
                }
            },
            Self::Param(idx) => quote! {
                match params.middles.iter().nth(#idx) {
                    None => return Err(ircv3_parse::DeError::not_found_param(#idx)),
                    Some(v) if v != #value => {
                        return Err(ircv3_parse::DeError::not_found_with_context(
                            "param",
                            #idx.to_string(),
                            format!("expected `{}`, got `{}`", #value, v)
                        ))
                    }
                    _ => {}
                }
            },
            Self::Trailing => quote! {
                match params.trailing.raw() {
                    None => return Err(ircv3_parse::DeError::not_found_trailing()),
                    Some(v) if v != #value => {
                        return Err(ircv3_parse::DeError::not_found_with_context(
                            "trailing",
                            "",
                            format!("expected `{}`, got `{}`", #value, v)
                        ))
                    }
                    _ => {}
                }
            },
            Self::Command => unreachable!("Command is handled separately in UnitStructAttrs"),
            Self::Params => unreachable!("Params is rejected during validation"),
        }
    }

    pub fn expand_unit_ser(&self, value: &LitStr) -> TokenStream {
        match self {
            Self::Tag(key) => quote! {
                serialize.tags().insert_tag(#key, Some(#value))?;
            },
            Self::TagFlag(key) => quote! {
                serialize.tags().insert_flag(#key)?;
            },
            Self::Source(inner) => match inner {
                Source::Name => quote! { serialize.source().set_name(#value)?; },
                Source::User => quote! { serialize.source().set_user(#value)?; },
                Source::Host => quote! { serialize.source().set_host(#value)?; },
            },
            Self::Param(_) => quote! {
                serialize.params().push(#value)?;
            },
            Self::Trailing => quote! {
                serialize.set_trailing(#value)?;
            },
            Self::Command => unreachable!("Command is handled separately in UnitStructAttrs"),
            Self::Params => unreachable!("Params is rejected during validation"),
        }
    }
}

fn expand_tag_value(key: &LitStr, ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => match default {
            Some(d) => {
                let fallback = expand_fallback(d);
                quote! {
                    msg.tags()
                        .and_then(|tags| tags.get(#key))
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| #fallback)
                }
            }
            None => quote! {
                tags.get(#key)
                    .ok_or_else(|| ircv3_parse::DeError::not_found_tag(#key))?
                    .as_str()
            },
        },
        String => match default {
            Some(d) => {
                let fallback = expand_fallback(d);
                quote! {
                    msg.tags()
                        .and_then(|tags| tags.get(#key))
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| #fallback)
                }
            }
            None => quote! {
                tags.get(#key)
                    .ok_or_else(|| ircv3_parse::DeError::not_found_tag(#key))?
                    .to_string()
            },
        },
        Option(inner) if type_check::is_str(inner) => {
            if default.is_some() {
                quote! { msg.tags().and_then(|tags| tags.get(#key)).map(|s| s.as_str()) }
            } else {
                quote! { tags.get(#key).map(|s| s.as_str()) }
            }
        }
        Option(inner) if type_check::is_string(inner) => {
            if default.is_some() {
                quote! { msg.tags().and_then(|tags| tags.get(#key)).map(|s| s.to_string()) }
            } else {
                quote! { tags.get(#key).map(|s| s.to_string()) }
            }
        }
        Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
        _ => expand_from_message(ty, default),
    }
}

fn expand_tag_flag_value(key: &LitStr, ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Bool => match default {
            Some(_) => quote! {
                msg.tags()
                    .map(|tags| tags.get_flag(#key))
                    .unwrap_or_default()
            },
            None => quote! { tags.get_flag(#key) },
        },
        Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
        _ => expand_from_message(ty, default),
    }
}

fn expand_source_value(source: &Source, ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    use TypeKind::*;
    match source {
        Source::Name => match TypeKind::classify(ty) {
            Str => match default {
                Some(d) => {
                    let fallback = expand_fallback(d);
                    quote! {
                        msg.source()
                            .map(|source| source.name)
                            .unwrap_or_else(|| #fallback)
                    }
                }
                None => quote! { source.name },
            },
            String => match default {
                Some(d) => {
                    let fallback = expand_fallback(d);
                    quote! {
                        msg.source()
                            .map(|source| source.name.to_string())
                            .unwrap_or_else(|| #fallback)
                    }
                }
                None => quote! { source.name.to_string() },
            },
            Option(inner) if type_check::is_str(inner) => {
                if default.is_some() {
                    quote! { msg.source().map(|source| source.name) }
                } else {
                    quote! { source.name }
                }
            }
            Option(inner) if type_check::is_string(inner) => {
                if default.is_some() {
                    quote! { msg.source().map(|source| source.name.to_string()) }
                } else {
                    quote! { source.name.to_string() }
                }
            }
            Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
            _ => expand_from_message(ty, default),
        },
        Source::User => match TypeKind::classify(ty) {
            Str => match default {
                Some(d) => {
                    let fallback = expand_fallback(d);
                    quote! {
                        msg.source()
                            .and_then(|source| source.user)
                            .unwrap_or_else(|| #fallback)
                    }
                }
                None => quote! {
                    source.user
                        .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?
                },
            },
            String => match default {
                Some(d) => {
                    let fallback = expand_fallback(d);
                    quote! {
                        msg.source()
                            .and_then(|source| source.user)
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| #fallback)
                    }
                }
                None => quote! {
                    source.user
                        .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?
                        .to_string()
                },
            },
            Option(inner) if type_check::is_str(inner) => {
                if default.is_some() {
                    quote! { msg.source().and_then(|source| source.user) }
                } else {
                    quote! { source.user }
                }
            }
            Option(inner) if type_check::is_string(inner) => {
                if default.is_some() {
                    quote! { msg.source().and_then(|source| source.user).map(|s| s.to_string()) }
                } else {
                    quote! { source.user.map(|s| s.to_string()) }
                }
            }
            Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
            _ => expand_from_message(ty, default),
        },
        Source::Host => match TypeKind::classify(ty) {
            Str => match default {
                Some(d) => {
                    let fallback = expand_fallback(d);
                    quote! {
                        msg.source()
                            .and_then(|source| source.host)
                            .unwrap_or_else(|| #fallback)
                    }
                }
                None => quote! {
                    source.host
                        .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?
                },
            },
            String => match default {
                Some(d) => {
                    let fallback = expand_fallback(d);
                    quote! {
                        msg.source()
                            .and_then(|source| source.host)
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| #fallback)
                    }
                }
                None => quote! {
                    source.host
                        .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?
                        .to_string()
                },
            },
            Option(inner) if type_check::is_str(inner) => {
                if default.is_some() {
                    quote! { msg.source().and_then(|source| source.host) }
                } else {
                    quote! { source.host }
                }
            }
            Option(inner) if type_check::is_string(inner) => {
                if default.is_some() {
                    quote! { msg.source().and_then(|source| source.host).map(|s| s.to_string()) }
                } else {
                    quote! { source.host.map(|s| s.to_string()) }
                }
            }
            Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
            _ => expand_from_message(ty, default),
        },
    }
}

fn expand_param_value(idx: &usize, ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => match default {
            Some(d) => {
                let fallback = expand_fallback(d);
                quote! {
                    params.middles.iter().nth(#idx)
                        .unwrap_or_else(|| #fallback)
                }
            }
            None => quote! {
                params.middles.iter().nth(#idx)
                    .ok_or_else(|| ircv3_parse::DeError::not_found_param(#idx))?
            },
        },
        String => match default {
            Some(d) => {
                let fallback = expand_fallback(d);
                quote! {
                    params.middles.iter().nth(#idx)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| #fallback)
                }
            }
            None => quote! {
                params.middles.iter().nth(#idx)
                    .ok_or_else(|| ircv3_parse::DeError::not_found_param(#idx))?
                    .to_string()
            },
        },
        Option(inner) if type_check::is_str(inner) => {
            quote! { params.middles.iter().nth(#idx) }
        }
        Option(inner) if type_check::is_string(inner) => {
            quote! { params.middles.iter().nth(#idx).map(|s| s.to_string()) }
        }
        Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
        _ => expand_from_message(ty, default),
    }
}

fn expand_params_value(ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Vec(inner) if type_check::is_str(inner) => {
            quote! { params.middles.to_vec() }
        }
        Vec(inner) if type_check::is_string(inner) => {
            quote! {
                params.middles.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            }
        }
        Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
        _ => expand_from_message(ty, default),
    }
}

fn expand_trailing_value(ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => match default {
            Some(d) => {
                let fallback = expand_fallback(d);
                quote! {
                    params.trailing.raw()
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| #fallback)
                }
            }
            None => quote! { params.trailing.as_str() },
        },
        String => match default {
            Some(d) => {
                let fallback = expand_fallback(d);
                quote! {
                    params.trailing.raw()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| #fallback)
                }
            }
            None => quote! { params.trailing.to_string() },
        },
        Option(inner) if type_check::is_str(inner) => quote! {
            params.trailing.raw().filter(|s| !s.is_empty())
        },
        Option(inner) if type_check::is_string(inner) => quote! {
            params.trailing.raw()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        },
        Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
        _ => expand_from_message(ty, default),
    }
}

fn expand_command_value(ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => quote! { command.as_str() },
        String => quote! { command.to_string() },
        Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
        _ => expand_from_message(ty, default),
    }
}

fn expand_tag_ser(key: &LitStr, ty: &Type, accessor: &TokenStream) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => quote! {
            serialize.tags().insert_tag(#key, Some(#accessor))?;
        },
        String => quote! {
            serialize.tags().insert_tag(#key, Some(#accessor.as_ref()))?;
        },
        Option(inner) if type_check::is_str(inner) => quote! {
            serialize.tags().insert_tag(#key, #accessor)?;
        },
        Option(inner) if type_check::is_string(inner) => quote! {
            serialize.tags().insert_tag(#key, #accessor.as_deref())?;
        },
        Option(_) => quote! {
            if let Some(value) = &#accessor {
                value.to_message(serialize)?;
            }
        },
        _ => {
            if type_check::is_primitive(ty) {
                quote! {
                    serialize.tags().insert_tag(#key, &#accessor.to_string())?;
                }
            } else {
                quote! {
                    #accessor.to_message(serialize)?;
                }
            }
        }
    }
}

fn expand_tag_flag_ser(key: &LitStr, ty: &Type, accessor: &TokenStream) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Bool => quote! {
            if #accessor {
                serialize.tags().insert_flag(#key)?;
            }
        },
        Option(_) => quote! {
            if #accessor.is_some() {
                serialize.tags().insert_flag(#key)?;
            }
        },
        _ => {
            if type_check::is_primitive(ty) {
                quote! {
                    serialize.tags().insert_flag(&#key.to_string())?;
                }
            } else {
                quote! {
                    #accessor.to_message(serialize)?;
                }
            }
        }
    }
}

fn expand_source_ser(source: &Source, ty: &Type, accessor: &TokenStream) -> TokenStream {
    use TypeKind::*;
    match source {
        Source::Name => match TypeKind::classify(ty) {
            Str => quote! { serialize.source().set_name(#accessor)?; },
            String => quote! { serialize.source().set_name(#accessor.as_ref())?; },
            Option(inner) if type_check::is_str(inner) => quote! {
                if let Some(value) = #accessor {
                    serialize.source().set_name(value)?;
                }
            },
            Option(inner) if type_check::is_string(inner) => quote! {
                if let Some(value) = &#accessor {
                    serialize.source().set_name(value.as_ref())?;
                }
            },
            Option(_) => quote! {
                if let Some(value) = &#accessor {
                    value.to_message(serialize)?;
                }
            },
            _ => {
                if type_check::is_primitive(ty) {
                    quote! { serialize.source().set_name(&#accessor.to_string())?; }
                } else {
                    quote! { #accessor.to_message(serialize)?; }
                }
            }
        },
        Source::User => match TypeKind::classify(ty) {
            Str => quote! { serialize.source().set_user(#accessor)?; },
            String => quote! { serialize.source().set_user(#accessor.as_ref())?; },
            Option(inner) if type_check::is_str(inner) => quote! {
                if let Some(value) = #accessor {
                    serialize.source().set_user(value)?;
                }
            },
            Option(inner) if type_check::is_string(inner) => quote! {
                if let Some(value) = &#accessor {
                    serialize.source().set_user(value.as_ref())?;
                }
            },
            Option(_) => quote! {
                if let Some(value) = &#accessor {
                    value.to_message(serialize)?;
                }
            },
            _ => {
                if type_check::is_primitive(ty) {
                    quote! { serialize.source().set_user(&#accessor.to_string())?; }
                } else {
                    quote! { #accessor.to_message(serialize)?; }
                }
            }
        },
        Source::Host => match TypeKind::classify(ty) {
            Str => quote! { serialize.source().set_host(#accessor)?; },
            String => quote! { serialize.source().set_host(#accessor.as_ref())?; },
            Option(inner) if type_check::is_str(inner) => quote! {
                if let Some(value) = #accessor {
                    serialize.source().set_host(value)?;
                }
            },
            Option(inner) if type_check::is_string(inner) => quote! {
                if let Some(value) = &#accessor {
                    serialize.source().set_host(value.as_ref())?;
                }
            },
            Option(_) => quote! {
                if let Some(value) = &#accessor {
                    value.to_message(serialize)?;
                }
            },
            _ => {
                if type_check::is_primitive(ty) {
                    quote! { serialize.source().set_host(&#accessor.to_string())?; }
                } else {
                    quote! { #accessor.to_message(serialize)?; }
                }
            }
        },
    }
}

fn expand_param_ser(ty: &Type, accessor: &TokenStream) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => quote! { serialize.params().push(#accessor)?; },
        String => quote! { serialize.params().push(#accessor.as_ref())?; },
        Option(inner) if type_check::is_str(inner) => quote! {
            if let Some(value) = #accessor {
                serialize.params().push(value)?;
            },
        },
        Option(inner) if type_check::is_string(inner) => quote! {
            if let Some(value) = &#accessor {
                serialize.params().push(value.as_ref())?;
            }
        },
        Option(_) => quote! {
            if let Some(value) = &#accessor {
                value.to_message(serialize)?;
            }
        },
        _ => {
            if type_check::is_primitive(ty) {
                quote! { serialize.params().push(&#accessor.to_string())?; }
            } else {
                quote! { #accessor.to_message(serialize)?; }
            }
        }
    }
}

fn expand_params_ser(ty: &Type, accessor: &TokenStream) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Vec(inner) if type_check::is_str(inner) => quote! {
            for value in &#accessor {
                serialize.params().push(value)?;
            }
        },
        Vec(inner) if type_check::is_string(inner) => quote! {
            for value in &#accessor {
                serialize.params().push(value.as_ref())?;
            }
        },
        _ => quote! {
            #accessor.to_message(serialize)?;
        },
    }
}

fn expand_trailing_ser(ty: &Type, accessor: &TokenStream) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => quote! { serialize.set_trailing(#accessor)?; },
        String => quote! { serialize.set_trailing(#accessor.as_ref())?; },
        Option(inner) if type_check::is_str(inner) => quote! {
            if let Some(value) = #accessor {
                serialize.set_trailing(value)?;
            }
        },
        Option(inner) if type_check::is_string(inner) => quote! {
            if let Some(value) = &#accessor {
                serialize.set_trailing(value.as_ref())?;
            }
        },
        Option(_) => quote! {
            if let Some(value) = &#accessor {
                value.to_message(serialize)?;
            }
        },
        _ => {
            if type_check::is_primitive(ty) {
                quote! { serialize.set_trailing(&#accessor.to_string())?; }
            } else {
                quote! { #accessor.to_message(serialize)?; }
            }
        }
    }
}

fn expand_command_ser(ty: &Type, accessor: &TokenStream) -> TokenStream {
    use TypeKind::*;
    match TypeKind::classify(ty) {
        Str => quote! {
            serialize.set_command(ircv3_parse::Commands::from(#accessor));
        },
        String => quote! {
            serialize.set_command(ircv3_parse::Commands::from(#accessor.as_ref()));
        },
        _ => {
            if type_check::is_primitive(ty) {
                quote! { serialize.set_command(&#accessor.to_string()); }
            } else {
                quote! { #accessor.to_message(serialize)?; }
            }
        }
    }
}

fn expand_from_message(ty: &Type, default: Option<&FieldDefault>) -> TokenStream {
    if let Some(d) = default {
        let fallback = expand_fallback(d);
        quote! { <#ty>::from_message(&msg).unwrap_or_else(|_| #fallback) }
    } else {
        quote! { <#ty>::from_message(&msg)? }
    }
}

fn expand_fallback(default: &FieldDefault) -> TokenStream {
    match default {
        FieldDefault::Trait => quote! { Default::default() },
        FieldDefault::Path(path) => {
            let path_ident = Ident::new(&path.value(), path.span());
            quote! { #path_ident() }
        }
    }
}
