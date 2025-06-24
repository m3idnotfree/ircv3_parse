pub mod builder;
pub mod components;
pub mod validators;

mod error;
mod escape;
mod rfc1123;
mod scanner;

pub use error::IRCError;
pub use escape::unescaped_to_escaped;

use components::Message;
use scanner::Scanner;

pub(crate) const NUL: u8 = b'\0';
pub(crate) const SPACE: u8 = b' ';
pub(crate) const CR: u8 = b'\r';
pub(crate) const LF: u8 = b'\n';
pub(crate) const HYPEN: u8 = b'-';
pub(crate) const COLON: u8 = b':';
pub(crate) const SEMICOLON: u8 = b';';
pub(crate) const AT: u8 = b'@';

pub fn parse(input: &str) -> Result<Message, IRCError> {
    if input.is_empty() {
        return Err(IRCError::EmptyInput);
    }

    let scanner = Scanner::new(input)?;
    Ok(Message::new(input, scanner))
}
