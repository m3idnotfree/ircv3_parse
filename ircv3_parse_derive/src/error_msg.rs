use syn::Ident;

pub fn duplicate_attribute(attribute: &str) -> String {
    format!("duplicate `{attribute}` attribute")
}

pub fn unknown_irc_attribute(token_stream: proc_macro2::TokenStream) -> String {
    format!("unknown IRC attribute: {token_stream}")
}

pub fn unsupported_type(component: &str, field: &Ident, ty: proc_macro2::TokenStream) -> String {
    let ty_str = ty.to_string().replace(" ", "");
    format!("unsupported type for {component} field `{field}`: {ty_str}")
}
