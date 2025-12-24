use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M1 {
    field: String,
}

#[derive(FromMessage)]
struct M2 {
    #[irc()]
    field: String,
}

fn main() {}
