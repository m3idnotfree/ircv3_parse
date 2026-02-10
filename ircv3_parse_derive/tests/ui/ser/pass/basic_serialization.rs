use ircv3_parse_derive::ToMessage;

#[derive(ToMessage)]
#[irc(command = "PRIVMSG", crlf)]
struct FullMessage<'a> {
    #[irc(tag)]
    subscriber: Option<&'a str>,
    #[irc(tag = "msgid")]
    msg_id: Option<String>,

    #[irc(tag_flag)]
    vip: bool,
    #[irc(tag_flag = "m-1")]
    m_1: bool,

    #[irc(source = "name")]
    nick: &'a str,
    #[irc(source = "user")]
    user: Option<&'a str>,
    #[irc(source = "host")]
    host: Option<&'a str>,

    #[irc(param)]
    param: String,
    #[irc(param = 0)]
    channel: &'a str,
    #[irc(params)]
    all_params: Vec<&'a str>,
    #[irc(params)]
    all_params_string: Vec<String>,

    #[irc(trailing)]
    message1: &'a str,
    #[irc(trailing)]
    message2: String,

    #[irc(command = "NOTICE")]
    cmd: String,
}

fn main() {}
