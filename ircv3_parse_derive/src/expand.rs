use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, Ident, LitStr, Result};

use crate::{
    ast::{Enum, Field, FieldStruct, Input, Struct, UnitStruct, Variant, VariantFields},
    attr::{
        EnumAttrs, EnumKind, FieldAttrs, FieldDefault, FieldKind, Rename, Source, StructAttrs,
        UnitStructAttrs,
    },
    component_set::ComponentSet,
    ser::SerializationBuilder,
    type_check::TypeKind,
};

pub fn derive_from_message(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node, "FromMessage")?;
    input.validate()?;

    match input {
        Input::Struct(input) => Ok(input.expand_de()),
        Input::Enum(input) => Ok(input.expand_de()),
    }
}

pub fn derive_to_message(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node, "ToMessage")?;

    input.validate()?;
    input.validate_ser()?;

    match input {
        Input::Struct(input) => input.expand_ser(node),
        Input::Enum(input) => Err(Error::new_spanned(
            input.ident,
            "ToMessage only supports structs",
        )),
    }
}

impl<'a> Struct<'a> {
    pub fn expand_de(&self) -> TokenStream {
        match self {
            Struct::Unit(input) => input.expand_de(),
            Struct::Unnamed(input) => input.expand_de(false),
            Struct::Named(input) => input.expand_de(true),
        }
    }

    pub fn expand_ser(&self, node: &DeriveInput) -> Result<TokenStream> {
        match self {
            Struct::Unit(input) => Err(Error::new_spanned(
                input.ident,
                "ToMessage only supports named structs",
            )),
            Struct::Unnamed(input) => Err(Error::new_spanned(
                input.ident,
                "ToMessage only supports named structs",
            )),
            Struct::Named(input) => input.expand_ser(node),
        }
    }
}

impl<'a> Enum<'a> {
    pub fn expand_de(&self) -> TokenStream {
        let mut components = ComponentSet::default();

        self.add_to(&mut components);

        let body = if let EnumKind::TagFlag(_) = &self.attrs.kind {
            self.expand_tag_flag_de()
        } else {
            self.expand_match_de()
        };

        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();
        let msg_lifetime = self.generics.msg_lifetime();

        let setup = components.expand();

        quote! {
            impl #impl_generics ircv3_parse::message::de::FromMessage<#msg_lifetime>
                for #name #ty_generics #where_clause
            {
                fn from_message(
                    msg: &ircv3_parse::Message<#msg_lifetime>
                ) -> Result<Self, ircv3_parse::DeError> {
                    #(#setup)*

                    #body
                }
            }
        }
    }

