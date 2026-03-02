use ircv3_parse_derive::ToMessage;

#[derive(ToMessage)]
#[irc(trailing)]
enum A {
    #[irc(value = "hello", pick)]
    Hello,
    #[irc(value = "bye", pick)]
    Bye,
}

#[derive(ToMessage)]
#[irc(trailing)]
enum B {
    #[irc(value = "hello", pick)]
    #[irc(value = "hi", pick)]
    Hello,
}

#[derive(ToMessage)]
#[irc(trailing)]
enum C {
    #[irc(value = "hello")]
    #[irc(value = "hi")]
    Hello,
}

#[derive(ToMessage)]
#[irc(trailing)]
enum D {
    #[irc(value = "hello", pick)]
    Hello,
}

fn main() {}
