use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct TestMessage {
    #[irc(tag = "key")]
    field: String,
}

fn main() {}
