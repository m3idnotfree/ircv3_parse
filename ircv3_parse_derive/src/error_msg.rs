use syn::Ident;

pub fn cannot_be_empty(attribute: &str) -> String {
    format!("`{attribute}` value cannot be empty")
}

pub fn duplicate_attribute(attribute: &str) -> String {
    format!("duplicate `{attribute}`")
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
    "field-level `#[irc(command)]` cannot have a value (use struct-level `#[irc(command = \"CMD\")]` for validation instead)"
}

pub fn cannot_have_value(attribute: &str) -> String {
    format!("`{attribute}` does not take a value")
}

pub fn present_only_for_tag_flag() -> &'static str {
    "#[irc(present)] is only valid on `tag_flag` enum variants"
}

pub fn tag_flag_enum_requires() -> &'static str {
    "tag_flag enum requires exactly 2 variants"
}

pub fn value_not_allowed_on_tag_flag() -> &'static str {
    "#[irc(value)] is not allowed on `tag_flag` enum variants, use #[irc(present)] instead"
}

pub fn tag_flag_requires_present() -> &'static str {
    "tag_flag enum requires exactly one variant marked with #[irc(present)]"
}

pub fn tag_flag_absent_must_be_unit() -> &'static str {
    "the absent variant of a `tag_flag` enum must be a unit variant (no fields)"
}

pub fn default_variant_must_be_unit(name: &str) -> String {
    format!("default variant `{name}` must be a unit variant (no fields)")
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

pub fn default_requires_component() -> &'static str {
    "`default` can only be used together with a component attribute \
    (`tag`, `tag_flag`, `source`, `param`, `params`, or `trailing`)"
}

pub fn default_variant_not_found(name: &str) -> String {
    format!("default variant `{name}` not found in enum")
}

pub fn command_field_cannot_be_option() -> &'static str {
    "command field cannot be Option<&str> or Option<String> (use &str or String instead)"
}

pub fn rename_not_allowed_with_command() -> &'static str {
    "`rename` is not allowed with `command` (commands are always matched as uppercase)"
}

pub fn unknown_rename_rule(other: &str) -> String {
    format!("unknown rename rule `{other}` (valid: lowercase, UPPERCASE, or kebab-case)")
}

pub fn enum_requires_component() -> &'static str {
    "enum `#[irc(...)]` requires a component attribute \
    (`tag`, `tag_flag`, `source`, `param`, `trailing`, or `command`)"
}

pub fn value_not_supported_for(kind: &str) -> String {
    format!("`value` is not supported for `{kind}`")
}

pub fn value_requires_component() -> &'static str {
    "`value` requires a component attribute \
        (`tag`, `tag_flag`, `source`,`param`, or `trailing`)"
}

pub fn pick_must_follow_value() -> &'static str {
    "`pick` must immediately follow a `value` in the same `#[irc(...)]` attribute"
}

pub fn pick_required_for_multiple_values() -> &'static str {
    "variant with multiple `#[irc(value)]` attributes requires exactly one `#[irc(value = \"...\", pick)]`"
}

pub fn pick_not_needed_for_single_value() -> &'static str {
    "`pick` is not needed when there is only one `#[irc(value)]`"
}

pub fn no_field_irc_attrs_requires_single_unnamed() -> &'static str {
    "a variant with no field-level `#[irc(...)]` attributes must have exactly one unnamed field"
}

pub fn skip_none_requires_tag_option() -> &'static str {
    "`skip_none` is only allowed on `#[irc(tag = \"...\")]` fields \
        with `Option<String>` or `Option<&str>` type"
}
