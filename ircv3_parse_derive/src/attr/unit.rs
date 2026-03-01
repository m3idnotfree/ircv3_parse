use proc_macro2::{Span, TokenTree};
use syn::{spanned::Spanned, token::Eq, Attribute, Error, LitStr, Path, Result};

use crate::error_msg;

use super::{parse_required_lit_str, FieldKind, COMMAND, IRC, VALUE};

pub struct UnitStructAttrs {
    pub command: Option<LitStr>,
    pub check: Option<FieldKind>,
    pub value: Option<LitStr>,
    pub unknown: Vec<Path>,
}

impl UnitStructAttrs {
    pub fn parse(attrs: &[Attribute]) -> Result<Self> {
        let mut command = None;
        let mut command_span: Option<Span> = None;
        let mut check: Option<FieldKind> = None;
        let mut check_span: Option<Span> = None;
        let mut value = None;
        let mut value_span: Option<Span> = None;
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

                if let Some(field_kind) = FieldKind::try_parse(&meta, None)? {
                    if let Some(existing) = &check {
                        let mut err = meta.error(error_msg::duplicate_unit_struct_attribute(
                            existing.name(),
                            field_kind.name(),
                        ));

                        if let Some(first) = check_span {
                            err.combine(Error::new(
                                first,
                                error_msg::first_defined_here(existing.name()),
                            ));
                        }

                        return Err(err);
                    }

                    check = Some(field_kind);
                    check_span = Some(meta.path.span());

                    return Ok(());
                }

                if meta.path.is_ident(VALUE) {
                    let lit = parse_required_lit_str(&meta, VALUE)?;

                    if value.is_some() {
                        let mut err = meta.error(error_msg::duplicate_attribute(VALUE));
                        if let Some(first) = value_span {
                            err.combine(Error::new(first, error_msg::first_defined_here(VALUE)));
                        }

                        return Err(err);
                    }

                    value = Some(lit);
                    value_span = Some(meta.path.span());

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
            check,
            value,
            unknown,
        })
    }
}
