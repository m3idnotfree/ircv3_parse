use quote::{quote, ToTokens};
use syn::LitStr;
use syn::{DeriveInput, Error, Result};

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
                        return Err(Error::new(
                            s.span(),
                            error_msg::duplicate_attribute(COMMAND),
                        ));
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

    pub fn expand_validation(&self) -> proc_macro2::TokenStream {
        if let Some(cmd) = &self.command {
            quote! {
                if msg.command() != #cmd {
                    return Err(ircv3_parse::ExtractError::invalid_command(
                        #cmd,
                        msg.command().as_str()
                    ));
                }
            }
        } else {
            quote! {}
        }
    }
}
