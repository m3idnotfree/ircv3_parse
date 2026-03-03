use ircv3_parse::FromMessage;

#[derive(FromMessage)]
struct TestMessage {
    #[irc(tag = "key")]
    field: String,
}

fn main() {}
