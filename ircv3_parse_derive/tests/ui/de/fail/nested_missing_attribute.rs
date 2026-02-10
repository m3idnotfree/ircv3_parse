use ircv3_parse_derive::{FromMessage, ToMessage};

#[derive(FromMessage)]
struct MsgId(#[irc(tag = "msgid")] String);

#[derive(FromMessage, ToMessage)]
struct BothDerives {
    #[irc(source)]
    nick: String,
    msg_id: MsgId,
}

fn main() {}
