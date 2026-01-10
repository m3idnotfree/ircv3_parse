use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::token::Eq;
use syn::{meta::ParseNestedMeta, Error, Result};
use syn::{Field, Ident, LitInt, LitStr};

use crate::error_msg;
use crate::extractors::{CommandField, ParamField, SourceField, Tag, TrailingField};
use crate::ser::SerializationBuilder;
use crate::MessageComponents;
use crate::TypeKind;
use crate::{COMMAND, IRC, PARAM, PARAMS, SOURCE, TAG, TAG_FLAG, TRAILING, WITH};

pub struct FieldAttribute {
    kind: FieldKind,
    with: Option<LitStr>,
}

impl FieldAttribute {
    pub fn parse(field: &Field, field_name: &Ident) -> Result<Self> {
        let mut field_kind: Option<FieldKind> = None;
        let mut with: Option<LitStr> = None;

        for attr in &field.attrs {
            if !attr.path().is_ident(IRC) {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let attr_type = AttributeType::parse(&meta, field_name)?;

                if let AttributeType::With(value) = attr_type {
                    if with.is_some() {
                        return Err(meta.error(error_msg::duplicate_attribute(WITH)));
                    }

                    with = Some(value);
                    return Ok(());
                }

                if let Some(existing) = &field_kind {
                    return Err(meta.error(format!(
                        "field cannot have multiple extraction attributes (found both `{}` and `{}`)",
                        existing.name(),
                        attr_type.name()
                    )));
                }

                field_kind = Some(FieldKind::from_attribute_type(attr_type)?);
                Ok(())
            })?;
        }

        let kind = field_kind.ok_or_else(|| {
            Error::new_spanned(
                field,
                "field must have at least one IRC extraction attribute",
            )
        })?;

        Ok(Self { kind, with })
    }

    pub fn expand(&self, field: &Field, field_name: &Ident) -> Result<proc_macro2::TokenStream> {
        self.kind.expand(field, field_name, &self.with)
    }

    pub fn mark_components(&self, components: &mut MessageComponents) {
        use FieldKind::*;

        match &self.kind {
            Tag { .. } => {
                components.mark_tags();
            }
            Source(_) => {
                components.mark_source();
            }
            Param(_) | Params | Trailing => {
                components.mark_params();
            }
            Command(_) => {
                components.mark_command();
            }
        };
    }

    pub fn command_field(&self) -> Option<LitStr> {
        match &self.kind {
            FieldKind::Command(cmd) => cmd.0.clone(),
            _ => None,
        }
    }

    pub fn expand_de(
        self,
        field: &Field,
        field_name: &Ident,
        builder: &mut SerializationBuilder,
    ) -> Result<()> {
        self.kind.expand_de(field, field_name, builder)
    }
}

enum AttributeType {
    Tag(LitStr),
    TagFlag(LitStr),
    Source(LitStr),
    Param(LitInt),
    Params,
    Trailing,
    Command(Option<LitStr>),
    With(LitStr),
}

impl AttributeType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Tag(_) => TAG,
            Self::TagFlag(_) => TAG_FLAG,
            Self::Source(_) => SOURCE,
            Self::Param(_) => PARAM,
            Self::Params => PARAMS,
            Self::Trailing => TRAILING,
            Self::Command(_) => COMMAND,
            Self::With(_) => WITH,
        }
    }

    pub fn parse(meta: &ParseNestedMeta<'_>, field_name: &Ident) -> Result<Self> {
        if meta.path.is_ident(TAG) {
            let key = if meta.input.peek(Eq) {
                let lit: LitStr = meta.value()?.parse()?;
                lit
            } else {
                LitStr::new(&field_name.to_string(), field_name.span())
            };

            return Ok(Self::Tag(key));
        }

        if meta.path.is_ident(TAG_FLAG) {
            let key = if meta.input.peek(Eq) {
                let lit: LitStr = meta.value()?.parse()?;
                lit
            } else {
                LitStr::new(&field_name.to_string(), field_name.span())
            };

            return Ok(Self::TagFlag(key));
        }

        if meta.path.is_ident(SOURCE) {
            let key = if meta.input.peek(Eq) {
                let lit: LitStr = meta.value()?.parse()?;
                lit
            } else {
                LitStr::new("name", field_name.span())
            };

            return Ok(Self::Source(key));
        }

        if meta.path.is_ident(PARAM) {
            let key = if meta.input.peek(Eq) {
                let lit: LitInt = meta.value()?.parse()?;
                lit
            } else {
                LitInt::new("0", field_name.span())
            };

            return Ok(Self::Param(key));
        }

        if meta.path.is_ident(PARAMS) {
            return Ok(Self::Params);
        }

        if meta.path.is_ident(TRAILING) {
            return Ok(Self::Trailing);
        }

        if meta.path.is_ident(COMMAND) {
            let value = if meta.input.peek(Eq) {
                Some(meta.value()?.parse()?)
            } else {
                None
            };

            return Ok(Self::Command(value));
        }

        if meta.path.is_ident(WITH) {
            return Ok(Self::With(meta.value()?.parse()?));
        }

        Err(meta.error(error_msg::unknown_irc_attribute(
            meta.path.to_token_stream(),
        )))
    }
}

