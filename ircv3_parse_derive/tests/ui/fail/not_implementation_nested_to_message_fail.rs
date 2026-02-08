use ircv3_parse_derive::ToMessage;

struct Tag(String);

#[derive(ToMessage)]
struct A {
    tag: Option<Tag>,
}

fn main() {}
