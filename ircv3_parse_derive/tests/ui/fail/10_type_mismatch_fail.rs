use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(source = "user")]
    field: u16,
}

fn main() {}
