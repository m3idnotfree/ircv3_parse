use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(invalid)]
    field1: String,
    #[irc(invalid2)]
    field2: String,

    missing: String,
}

fn main() {}