enum FieldKind {
    Tag(Tag),
    Source(SourceField),
    Param(ParamField),
    Params,
    Trailing,
    Command(CommandField),
}

impl FieldKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Tag(tag_type) => match tag_type {
                Tag::Value(_) => TAG,
                Tag::Flag(_) => TAG_FLAG,
            },
            Self::Source(_) => SOURCE,
            Self::Param(_) => PARAM,
            Self::Params => PARAMS,
            Self::Trailing => TRAILING,
            Self::Command(_) => COMMAND,
        }
    }

    pub fn from_attribute_type(attr_type: AttributeType) -> Result<Self> {
        match attr_type {
            AttributeType::Tag(key) => Ok(Self::Tag(Tag::Value(key))),
            AttributeType::TagFlag(key) => Ok(Self::Tag(Tag::Flag(key))),
            AttributeType::Source(value) => Ok(Self::Source(SourceField::parse(&value)?)),
            AttributeType::Param(value) => Ok(Self::Param(ParamField::new(&value)?)),
            AttributeType::Params => Ok(Self::Params),
            AttributeType::Trailing => Ok(Self::Trailing),
            AttributeType::Command(value) => Ok(Self::Command(CommandField::new(value))),
            AttributeType::With(_) => Err(Error::new(
                Span::call_site(),
                "`with` is not an extraction attribute",
            )),
        }
    }

    pub fn expand(
        &self,
        field: &Field,
        field_name: &Ident,
        with: &Option<LitStr>,
    ) -> Result<proc_macro2::TokenStream> {
        match &self {
            Self::Tag(tag_type) => tag_type.expand(field, field_name, with),
            Self::Source(source) => source.expand(field, field_name, with),
            Self::Param(param) => param.expand(field, field_name, with),
            Self::Params => expand_params_vec(field, field_name, with),
            Self::Trailing => TrailingField::expand(field, field_name, with),
            Self::Command(_) => CommandField::expand(field, field_name, with),
        }
    }

    pub fn expand_de(
        self,
        field: &Field,
        field_name: &Ident,
        builder: &mut SerializationBuilder,
    ) -> Result<()> {
        match self {
            Self::Tag(tag_type) => tag_type.expand_de(field, field_name, builder),
            Self::Source(source) => source.expand_de(field, field_name, builder),
            Self::Param(param) => param.expand_de(field, field_name, builder),
            Self::Params => expand_vec_de(field, field_name, builder),
            Self::Trailing => TrailingField::expand_de(field, field_name, builder),
            Self::Command(cmd) => cmd.expand_de(field, field_name, builder),
        }
    }
}

fn expand_params_vec(
    field: &Field,
    field_name: &Ident,
    with: &Option<LitStr>,
) -> Result<proc_macro2::TokenStream> {
    if let Some(with_fn) = &with {
        let with_fn = Ident::new(&with_fn.value(), with_fn.span());
        return Ok(quote! { #field_name: #with_fn(params.middles.to_vec()) });
    }

    match TypeKind::classify(&field.ty) {
        TypeKind::Vec(inner) if matches!(TypeKind::classify(inner), TypeKind::Str) => {
            Ok(quote! {#field_name: params.middles.to_vec() })
        }
        TypeKind::Vec(inner) if matches!(TypeKind::classify(inner), TypeKind::String) => Ok(
            quote! {#field_name: params.middles.iter().map(|s| s.to_string()).collect::<Vec<_>>() },
        ),
        _ => Err(Error::new_spanned(
            field,
            error_msg::unsupported_type(PARAMS, field_name, field.ty.to_token_stream()),
        )),
    }
}

fn expand_vec_de(
    field: &Field,
    field_name: &Ident,
    builder: &mut SerializationBuilder,
) -> Result<()> {
    use TypeKind::*;

    match TypeKind::classify(&field.ty) {
        Vec(ty) => match TypeKind::classify(ty) {
            Str => {
                builder.params_push(quote! {
                    params.extend(&self.#field_name)?;
                });
                Ok(())
            }
            String => {
                builder.params_push(quote! {
                    params.extend(&self.#field_name)?;
                });
                Ok(())
            }
            _ => {
                builder.custom_params(quote! {
                    for p in &self.#field_name {
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
