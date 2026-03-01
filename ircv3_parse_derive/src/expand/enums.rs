use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::{
    ast::{Enum, Field, Variant, VariantFields},
    attr::{EnumAttrs, EnumKind, Rename, Source},
    component_set::ComponentSet,
    type_check::TypeKind,
};

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

                let body = fields.iter().map(|f| f.expand_de());
                quote! { (#(#body),*) }
            }
        }
    }

    pub fn expand_field_default(&self) -> TokenStream {
        match self {
            Self::Unit => quote! {},
            Self::Named(fields) => {
                let body = fields.iter().map(|f| f.expand_field_default());
                quote! { {#(#body),*} }
            }
            Self::Unnamed(fields) => {
                let body = fields.iter().map(|f| f.expand_field_default());
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
