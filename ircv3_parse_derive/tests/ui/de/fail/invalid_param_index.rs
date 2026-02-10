use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(param = "first")]
    field: String,
}

fn main() {}
