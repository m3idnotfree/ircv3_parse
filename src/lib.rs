//! IRCv3 parse
//!
//! # Example
//! ```
//! use std::collections::HashMap;
//! use ircv3_parse::IRCv3;
//! fn main () {
//!     let msg = "@badge-info=;badges=broadcaster/1;client-nonce=997dcf443c31e258c1d32a8da47b6936;color=#0000FF;display-name=abc;emotes=;first-msg=0;flags=0-6:S.7;id=eb24e920-8065-492a-8aea-266a00fc5126;mod=0;room-id=713936733;subscriber=0;tmi-sent-ts=1642786203573;turbo=0;user-id=713936733;user-type= :abc!abc@abc.tmi.twitch.tv PRIVMSG #xyz :HeyGuys\r\n";
//!     let ircv3_message = IRCv3::parse(msg);
//!
//!     let expected_tags = HashMap::from([
//!         ("badge-info", ""),
//!         ("subscriber", "0"),
//!         ("id", "eb24e920-8065-492a-8aea-266a00fc5126"),
//!         ("user-id", "713936733"),
//!         ("emotes", ""),
//!         ("tmi-sent-ts", "1642786203573"),
//!         ("client-nonce", "997dcf443c31e258c1d32a8da47b6936"),
//!         ("mod", "0"),
//!         ("badges", "broadcaster/1"),
//!         ("room-id", "713936733"),
//!         ("flags", "0-6:S.7"),
//!         ("color", "#0000FF"),
//!         ("turbo", "0"),
//!         ("display-name", "abc"),
//!         ("first-msg", "0"),
//!         ("user-type", "")]);
//!
//!
//!
//     assert!(ircv3_message.tags.is_some());
//     let tags = ircv3_message.tags.clone().unwrap();
//     assert_eq!(Some("eb24e920-8065-492a-8aea-266a00fc5126".to_string()), tags.get("id"));
//     assert_eq!(None, tags.get("n"));
//!
//!     assert!(ircv3_message.source.is_some());
//!     let source = ircv3_message.source.clone().unwrap();
//!     assert_eq!("abc", source.servername_nick);
//!     assert_eq!(Some("abc".to_string()), source.user);
//!     assert_eq!(Some("abc.tmi.twitch.tv".to_string()), source.host);
//!
//!     assert_eq!("PRIVMSG".to_string(),ircv3_message.command);
//!
//!     let mut params = ircv3_message.params.clone();
//!     assert_eq!(Some("HeyGuys".to_string()), params.message);
//!
//!     let channel = params.channel.unwrap();
//!     assert_eq!("#xyz".to_string(), channel.name);
//!     assert_eq!(None, channel.alt);
//! }
//!```
//!
//! # Custom params middle parse
//```
// use ircv3_parse::{IRCv3Builder, ParamsParse};
// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_until},
//     character::complete::{not_line_ending, space1},
//     sequence::tuple,
//     IResult,
//};
//
// fn main() {
//     let msg = ":foo!foo@foo.tmi.abcdef.gh PRIVMSG guest w :bleedPurple";
//     let result = IRCv3Builder::new(WhoAmI::default()).parse(msg);
//
//     assert_eq!("guest".to_string(), result.params.stats);
//     assert_eq!("w".to_string(), result.params.user);
// }
//
// #[derive(Default)]
// struct WhoAmI {
//     pub stats: String,
//     pub user: String,
// }
pub mod validators;
// impl ParamsParse for WhoAmI {
//     fn parse(&self, _: &str, middle: ircv3_parse::IRCv3ParamsBase) -> Self
//     where
//         Self: Sized,
//     {
//         let join_middle = middle.middle.join(" ");
//         let (_, (who, user)) = whoami(join_middle.as_str()).unwrap();
//         WhoAmI {
//             stats: who.to_string(),
//             user: user.to_string(),
//         }
//     }
// Re-export commonly used types
mod unescaped_to_escaped;
pub use validators::custom_nick_validator;
// }
//
// pub fn whoami(msg: &str) -> IResult<&str, (&str, &str)> {
//     let (remain, who) = alt((owner_user, guest_user))(msg)?;
//     Ok((remain, who))
// }
//
// fn owner_user(msg: &str) -> IResult<&str, (&str, &str)> {
//     let (remain, (gust, _, user)) = tuple((
//         tag("owner"),
//         space1,
//         alt((take_until(" "), not_line_ending)),
//     ))(msg)?;
//
//     Ok((remain, (gust, user)))
// }
//
// fn guest_user(msg: &str) -> IResult<&str, (&str, &str)> {
//     let (remain, (gust, _, user)) = tuple((
//         tag("guest"),
//         space1,
//         alt((take_until(" "), not_line_ending)),
//     ))(msg)?;
//
//     Ok((remain, (gust, user)))
// }
// ```

mod source;
use std::collections::VecDeque;

pub use source::IRCv3Source;
pub(crate) mod command;
mod params;
pub use params::{IRCv3Params, IRCv3ParamsBase, ParamsParse};
pub mod builder;
// pub use builder::IRCv3Builder;
mod message;
pub use message::{IRCv3Message, IRCv3MessageBase};

#[derive(Debug)]
pub struct IRCv3;

impl IRCv3 {
    pub fn parse(msg: &str) -> IRCv3Message<IRCv3Params> {
        crate::builder::parse(msg, IRCv3Params::default())
    }

    // pub fn parse_vecdeque(msg: &str) -> VecDeque<IRCv3Message<IRCv3Params>> {
    //     IRCv3Builder::default().parse_vecdeque(msg)
    // }
}
