use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
#[irc(cmd = "PrivMsg")]
struct M {
    #[irc(tag = "key2")]
    field: String,
}

fn main() {}
