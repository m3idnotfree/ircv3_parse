//! Zero-copy IRC message parser with IRCv3 support.
//!
//! ## Quick Start
//!
//! ### Using Derive Macro (Recommended)
//!
//! ```rust
//! use ircv3_parse::FromMessage;
//!
//! #[derive(FromMessage)]
//! #[irc(command = "PRIVMSG")]
//! struct PrivMsg<'a> {
//!     #[irc(source = "name")]
//!     nick: &'a str,
//!     #[irc(trailing)]
//!     message: &'a str
//! }
//!
//! let input = ":nick!user@example.com PRIVMSG #channel :Hello everyone!";
//! let msg: PrivMsg = ircv3_parse::from_str(input)?;
//!
//! println!("From: {}", msg.nick);
//! println!("Message: {}", msg.message);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## FromMessage Derive Attributes
//!
//! The `FromMessage` derive macro supports both `&str` and `String` field types.
//!
//! ### Struct-Level Attributes
//!
//! - `#[irc(command = "COMMAND")]` - Validates that the command matches (case-insensitive)
//!
//! ### Field-Level Attributes
//!
//! - `#[irc(tag = "key")]` - Extract tag value
//! - `#[irc(tag_flag = "key")]` - Extract tag flag
//! - `#[irc(source = "component")]` - Extract source component (`name`, `user`, or `host`)
//! - `#[irc(param = N)]` - Extract parameter by index
//! - `#[irc(params)]` - Extract all parameters into a `Vec`
//! - `#[irc(trailing)]` - Extract trailing parameter
//! - `#[irc(command)]` - Extract command
//! - `#[irc(with = "function")]` - Custom extraction
//!
//! ## Manual FromMessage Implementation
//!
//! For more complex parsing logic, implement the `FromMessage` trait manually.
//!
//! ### Understanding Message Structure
//!
//! The [`Message`] struct provides access to IRC message components:
//!
//! - [`Message::tags()`] - Returns [`Tags`](components::Tags)
//! - [`Message::source()`] - Returns [`Source`](components::Source)
//! - [`Message::command()`] - Returns [`Commands`](components::Commands)
//! - [`Message::params()`] - Returns [`Params`](components::Params)
//!
//! ### Example Implementation
//!
//! ```rust
//! use ircv3_parse::{extract::FromMessage, ExtractError, Message};
//!
//! struct PrivMsg<'a> {
//!     color: Option<&'a str>,
//!     channel: &'a str,
//!     nick: &'a str,
//!     message: &'a str,
//! }
//!
//! impl<'a> FromMessage<'a> for PrivMsg<'a> {
//!     fn from_message(msg: &Message<'a>) -> Result<Self, ExtractError> {
//!         // Validate command
//!         let command = msg.command();
//!         if !command.is_privmsg() {
//!             return Err(ExtractError::invalid_command("PRIVMSG", command.as_str()));
//!         }
//!
//!         // Extract tags (optional)
//!         let color = msg.tags()
//!             .and_then(|tags| tags.get("color"))
//!             .map(|v| v.as_str());
//!
//!         // Extract source (required)
//!         let source = msg.source()
//!             .ok_or_else(|| ExtractError::missing_source())?;
//!         let nick = source.name;
//!
//!         // Extract parameters
//!         let params = msg.params();
//!         let channel = params.middles.first()
//!             .ok_or_else(|| ExtractError::missing_param_field("channel", 0))?;
//!         let message = params.trailing.as_str();
//!
//!         Ok(Self {
//!             color,
//!             channel,
//!             nick,
//!             message,
//!         })
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Error Handling
//!
//! ```rust
//! use ircv3_parse::FromMessage;
//!
//! #[derive(FromMessage)]
//! #[irc(command = "PRIVMSG")]
//! struct PrivMsg {
//!     #[irc(trailing)]
//!     message: String
//! }
//!
//! let input = "NOTICE all :hi";
//! let result = ircv3_parse::from_str::<PrivMsg>(input);
//!
//! if let Err(e) = result {
//!     if e.is_parse_error() {
//!         println!("Invalid IRC message format: {e}");
//!     }
//!
//!     if e.is_invalid_command() {
//!         println!("Expected PRIVMSG, got {input}");
//!     }
//!
//!     if e.is_missing_tags() {
//!         println!("Message has no tags component");
//!     }
//!
//!     if e.is_missing_source() {
//!         println!("Message has no source component");
//!     }
//!
//!     if e.is_missing_param() {
//!         println!("Message has no parameters");
//!     }
//!
//!     if e.is_missing_tag() {
//!         println!("Specific tag not found");
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Building Messages
//!
//! ```rust
//! use ircv3_parse::{builder::legacy::MessageBuilder, components::Commands};
//!
//! let message = MessageBuilder::new(Commands::PRIVMSG)
//!     .with_tags(|tags| {
//!         tags.add("id", Some("123"))?
//!             .add("color", None)?
//!             .add_flag("subscriber")
//!     })?
//!     .with_source("nick", |source| {
//!         source.with_user("user")?.with_host("example.com")
//!     })?
//!     .with_params(|params| params.add("#channel"))?
//!     .with_trailing("hi")?
//!     .finish();
//!
//! let bytes = message.to_bytes();
//! // Result: @id=123;color=;subscriber :nick!user@example.com PRIVMSG #channel :hi\r\n
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Builder Order
//!
//! Components must be added in the correct order:
//!
//! 1. Command (required) - [`MessageBuilder::new()`](builder::MessageBuilder::new()) with
//!    [`Commands`](components::Commands)
//! 2. Tags (optional) - [`with_tags()`](builder::MessageBuilder::with_tags())
//! 3. Source (optional) - [`with_source()`](builder::MessageBuilder::with_source())
//! 4. Middle parameters (optional) - [`with_params()`](builder::MessageBuilder::with_params())
//! 5. Trailing parameter (optional) - [`with_trailing()`](builder::MessageBuilder::with_trailing())
//!
//!
//! ## Feature Flags
//!
//! - `derive` - Enables the `FromMessage` derive macro
//! - `serde` - Enables `Serialize` implementation for [`Message`]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub(crate) mod compat {
    pub use core::{
        fmt::{Debug, Display, Formatter, Result as FmtResult},
        iter::Map,
        str::{Split, SplitAsciiWhitespace},
    };

    #[cfg(not(feature = "std"))]
    pub use alloc::{
        format,
        string::{String, ToString},
        vec::Vec,
    };

    #[cfg(feature = "std")]
    pub use std::{
        format,
        string::{String, ToString},
        vec::Vec,
    };
}

