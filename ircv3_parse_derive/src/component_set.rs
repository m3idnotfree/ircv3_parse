use quote::quote;

#[derive(Default)]
pub struct ComponentSet {
    tags: bool,
    source: bool,
    params: bool,
    command: bool,
}

impl ComponentSet {
    pub fn add_tags(&mut self) {
        self.tags = true;
    }

    pub fn add_source(&mut self) {
        self.source = true;
    }

    pub fn add_params(&mut self) {
        self.params = true;
    }

    pub fn add_command(&mut self) {
        self.command = true;
    }

    pub fn expand(&self) -> Vec<proc_macro2::TokenStream> {
        let mut result = Vec::new();

        if self.tags {
            result.push(
                quote! { let tags = msg.tags().ok_or_else(|| ircv3_parse::DeError::tags_component_not_found())?; },
            );
        }

        if self.source {
            result
                .push(quote! { let source = msg.source().ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?; });
        }

        if self.command {
            result.push(quote! { let command = msg.command(); });
        }

        if self.params {
            result.push(quote! { let params = msg.params(); });
        }

        result
    }
}
