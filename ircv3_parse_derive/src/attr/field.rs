use proc_macro2::{Span, TokenTree};
use syn::{
    meta::ParseNestedMeta, spanned::Spanned, token::Eq, Attribute, Error, Ident, LitInt, LitStr,
    Path, Result,
};

use crate::{component_set::ComponentSet, error_msg};

use super::{
    parse_lit_str, parse_required_lit_str, COMMAND, CRLF, DEFAULT, IRC, PARAM, PARAMS, SOURCE, TAG,
    TAG_FLAG, TRAILING, WITH,
};

pub struct StructAttrs {
    pub command: Option<LitStr>,
    pub crlf: bool,
    pub unknown: Vec<Path>,
}

pub struct FieldAttrs {
    pub kind: Option<FieldKind>,
    pub with: Option<LitStr>,
    pub default: Option<FieldDefault>,
    pub unknown: Vec<Path>,
}

pub enum FieldKind {
    Tag(LitStr),
    TagFlag(LitStr),
    Source(Source),
    Param(usize),
    Params,
    Trailing,
    Command,
}

pub enum Source {
    Name,
    User,
    Host,
}

pub enum FieldDefault {
    Path(LitStr),
    Trait,
}

pub enum Rename {
    Lowercase,
    Uppercase,
    KebabCase,
}

