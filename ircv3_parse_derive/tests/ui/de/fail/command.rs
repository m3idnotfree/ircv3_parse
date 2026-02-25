use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct A {
    #[irc(command = "PRIVMSG")]
    cmd: String,
}

#[derive(FromMessage)]
struct B {
    #[irc(command)]
    cmd: Option<String>,
}

#[derive(FromMessage)]
struct C<'a> {
    #[irc(command)]
    cmd: Option<&'a str>,
}

#[derive(FromMessage)]
#[irc(command, rename = "lowercase")]
enum D {
    PrivMsg,
    Join,
}

fn main() {}
