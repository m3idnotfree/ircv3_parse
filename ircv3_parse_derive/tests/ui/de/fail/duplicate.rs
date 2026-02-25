use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct A {
    #[irc(tag = "key1")]
    #[irc(tag = "key2")]
    field: String,
}

#[derive(FromMessage)]
struct B {
    #[irc(tag = "key1", tag = "key2")]
    field: String,
}

#[derive(FromMessage)]
struct C {
    #[irc(tag = "key")]
    #[irc(param)]
    field: String,
}

#[derive(FromMessage)]
struct D {
    #[irc(tag = "key", with = "fn1")]
    #[irc(with = "fn2")]
    field: String,
}

#[derive(FromMessage)]
struct E {
    #[irc(tag = "key", default, default)]
    field: Option<String>,
}

#[derive(FromMessage)]
struct F {
    #[irc(tag = "key", default)]
    #[irc(default)]
    field: Option<String>,
}

#[derive(FromMessage)]
#[irc(tag = "key", source)]
struct G;

#[derive(FromMessage)]
#[irc(tag_flag = "moderator")]
enum H {
    #[irc(present)]
    A,
    #[irc(present)]
    B,
}

fn main() {}
