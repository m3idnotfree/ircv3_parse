use syn::Ident;

pub fn cannot_be_empty(attribute: &str) -> String {
    format!("`{attribute}` value cannot be empty")
}

pub fn duplicate_attribute(attribute: &str) -> String {
    format!("duplicate `{attribute}` attribute")
}

pub fn first_defined_here(attribute: &str) -> String {
    format!("first `{attribute}` defined here")
}

pub fn required_value(component: &str) -> String {
    format!("`{component}` requires a value")
}

pub fn unknown_irc_attribute(token_stream: proc_macro2::TokenStream) -> String {
    format!("unknown `#[irc]` attribute `{token_stream}`")
}

pub fn invalid_source_field() -> &'static str {
    "invalid source field (valid options: `name`, `user`, or `host`)"
}

pub fn multiple_extraction_attributes(first: &str, second: &str) -> String {
    format!(
        "field cannot have multiple extraction attributes (found both `{first}` and `{second}`)"
    )
}

pub fn cannot_have_a_value_command() -> &'static str {
    "field-level #[irc(command)] cannot have a value (use struct-level #[irc(command = \"CMD\" for validation instead)])"
}

pub fn unit_struct_requires_at_least_one(name: &Ident) -> String {
    format!("unit struct `{name}` requires at least one IRC attribute")
}

pub fn duplicate_unit_struct_attribute(first: &str, second: &str) -> String {
    format!(
        "unit struct can only have one of `tag`, `tag_flag`, `source`, \
        `param`, `params` or `trailing` (found both `{first}` and `{second}`)"
    )
}

pub fn command_field_cannot_be_option() -> &'static str {
    "command field cannot be Option<&str> or Option<String> (use &str or String instead)"
}

pub fn unsupported_type(component: &str, field: &Ident, ty: proc_macro2::TokenStream) -> String {
    let ty_str = ty.to_string().replace(" ", "");
    format!("unsupported type for {component} field `{field}`: {ty_str}")
}

pub fn unsupported_unnamed_type(
    component: &str,
    idx: usize,
    ty: proc_macro2::TokenStream,
) -> String {
    let ty_str = ty.to_string().replace(" ", "");
    format!("unsupported type for {component} field `{idx}`: {ty_str}")
}

pub fn nested_field_requires_attribute(field: &Ident) -> String {
    format!("field `{field}` requires an IRC attribute for ToMessage serialization")
}
