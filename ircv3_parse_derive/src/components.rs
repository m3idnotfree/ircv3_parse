use quote::quote;

#[derive(Default)]
pub struct MessageComponents {
    tags: bool,
    source: bool,
    params: bool,
    command: bool,
}

impl MessageComponents {
    pub fn mark_tags(&mut self) {
        self.tags = true;
    }

    pub fn mark_source(&mut self) {
        self.source = true;
    }

    pub fn mark_params(&mut self) {
        self.params = true;
    }

    pub fn mark_command(&mut self) {
        self.command = true;
    }

    pub fn expand(&self) -> Vec<proc_macro2::TokenStream> {
        let mut result = Vec::new();

        if self.tags {
            result.push(
                quote! { let tags = msg.tags().ok_or(ircv3_parse::DeError::missing_tags())?; },
            );
        }

        if self.source {
            result
                .push(quote! { let source = msg.source().ok_or(ircv3_parse::DeError::missing_source())?; });
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