#[cfg(feature = "derive")]
pub use ircv3_parse_derive::FromMessage;

pub mod builder;
pub mod components;
pub mod error;
pub mod extract;
pub mod validators;

mod rfc1123;
mod scanner;
mod unescape;

pub use components::{Commands, Message};
pub use error::{ExtractError, IRCError};
pub use unescape::unescape;

use scanner::Scanner;

pub(crate) const NUL: u8 = b'\0';
pub(crate) const SPACE: u8 = b' ';
pub(crate) const CR: u8 = b'\r';
pub(crate) const LF: u8 = b'\n';
pub(crate) const HYPEN: u8 = b'-';
pub(crate) const COLON: u8 = b':';
pub(crate) const SEMICOLON: u8 = b';';
pub(crate) const AT: u8 = b'@';
pub(crate) const BANG: u8 = b'!';
pub(crate) const EQ: u8 = b'=';

/// Parse an IRC message from a string.
///
/// Low-level parsing function.
///
/// # Examples
///
/// ```rust
/// let msg = ircv3_parse::parse(":nick!user@example.com PRIVMSG #channel :Hi")?;
///
/// println!("Command: {}", msg.command());
/// if let Some(source) = msg.source() {
///     println!("Source: {}", source);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// Returns [`IRCError`]
pub fn parse<'a>(input: &'a str) -> Result<Message<'a>, IRCError> {
    if input.is_empty() {
        return Err(IRCError::EmptyInput);
    }

    let scanner = Scanner::new(input)?;
    Ok(Message::new(input, scanner))
}

/// Parse an IRC message into a type implementing [`extract::FromMessage`].
///
/// Convenience function for types using `[derive(FromMessage)]` or manually implementing the
/// trait.
///
/// # Examples
///
/// ```rust
/// use ircv3_parse::FromMessage;
///
/// #[derive(FromMessage)]
/// #[irc(command = "PRIVMSG")]
/// struct PrivMsg<'a> {
///     #[irc(source = "name")]
///     nick: &'a str,
///     #[irc(trailing)]
///     message: &'a str,
/// }
///
/// let msg: PrivMsg = ircv3_parse::from_str(":nick!user@example.com PRIVMSG #channel :Hi")?;
/// assert_eq!("nick", msg.nick);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// Returns [`ExtractError`]
pub fn from_str<'a, T: extract::FromMessage<'a>>(s: &'a str) -> Result<T, ExtractError> {
    T::from_str(s)
}
