use ircv3_parse::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(invalid)]
    field1: String,
    #[irc(invalid2)]
    field2: String,

    missing: String,
}

fn main() {}
