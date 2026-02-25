use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct A {
    #[irc(tag = "")]
    field: String,
}

#[derive(FromMessage)]
struct B {
    #[irc(source = "")]
    field: String,
}

#[derive(FromMessage)]
struct C {
    #[irc(param = "")]
    field: String,
}

#[derive(FromMessage)]
struct D {
    #[irc(tag = "key", with = "")]
    field: String,
}

#[derive(FromMessage)]
struct E {
    #[irc(tag = "key", default = "")]
    field: String,
}

#[derive(FromMessage)]
#[irc(tag = "tier")]
enum F {
    #[irc(value = "")]
    Bronze,
}

fn main() {}
