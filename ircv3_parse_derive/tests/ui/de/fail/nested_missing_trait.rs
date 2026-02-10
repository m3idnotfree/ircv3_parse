use ircv3_parse_derive::FromMessage;

struct Tag(String);

#[derive(FromMessage)]
struct A {
    tag: Option<Tag>,
}

fn main() {}
