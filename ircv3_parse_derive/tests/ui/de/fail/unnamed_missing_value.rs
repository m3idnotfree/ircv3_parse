use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct MissingTagValue(#[irc(tag)] String);

#[derive(FromMessage)]
struct MissingTagFlagValue(#[irc(tag_flag)] String);

fn main() {}
