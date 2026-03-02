use proc_macro2::{Span, TokenTree};
use syn::{
    meta::ParseNestedMeta, spanned::Spanned, token::Eq, Attribute, Error, LitInt, LitStr, Path,
    Result,
};

use crate::{component_set::ComponentSet, error_msg};

use super::{
    parse_lit_str, parse_required_lit_str, Rename, Source, COMMAND, CRLF, DEFAULT, IRC, PARAM,
    PICK, PRESENT, RENAME, SOURCE, TAG, TAG_FLAG, TRAILING, VALUE,
};

pub struct EnumAttrs {
    pub kind: EnumKind,
    pub rename: Rename,
    pub default: Option<LitStr>,
    pub crlf: bool,
    pub unknown: Vec<Path>,
}

pub enum EnumKind {
    Tag(LitStr),
    TagFlag(LitStr),
    Source(Source),
    Param(LitInt),
    Trailing,
    Command,
}

pub struct VariantAttrs {
    pub values: Vec<LitStr>,
    pub pick: Option<LitStr>,
    pub present: Option<Span>,
    pub unknown: Vec<Path>,
}

impl EnumAttrs {
    pub fn parse(attrs: &[Attribute]) -> Result<Self> {
        let mut kind: Option<EnumKind> = None;
        let mut kind_span: Option<Span> = None;
        let mut rename: Option<Rename> = None;
        let mut rename_span: Option<Span> = None;
        let mut default: Option<LitStr> = None;
        let mut default_span: Option<Span> = None;
        let mut crlf = false;
        let mut crlf_span: Option<Span> = None;
        let mut unknown = Vec::new();

        for attr in attrs {
            if !attr.path().is_ident(IRC) {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(RENAME) {
                    let key = parse_required_lit_str(&meta, RENAME)?;

                    if rename.is_some() {
                        let mut err = meta.error(error_msg::duplicate_attribute(RENAME));
                        if let Some(first) = rename_span {
                            err.combine(Error::new(first, error_msg::first_defined_here(RENAME)));
                        }

                        return Err(err);
                    }

                    rename = Some(Rename::parse(&key)?);
                    rename_span = Some(meta.path.span());

                    return Ok(());
                }

                if meta.path.is_ident(DEFAULT) {
                    let name = parse_required_lit_str(&meta, DEFAULT)?;

                    if default.is_some() {
                        let mut err = meta.error(error_msg::duplicate_attribute(DEFAULT));
                        if let Some(first) = default_span {
                            err.combine(Error::new(first, error_msg::first_defined_here(DEFAULT)));
                        }

                        return Err(err);
                    }

                    default = Some(name);
                    default_span = Some(meta.path.span());

                    return Ok(());
                }

                if let Some(enum_kind) = EnumKind::try_parse(&meta)? {
                    if let Some(existing) = &kind {
                        let mut err = meta.error(error_msg::multiple_extraction_attributes(
                            existing.name(),
                            enum_kind.name(),
                        ));

                        if let Some(first) = kind_span {
                            err.combine(Error::new(
                                first,
                                error_msg::first_defined_here(existing.name()),
                            ));
                        }

                        return Err(err);
                    }

                    kind = Some(enum_kind);
                    kind_span = Some(meta.path.span());

                    return Ok(());
                }

                if meta.path.is_ident(CRLF) {
                    if crlf {
                        let mut err = meta.error(error_msg::duplicate_attribute(CRLF));
                        if let Some(first) = crlf_span {
                            err.combine(Error::new(first, error_msg::first_defined_here(CRLF)));
                        }

                        return Err(err);
                    }

                    crlf = true;
                    crlf_span = Some(meta.path.span());

                    return Ok(());
                }

                unknown.push(meta.path.clone());
                if meta.input.peek(Eq) {
                    meta.value()?.parse::<TokenTree>()?;
                }

                Ok(())
            })?;
        }

        let kind = kind
            .ok_or_else(|| Error::new(Span::call_site(), error_msg::enum_requires_component()))?;

        if matches!(kind, EnumKind::Command) {
            if let Some(span) = rename_span {
                return Err(Error::new(
                    span,
                    error_msg::rename_not_allowed_with_command(),
                ));
            }
        }

