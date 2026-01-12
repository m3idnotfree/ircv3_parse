//! Zero-copy IRC message parser with IRCv3 support.
//!
//! ## Key Features
//!
//! - **Zero-copy parsing**: Message components are slices into the original input string
//! - **IRCv3 support**: Full support for message tags, source, and all IRCv3 features
//! - **Derive macros**: `FromMessage` and `ToMessage` for easy message extraction and generation
//! - **Manual implementations**: Full control over parsing and serialization when needed
//! - **Builder pattern**: Flexible, order-independent message construction with [`MessageBuilder`]
//! - **`no_std` compatible**: Works in embedded and `no_std` environments (requires `alloc`)
//!
//! ## Quick Start
//!
//! ### Parsing Messages with FromMessage
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
//! ### Building Messages with ToMessage
//!
//! ```rust
//! use ircv3_parse::ToMessage;
//!
//! #[derive(ToMessage)]
//! #[irc(command = "PRIVMSG", crlf)]
//! struct PrivMsg<'a> {
//!     #[irc(tag)]
//!     msgid: &'a str,
//!     #[irc(param)]
//!     channel: &'a str,
//!     #[irc(trailing)]
//!     message: &'a str,
//! }
//!
//! let msg = PrivMsg {
//!     msgid: "123",
//!     channel: "#channel",
//!     message: "hi",
//! };
//!
//! let output = ircv3_parse::to_message(&msg)?;
//! assert_eq!(b"@msgid=123 PRIVMSG #channel :hi\r\n", output.as_ref());
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
//! #### Tag
//!
//! - `#[irc(tag)]` - Extract tag value using field name as key
//! - `#[irc(tag = "key")]` - Extract tag value with custom key
//!
//! #### Tag Flag
//!
//! - `#[irc(tag_flag)]` - Extract tag flag using field name as key (returns `bool`)
//! - `#[irc(tag_flag = "key")]` - Extract tag flag with custom key (returns `bool`)
//!
//! #### Source
//!
//! - `#[irc(source)]` - Extract source `name` component
//! - `#[irc(source = "component")]` - Extract source component (`name`, `user`, or `host`)
//!
//! #### Parameter
//!
//! - `#[irc(param)]` - Extract first parameter (index 0)
//! - `#[irc(param = N)]` - Extract parameter at index N
//! - `#[irc(params)]` - Extract all parameters into a `Vec`
//!
//! #### Trailing Parameter
//!
//! - `#[irc(trailing)]` - Extract trailing parameter
//!
//! #### Command
//!
//! - `#[irc(command)]` - Extract command value
//! - `#[irc(command = "COMMAND")]` - Extract and validate command matches "COMMAND"
//!     - If field-level `command` is set, struct-level `command` is ignored
//!     - If multiple `command` attributes exist, the last one is used
//!
//! #### Custom Extraction
//!
//! - `#[irc(with = "function")]` - Use custom extraction function
//!
//! ## Manual [`FromMessage`](message::de::FromMessage) Implementation
//!
//! For more complex parsing logic, implement the `FromMessage` trait manually.
//!
//! ### Understanding Message Structure
//!
//! The [`Message`] struct provides access to IRC message components:
//!
//! - [`Message::tags()`] - Returns [`Tags`](components::Tags)
//! - [`Message::source()`] - Returns [`Source`](components::Source)
//! - [`Message::command()`] - Returns [`Commands`]
//! - [`Message::params()`] - Returns [`Params`](components::Params)
//!
//! ### Example Implementation
//!
//! ```rust
//! use ircv3_parse::{message::de::FromMessage, DeError, Message};
//!
//! struct PrivMsg<'a> {
//!     color: Option<&'a str>,
//!     channel: &'a str,
//!     nick: &'a str,
//!     message: &'a str,
//! }
//!
//! impl<'a> FromMessage<'a> for PrivMsg<'a> {
//!     fn from_message(msg: &Message<'a>) -> Result<Self, DeError> {
//!         // Validate command
//!         let command = msg.command();
//!         if !command.is_privmsg() {
//!             return Err(DeError::invalid_command("PRIVMSG", command.as_str()));
//!         }
//!
//!         // Extract tags (optional)
//!         let color = msg.tags()
//!             .and_then(|tags| tags.get("color"))
//!             .map(|v| v.as_str());
//!
//!         // Extract source (required)
//!         let source = msg.source()
//!             .ok_or_else(|| DeError::missing_source())?;
//!         let nick = source.name;
//!
//!         // Extract parameters
//!         let params = msg.params();
//!         let channel = params.middles.first()
//!             .ok_or_else(|| DeError::missing_param_field("channel", 0))?;
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
//! ## ToMessage Derive Attributes
//!
//! The `ToMessage` derive macro supports both `&str` and `String` field types.
//!
//! ### Struct-Level Attributes
//!
//! - `#[irc(command = "COMMAND")]` - Sets the default command for this message type
//! - `#[irc(crlf)]` - Explicitly appends `\r\n` at the end of the message
//!
//! ### Field-Level Attributes
//!
//! #### Tag
//!
//! - `#[irc(tag)]` - Serializes field as tag using the field name as key
//! - `#[irc(tag = "key")]` - Serializes field as tag with custom key
//!
//! #### Tag Flag
//!
//! - `#[irc(tag_flag)]` - Serializes boolean field as tag flag using field name as key
//! - `#[irc(tag_flag = "key")]` - Serializes boolean field as tag flag with custom key
//!
//! #### Source
//!
//! - `#[irc(source)]` - Serializes field as source name component (`source = "name"`)
//! - `#[irc(source = "name|user|host")]` - Serializes field as source component
//!     - **Note**: `name` is **required** when using `user` or `host`
//!
//! #### Parameter
//!
//! - `#[irc(param)]` - Serializes field as a middle parameter
//! - `#[irc(params)]` - Serializes field as multiple middle parameters
//! - `#[irc(param = N)]` - Serializes field as a middle parameter
//!     - **Note**: The index `N` is ignored during serialization
//!     - `FromMessage` uses the index to extract the Nth parameter
//!     - `ToMessage` always serializes fields in declaration order
//!
//! #### Trailing Parameter
//!
//! - `#[irc(trailing)]` - Serializes field as the trailing parameter
//!
//! #### Command
//!
//! - `#[irc(command)]` - Serializes field as the IRC command
//! - `#[irc(command = "COMMAND")]` - Uses the specified command string
//!   - If field-level `command` is set, struct-level `command` is ignored
//!   - If multiple `command` attributes exist, the last one takes precedence
//!
//! ## Manual [`ToMessage`](message::ser::ToMessage) Implementation
//!
//! **Note**: Component serialization order is important and must follow this sequence:
//! 1. tags (optional)
//! 2. source (optional)
//! 3. command (required)
//! 4. params (optional)
//! 5. trailing (optional)
//! 6. crlf (optional)
//!
//! ### Example Implementation
//!
//! ```rust
//! use ircv3_parse::message::ser::ToMessage;
//!
//! struct PrivMsg<'a> {
//!     msgid: &'a str,
//!     subscriber: bool,
//!     channel: &'a str,
//!     message: String,
//! }
//!
//! impl ToMessage for PrivMsg<'_> {
//!     fn to_message<S: ircv3_parse::message::ser::MessageSerializer>(
//!         &self,
//!         serialize: &mut S,
//!     ) -> Result<(), ircv3_parse::IRCError> {
//!         use ircv3_parse::Commands;
//!
//!         {
//!             use ircv3_parse::message::ser::SerializeTags;
//!
//!             let mut tags = serialize.tags();
//!             tags.tag("msgid", Some(self.msgid))?;
//!             if self.subscriber {
//!                 tags.flag("subscriber")?;
//!             }
//!             // You can skip tags.end() because Drop handles it
//!             tags.end();
//!         }
//!
//!         Commands::PRIVMSG.to_message(serialize)?;
//!
//!         {
//!             use ircv3_parse::message::ser::SerializeParams;
//!
//!             let mut params = serialize.params();
//!             params.push(self.channel)?;
//!             // You can skip params.end() because Drop handles it
//!             params.end();
//!         }
//!
//!         serialize.trailing(self.message.as_ref())?;
//!
//!         serialize.end()?;
//!         Ok(())
//!     }
//! }
//!
//! let msg = PrivMsg {
//!     msgid: "1",
//!     subscriber: false,
//!     channel: "#channel",
//!     message: "hi".to_string(),
//! };
//!
//! let msg = ircv3_parse::to_message(&msg)?;
//!
//! assert_eq!("@msgid=1 PRIVMSG #channel :hi\r\n", msg);
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Using [`MessageBuilder`]
//!
//! The builder pattern allows order-independent message construction.
//!
//! ```rust
//! use ircv3_parse::{Commands, MessageBuilder};
//!
//! let mut msg = MessageBuilder::new(Commands::PRIVMSG);
//! msg.add_tag("tag1", Some("value1"))?
//!     .add_tag("tag2", None)?
//!     .add_tag_flag("flag")?;
//!
//! // source name must be set before user or host
//! msg.set_source_name("nick")?;
//! msg.set_source_user("user")?;
//! msg.set_source_host("example.com")?;
//!
//! msg.set_trailing("hi")?;
//!
//! let actual = msg.build();
//! assert_eq!(
//!     b"@tag1=value1;tag2=;flag :nick!user@example.com PRIVMSG :hi\r\n",
//!     actual.as_ref()
//! );
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! ## Error Handling
//!
//! ### FromMessage
//!
//! Returns [`DeError`] for deserialization failures.
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
//! ### ToMessage
//!
//! Returns [`IRCError`] for serialization failures.
//!
//! ## Feature Flags
//!
//! - **`std`** (enabled by default) - Enables standard library support
//! - **`derive`** - Enables `FromMessage` and `ToMessage` derive macros (recommended)
//! - **`serde`** - Enables `Serialize` implementation for [`Message`]
//!
//! ## Using in `no_std` Environments
//!
//! This crate supports `no_std` environments with the `alloc` crate:
//!
//! ```toml
//! [dependencies]
//! ircv3_parse = { version = "3", default-features = false, features = ["derive"] }
//! ```
//!
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
pub use ircv3_parse_derive::{FromMessage, ToMessage};

pub mod builder;
pub mod components;
pub mod error;
pub mod message;
pub mod validators;

mod rfc1123;
mod scanner;
mod unescape;

pub use components::Commands;
pub use error::{DeError, IRCError};
pub use message::{Message, MessageBuilder};
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

/// Parse an IRC message into a type implementing [`message::de::FromMessage`].
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
/// Returns [`DeError`]
pub fn from_str<'a, T: crate::message::de::FromMessage<'a>>(s: &'a str) -> Result<T, DeError> {
    T::from_str(s)
}

/// Serialize a custom data structure as Bytes.
///
/// # Errors
///
/// Returns [`IRCError`]
pub fn to_message<T: crate::message::ser::ToMessage>(t: &T) -> Result<bytes::Bytes, IRCError> {
    t.to_bytes()
}
