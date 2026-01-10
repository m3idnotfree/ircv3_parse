pub(crate) mod error_msg;

mod components;
mod extractors;
mod field_attribute;
mod msg_lifetime;
mod ser;
mod struct_attribute;
mod type_check;

pub(crate) use type_check::TypeKind;

use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;
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
///
/// - `#[irc(command = "COMMAND")]` - Validates command matches
///
/// ## Field-level
///
/// **Tag Extraction:**
/// - `#[irc(tag)]` - Extract tag value using field name as key
/// - `#[irc(tag = "key")]` - Extract tag value with custom key
///
/// **Tag Flag Extraction:**
/// - `#[irc(tag_flag)]` - Extract tag flag using field name as key (returns `bool`)
/// - `#[irc(tag_flag = "key")]` - Extract tag flag with custom key (returns `bool`)
///
/// **Source Extraction:**
/// - `#[irc(source)]` - Extract source `name` component
/// - `#[irc(source = "name|user|host")]` - Extract source component
///
/// **Parameter Extraction:**
/// - `#[irc(param)]` - Extract first parameter (index 0)
/// - `#[irc(param = N)]` - Extract parameter at index N
/// - `#[irc(params)]` - Extract all parameter into a `Vec`
///
/// **Trailing Parameter:**
/// - `#[irc(trailing)]` - Extract trailing parameter
///
/// **Command Extraction:**
/// - `#[irc(command)]` - Extract command value
/// - `#[irc(command = "COMMAND)]` - Extract and validate command matches "COMMAND"
///     - If field-level `command` is set, struct-level `command` is ignored
///     - If multiple `command` attribute exist, the last one is used
///
/// **Custom Extraction:**
/// - `#[irc(with = "function")]` - Use custom extraction function
///
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

    let fields = extract_named_fields(&input, "FromMessage")?;

    let mut components = MessageComponents::default();
    let mut expand_fields: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut errors = Vec::new();

    let mut commands: Option<LitStr> = None;

    for field in fields.iter() {
        let field_name = match field.ident.as_ref() {
            Some(field_name) => field_name,
            None => continue,
        };

        let attribute = match FieldAttribute::parse(field, field_name) {
            Ok(attr) => attr,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        attribute.mark_components(&mut components);

        commands = attribute.command_field();

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
    let command_validation = struct_attrs.expand_validation(commands);
    let setup_code = components.expand();

    Ok(quote! {
        impl #impl_generics ircv3_parse::message::de::FromMessage<#msg_lifetime>
            for #struct_name #struct_ty_generics #where_clause
        {
            fn from_message(
                msg: &ircv3_parse::Message<#msg_lifetime>
            ) -> Result<Self, ircv3_parse::DeError> {
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

/// Derives `ToMessage` implementation for structs to serialize IRC messages.
///
/// # Attributes
///
/// ## Struct-level
///
/// - `#[irc(command = "COMMAND")]` - Sets the default command for this message type
/// - `#[irc(crlf)]` - Explicitly appends `\r\n` at the end of the message
///
/// ## Field-level
///
/// ### Tags
/// - `#[irc(tag)]` - Serializes field as tag using the field name as key
/// - `#[irc(tag = "key")]` - Serializes field as tag with custom key
///
/// ### Tag Flags
/// - `#[irc(tag_flag)]` - Serializes boolean field as tag flag using field name as key
/// - `#[irc(tag_flag = "key")]` - Serializes boolean field as tag flag with custom key
///
/// ### Source
/// - `#[irc(source)]` - Serializes field as source name component (`source = "name"`)
/// - `#[irc(source = "name|user|host")]` - Serializes field as source component
///     - **Note**: `name` is **required** when using `user` or `host`
///
/// ### Parameters
/// - `#[irc(param)]` - Serializes field as a middle parameter
/// - `#[irc(params)]` - Serializes field as multiple middle parameters
/// - `#[irc(param = N)]` - Serializes field as a middle parameter
///     - **Note**: The index `N` is ignored during serialization
///     - `FromMessage` uses the index to extract the Nth parameter
///     - `ToMessage` always serializes fields in declaration order
///
/// ### Trailing Parameter
/// - `#[irc(trailing)]` - Serializes field as the trailing parameter
///
/// ### Command
/// - `#[irc(command)]` - Serializes field as the IRC command
/// - `#[irc(command = "COMMAND")]` - Uses the specified command string
///   - If field-level `command` is set, struct-level `command` is ignored
///   - If multiple `command` attributes exist, the last one takes precedence
///
#[proc_macro_derive(ToMessage, attributes(irc))]
pub fn derive_to_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    ser::derive_to_message_impl(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
