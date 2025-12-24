pub(crate) mod error_msg;

mod components;
mod extractors;
mod field_attribute;
mod msg_lifetime;
mod struct_attribute;
mod type_check;

pub(crate) use type_check::TypeKind;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Result};

use components::MessageComponents;
use extractors::extract_named_fields;
use field_attribute::FieldAttribute;
use msg_lifetime::get_or_create_msg_lifetime;
use struct_attribute::StructAttribute;

pub(crate) const IRC: &str = "irc";
pub(crate) const COMMAND: &str = "command";
pub(crate) const TAG: &str = "tag";
pub(crate) const TAG_FLAG: &str = "tag_flag";
pub(crate) const SOURCE: &str = "source";
pub(crate) const PARAM: &str = "param";
pub(crate) const PARAMS: &str = "params";
pub(crate) const TRAILING: &str = "trailing";
pub(crate) const WITH: &str = "with";

/// Derives `FromMessage` implementation for structs
///
/// # Attributes
///
/// ## Struct-level
/// - `#[irc(command = "COMMAND")]` - Validates command matches
///
/// ## Field-level
/// - `#[irc(tag = "key")]` - Extract tag value
/// - `#[irc(tag_flag = "key")]` - Extract tag flag (bool)
/// - `#[irc(source = "name|user|host")]` - Extract source component
/// - `#[irc(param = N)]` - Extract Nth parameter
/// - `#[irc(params)]` - Extract all parameter
/// - `#[irc(trailing)]` - Extract trailing message
/// - `#[irc(command)]` - Extract command
/// - `#[irc(with = "function")]` - Custom converter function
/// ```
#[proc_macro_derive(FromMessage, attributes(irc))]
pub fn derive_from_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_from_message_impl(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_from_message_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let (_, struct_ty_generics, _) = input.generics.split_for_impl();

    let mut impl_block_generics = input.generics.clone();
    let msg_lifetime = get_or_create_msg_lifetime(&mut impl_block_generics);
    let (impl_generics, _, where_clause) = impl_block_generics.split_for_impl();

    let struct_attrs = StructAttribute::parse(&input)?;

    let fields = extract_named_fields(&input)?;

    let mut components = MessageComponents::default();
    let mut expand_fields: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut errors = Vec::new();

    for field in fields.iter() {
        let field_name = match field.ident.as_ref() {
            Some(field_name) => field_name,
            None => continue,
        };

        let attribute = match FieldAttribute::parse(field) {
            Ok(attr) => attr,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        attribute.mark_components(&mut components);

        let expand = match attribute.expand(field, field_name) {
            Ok(expand) => expand,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        expand_fields.push(expand);
    }

    if let Some(e) = combine_errors(errors) {
        return Err(e);
    }

    let struct_name = &input.ident;
    let command_validation = struct_attrs.expand_validation();
    let setup_code = components.expand();

    Ok(quote! {
        impl #impl_generics ircv3_parse::extract::FromMessage<#msg_lifetime>
            for #struct_name #struct_ty_generics #where_clause
        {
            fn from_message(
                msg: &ircv3_parse::Message<#msg_lifetime>
            ) -> Result<Self, ircv3_parse::ExtractError> {
                #command_validation

                #(#setup_code)*

                Ok(Self {
                    #(#expand_fields),*
                })

            }
        }
    })
}

fn combine_errors(errors: Vec<Error>) -> Option<Error> {
    errors.into_iter().reduce(|mut a, b| {
        a.combine(b);
        a
    })
}
