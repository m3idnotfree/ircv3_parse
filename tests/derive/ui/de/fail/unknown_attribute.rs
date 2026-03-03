use ircv3_parse::FromMessage;

#[derive(FromMessage)]
#[irc(cmd)]
struct A {
    #[irc(tag = "key")]
    field: String,
}

#[derive(FromMessage)]
struct B {
    #[irc(unknown)]
    field: String,
}

#[derive(FromMessage)]
struct C {
    #[irc(tag = "key", unknown)]
    field: String,
}

#[derive(FromMessage)]
#[irc(param)]
enum D {
    #[irc(unknown)]
    Channel,
    Server,
}

fn main() {}
