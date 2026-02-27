use ircv3_parse::de::FromMessage as _;
use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
#[irc(tag = "msgid")]
struct T;
#[derive(FromMessage)]
#[irc(tag_flag = "subcriber")]
struct TF;

#[derive(FromMessage)]
#[irc(source)]
struct S;
#[derive(FromMessage)]
#[irc(source = "name")]
struct SN;
#[derive(FromMessage)]
#[irc(source = "user")]
struct SS;
#[derive(FromMessage)]
#[irc(source = "host")]
struct SH;

#[derive(FromMessage)]
#[irc(command = "PRIVMSG")]
struct C;

#[derive(FromMessage)]
#[irc(param)]
struct P;
#[derive(FromMessage)]
#[irc(param = 0)]
struct P0;

#[derive(FromMessage)]
#[irc(trailing)]
struct TR;

fn main() {}
