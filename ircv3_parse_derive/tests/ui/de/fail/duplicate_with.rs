use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M {
    #[irc(tag = "key", with = "fn1")]
    #[irc(with = "fn2")]
    field: String,
}

fn main() {}
