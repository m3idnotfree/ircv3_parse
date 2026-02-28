mod ast;
mod attr;
mod component_set;
mod error_msg;
mod expand;
mod type_check;
mod valid;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

/// Derives `FromMessage` implementation for structs and enums
///
/// ## Struct-level
///
/// - `#[irc(command = "COMMAND")]` - Validates command matches
///
/// ## Field-level
///
/// ### Tag Extraction
/// - `#[irc(tag)]` - Extract tag value using field name as key
/// - `#[irc(tag = "key")]` - Extract tag value with custom key
///
/// ### Tag Flag Extraction
/// - `#[irc(tag_flag)]` - Extract tag flag using field name as key (returns `bool`)
/// - `#[irc(tag_flag = "key")]` - Extract tag flag with custom key (returns `bool`)
///
/// ### Source Extraction
/// - `#[irc(source)]` - Extract source `name` component
/// - `#[irc(source = "name|user|host")]` - Extract source component
///
/// ### Parameter Extraction
/// - `#[irc(param)]` - Extract first parameter (index 0)
/// - `#[irc(param = N)]` - Extract parameter at index N
/// - `#[irc(params)]` - Extract all parameter into a `Vec`
///
/// ### Trailing Parameter
/// - `#[irc(trailing)]` - Extract trailing parameter
///
/// ### Command Extraction
/// - `#[irc(command)]` - Extract command value
///
/// **Note**: Field-level `#[irc(command)]` cannot have a value. Use struct-level
/// `#[irc(command = "COMMAND")]` for validation. Both can be used together:
/// struct-level validates, field-level extracts.
///
/// ### Custom Extraction
/// - `#[irc(with = "function")]` - Use custom extraction function
///
/// ### Default Value
///
/// - `#[irc(tag = "key", default)]` - Uses `Default::default()` when the component is absent
/// - `#[irc(tag = "key", default = "function")]` - Calls `function()` when the component is absent
///
/// **`Option<T>` fields**: `default` is ignored. `Option<T>` already returns `None`
/// when the component is absent, regardless of whether `default` is present.
///
/// **Trailing**: When `default` is used with `trailing`, an empty trailing parameter
/// (`:` with no content) is treated as absent and the default value is used.
///
/// **Constraints**:
/// - `default` requires a component attribute (`tag`, `source`, `param`, etc.)
/// - `default` cannot be specified more than once
///
/// ## Enum-level
///
/// Exactly one of the following is required on the enum itself:
///
/// - `#[irc(tag = "key")]` - Extract tag value
/// - `#[irc(tag_flag = "key")]` - Boolean flag match (see **Tag Flag Enum** below)
/// - `#[irc(source = "name|user|host")]` - Extract source component
/// - `#[irc(param = N)]` - Extract parameter at index N
/// - `#[irc(trailing)]` - Extract trailing parameter
/// - `#[irc(command)]` - Match the command
///
/// ### Default variant
/// - `#[irc(default = "VariantNamea")]` - appended to fall back
///
/// ### Rename
/// - `#[irc(rename = "lowercase|UPPERCASE|kebab-case")]` - controls how
///   variant names are converted to match strings.
///
/// **Note**: The default is `lowercase` for all components except `command`, which defaults to `UPPERCASE`.
///
/// `rename` is not allowed on `command` enums. Use `#[irc(value = "...")]` on
/// individual variants instead.
///
/// ### Variant-level
///
/// - `#[irc(value = "pattern")]` - Use `pattern` as the match string instead of
///   the renamed variant name. Multiple `value` attributes can be specified on a
///   single variant to match multiple strings:
///   When `#[irc(value)]` is present, `rename` is ignored for that variant.
///
/// ### Tag Flag Enum
///
/// `tag_flag` enums must have **exactly 2 variants**. Exactly one variant must carry
///
/// - `#[irc(present)]` - marks the "flag is set" variant (required, exactly once)
/// - `#[irc(value)]` is not allowed on `tag_flag` variants
///
/// ## Nested Types
///
/// Fields can use custom types that implement `FromMessage`. The macro will
/// automatically call `<Type>::from_message()` for custom types.
///
/// **Behavior:**
/// - If a field has **no attribute**, the macro calls `<Type>::from_message()`
/// - If a field has an attribute but the type is **not** `&str`, `String`,
///   `Option<&str>`, or `Option<String>`, the attribute is **ignored** and
///   `<Type>::from_message()` is called instead
///
#[proc_macro_derive(FromMessage, attributes(irc))]
pub fn derive_from_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive_from_message(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
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
///
///   **Note**: `name` is **required** when using `user` or `host`
///
/// ### Parameters
/// - `#[irc(param)]` - Serializes field as a middle parameter
/// - `#[irc(params)]` - Serializes field as multiple middle parameters
/// - `#[irc(param = N)]` - Serializes field as a middle parameter
///
///   **Note**: The index `N` is ignored during serialization
///     - `FromMessage` uses the index to extract the Nth parameter
///     - `ToMessage` always serializes fields in declaration order
///
/// ### Trailing Parameter
/// - `#[irc(trailing)]` - Serializes field as the trailing parameter
///
/// ### Command
/// - `#[irc(command)]` - Serializes field value as the IRC command
///
///   **Priority**: Field-level `#[irc(command)]` takes precedence over struct-level
///   `#[irc(command = "COMMAND")]`. If both are present, the field value is used.
///   If only struct-level is set, that value is used as the default command.
///
#[proc_macro_derive(ToMessage, attributes(irc))]
pub fn derive_to_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    expand::derive_to_message(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
