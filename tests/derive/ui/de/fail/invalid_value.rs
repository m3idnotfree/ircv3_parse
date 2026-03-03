use ircv3_parse::FromMessage;

#[derive(FromMessage)]
struct A {
    #[irc(source = "invalid")]
    field: String,
}

#[derive(FromMessage)]
struct B {
    #[irc(param = "first")]
    field: String,
}

fn main() {}
