use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(source = "invalid")]
    field: String,
}

fn main() {}
