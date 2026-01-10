use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;
use syn::{DeriveInput, Error, Result};

use crate::extract_named_fields;
use crate::FieldAttribute;
use crate::StructAttribute;

pub fn derive_to_message_impl(input: DeriveInput) -> Result<TokenStream> {
    let fields = extract_named_fields(&input, "ToMessage")?;

    let mut errors = Vec::new();
    let mut builder = SerializationBuilder::new(&input)?;

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

        match attribute.expand_de(field, field_name, &mut builder) {
            Ok(_) => {}
            Err(e) => {
                errors.push(e);
            }
        };
    }

    if let Some(e) = crate::combine_errors(errors) {
        return Err(e);
    }

    builder.expand(&input)
}

struct SerializeCommand(Option<LitStr>);

impl SerializeCommand {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn set(&mut self, code: Option<LitStr>) {
        self.0 = code;
    }

    pub fn expend(self, struct_command: &Option<LitStr>) -> TokenStream {
        self.0
            .as_ref()
            .or(struct_command.as_ref())
            .map(|cmd| {
                quote! {
                    { serialize.command(ircv3_parse::Commands::from(#cmd)); }
                }
            })
            .unwrap_or_default()
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
                "source `name` field is required when using `user` or `host fields",
            ));
        }
        Ok(())
    }

    pub fn expend(self, input: &DeriveInput) -> Result<TokenStream> {
        if !self.has_components() {
            return Ok(quote! {});
        }

        self.validate(input)?;

        let name = self.name;
        let user = self.user;
        let host = self.host;

        Ok(quote! {
            {
                use ircv3_parse::message::ser::SerializeSource;
                #name
                #user
                #host
                source.end();
            }
        })
    }
}

pub struct SerializationBuilder {
    tag_fields: Vec<TokenStream>,
    custom_tags: Vec<TokenStream>,
    source: Source,
    custom_source: Vec<TokenStream>,
    params: Vec<TokenStream>,
    custom_params: Vec<TokenStream>,
    trailing: Option<TokenStream>,
    custom_trailing: Vec<TokenStream>,
    field_command: SerializeCommand,
    struct_attrs: StructAttribute,
}

impl SerializationBuilder {
    pub fn new(input: &DeriveInput) -> Result<Self> {
        let struct_attrs = StructAttribute::parse(input)?;
        Ok(Self {
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
        })
    }

    pub fn tag(&mut self, code: TokenStream) {
        self.tag_fields.push(code);
    }

    pub fn custom_tag(&mut self, code: TokenStream) {
        self.custom_tags.push(code);
    }

    pub fn field_command(&mut self, code: Option<LitStr>) {
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
                {
                    use ircv3_parse::message::ser::SerializeTags;
                    let mut tags = serialize.tags();
                    #(#tags)*
                    tags.end();
                }
            }
        } else {
            quote! {}
        };

        let source_code = self.source.expend(input)?;
        let command_code = self.field_command.expend(self.struct_attrs.command());
        let params_code = if !self.params.is_empty() {
            let p = &self.params;
            quote! {
                {
                    use ircv3_parse::message::ser::SerializeParams;
                    let mut params = serialize.params();
                    #(#p)*
                    params.end();

                }
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
            impl #impl_generics ircv3_parse::message::ser::ToMessage
                for #struct_name #ty_generics #where_clause
            {
                fn to_message<S: ircv3_parse::message::ser::MessageSerializer>(
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