    fn expand_tag_flag_de(&self) -> TokenStream {
        let present = self
            .variants
            .iter()
            .find(|v| v.attrs.present.is_some())
            .expect("validation ensures exactly one variant has #[irc(present)]");
        let absent = self
            .variants
            .iter()
            .find(|v| v.attrs.present.is_none())
            .expect("validation ensures one absent variant");

        let present_ident = present.ident;
        let present_fields = present.fields.expand_fields();
        let absent_ident = absent.ident;
        let absent_fields = absent.fields.expand_fields();

        let default_arm = self.find_default_arm();
        let setup = self.attrs.expand_value_setup(default_arm.as_ref());

        quote! {
            #setup
            if value {
                Ok(Self::#present_ident #present_fields)
            } else {
                Ok(Self::#absent_ident #absent_fields)
            }
        }
    }

    fn expand_match_de(&self) -> TokenStream {
        let default_arm = self.find_default_arm();
        let setup = self.attrs.expand_value_setup(default_arm.as_ref());
        let arms = self.expand_arms();
        let catch_all = self.expand_catch_all(default_arm);

        quote! {
            #setup
            match value {
                #(#arms,)*
                _ => #catch_all
            }
        }
    }

    fn expand_arms(&self) -> Vec<TokenStream> {
        let rename = &self.attrs.rename;
        self.variants.iter().map(|v| v.expand_arm(rename)).collect()
    }

    fn add_to(&self, components: &mut ComponentSet) {
        self.attrs.add_to(components);
        self.variants.iter().for_each(|v| v.add_to(components));
    }

    fn expected_values(&self) -> String {
        let rename = &self.attrs.rename;

        self.variants
            .iter()
            .map(|variant| {
                if variant.attrs.values.is_empty() {
                    rename.apply(&variant.ident.to_string())
                } else {
                    variant
                        .attrs
                        .values
                        .iter()
                        .map(|lit| lit.value())
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn find_default_arm(&self) -> Option<TokenStream> {
        let default = self.attrs.default.as_ref();
        self.variants
            .iter()
            .find_map(|v| v.expand_default_arm(default))
    }

    fn expand_catch_all(&self, default_arm: Option<TokenStream>) -> TokenStream {
        let component = self.attrs.kind.name();
        let expected = self.expected_values();

        if let Some(default) = default_arm {
            default
        } else {
            quote! {
                Err(ircv3_parse::DeError::not_found_variant(
                    #component,
                    value,
                    #expected,
                ))
            }
        }
    }
}

impl<'a> UnitStruct<'a> {
    pub fn expand_de(&self) -> TokenStream {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();
        let msg_lifetime = self.generics.msg_lifetime();

        let impl_body = self.attrs.expand_de();

        quote! {
            impl #impl_generics ircv3_parse::message::de::FromMessage<#msg_lifetime>
                for #name #ty_generics #where_clause
            {
                fn from_message(
                    msg: &ircv3_parse::Message<#msg_lifetime>
                ) -> Result<Self, ircv3_parse::DeError> {
                    #impl_body
                }
            }
        }
    }
}

impl<'a> FieldStruct<'a> {
    pub fn expand_de(&self, is_named: bool) -> TokenStream {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();
        let msg_lifetime = self.generics.msg_lifetime();

        let validation = self.attrs.expand_command_check();
        let components = self.components();
        let setup_code = components.expand();

        let fields: Vec<_> = self.fields.iter().map(|f| f.expand_de()).collect();

        let impl_body = if is_named {
            quote! {
                Ok(Self {
                    #(#fields),*
                })
            }
        } else {
            quote! {
                Ok(Self (
                    #(#fields),*
                ))
            }
        };

        quote! {
            impl #impl_generics ircv3_parse::message::de::FromMessage<#msg_lifetime>
                for #name #ty_generics #where_clause
            {
                fn from_message(
                    msg: &ircv3_parse::Message<#msg_lifetime>
                ) -> Result<Self, ircv3_parse::DeError> {
                    #validation
                    #(#setup_code)*

                    #impl_body
                }
            }
        }
    }

    pub fn expand_ser(&self, input: &DeriveInput) -> Result<TokenStream> {
        self.validate_ser()?;

        let mut builder = SerializationBuilder::new(&self.attrs);

        for field in &self.fields {
            field.expand_ser(&mut builder);
        }

        builder.expand(input)
    }
}

impl UnitStructAttrs {
    pub fn expand_de(&self) -> TokenStream {
        let mut components = ComponentSet::default();

        if self.command.is_some() {
            components.add_command();
        }
        if let Some(check) = &self.check {
            check.add_to(&mut components);
        }

        let setup_code = components.expand();

        let command_check = if let Some(cmd) = &self.command {
            quote! {
                if msg.command() != #cmd {
                    return Err(ircv3_parse::DeError::command_mismatch(
                        #cmd,
                        msg.command().as_str()
                    ));
                }
            }
        } else {
            quote! {}
        };

        let component_check = self
            .check
            .as_ref()
            .map(FieldKind::expand_component_check)
            .unwrap_or_default();

        quote! {
            #(#setup_code)*
            #command_check
            #component_check
            Ok(Self)
        }
    }
}

impl<'a> Variant<'a> {
    pub fn expand_arm(&self, rename: &Rename) -> TokenStream {
        let ident = &self.ident;
        let fields = self.fields.expand_fields();

        if self.attrs.values.is_empty() {
            let pattern = rename.apply(&self.ident.to_string());
            quote! { #pattern => Ok(Self::#ident #fields) }
        } else {
            let patterns = &self.attrs.values;
            quote! { #(#patterns)|* => Ok(Self::#ident #fields) }
        }
    }

    pub fn expand_default_arm(&self, default: Option<&LitStr>) -> Option<TokenStream> {
        let is_default = default.is_some_and(|d| d.value() == *self.ident.to_string());

        let ident = &self.ident;
        if is_default {
            let body = self.fields.expand_field_default();
            Some(quote! { Ok(Self::#ident #body )})
        } else {
            None
        }
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        self.fields.add_to(components);
    }
}

impl StructAttrs {
    pub fn expand_command_check(&self) -> TokenStream {
        if let Some(cmd) = &self.command {
            quote! {
                if msg.command() != #cmd {
                    return Err(ircv3_parse::DeError::command_mismatch(
                        #cmd,
                        msg.command().as_str()
                    ));
                }
            }
        } else {
            quote! {}
        }
    }

    pub fn expand_crlf(&self) -> proc_macro2::TokenStream {
        if self.crlf {
            quote! { serialize.end()?; }
        } else {
            quote! {}
        }
    }
}

impl EnumAttrs {
    pub fn expand_value_setup(&self, default_arm: Option<&TokenStream>) -> TokenStream {
        match &self.kind {
            EnumKind::Tag(key) => {
                if let Some(default) = default_arm {
                    quote! {
                        let tags = match msg.tags() {
                            Some(tags) => tags,
                            None => return #default,
                        };
                        let value = match tags.get(#key) {
                            Some(value) => value.as_str(),
                            None => return #default,
                        };
                    }
                } else {
                    quote! {
                        let value = tags.get(#key)
                            .ok_or_else(|| ircv3_parse::DeError::not_found_tag(#key))?
                            .as_str();
                    }
                }
            }
            EnumKind::TagFlag(key) => {
                if let Some(default) = default_arm {
                    quote! {
                        let tags = match msg.tags() {
                            Some(tags) => tags,
                            None => return #default,
                        };
                        let value = tags.get_flag(#key);
                    }
                } else {
                    quote! {
                        let value = tags.get_flag(#key);
                    }
                }
            }
            EnumKind::Source(component) => match component {
                Source::Name => {
                    if let Some(default) = default_arm {
                        return quote! {
                            let source = match msg.source() {
                                Some(s) => s,
                                None => return #default,
                            };
                            let value = source.name;
                        };
                    }
                    quote! { let value = source.name; }
                }
                Source::User | Source::Host => {
                    let field = match component {
                        Source::User => quote! { user },
                        Source::Host => quote! { host },
                        _ => unreachable!(),
                    };

                    if let Some(default) = default_arm {
                        return quote! {
                            let value = match msg.source().and_then(|s| s.#field) {
                                Some(value) => value,
                                None => return #default,
                            };
                        };
                    }
                    quote! {
                        let value = source.#field
                            .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?;
                    }
                }
            },
            EnumKind::Param(idx) => {
                if let Some(default) = default_arm {
                    quote! {
                        let value = match params.middles.iter().nth(#idx) {
                            Some(value) => value,
                            None => return #default,
                        };
                    }
                } else {
                    quote! {
                        let value = params.middles.iter().nth(#idx)
                            .ok_or_else(|| ircv3_parse::DeError::not_found_param(#idx))?;
                    }
                }
            }
            EnumKind::Trailing => {
                if let Some(default) = default_arm {
                    quote! {
                        let value = match params.trailing.raw() {
                            Some(value) => value,
                            None => return #default
                        };
                    }
                } else {
                    quote! {
                        let value = params.trailing.as_str();
                    }
                }
            }
            EnumKind::Command => {
                quote! {
                    let value = msg.command().as_str();
                }
            }
        }
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        if self.default.is_some() {
            match self.kind {
                EnumKind::Tag(_) | EnumKind::TagFlag(_) => return,
                EnumKind::Source(_) => return,
                _ => {}
            }
        }

        self.kind.add_to(components);
    }
}

impl<'a> VariantFields<'a> {
    pub fn expand_fields(&self) -> TokenStream {
        match self {
            Self::Unit => quote! {},
            Self::Named(fields) => {
                let body: Vec<_> = fields.iter().map(|f| f.expand_de()).collect();
                quote! { {#(#body),*} }
            }
            Self::Unnamed(fields) => {
                if fields.len() == 1 {
                    let field = &fields[0];
                    use TypeKind::*;
                    match TypeKind::classify(&field.field.ty) {
                        Str => return quote! { (value) },
                        String => return quote! { (value.to_string()) },
                        _ => {
                            let ty = &field.field.ty;
                            return quote! {
                                 (<#ty>::from_message(&msg)?)
                            };
                        }
                    }
                }

                let body: Vec<_> = fields.iter().map(|f| f.expand_de()).collect();
                quote! { (#(#body),*) }
            }
        }
    }

    pub fn expand_field_default(&self) -> TokenStream {
        match self {
            Self::Unit => quote! {},
            Self::Named(fields) => {
                let body: Vec<_> = fields.iter().map(|f| f.expand_field_default()).collect();
                quote! { {#(#body),*} }
            }
            Self::Unnamed(fields) => {
                let body: Vec<_> = fields.iter().map(|f| f.expand_field_default()).collect();
                quote! { (#(#body),*) }
            }
        }
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        self.fields().iter().for_each(|f| f.add_to(components))
    }

    fn fields(&self) -> &[Field<'a>] {
        match self {
            Self::Unit => &[],
            Self::Named(fields) | Self::Unnamed(fields) => fields,
        }
    }
}

impl<'a> Field<'a> {
    pub fn expand_de(&self) -> TokenStream {
        self.attrs.expand_de(self.field)
    }

    pub fn expand_ser(&self, builder: &mut SerializationBuilder) {
        let field_name = &self.field.ident.clone().unwrap();
        self.attrs.expand_ser(self.field, field_name, None, builder);
    }

    pub fn expand_field_default(&self) -> TokenStream {
        if let Some(field_name) = &self.field.ident {
            quote! { #field_name: Default::default() }
        } else {
            quote! { Default::default() }
        }
    }
}

impl FieldAttrs {
    pub fn expand_de(&self, field: &syn::Field) -> TokenStream {
        if let Some(kind) = &self.kind {
            kind.expand_de(field, self.with.as_ref(), self.default.as_ref())
        } else if let Some(with_fn) = &self.with {
            expand_with(field, with_fn)
        } else {
            expand_nested(field)
        }
    }

    pub fn expand_ser(
        &self,
        field: &syn::Field,
        field_name: &Ident,
        with: Option<&LitStr>,
        builder: &mut SerializationBuilder,
    ) {
        if let Some(kind) = &self.kind {
            kind.expand_ser(field, field_name, with, builder)
        }
    }
}

impl FieldKind {
    fn expand_de(
        &self,
        field: &syn::Field,
        with: Option<&LitStr>,
        default: Option<&FieldDefault>,
    ) -> TokenStream {
        if let Some(with_fn) = with {
            return self.expand_with(field, with_fn);
        }

        let value = self.expand_value(field, default);
        if let Some(field_name) = &field.ident {
            quote! { #field_name: #value }
        } else {
            quote! { #value }
        }
    }

    fn expand_with(&self, field: &syn::Field, with_fn: &LitStr) -> TokenStream {
        let with_ident = Ident::new(&with_fn.value(), with_fn.span());
        let input = self.with_input();
        if let Some(field_name) = &field.ident {
            quote! { #field_name: #with_ident(#input) }
        } else {
            quote! { #with_ident(#input) }
        }
    }

    pub fn expand_ser(
        &self,
        field: &syn::Field,
        field_name: &Ident,
        _with: Option<&LitStr>,
        builder: &mut SerializationBuilder,
    ) {
        use TypeKind::*;

        match self {
            Self::Tag(key) => match TypeKind::classify(&field.ty) {
                Str => {
                    builder.tag(quote! {
                        tags.tag(#key, Some(self.#field_name))?;
                    });
                }
                String => {
                    builder.tag(quote! {
                        tags.tag(#key, Some(self.#field_name.as_ref()))?;
                    });
                }
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    builder.tag(quote! {
                        tags.tag(#key, self.#field_name)?;
                    });
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    builder.tag(quote! {
                        tags.tag(#key, self.#field_name.as_deref())?;
                    });
                }
                _ => {
                    builder.custom_tag(quote! {
                        self.#field_name.to_message(serialize)?;
                    });
                }
            },
            Self::TagFlag(key) => match TypeKind::classify(&field.ty) {
                Bool => {
                    builder.tag(quote! {
                        if self.#field_name {
                            tags.flag(#key)?;
                        }
                    });
                }
                Option(inner) if matches!(TypeKind::classify(inner), Bool) => {
                    builder.tag(quote! {
                        if let Some(true) = self.#field_name {
                            tags.flag(#key)?;
                        }
                    });
                }
                _ => {
                    builder.custom_tag(quote! {
                        self.#field_name.to_message(serialize)?;
                    });
                }
            },
            Self::Source(component) => match component {
                Source::Name => match TypeKind::classify(&field.ty) {
                    Str => {
                        builder.set_source_name(quote! {
                            source.name(self.#field_name)?;
                        });
                    }
                    String => {
                        builder.set_source_name(quote! {
                            source.name(self.#field_name.as_ref())?;
                        });
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                        builder.set_source_name(quote! {
                            if let Some(field) = self.#field_name {
                                source.name(field)?;
                            }
                        });
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), String) => {
                        builder.set_source_name(quote! {
                            if let Some(field) = &self.#field_name {
                                source.name(field.as_ref())?;
                            }
                        });
                    }
                    _ => {
                        builder.custom_source(quote! {
                            self.#field_name.to_message(serialize)?;
                        });
                    }
                },
                Source::User => match TypeKind::classify(&field.ty) {
                    Str => {
                        builder.set_source_user(quote! {
                            source.user(self.#field_name)?;
                        });
                    }
                    String => {
                        builder.set_source_user(quote! {
                            source.user(self.#field_name.as_ref())?;
                        });
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                        builder.set_source_user(quote! {
                            if let Some(field) = self.#field_name {
                                source.user(field)?;
                            }
                        });
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), String) => {
                        builder.set_source_user(quote! {
                            if let Some(field) = &self.#field_name {
                                source.user(field.as_ref())?;
                            }
                        });
                    }
                    _ => {
                        builder.custom_source(quote! {
                            self.#field_name.to_message(serialize)?;
                        });
                    }
                },
                Source::Host => match TypeKind::classify(&field.ty) {
                    Str => {
                        builder.set_source_host(quote! {
                            source.host(self.#field_name)?;
                        });
                    }
                    String => {
                        builder.set_source_host(quote! {
                            source.host(self.#field_name.as_ref())?;
                        });
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                        builder.set_source_host(quote! {
                            if let Some(field) = self.#field_name {
                                source.host(field)?;
                            }
                        });
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), String) => {
                        builder.set_source_host(quote! {
                            if let Some(field) = &self.#field_name {
                                source.host(field.as_ref())?;
                            }
                        });
                    }
                    _ => {
                        builder.custom_source(quote! {
                            self.#field_name.to_message(serialize)?;
                        });
                    }
                },
            },
            Self::Param(_idx) => match TypeKind::classify(&field.ty) {
                Str => {
                    builder.params_push(quote! {
                        params.push(self.#field_name)?;
                    });
                }
                String => {
                    builder.params_push(quote! {
                        params.push(self.#field_name.as_ref())?;
                    });
                }
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    builder.params_push(quote! {
                        if let Some(p) = self.#field_name {
                            params.push(p)?;
                        }
                    });
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    builder.params_push(quote! {
                        if let Some(p) = &self.#field_name {
                            params.push(p.as_ref())?;
                        }
                    });
                }
                _ => {
                    builder.custom_params(quote! {
                        self.#field_name.to_message(serialize)?;
                    });
                }
            },
            Self::Params => {
                builder.params_push(quote! {
                    for p in &self.#field_name {
                        params.push(p)?;
                    }
                });
            }
            Self::Trailing => match TypeKind::classify(&field.ty) {
                Str => {
                    builder.set_trailing(quote! {
                        serialize.trailing(self.#field_name)?;
                    });
                }
                String => {
                    builder.set_trailing(quote! {
                        serialize.trailing(self.#field_name.as_ref())?;
                    });
                }
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    builder.set_trailing(quote! {
                        if let Some(t) = self.#field_name {
                            serialize.trailing(t)?;
                        }
                    });
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    builder.set_trailing(quote! {
                        if let Some(t) = &self.#field_name {
                            serialize.trailing(t.as_ref())?;
                        }
                    });
                }
                _ => {
                    builder.custom_trailing(quote! {
                        self.#field_name.to_message(serialize)?;
                    });
                }
            },
            Self::Command => match TypeKind::classify(&field.ty) {
                Str => {
                    builder.field_command(quote! {
                        serialize.command(ircv3_parse::Commands::from(self.#field_name));
                    });
                }
                String => {
                    builder.field_command(quote! {
                        serialize.command(ircv3_parse::Commands::from(self.#field_name.as_ref()));
                    });
                }
                _ => {
                    builder.field_command(quote! {
                        self.#field_name.to_message(serialize)?;
                    });
                }
            },
        }
    }

    fn with_input(&self) -> TokenStream {
        match self {
            Self::Tag(key) => quote! { tags.get(#key).map(|s| s.as_str()) },
            Self::TagFlag(key) => quote! { tags.get_flag(#key) },
            Self::Source(component) => match component {
                Source::Name => quote! { source.name },
                Source::User => quote! { source.user },
                Source::Host => quote! { source.host },
            },
            Self::Param(idx) => quote! { params.middles.iter().nth(#idx) },
            Self::Params => quote! { params.middles.to_vec() },
            Self::Trailing => quote! { params.trailing.raw() },
            Self::Command => quote! { command.as_str() },
        }
    }

    fn expand_value(&self, field: &syn::Field, default: Option<&FieldDefault>) -> TokenStream {
        use TypeKind::*;
        match self {
            Self::Tag(key) => match TypeKind::classify(&field.ty) {
                Str => {
                    if let Some(d) = default {
                        let fallback = expand_default_fallback(d);
                        quote! {
                            msg.tags()
                                .and_then(|tags| tags.get(#key))
                                .map(|s| s.as_str())
                                .unwrap_or_else(|| #fallback)
                        }
                    } else {
                        quote! {
                            tags.get(#key)
                                .ok_or_else(|| ircv3_parse::DeError::not_found_tag(#key))?
                                .as_str()
                        }
                    }
                }
                String => {
                    if let Some(d) = default {
                        let fallback = expand_default_fallback(d);
                        quote! {
                            msg.tags()
                                .and_then(|tags| tags.get(#key))
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| #fallback)
                        }
                    } else {
                        quote! {
                            tags.get(#key)
                                .ok_or_else(|| ircv3_parse::DeError::not_found_tag(#key))?
                                .to_string()
                        }
                    }
                }
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    if default.is_some() {
                        quote! {
                            msg.tags()
                                .and_then(|tags| tags.get(#key))
                                .map(|s| s.as_str())
                        }
                    } else {
                        quote! { tags.get(#key).map(|s| s.as_str()) }
                    }
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    if default.is_some() {
                        quote! {
                            msg.tags()
                                .and_then(|tags| tags.get(#key))
                                .map(|s| s.to_string())
                        }
                    } else {
                        quote! { tags.get(#key).map(|s| s.to_string()) }
                    }
                }
                Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
                _ => expand_from_message(&field.ty, default),
            },
            Self::TagFlag(key) => match TypeKind::classify(&field.ty) {
                Bool => {
                    if default.is_some() {
                        quote! {
                            msg.tags()
                                .map(|tags| tags.get_flag(#key))
                                .unwrap_or_default()
                        }
                    } else {
                        quote! { tags.get_flag(#key) }
                    }
                }
                Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
                _ => expand_from_message(&field.ty, default),
            },
            Self::Source(component) => {
                let (accessor, is_opt, closure_accessor) = match component {
                    Source::Name => (
                        quote! { source.name },
                        false,
                        quote! { |source| source.name },
                    ),
                    Source::User => (
                        quote! { source.user },
                        true,
                        quote! { |source| source.user },
                    ),
                    Source::Host => (
                        quote! { source.host },
                        true,
                        quote! { |source| source.host },
                    ),
                };

                match TypeKind::classify(&field.ty) {
                    Str => {
                        if is_opt {
                            if let Some(d) = default {
                                let fallback = expand_default_fallback(d);
                                quote! {
                                    msg.source()
                                        .and_then(#closure_accessor)
                                        .unwrap_or_else(|| #fallback)
                                }
                            } else {
                                quote! {
                                    #accessor
                                        .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?
                                }
                            }
                        } else if let Some(d) = default {
                            let fallback = expand_default_fallback(d);
                            quote! {
                                msg.source()
                                    .map(#closure_accessor)
                                    .unwrap_or_else(|| #fallback)
                            }
                        } else {
                            quote! { #accessor }
                        }
                    }
                    String => {
                        if is_opt {
                            if let Some(d) = default {
                                let fallback = expand_default_fallback(d);
                                quote! {
                                    msg.source()
                                        .and_then(#closure_accessor)
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| #fallback)
                                }
                            } else {
                                quote! {
                                    #accessor
                                        .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?
                                        .to_string()
                                }
                            }
                        } else if let Some(d) = default {
                            let fallback = expand_default_fallback(d);
                            quote! {
                                msg.source()
                                    .map(|source| source.name.to_string())
                                    .unwrap_or_else(|| #fallback)
                            }
                        } else {
                            quote! { #accessor.to_string() }
                        }
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                        if default.is_some() {
                            if is_opt {
                                quote! { msg.source().and_then(#closure_accessor) }
                            } else {
                                quote! { msg.source().map(#closure_accessor) }
                            }
                        } else {
                            quote! { #accessor }
                        }
                    }
                    Option(inner) if matches!(TypeKind::classify(inner), String) => {
                        if default.is_some() {
                            if is_opt {
                                quote! {
                                    msg.source()
                                        .and_then(#closure_accessor)
                                        .map(|s| s.to_string())
                                }
                            } else {
                                quote! { msg.source().map(|source| source.name.to_string()) }
                            }
                        } else {
                            quote! { #accessor.map(|s| s.to_string()) }
                        }
                    }
                    Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
                    _ => expand_from_message(&field.ty, default),
                }
            }
            Self::Param(idx) => match TypeKind::classify(&field.ty) {
                Str => {
                    if let Some(d) = default {
                        let fallback = expand_default_fallback(d);
                        quote! {
                            params.middles.iter().nth(#idx)
                                .unwrap_or_else(|| #fallback)
                        }
                    } else {
                        quote! {
                            params.middles.iter().nth(#idx)
                                .ok_or_else(|| ircv3_parse::DeError::not_found_param(#idx))?
                        }
                    }
                }
                String => {
                    if let Some(d) = default {
                        let fallback = expand_default_fallback(d);
                        quote! {
                            params.middles.iter().nth(#idx)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| #fallback)
                        }
                    } else {
                        quote! {
                            params.middles.iter().nth(#idx)
                                .ok_or_else(|| ircv3_parse::DeError::not_found_param(#idx))?
                                .to_string()
                        }
                    }
                }
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    quote! { params.middles.iter().nth(#idx) }
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    quote! { params.middles.iter().nth(#idx).map(|s| s.to_string()) }
                }
                Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
                _ => expand_from_message(&field.ty, default),
            },
            Self::Params => match TypeKind::classify(&field.ty) {
                Vec(inner) if matches!(TypeKind::classify(inner), Str) => {
                    quote! { params.middles.to_vec() }
                }
                Vec(inner) if matches!(TypeKind::classify(inner), String) => {
                    quote! {
                        params.middles.iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    }
                }
                Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
                _ => expand_from_message(&field.ty, default),
            },
            Self::Trailing => match TypeKind::classify(&field.ty) {
                Str => {
                    if let Some(d) = default {
                        let fallback = expand_default_fallback(d);
                        quote! {
                            params.trailing.raw()
                                .filter(|s| !s.is_empty())
                                .unwrap_or_else(|| #fallback)
                        }
                    } else {
                        quote! {
                            params.trailing.as_str()
                        }
                    }
                }
                String => {
                    if let Some(d) = default {
                        let fallback = expand_default_fallback(d);
                        quote! {
                            params.trailing.raw()
                                .filter(|s| !s.is_empty())
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| #fallback)
                        }
                    } else {
                        quote! {
                            params.trailing.to_string()
                        }
                    }
                }
                Option(inner) if matches!(TypeKind::classify(inner), Str) => {
                    quote! {
                        params.trailing.raw().
                            filter(|s| !s.is_empty())
                    }
                }
                Option(inner) if matches!(TypeKind::classify(inner), String) => {
                    quote! {
                        params.trailing.raw()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                    }
                }
                Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
                _ => expand_from_message(&field.ty, default),
            },
            Self::Command => match TypeKind::classify(&field.ty) {
                Str => quote! { command.as_str() },
                String => quote! { command.to_string() },
                Option(inner) => quote! { <#inner>::from_message(&msg).ok() },
                _ => expand_from_message(&field.ty, default),
            },
        }
    }

    fn expand_component_check(&self) -> TokenStream {
        match self {
            Self::Tag(key) => quote! {
                if tags.get(#key).is_none() {
                    return Err(ircv3_parse::DeError::not_found_tag(#key));
                }
            },
            Self::TagFlag(key) => quote! {
                if !tags.get_flag(#key) {
                    return Err(ircv3_parse::DeError::not_found_tag(#key));
                }
            },
            Self::Source(component) => match component {
                Source::Name => quote! {},
                Source::User => {
                    let name = component.name();
                    quote! {
                        if source.user.is_none() {
                            return Err(ircv3_parse::DeError::not_found_source(#name))
                        }
                    }
                }
                Source::Host => {
                    let name = component.name();
                    quote! {
                        if source.host.is_none() {
                            return Err(ircv3_parse::DeError::not_found_source(#name))
                        }
                    }
                }
            },
            Self::Param(idx) => quote! {
                if params.middles.iter().nth(#idx).is_none() {
                    return Err(ircv3_parse::DeError::not_found_param(#idx));
                }
            },
            Self::Params => quote! {
                if params.middles.is_empty() {
                    return Err(ircv3_parse::DeError::not_found_param(0));
                }
            },
            Self::Trailing => quote! {
                if params.trailing.raw().is_none() {
                    return Err(ircv3_parse::DeError::not_found_trailing());
                }
            },
            Self::Command => {
                unreachable!("Command is handled separately in UnitStructAttrs")
            }
        }
    }
}

fn expand_with(field: &syn::Field, with_fn: &LitStr) -> TokenStream {
    let with_ident = Ident::new(&with_fn.value(), with_fn.span());
    if let Some(field_name) = &field.ident {
        quote! { #field_name: #with_ident(&msg) }
    } else {
        quote! { #with_ident(&msg) }
    }
}

fn expand_from_message(ty: &syn::Type, default: Option<&FieldDefault>) -> TokenStream {
    if let Some(d) = default {
        let fallback = expand_default_fallback(d);
        quote! { <#ty>::from_message(&msg).unwrap_or_else(|_| #fallback) }
    } else {
        quote! { <#ty>::from_message(&msg)? }
    }
}

fn expand_default_fallback(default: &FieldDefault) -> TokenStream {
    match default {
        FieldDefault::Trait => quote! { Default::default() },
        FieldDefault::Path(path) => {
            let path_ident = Ident::new(&path.value(), path.span());
            quote! { #path_ident() }
        }
    }
}

fn expand_nested(field: &syn::Field) -> TokenStream {
    use TypeKind::*;
    let value = match TypeKind::classify(&field.ty) {
        Option(inner) => quote! {
            <#inner>::from_message(&msg).ok()
        },
        _ => {
            let ty = &field.ty;
            quote! {
                <#ty>::from_message(&msg)?
            }
        }
    };

    if let Some(field_name) = &field.ident {
        quote! { #field_name: #value }
    } else {
        value
    }
}
