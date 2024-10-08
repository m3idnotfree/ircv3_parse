//! IRCv3 parse
//!
//! # Example
//! ```no_run
//! use std::collections::HashMap;
//!
//! let msg = "@badge-info=;badges=broadcaster/1;client-nonce=997dcf443c31e258c1d32a8da47b6936;color=#0000FF;display-name=abc;emotes=;first-msg=0;flags=0-6:S.7;id=eb24e920-8065-492a-8aea-266a00fc5126;mod=0;room-id=713936733;subscriber=0;tmi-sent-ts=1642786203573;turbo=0;user-id=713936733;user-type= :abc!abc@abc.tmi.twitch.tv PRIVMSG #xyz :HeyGuys\r\n";
//! let ircv3_message = ircv3_parse::parse(msg);
//! let expeced_tags= HashMap::from([
//!     ("badge-info", ""),
//!     ("subscriber", "0"),
//!     ("id", "eb24e920-8065-492a-8aea-266a00fc5126"),
//!     ("user-id", "713936733"),
//!     ("emotes", ""),
//!     ("tmi-sent-ts", "1642786203573"),
//!     ("client-nonce", "997dcf443c31e258c1d32a8da47b6936"),
//!     ("mod", "0"),
//!     ("badges", "broadcaster/1"),
//!     ("room-id", "713936733"),
//!     ("flags", "0-6:S.7"),
//!     ("color", "#0000FF"),
//!     ("turbo", "0"),
//!     ("display-name", "abc"),
//!     ("first-msg", "0"),
//!     ("user-type", "")]);
//!
//!
//!
//! assert!(ircv3_message.prefix.is_some());
//! let prefix = ircv3_message.prefix.unwrap();
//! assert_eq!(prefix.servername_nick, "abc");
//! assert_eq!(prefix.user, Some("abc".to_string()));
//! assert_eq!(prefix.host, Some("abc.tmi.twitch.tv".to_string()));
//!
//! let params = ircv3_message.params;
//! assert_eq!(ircv3_message.command, "PRIVMSG".to_string());
//! assert_eq!(params.channel, Some("xyz".to_string()));
//! assert_eq!(params.message, Some("HeyGuys".to_string()));
//!
//!```

use std::collections::VecDeque;

use ircv3_tags::IRCv3Tags;
use nom::sequence::tuple;

mod source;
pub use source::*;
mod command;
pub use command::*;
mod params;
pub use params::*;

#[derive(Debug)]
pub struct IRCv3Message {
    pub tags: Option<IRCv3Tags>,
    pub source: Option<IRCv3Source>,
    pub command: String,
    pub params: IRCv3Params,
}

pub fn parse(msg: &str) -> IRCv3Message {
    let (_, (tags, source, command, params)) = tuple((
        ircv3_tags::parse_nom,
        source_parse,
        command_parse,
        params_parse,
    ))(msg)
    .unwrap();

    IRCv3Message {
        tags,
        source,
        command: command.to_string(),
        params,
    }
}

pub fn parse_vecdeque(msg: &str) -> VecDeque<IRCv3Message> {
    parse_inner(msg, VecDeque::new())
}

fn parse_inner(msg: &str, mut result: VecDeque<IRCv3Message>) -> VecDeque<IRCv3Message> {
    if msg.is_empty() {
        result
    } else {
        let (msg, (tags, source, command, params)) = tuple((
            ircv3_tags::parse_nom,
            source_parse,
            command_parse,
            params_parse,
        ))(msg)
        .unwrap();

        result.push_back(IRCv3Message {
            tags,
            source,
            command: command.to_string(),
            params,
        });

        parse_inner(msg, result)
    }
}
