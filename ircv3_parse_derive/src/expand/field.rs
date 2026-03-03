use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Index, LitStr, Member, Type};

use crate::{
    ast::{Field, FieldStruct},
    attr::{FieldAttrs, StructAttrs},
    type_check::TypeKind,
};

impl<'a> FieldStruct<'a> {
    pub fn expand_de(&self) -> TokenStream {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();
        let msg_lifetime = self.generics.msg_lifetime();

        let validation = self.attrs.expand_command_check();
        let setup_code = self.components().expand();
        let impl_body = self.expand_de_body();

        quote! {
            impl #impl_generics ircv3_parse::de::FromMessage<#msg_lifetime>
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

    fn expand_de_body(&self) -> TokenStream {
        let fields = self.fields.iter().map(|f| f.expand_de());
        let is_named = self.fields[0].field.ident.is_some();

        let body = if is_named {
            quote! { {#(#fields),*} }
        } else {
            quote! { (#(#fields),*) }
        };

        quote! { Ok(Self #body) }
    }

    pub fn expand_ser(&self) -> TokenStream {
        let impl_body = self.fields.iter().enumerate().map(|(idx, field)| {
            let member = if let Some(ident) = &field.field.ident {
                Member::Named(ident.clone())
            } else {
                Member::Unnamed(Index::from(idx))
            };
            field.expand_with_accessor(&quote! { self.#member })
        });

        let name = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split();
        let command_expand = self.attrs.expand_ser();
        let crlf_expand = self.attrs.expand_crlf();

        quote! {
            impl #impl_generics ircv3_parse::ser::ToMessage
                for #name #ty_generics #where_clause
            {
                fn to_message<S: ircv3_parse::ser::MessageSerializer>(
                    &self,
                    serialize: &mut S
                ) -> Result<(), ircv3_parse::IRCError> {
                    #command_expand
                    #(#impl_body)*
                    #crlf_expand
                    Ok(())
                }
            }
        }
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

    pub fn expand_ser(&self) -> TokenStream {
        if let Some(cmd) = &self.command {
            quote! {
                serialize.set_command(ircv3_parse::Commands::from(#cmd));
            }
        } else {
            quote! {}
        }
    }

    pub fn expand_crlf(&self) -> TokenStream {
        if self.crlf {
            quote! { serialize.end()?; }
        } else {
            quote! {}
        }
    }
}

impl<'a> Field<'a> {
    pub fn expand_de(&self) -> TokenStream {
        self.attrs.expand_de(self.field)
    }

    pub fn expand_with_accessor(&self, accessor: &TokenStream) -> TokenStream {
        self.attrs.expand_with_accessor(&self.field.ty, accessor)
    }

    pub fn expand_default(&self) -> TokenStream {
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

    pub fn expand_with_accessor(&self, ty: &Type, accessor: &TokenStream) -> TokenStream {
        if self.skip {
            return quote! {};
        }

        if let Some(kind) = &self.kind {
            kind.expand_with_accessor(ty, accessor, self.skip_none)
        } else {
            use TypeKind::*;
            match TypeKind::classify(ty) {
                Option(_) => quote! {
                    if let Some(value) = &#accessor {
                        value.to_message(serialize)?;
                    }
                },
                _ => quote! {
                    #accessor.to_message(serialize)?;
                },
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
