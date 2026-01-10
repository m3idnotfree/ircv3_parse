use ircv3_parse_derive::ToMessage;

#[derive(ToMessage)]
struct M<'a> {
    #[irc(source = "user")]
    field: &'a str,
}

fn main() {}
