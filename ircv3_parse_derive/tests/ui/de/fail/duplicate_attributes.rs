use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M1 {
    #[irc(tag = "key1")]
    #[irc(tag = "key2")]
    field: String,
}

#[derive(FromMessage)]
struct M2 {
    #[irc(tag = "key1", tag = "key2")]
    field: String,
}

fn main() {}
