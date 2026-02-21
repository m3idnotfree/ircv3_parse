use ircv3_parse_derive::FromMessage;

fn default_id() -> String {
    "default-id".to_string()
}

#[derive(FromMessage)]
struct TDT {
    #[irc(tag = "msgid", default)]
    msg_id: String,
}

#[derive(FromMessage)]
struct TDF {
    #[irc(tag = "msgid", default = "default_id")]
    msg_id: String,
}

#[derive(FromMessage)]
struct PDT {
    #[irc(param = 0, default)]
    channel: String,
}

#[derive(FromMessage)]
struct SDT {
    #[irc(source = "user", default)]
    user: String,
}

fn main() {}