impl StructAttrs {
    pub fn parse(attrs: &[Attribute]) -> Result<Self> {
        let mut command = None;
        let mut command_span: Option<Span> = None;
        let mut crlf = false;
        let mut crlf_span: Option<Span> = None;
        let mut unknown = Vec::new();

        for attr in attrs {
            if !attr.path().is_ident(IRC) {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(COMMAND) {
                    let lit = parse_required_lit_str(&meta, COMMAND)?;

                    if command.is_some() {
                        let mut err = meta.error(error_msg::duplicate_attribute(COMMAND));
                        if let Some(first) = command_span {
                            err.combine(Error::new(first, error_msg::first_defined_here(COMMAND)));
                        }

                        return Err(err);
                    }

                    command = Some(lit);
                    command_span = Some(meta.path.span());

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

        Ok(Self {
            command,
            crlf,
            unknown,
        })
    }
}

impl FieldAttrs {
    pub fn parse(field: &syn::Field) -> Result<Self> {
        let mut kind: Option<FieldKind> = None;
        let mut kind_span: Option<Span> = None;
        let mut with: Option<LitStr> = None;
        let mut with_span: Option<Span> = None;
        let mut default: Option<FieldDefault> = None;
        let mut default_span: Option<Span> = None;
        let mut unknown = Vec::new();

        let field_name = field.ident.as_ref();

        for attr in &field.attrs {
            if !attr.path().is_ident(IRC) {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(WITH) {
                    let lit = parse_lit_str(&meta, WITH)?;

                    if with.is_some() {
                        let mut err = meta.error(error_msg::duplicate_attribute(WITH));
                        if let Some(first) = with_span {
                            err.combine(Error::new(first, error_msg::first_defined_here(WITH)));
                        }

                        return Err(err);
                    }

                    with = Some(lit);
                    with_span = Some(meta.path.span());

                    return Ok(());
                }

                if meta.path.is_ident(DEFAULT) {
                    let value = if meta.input.peek(Eq) {
                        let lit = parse_lit_str(&meta, DEFAULT)?;

                        FieldDefault::Path(lit)
                    } else {
                        FieldDefault::Trait
                    };

                    if default.is_some() {
                        let mut err = meta.error(error_msg::duplicate_attribute(DEFAULT));
                        if let Some(first) = default_span {
                            err.combine(Error::new(first, error_msg::first_defined_here(DEFAULT)));
                        }

                        return Err(err);
                    }

                    default = Some(value);
                    default_span = Some(meta.path.span());

                    return Ok(());
                }

                if let Some(field_kind) = FieldKind::try_parse(&meta, field_name)? {
                    if let Some(existing) = &kind {
                        let mut err = meta.error(error_msg::multiple_extraction_attributes(
                            existing.name(),
                            field_kind.name(),
                        ));

                        if let Some(first) = kind_span {
                            err.combine(Error::new(
                                first,
                                error_msg::first_defined_here(existing.name()),
                            ));
                        }

                        return Err(err);
                    }

                    kind = Some(field_kind);
                    kind_span = Some(meta.path.span());

                    return Ok(());
                }

                unknown.push(meta.path.clone());
                if meta.input.peek(Eq) {
                    meta.value()?.parse::<TokenTree>()?;
                }

                Ok(())
            })?;
        }

        if default.is_some() && kind.is_none() {
            if let Some(span) = default_span {
                return Err(Error::new(span, error_msg::default_requires_component()));
            }
        }

        Ok(Self {
            kind,
            with,
            default,
            unknown,
        })
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        if let Some(kind) = &self.kind {
            if self.default.is_some() {
                match kind {
                    FieldKind::Tag(_) | FieldKind::TagFlag(_) => return,
                    FieldKind::Source(_) => return,
                    _ => {}
                }
            }

            kind.add_to(components);
        }
    }
}

impl FieldKind {
    pub fn try_parse(meta: &ParseNestedMeta, field_name: Option<&Ident>) -> Result<Option<Self>> {
        if meta.path.is_ident(TAG) {
            let key = if meta.input.peek(Eq) {
                parse_lit_str(meta, TAG)?
            } else if let Some(name) = field_name {
                LitStr::new(&name.to_string(), name.span())
            } else {
                return Err(meta.error(error_msg::required_value(TAG)));
            };

            return Ok(Some(Self::Tag(key)));
        }

        if meta.path.is_ident(TAG_FLAG) {
            let key = if meta.input.peek(Eq) {
                parse_lit_str(meta, TAG_FLAG)?
            } else if let Some(name) = field_name {
                LitStr::new(&name.to_string(), name.span())
            } else {
                return Err(meta.error(error_msg::required_value(TAG_FLAG)));
            };

            return Ok(Some(Self::TagFlag(key)));
        }

        if meta.path.is_ident(SOURCE) {
            let source = if meta.input.peek(Eq) {
                let lit: LitStr = meta.value()?.parse()?;

                Source::parse(lit)?
            } else {
                Source::Name
            };

            return Ok(Some(Self::Source(source)));
        }

        if meta.path.is_ident(PARAM) {
            let idx = if meta.input.peek(Eq) {
                let lit: LitInt = meta.value()?.parse()?;
                lit.base10_parse()?
            } else {
                0
            };

            return Ok(Some(Self::Param(idx)));
        }

        if meta.path.is_ident(PARAMS) {
            return Ok(Some(Self::Params));
        }

        if meta.path.is_ident(TRAILING) {
            return Ok(Some(Self::Trailing));
        }

        if meta.path.is_ident(COMMAND) {
            if meta.input.peek(Eq) {
                return Err(meta.error(error_msg::cannot_have_a_value_command()));
            }

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
            Self::Params => PARAMS,
            Self::Trailing => TRAILING,
            Self::Command => COMMAND,
        }
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        match self {
            Self::Tag(_) | Self::TagFlag(_) => components.add_tags(),
            Self::Source(_) => components.add_source(),
            Self::Param(_) | Self::Params | Self::Trailing => components.add_params(),
            Self::Command => components.add_command(),
        }
    }
}

impl Source {
    pub fn parse(lit: LitStr) -> Result<Self> {
        let value = lit.value();

        if value.is_empty() {
            return Err(Error::new(lit.span(), error_msg::cannot_be_empty(SOURCE)));
        }

        match value.as_str() {
            "name" => Ok(Self::Name),
            "user" => Ok(Self::User),
            "host" => Ok(Self::Host),
            _ => Err(Error::new(lit.span(), error_msg::invalid_source_field())),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Name => "name",
            Self::User => "user",
            Self::Host => "host",
        }
    }
}

impl Rename {
    pub fn parse(lit: &LitStr) -> Result<Self> {
        match lit.value().as_str() {
            "lowercase" => Ok(Self::Lowercase),
            "UPPERCASE" => Ok(Self::Uppercase),
            "kebab-case" => Ok(Self::KebabCase),
            other => Err(Error::new(
                lit.span(),
                error_msg::unknown_rename_rule(other),
            )),
        }
    }

    pub fn apply(&self, ident: &str) -> String {
        match self {
            Self::Lowercase => ident.to_lowercase(),
            Self::Uppercase => ident.to_uppercase(),
            Self::KebabCase => {
                use heck::ToKebabCase;
                ident.to_kebab_case()
            }
        }
    }
}
