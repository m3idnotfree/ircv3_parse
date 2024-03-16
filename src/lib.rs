//! IRCv3 parse
//!
//! # Example
//! ```no_run
//! use std::collections::HashMap;
//! use ircv3_parse::ircv3_parse;
//! let msg = "@badge-info=;badges=broadcaster/1;client-nonce=997dcf443c31e258c1d32a8da47b6936;color=#0000FF;display-name=abc;emotes=;first-msg=0;flags=0-6:S.7;id=eb24e920-8065-492a-8aea-266a00fc5126;mod=0;room-id=713936733;subscriber=0;tmi-sent-ts=1642786203573;turbo=0;user-id=713936733;user-type= :abc!abc@abc.tmi.twitch.tv PRIVMSG #xyz :HeyGuys\r\n";
//! let (tags, prefix, command, params) = ircv3_parse(msg);
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
//! assert_eq!(prefix.as_ref(), &Some(("abc", Some("abc@abc.tmi.twitch.tv"))));
//! assert_eq!(command, "PRIVMSG");
//! assert_eq!(params.channel(), Some("#xyz"));
//! assert_eq!(params.message(), Some("HeyGuys"));
//!
//!```

use nom::{bytes::complete::take_until, sequence::tuple, IResult};

pub use ircv3_tags::IRCv3Tags;
mod prefix;
pub use prefix::*;
mod params;
pub use params::*;

struct Ircv3Parse;
impl Ircv3Parse {
    pub fn parse(msg: &str) -> IResult<&str, (IRCv3Tags, IRCv3Prefix, &str, IRCv3Params)> {
        let (remain, (tags, prefix, command, params)) = tuple((
            IRCv3Tags::parse,
            IRCv3Prefix::parse,
            take_until(" "),
            IRCv3Params::parse,
        ))(msg)?;

        Ok((
            remain,
            (tags, prefix, command, params), // },
        ))
    }
}

/// tags, prefix, command, params
pub fn ircv3_parse(msg: &str) -> (IRCv3Tags, IRCv3Prefix, &str, IRCv3Params) {
    let (_, result) = Ircv3Parse::parse(msg).unwrap();

    result
}
