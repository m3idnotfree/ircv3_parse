use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(unknown)]
    field: String,
}

fn main() {}
