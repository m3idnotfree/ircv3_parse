#![allow(unused)]
#[allow(unused_imports)]
use ircv3_parse::message::de::FromMessage as _;
use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct TrailingStr<'a>(#[irc(trailing)] &'a str);
#[derive(FromMessage)]
struct TrailingString(#[irc(trailing)] String);
#[derive(FromMessage)]
struct TrailingOptionStr<'a>(#[irc(trailing)] Option<&'a str>);
#[derive(FromMessage)]
struct TrailingOptionString(#[irc(trailing)] Option<String>);

#[derive(FromMessage)]
struct Message(#[irc(trailing)] Option<String>);
#[derive(FromMessage)]
struct TrailingNested(Message);

fn main() {}
