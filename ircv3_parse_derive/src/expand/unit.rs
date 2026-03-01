use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::{ast::UnitStruct, attr::UnitStructAttrs, component_set::ComponentSet};

impl<'a> UnitStruct<'a> {
    pub fn expand_de(&self) -> TokenStream {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();
        let msg_lifetime = self.generics.msg_lifetime();

        let expected_value = self.expected_value();
        let impl_body = self.attrs.expand_de(&expected_value);

        quote! {
            impl #impl_generics ircv3_parse::de::FromMessage<#msg_lifetime>
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

    pub fn expand_ser(&self) -> TokenStream {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();

        let expected_value = self.expected_value();
        let impl_body = self.attrs.expand_ser(&expected_value);

        quote! {
            impl #impl_generics ircv3_parse::ser::ToMessage
                for #name #ty_generics #where_clause
            {
                fn to_message<S: ircv3_parse::ser::MessageSerializer>(
                    &self,
                    serialize: &mut S
                ) -> Result<(), ircv3_parse::IRCError> {
                    #impl_body
                    Ok(())
                }
            }
        }
    }

    fn expected_value(&self) -> LitStr {
        if let Some(lit) = &self.attrs.value {
            lit.clone()
        } else {
            use heck::ToKebabCase;
            let kebab = self.ident.to_string().to_kebab_case();
            LitStr::new(&kebab, self.ident.span())
        }
    }
}

impl UnitStructAttrs {
    pub fn expand_de(&self, expected_value: &LitStr) -> TokenStream {
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
            .map(|f| f.expand_unit_de(expected_value))
            .unwrap_or_default();

        quote! {
            #(#setup_code)*
            #command_check
            #component_check
            Ok(Self)
        }
    }

    pub fn expand_ser(&self, expected_value: &LitStr) -> TokenStream {
        let command = if let Some(cmd) = &self.command {
            quote! {
                serialize.set_command(ircv3_parse::Commands::from(#cmd));
            }
        } else {
            quote! {}
        };

        let body = self
            .check
            .as_ref()
            .map(|check| check.expand_unit_ser(expected_value))
            .unwrap_or_default();

        quote! {
            #command
            #body
        }
    }
}