        let rename = rename.unwrap_or(match kind {
            EnumKind::Command => Rename::Uppercase,
            _ => Rename::Lowercase,
        });

        Ok(Self {
            kind,
            rename,
            default,
            crlf,
            unknown,
        })
    }
}

impl EnumKind {
    pub fn try_parse(meta: &ParseNestedMeta) -> Result<Option<Self>> {
        if meta.path.is_ident(TAG) {
            let key = parse_required_lit_str(meta, TAG)?;
            return Ok(Some(Self::Tag(key)));
        }

        if meta.path.is_ident(TAG_FLAG) {
            let key = parse_required_lit_str(meta, TAG_FLAG)?;
            return Ok(Some(Self::TagFlag(key)));
        }

        if meta.path.is_ident(SOURCE) {
            let source = if meta.input.peek(Eq) {
                let lit = parse_lit_str(meta, SOURCE)?;

                Source::parse(lit)?
            } else {
                Source::Name
            };

            return Ok(Some(Self::Source(source)));
        }

        if meta.path.is_ident(PARAM) {
            let idx = if meta.input.peek(Eq) {
                meta.value()?.parse()?
            } else {
                LitInt::new("0", Span::call_site())
            };

            return Ok(Some(Self::Param(idx)));
        }

        if meta.path.is_ident(TRAILING) {
            return Ok(Some(Self::Trailing));
        }

        if meta.path.is_ident(COMMAND) {
            return Ok(Some(Self::Command));
        }

        Ok(None)
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Tag(_) => TAG,
            Self::TagFlag(_) => TAG_FLAG,
            Self::Source(_) => SOURCE,
            Self::Param(_) => PARAM,
            Self::Trailing => TRAILING,
            Self::Command => COMMAND,
        }
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        match self {
            Self::Tag(_) | Self::TagFlag(_) => components.add_tags(),
            Self::Source(_) => components.add_source(),
            Self::Param(_) | Self::Trailing => components.add_params(),
            Self::Command => components.add_command(),
        }
    }
}

impl VariantAttrs {
    pub fn parse(attrs: &[Attribute]) -> Result<Self> {
        let mut values: Vec<LitStr> = Vec::new();
        let mut pick: Option<LitStr> = None;
        let mut present: Option<Span> = None;
        let mut unknown = Vec::new();

        for attr in attrs {
            if !attr.path().is_ident(IRC) {
                continue;
            }

            let mut last_value_in_attr: Option<LitStr> = None;

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(VALUE) {
                    let lit = parse_required_lit_str(&meta, VALUE)?;
                    last_value_in_attr = Some(lit.clone());
                    values.push(lit);
                    return Ok(());
                }

                if meta.path.is_ident(PICK) {
                    if meta.input.peek(Eq) {
                        return Err(Error::new(
                            meta.path.span(),
                            error_msg::cannot_have_value(PICK),
                        ));
                    }

                    if let Some(first) = &pick {
                        let mut err = meta.error(error_msg::duplicate_attribute(PICK));
                        err.combine(Error::new(
                            first.span(),
                            error_msg::first_defined_here(PICK),
                        ));
                        return Err(err);
                    }

                    let Some(last) = &last_value_in_attr else {
                        return Err(Error::new(
                            meta.path.span(),
                            error_msg::pick_must_follow_value(),
                        ));
                    };

                    pick = Some(LitStr::new(&last.value(), meta.path.span()));
                    return Ok(());
                }

                if meta.path.is_ident(PRESENT) {
                    if meta.input.peek(Eq) {
                        return Err(Error::new(
                            meta.path.span(),
                            error_msg::cannot_have_value(PRESENT),
                        ));
                    }

                    if let Some(first_span) = present {
                        let mut err = meta.error(error_msg::duplicate_attribute(PRESENT));
                        err.combine(Error::new(
                            first_span,
                            error_msg::first_defined_here(PRESENT),
                        ));

                        return Err(err);
                    }

                    present = Some(meta.path.span());

                    return Ok(());
                }

                unknown.push(meta.path.clone());
                if meta.input.peek(Eq) {
                    meta.value()?.parse::<TokenTree>()?;
                }

                Ok(())
            })?;
        }

        Ok(Self {
            values,
            pick,
            present,
            unknown,
        })
    }
}
