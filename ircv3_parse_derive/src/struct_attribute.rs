use quote::{quote, ToTokens};
use syn::LitStr;
use syn::{DeriveInput, Result};

use crate::error_msg;
use crate::{COMMAND, IRC};

pub struct StructAttribute {
    command: Option<LitStr>,
}

impl StructAttribute {
    pub fn parse(input: &DeriveInput) -> Result<Self> {
        let mut command: Option<LitStr> = None;

        for attr in &input.attrs {
            if !attr.path().is_ident(IRC) {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(COMMAND) {
                    let s: LitStr = meta.value()?.parse()?;

                    if command.is_some() {
                        return Err(meta.error(error_msg::duplicate_attribute(COMMAND)));
                    }

                    command = Some(s);
                    return Ok(());
                }

                Err(meta.error(error_msg::unknown_irc_attribute(
                    meta.path.to_token_stream(),
                )))
            })?;
        }

        Ok(Self { command })
    }

    pub fn expand_validation(&self, command: Option<LitStr>) -> proc_macro2::TokenStream {
        command
            .as_ref()
            .or(self.command.as_ref())
            .map(|cmd| {
                quote! {
                    if msg.command() != #cmd {
                        return Err(ircv3_parse::DeError::invalid_command(
                            #cmd,
                            msg.command().as_str()
                        ));
                    }
                }
            })
            .unwrap_or(quote! {})
    }
}
