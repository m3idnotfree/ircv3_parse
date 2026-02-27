use quote::ToTokens;
use syn::{Error, Ident, Result};

use crate::{
    ast::{Enum, Field, FieldStruct, Input, Struct, UnitStruct, Variant, VariantFields},
    attr::{
        EnumAttrs, EnumKind, FieldAttrs, FieldKind, StructAttrs, UnitStructAttrs, VariantAttrs,
        PRESENT,
    },
    error_msg,
    type_check::TypeKind,
};

impl<'a> Input<'a> {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Struct(strut) => strut.validate(),
            Self::Enum(e) => e.validate(),
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
            Self::Fields(named) => named.validate(),
        }
    }

    pub fn validate_ser(&self) -> Result<()> {
        match self {
            Self::Unit(_) => Ok(()),
            Self::Fields(s) => s.validate_ser(),
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

impl<'a> Enum<'a> {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        self.check_default(&mut errors);
        self.check_tag_flag(&mut errors);

        if let Err(err) = self.attrs.validate() {
            errors.push(err)
        }

        for variant in &self.variants {
            if let Err(err) = variant.validate() {
                errors.push(err);
            }
        }

        combine_errors(errors)
    }

    fn check_default(&self, errors: &mut Vec<Error>) {
        if let Some(default) = &self.attrs.default {
            let name = default.value();
            match self
                .variants
                .iter()
                .find(|variant| *variant.ident.to_string() == name)
            {
                None => errors.push(Error::new_spanned(
                    default,
                    error_msg::default_variant_not_found(&name),
                )),
                Some(variant) if !matches!(variant.fields, VariantFields::Unit) => {
                    errors.push(Error::new_spanned(
                        variant.ident,
                        error_msg::default_variant_must_be_unit(&name),
                    ));
                }
                Some(_) => {}
            }
        }
    }

    fn check_tag_flag(&self, errors: &mut Vec<Error>) {
        if matches!(self.attrs.kind, EnumKind::TagFlag(_)) {
            if self.variants.len() != 2 {
                errors.push(Error::new_spanned(
                    self.ident,
                    error_msg::tag_flag_enum_requires(),
                ));
            }

            for variant in &self.variants {
                if variant.attrs.present.is_none() && !matches!(variant.fields, VariantFields::Unit)
                {
                    errors.push(Error::new_spanned(
                        variant.ident,
                        error_msg::tag_flag_absent_must_be_unit(),
                    ));
                }
            }

            for variant in &self.variants {
                for value in &variant.attrs.values {
                    errors.push(Error::new_spanned(
                        value,
                        error_msg::value_not_allowed_on_tag_flag(),
                    ));
                }
            }

            let present_count = self
                .variants
                .iter()
                .filter(|v| v.attrs.present.is_some())
                .count();
            match present_count {
                0 => errors.push(Error::new_spanned(
                    self.ident,
                    error_msg::tag_flag_requires_present(),
                )),
                1 => {}
                _ => {
                    let mut iter = self.variants.iter().filter(|v| v.attrs.present.is_some());

                    let first = iter.next().unwrap();
                    errors.push(Error::new(
                        first.attrs.present.unwrap(),
                        error_msg::first_defined_here(PRESENT),
                    ));

                    for variant in iter {
                        errors.push(Error::new(
                            variant.attrs.present.unwrap(),
                            error_msg::duplicate_attribute(PRESENT),
                        ));
                    }
                }
            }
        } else {
            for variant in &self.variants {
                if let Some(span) = variant.attrs.present {
                    errors.push(Error::new(span, error_msg::present_only_for_tag_flag()));
                }
            }
        }
    }
}

impl EnumAttrs {
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

impl<'a> Variant<'a> {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if let Err(err) = self.attrs.validate() {
            errors.push(err);
        }

        if let Err(err) = self.fields.validate() {
            errors.push(err);
        }

        combine_errors(errors)
    }
}

impl VariantAttrs {
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

impl<'a> VariantFields<'a> {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        match self {
            Self::Unit => {}
            Self::Named(fields) => {
                for field in fields {
                    if let Err(err) = field.validate() {
                        errors.push(err);
                    }
                }
            }
            Self::Unnamed(fields) => {
                for field in fields {
                    if let Err(err) = field.validate() {
                        errors.push(err);
                    }
                }
            }
        };

        combine_errors(errors)
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
