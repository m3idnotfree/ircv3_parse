use ircv3_parse_derive::ToMessage;

#[derive(ToMessage)]
struct A {
    #[irc(param, skip_none)]
    channel: Option<String>,
}

#[derive(ToMessage)]
struct B {
    #[irc(tag = "key", skip_none)]
    value: String,
}

#[derive(ToMessage)]
struct C {
    #[irc(trailing, skip_none)]
    message: Option<String>,
}

fn main() {}
