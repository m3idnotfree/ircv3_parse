use ircv3_parse::de::FromMessage as _;
use ircv3_parse::FromMessage;

#[derive(FromMessage)]
struct TagStr<'a>(#[irc(tag = "msgid")] &'a str);

#[derive(FromMessage)]
struct TagString(#[irc(tag = "msgid")] String);

#[derive(FromMessage)]
struct TagOptional(#[irc(tag = "msgid")] Option<String>);

#[derive(FromMessage)]
struct TagOptionalStr<'a>(#[irc(tag = "msgid")] Option<&'a str>);

#[derive(FromMessage)]
struct TagFlag(#[irc(tag_flag = "subscriber")] bool);

#[derive(FromMessage)]
struct MultiTag(
    #[irc(tag = "msgid")] String,
    #[irc(tag = "time")] Option<String>,
    #[irc(tag_flag = "subscriber")] bool,
);

#[derive(FromMessage)]
struct SourceName(#[irc(source)] String);

#[derive(FromMessage)]
struct SourceNameStr<'a>(#[irc(source)] &'a str);

#[derive(FromMessage)]
struct SourceUser(
    #[irc(source)] String,
    #[irc(source = "user")] Option<String>,
);

#[derive(FromMessage)]
struct SourceNameHost(#[irc(source)] String, #[irc(source = "host")] String);

#[derive(FromMessage)]
struct SourceFull(
    #[irc(source)] String,
    #[irc(source = "user")] Option<String>,
    #[irc(source = "host")] Option<String>,
);

#[derive(FromMessage)]
#[irc(command = "PRIVMSG")]
struct CommandStruct(#[irc(param)] String);

#[derive(FromMessage)]
struct CommandField(#[irc(command)] String);

#[derive(FromMessage)]
struct ParamSingle(#[irc(param)] String);

#[derive(FromMessage)]
struct ParamMultiple(#[irc(param = 0)] String, #[irc(param = 1)] String);

#[derive(FromMessage)]
struct ParamOptional(#[irc(param = 0)] String, #[irc(param = 1)] Option<String>);

#[derive(FromMessage)]
struct ParamsVec(#[irc(params)] Vec<String>);

#[derive(FromMessage)]
struct TrailingStr<'a>(#[irc(trailing)] &'a str);

#[derive(FromMessage)]
struct TrailingString(#[irc(trailing)] String);

#[derive(FromMessage)]
struct PrivMsg<'a>(
    #[irc(source)] &'a str,
    #[irc(param = 0)] &'a str,
    #[irc(trailing)] &'a str,
);

#[derive(FromMessage)]
struct JoinWithTags(
    #[irc(tag = "msgid")] Option<String>,
    #[irc(source)] String,
    #[irc(param)] String, // channel
);

#[derive(FromMessage)]
#[irc(command = "NOTICE")]
struct NoticeMsg(
    #[irc(tag_flag = "bot")] bool,
    #[irc(source)] String,
    #[irc(source = "host")] Option<String>,
    #[irc(param = 0)] String,
    #[irc(trailing)] String,
);

fn parse_num(s: Option<&str>) -> u64 {
    s.and_then(|x| x.parse().ok()).unwrap_or(0)
}

#[derive(FromMessage)]
struct WithNum(#[irc(tag = "time", with = "parse_num")] u64);

fn main() {}
