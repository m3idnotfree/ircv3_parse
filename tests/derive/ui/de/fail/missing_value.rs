use ircv3_parse::FromMessage;

#[derive(FromMessage)]
struct A(#[irc(tag)] String);

#[derive(FromMessage)]
struct B(#[irc(tag_flag)] String);

#[derive(FromMessage)]
struct C {
    #[irc(tag = "key", with)]
    field: String,
}

#[derive(FromMessage)]
#[irc(command)]
struct D;

fn main() {}
