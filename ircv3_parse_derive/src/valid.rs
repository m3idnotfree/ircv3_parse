use quote::ToTokens;
use syn::{Error, Ident, Result};

use crate::{
    ast::{Field, FieldStruct, Input, Struct, UnitStruct},
    attr::{FieldAttrs, FieldKind, StructAttrs, UnitStructAttrs},
    error_msg,
    type_check::TypeKind,
};

impl<'a> Input<'a> {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Struct(strut) => strut.validate(),
            Self::Enum(e) => Err(Error::new_spanned(
                e.ident,
                "FromMessage only supports structs",
            )),
        }
    }

    pub fn validate_ser(&self) -> Result<()> {
        match self {
            Self::Struct(strut) => strut.validate_ser(),
            Self::Enum(e) => Err(Error::new_spanned(
                e.ident,
                "ToMessage only supports structs",
            )),
        }
    }
}

impl<'a> Struct<'a> {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Unit(unit) => unit.validate(),
            Self::Unnamed(unnamed) => unnamed.validate(),
            Self::Named(named) => named.validate(),
        }
    }

    pub fn validate_ser(&self) -> Result<()> {
        match self {
            Self::Unit(_) | Self::Unnamed(_) => Ok(()),
            Self::Named(s) => s.validate_ser(),
        }
    }
}

impl<'a> UnitStruct<'a> {
    pub fn validate(&self) -> Result<()> {
        self.attrs.validate(self.ident)
    }
}

impl<'a> FieldStruct<'a> {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if let Err(e) = self.attrs.validate() {
            errors.push(e);
        }

        for field in &self.fields {
            if let Err(e) = field.validate() {
                errors.push(e);
            }
        }

        combine_errors(errors)
    }

    pub fn validate_ser(&self) -> Result<()> {
        let mut errors = Vec::new();

        for field in &self.fields {
            if let Err(e) = field.validate_ser() {
                errors.push(e);
            }
        }

        combine_errors(errors)
    }
}

impl UnitStructAttrs {
    pub fn validate(&self, name: &Ident) -> Result<()> {
        let mut errors = Vec::new();

        if self.command.is_none() && self.check.is_none() {
            errors.push(Error::new(
                name.span(),
                error_msg::unit_struct_requires_at_least_one(name),
            ));
        }

        for path in &self.unknown {
            errors.push(Error::new_spanned(
                path,
                error_msg::unknown_irc_attribute(path.to_token_stream()),
            ));
        }

        combine_errors(errors)
    }
}

impl StructAttrs {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        for path in &self.unknown {
            errors.push(Error::new_spanned(
                path,
                error_msg::unknown_irc_attribute(path.to_token_stream()),
            ));
        }

        combine_errors(errors)
    }
}

impl<'a> Field<'a> {
    pub fn validate(&self) -> Result<()> {
        self.attrs.validate(self.field)
    }

    pub fn validate_ser(&self) -> Result<()> {
        if self.attrs.kind.is_none() && self.attrs.with.is_none() {
            let msg = match &self.field.ident {
                Some(ident) => format!(
                    "field `{}` requires an #[irc(...)] attribute for ToMessage serialization",
                    ident
                ),
                None => {
                    "unnamed fields require an #[irc(...)] attribute for ToMessage serialization"
                        .to_string()
                }
            };

            Err(Error::new_spanned(self.field, msg))
        } else {
            Ok(())
        }
    }
}

impl FieldAttrs {
    pub fn validate(&self, field: &syn::Field) -> Result<()> {
        let mut errors = Vec::new();

        for path in &self.unknown {
            errors.push(Error::new_spanned(
                path,
                error_msg::unknown_irc_attribute(path.to_token_stream()),
            ));
        }

        if let Some(kind) = &self.kind {
            if let Err(e) = kind.validate(field) {
                errors.push(e);
            }
        }

        combine_errors(errors)
    }
}

impl FieldKind {
    fn validate(&self, field: &syn::Field) -> Result<()> {
        let ty_kind = TypeKind::classify(&field.ty);

        use TypeKind::*;
        match self {
            Self::Command => match ty_kind {
                Option(inner) if matches!(TypeKind::classify(inner), Str | String) => Err(
                    Error::new_spanned(field, error_msg::command_field_cannot_be_option()),
                ),
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}

fn combine_errors(errors: Vec<syn::Error>) -> syn::Result<()> {
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors
            .into_iter()
            .reduce(|mut acc, e| {
                acc.combine(e);
                acc
            })
            .unwrap())
    }
}
