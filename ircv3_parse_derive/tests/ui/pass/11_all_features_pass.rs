use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
#[irc(command = "PRIVMSG")]
struct FullMessage<'a> {
    #[irc(tag = "msgid")]
    msg_id: Option<String>,

    #[irc(tag_flag = "m-1")]
    m_1: bool,

    #[irc(source = "name")]
    nick: &'a str,

    #[irc(source = "user")]
    user: Option<&'a str>,

    #[irc(source = "host")]
    host: Option<&'a str>,

    #[irc(param = 0)]
    channel: &'a str,

    #[irc(params)]
    all_params: Vec<&'a str>,

    #[irc(trailing)]
    message: &'a str,

    #[irc(command)]
    cmd: String,
}

fn main() {}
