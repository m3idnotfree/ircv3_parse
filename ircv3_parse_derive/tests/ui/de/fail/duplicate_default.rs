use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M1 {
    #[irc(tag = "key1", default, default)]
    field: String,
}

#[derive(FromMessage)]
struct M2 {
    #[irc(tag = "key", default)]
    #[irc(default)]
    field: String,
}

fn main() {}
