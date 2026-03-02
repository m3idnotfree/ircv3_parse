use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{LitStr, Type};

use crate::{
    ast::{Enum, Field, Variant, VariantFields},
    attr::{EnumAttrs, EnumKind, FieldKind, Rename, Source},
    component_set::ComponentSet,
    type_check::TypeKind,
};

impl<'a> Enum<'a> {
    pub fn expand_de(&self) -> TokenStream {
        let mut components = ComponentSet::default();

        self.add_to(&mut components);

        let body = if let EnumKind::TagFlag(_) = &self.attrs.kind {
            self.expand_de_tag_flag()
        } else {
            self.expand_de_match()
        };

        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();
        let msg_lifetime = self.generics.msg_lifetime();

        let setup = components.expand();

        quote! {
            impl #impl_generics ircv3_parse::de::FromMessage<#msg_lifetime>
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

    pub fn expand_ser(&self) -> TokenStream {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();

        let body = if let EnumKind::TagFlag(_) = &self.attrs.kind {
            self.expand_ser_tag_flag()
        } else {
            self.expand_ser_match()
        };

        quote! {
            impl #impl_generics ircv3_parse::ser::ToMessage
                for #name #ty_generics #where_clause
            {
                fn to_message<S: ircv3_parse::ser::MessageSerializer>(
                    &self,
                    serialize: &mut S
                ) -> Result<(), ircv3_parse::IRCError> {
                    #body
                    Ok(())
                }
            }
        }
    }

    fn expand_ser_tag_flag(&self) -> TokenStream {
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

        let enum_attrs = &self.attrs;

        let present_ident = present.ident;
        let absent_ident = absent.ident;

        let value = present.variant_value(&enum_attrs.rename);
        let enum_body = self.attrs.expand_variant(&value);
        let present_fields = present.fields.expand_body(enum_attrs);

        let crlf = self.attrs.expand_crlf();

        quote! {
            match self {
                Self::#present_ident => { #enum_body #present_fields; }
                Self::#absent_ident => {}
            }
            #crlf
        }
    }

    fn expand_de_tag_flag(&self) -> TokenStream {
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

    fn expand_de_match(&self) -> TokenStream {
        let default_arm = self.find_default_arm();
        let setup = self.attrs.expand_value_setup(default_arm.as_ref());
        let arms = self.expand_de_arms();
        let catch_all = self.expand_catch_all(default_arm);

        quote! {
            #setup
            match value {
                #(#arms,)*
                _ => #catch_all
            }
        }
    }

    fn expand_ser_match(&self) -> TokenStream {
        let enum_attrs = &self.attrs;
        let arms = self.variants.iter().map(|v| v.expand_ser_arm(enum_attrs));
        let crlf_expand = self.attrs.expand_crlf();

        quote! {
            match self {
                #(#arms),*
            }
            #crlf_expand
        }
    }

    fn expand_de_arms(&self) -> Vec<TokenStream> {
        let rename = &self.attrs.rename;
        self.variants
            .iter()
            .map(|v| v.expand_de_arm(rename))
            .collect()
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

impl<'a> Variant<'a> {
    pub fn expand_de_arm(&self, rename: &Rename) -> TokenStream {
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

    pub fn expand_ser_arm(&self, enum_attrs: &EnumAttrs) -> TokenStream {
        let ident = &self.ident;
        let bindings = self.fields.expand_bindings();
        let body = self.expand_ser_arm_body(enum_attrs);
        quote! { Self::#ident #bindings => { #body } }
    }

    fn expand_ser_arm_body(&self, enum_attrs: &EnumAttrs) -> TokenStream {
        let value = self.variant_value(&enum_attrs.rename);
        let field_body = self.fields.expand_body(enum_attrs);
        let enum_body = enum_attrs.expand_variant(&value);
        quote! { #enum_body #field_body }
    }

    fn variant_value(&self, rename: &Rename) -> LitStr {
        if let Some(pick) = &self.attrs.pick {
            pick.clone()
        } else if let Some(first) = self.attrs.values.first() {
            first.clone()
        } else {
            LitStr::new(&rename.apply(&self.ident.to_string()), Span::call_site())
        }
    }

    pub fn expand_default_arm(&self, default: Option<&LitStr>) -> Option<TokenStream> {
        let is_default = default.is_some_and(|d| d.value() == *self.ident.to_string());

        let ident = &self.ident;
        if is_default {
            let body = self.fields.expand_default();
            Some(quote! { Ok(Self::#ident #body )})
        } else {
            None
        }
    }

    pub fn add_to(&self, components: &mut ComponentSet) {
        self.fields.add_to(components);
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
            EnumKind::Source(inner) => match inner {
                Source::Name => {
                    if let Some(default) = default_arm {
                        quote! {
                            let source = match msg.source() {
                                Some(s) => s,
                                None => return #default,
                            };
                            let value = source.name;
                        }
                    } else {
                        quote! { let value = source.name; }
                    }
                }
                Source::User => {
                    if let Some(default) = default_arm {
                        return quote! {
                            let value = match msg.source().and_then(|s| s.user) {
                                Some(value) => value,
                                None => return #default,
                            };
                        };
                    }

                    quote! {
                        let value = source.user
                            .ok_or_else(|| ircv3_parse::DeError::source_component_not_found())?;
                    }
                }
                Source::Host => {
                    if let Some(default) = default_arm {
                        return quote! {
                            let value = match msg.source().and_then(|s| s.host) {
                                Some(value) => value,
                                None => return #default,
                            };
                        };
                    }

                    quote! {
                        let value = source.host
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

    pub fn expand_variant(&self, value: &LitStr) -> TokenStream {
        if let EnumKind::Command = &self.kind {
            return quote! { serialize.set_command(ircv3_parse::Commands::from(#value)); };
        }
        let kind = self.kind.to_field_kind();
        kind.expand_unit_ser(value)
    }

    pub fn expand_crlf(&self) -> TokenStream {
        if self.crlf {
            quote! { serialize.end()?; }
        } else {
            quote! {}
        }
    }
}

impl EnumKind {
    pub fn expand_field_ser(&self, ty: &Type, accessor: &TokenStream) -> TokenStream {
        self.to_field_kind().expand_with_accessor(ty, accessor)
    }

    fn to_field_kind(&self) -> FieldKind {
        match self {
            EnumKind::Tag(key) => FieldKind::Tag(key.clone()),
            EnumKind::TagFlag(key) => FieldKind::TagFlag(key.clone()),
            EnumKind::Source(s) => FieldKind::Source(s.clone()),
            EnumKind::Param(_) => FieldKind::Param(0),
            EnumKind::Trailing => FieldKind::Trailing,
            EnumKind::Command => FieldKind::Command,
        }
    }
}

impl<'a> VariantFields<'a> {
    pub fn expand_fields(&self) -> TokenStream {
        match self {
            Self::Unit => quote! {},
            Self::Named(fields) => {
                let body = fields.iter().map(|f| f.expand_de());
                quote! { {#(#body),*} }
            }
            Self::Unnamed(fields) => {
                if fields.len() == 1 && !self.has_any_kind() {
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

                let body = fields.iter().map(|f| f.expand_de());
                quote! { (#(#body),*) }
            }
        }
    }

    pub fn expand_body(&self, attrs: &EnumAttrs) -> TokenStream {
        match self {
            Self::Unit => quote! {},
            Self::Named(fields) => {
                let body = fields.iter().map(|field| {
                    let binding = field.field.ident.as_ref().unwrap();
                    field.expand_with_accessor(&quote! { #binding })
                });
                quote! { #(#body)* }
            }
            Self::Unnamed(fields) => {
                if fields.len() == 1 && !self.has_any_kind() {
                    let field = &fields[0];
                    return attrs
                        .kind
                        .expand_field_ser(&field.field.ty, &quote! { __field_0 });
                }

                let body = fields.iter().enumerate().map(|(idx, field)| {
                    let binding = format_ident!("__field_{}", idx);
                    field.expand_with_accessor(&quote! { #binding })
                });
                quote! { #(#body)* }
            }
        }
    }

    pub fn expand_bindings(&self) -> TokenStream {
        match self {
            Self::Unit => quote! {},
            Self::Named(fields) => {
                let idents = fields.iter().map(|f| f.field.ident.as_ref().unwrap());
                quote! { { #(#idents),* } }
            }
            Self::Unnamed(fields) => {
                let idents = (0..fields.len()).map(|i| format_ident!("__field_{}", i));
                quote! { (#(#idents),*) }
            }
        }
    }

    pub fn expand_default(&self) -> TokenStream {
        match self {
            Self::Unit => quote! {},
            Self::Named(fields) => {
                let body = fields.iter().map(|f| f.expand_default());
                quote! { {#(#body),*} }
            }
            Self::Unnamed(fields) => {
                let body = fields.iter().map(|f| f.expand_default());
                quote! { (#(#body),*) }
            }
        }
    }

    pub fn has_any_kind(&self) -> bool {
        self.fields().iter().any(|f| f.attrs.kind.is_some())
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
