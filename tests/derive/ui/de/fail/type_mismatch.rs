use ircv3_parse::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(source = "user")]
    field: u16,
}

fn main() {}
