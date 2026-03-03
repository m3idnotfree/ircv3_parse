use ircv3_parse::{FromMessage, ToMessage};

struct Inner(String);

#[derive(FromMessage, ToMessage)]
struct A {
    inner: Inner,
}

#[derive(FromMessage, ToMessage)]
struct B {
    #[irc()]
    inner: Inner,
}

fn main() {}
