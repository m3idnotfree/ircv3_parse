use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, LitStr, Result};

use crate::attr::StructAttrs;

struct SerializeCommand(Option<TokenStream>);

impl SerializeCommand {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn set(&mut self, code: TokenStream) {
        self.0 = Some(code);
    }

    pub fn expand(self, struct_command: Option<&LitStr>) -> TokenStream {
        if let Some(field_code) = self.0 {
            field_code
        } else if let Some(cmd) = struct_command {
            quote! {
                { serialize.set_command(ircv3_parse::Commands::from(#cmd)); }
            }
        } else {
            quote! {}
        }
    }
}

struct Source {
    name: Option<TokenStream>,
    user: Option<TokenStream>,
    host: Option<TokenStream>,
}

impl Source {
    pub fn new() -> Self {
        Self {
            name: None,
            user: None,
            host: None,
        }
    }

    pub fn set_name(&mut self, code: TokenStream) {
        self.name = Some(code);
    }

    pub fn set_user(&mut self, code: TokenStream) {
        self.user = Some(code);
    }

    pub fn set_host(&mut self, code: TokenStream) {
        self.host = Some(code);
    }

    #[inline]
    pub fn has_components(&self) -> bool {
        self.name.is_some() || self.user.is_some() || self.host.is_some()
    }

    fn validate(&self, input: &DeriveInput) -> Result<()> {
        if self.name.is_none() && (self.user.is_some() || self.host.is_some()) {
            return Err(Error::new_spanned(
                &input.ident,
                "source `name` field is required when using `user` or `host` fields",
            ));
        }
        Ok(())
    }

    pub fn expand(self, input: &DeriveInput) -> Result<TokenStream> {
        if !self.has_components() {
            return Ok(quote! {});
        }

        self.validate(input)?;

        let name = self.name;
        let user = self.user;
        let host = self.host;

        Ok(quote! {
            {
                let  source = serialize.source();
                #name
                #user
                #host
            }
        })
    }
}

pub struct SerializationBuilder<'a> {
    tag_fields: Vec<TokenStream>,
    custom_tags: Vec<TokenStream>,
    source: Source,
    custom_source: Vec<TokenStream>,
    params: Vec<TokenStream>,
    custom_params: Vec<TokenStream>,
    trailing: Option<TokenStream>,
    custom_trailing: Vec<TokenStream>,
    field_command: SerializeCommand,
    struct_attrs: &'a StructAttrs,
}

impl<'a> SerializationBuilder<'a> {
    pub fn new(struct_attrs: &'a StructAttrs) -> Self {
        Self {
            tag_fields: Vec::new(),
            custom_tags: Vec::new(),
            source: Source::new(),
            custom_source: Vec::new(),
            params: Vec::new(),
            custom_params: Vec::new(),
            trailing: None,
            custom_trailing: Vec::new(),
            field_command: SerializeCommand::new(),
            struct_attrs,
        }
    }

    pub fn tag(&mut self, code: TokenStream) {
        self.tag_fields.push(code);
    }

    pub fn custom_tag(&mut self, code: TokenStream) {
        self.custom_tags.push(code);
    }

    pub fn field_command(&mut self, code: TokenStream) {
        self.field_command.set(code);
    }

    pub fn set_source_name(&mut self, code: TokenStream) {
        self.source.set_name(code);
    }

    pub fn set_source_user(&mut self, code: TokenStream) {
        self.source.set_user(code);
    }

    pub fn set_source_host(&mut self, code: TokenStream) {
        self.source.set_host(code);
    }

    pub fn custom_source(&mut self, code: TokenStream) {
        self.custom_source.push(code);
    }

    pub fn params_push(&mut self, code: TokenStream) {
        self.params.push(code);
    }

    pub fn custom_params(&mut self, code: TokenStream) {
        self.custom_params.push(code);
    }

    pub fn set_trailing(&mut self, code: TokenStream) {
        self.trailing = Some(code);
    }

    pub fn custom_trailing(&mut self, code: TokenStream) {
        self.custom_trailing.push(code);
    }

    pub fn expand(self, input: &DeriveInput) -> Result<TokenStream> {
        let tags_code = if !self.tag_fields.is_empty() {
            let tags = self.tag_fields;
            quote! {
                let tags = serialize.tags();
                #(#tags)*
            }
        } else {
            quote! {}
        };

        let source_code = self.source.expand(input)?;
        let command_code = self.field_command.expand(self.struct_attrs.command());
        let params_code = if !self.params.is_empty() {
            let p = &self.params;
            quote! {
                let params = serialize.params();
                #(#p)*
            }
        } else {
            quote! {}
        };

        let trailing_code = self.trailing.unwrap_or_default();

        let nested_tags = self.custom_tags;
        let nested_source = self.custom_source;
        let nested_params = self.custom_params;
        let nested_trailing = self.custom_trailing;

        let struct_name = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let crlf_expand = self.struct_attrs.expand_crlf();
        Ok(quote! {
            impl #impl_generics ircv3_parse::ser::ToMessage
                for #struct_name #ty_generics #where_clause
            {
                fn to_message<S: ircv3_parse::ser::MessageSerializer>(
                    &self,
                    serialize: &mut S
                ) -> Result<(), ircv3_parse::IRCError> {

                    #(#nested_tags)*
                    #tags_code

                    #(#nested_source)*
                    #source_code

                    #command_code

                    #(#nested_params)*
                    #params_code

                    #(#nested_trailing)*
                    #trailing_code

                    #crlf_expand
                    Ok(())
                }
            }
        })
    }
}
