use ircv3_parse::FromMessage;

#[derive(FromMessage)]
struct Tag(#[irc(tag = "msgid")] String);

#[derive(FromMessage)]
struct Source(#[irc(source)] String);

#[derive(FromMessage)]
struct Param(#[irc(param)] String);

#[derive(FromMessage)]
struct Trailing(#[irc(trailing)] String);

#[derive(FromMessage)]
struct M {
    tag: Option<Tag>,
    source: Source,
    param: Param,
    message: Trailing,
}

fn main() {}
