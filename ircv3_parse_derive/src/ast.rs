use proc_macro2::Span;
use syn::{
    Data, DataStruct, DeriveInput, Error, GenericParam, Ident, ImplGenerics, Lifetime,
    LifetimeParam, Result, TypeGenerics, WhereClause,
};

use crate::{
    attr::{FieldAttrs, StructAttrs, UnitStructAttrs},
    component_set::ComponentSet,
};

pub enum Input<'a> {
    Struct(Struct<'a>),
    Enum(Enum<'a>),
}

pub enum Struct<'a> {
    Unit(UnitStruct<'a>),
    Unnamed(FieldStruct<'a>),
    Named(FieldStruct<'a>),
}

pub struct Enum<'a> {
    pub ident: &'a Ident,
}

pub struct UnitStruct<'a> {
    pub ident: &'a Ident,
    pub generics: Generics<'a>,
    pub attrs: UnitStructAttrs,
}

pub struct FieldStruct<'a> {
    pub ident: &'a Ident,
    pub generics: Generics<'a>,
    pub attrs: StructAttrs,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub attrs: FieldAttrs,
    pub field: &'a syn::Field,
}

pub struct Generics<'a> {
    original: &'a syn::Generics,
    generics: syn::Generics,
}

impl<'a> Input<'a> {
    pub fn from_syn(input: &'a DeriveInput, name: &str) -> Result<Self> {
        match &input.data {
            Data::Struct(data_struct) => Struct::from_syn(input, data_struct).map(Input::Struct),
            Data::Enum(_) => Enum::from_syn(input).map(Input::Enum),
            _ => Err(Error::new_spanned(
                &input.ident,
                format!("{name} only supports structs"),
            )),
        }
    }
}

impl<'a> Struct<'a> {
    pub fn from_syn(input: &'a DeriveInput, data_struct: &'a DataStruct) -> Result<Self> {
        let ident = &input.ident;
        let generics = Generics::new(&input.generics);

        match &data_struct.fields {
            syn::Fields::Unit => {
                let attrs = UnitStructAttrs::parse(&input.attrs)?;

                Ok(Struct::Unit(UnitStruct {
                    ident,
                    generics,
                    attrs,
                }))
            }
            syn::Fields::Unnamed(fields) => {
                let attrs = StructAttrs::parse(&input.attrs)?;
                let fields = fields
                    .unnamed
                    .iter()
                    .map(Field::parse)
                    .collect::<Result<Vec<_>>>()?;

                Ok(Struct::Unnamed(FieldStruct {
                    ident,
                    generics,
                    attrs,
                    fields,
                }))
            }
            syn::Fields::Named(fields) => {
                let attrs = StructAttrs::parse(&input.attrs)?;
                let fields = fields
                    .named
                    .iter()
                    .map(Field::parse)
                    .collect::<Result<Vec<_>>>()?;

                Ok(Struct::Named(FieldStruct {
                    ident,
                    generics,
                    attrs,
                    fields,
                }))
            }
        }
    }
}

impl<'a> Enum<'a> {
    pub fn from_syn(input: &'a DeriveInput) -> Result<Self> {
        Err(Error::new_spanned(
            &input.ident,
            "FromMessage only supports structs",
        ))
    }
}

impl<'a> FieldStruct<'a> {
    pub fn components(&self) -> ComponentSet {
        let mut components = ComponentSet::default();

        for field in &self.fields {
            field.add_to(&mut components);
        }

        components
    }
}

impl<'a> Field<'a> {
    pub fn parse(field: &'a syn::Field) -> Result<Self> {
        FieldAttrs::parse(field).map(|attrs| Self { attrs, field })
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        self.attrs.add_to(components);
    }
}

impl<'a> Generics<'a> {
    pub fn new(generics: &'a syn::Generics) -> Self {
        let mut impl_generics = generics.clone();

        if impl_generics.lifetimes().next().is_none() {
            let lifetime = Lifetime::new("'_msg", Span::call_site());
            impl_generics
                .params
                .insert(0, GenericParam::Lifetime(LifetimeParam::new(lifetime)));
        }

        Self {
            generics: impl_generics,
            original: generics,
        }
    }

    pub fn split(&self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>) {
        let (impl_generics, _, where_clause) = self.generics.split_for_impl();
        let (_, struct_ty_generics, _) = self.original.split_for_impl();

        (impl_generics, struct_ty_generics, where_clause)
    }

    pub fn msg_lifetime(&self) -> Lifetime {
        self.generics
            .lifetimes()
            .next()
            .map(|param| param.lifetime.clone())
            .expect("msg_lifetime should exist after new()")
    }
}
