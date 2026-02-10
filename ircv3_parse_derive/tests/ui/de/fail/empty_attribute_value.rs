use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M1 {
    #[irc(tag = "")]
    field: String,
}

#[derive(FromMessage)]
struct M2 {
    #[irc(source = "")]
    field: String,
}

fn main() {}
