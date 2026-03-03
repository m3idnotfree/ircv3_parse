use ircv3_parse::ToMessage;

struct Tag(String);

#[derive(ToMessage)]
struct A {
    tag: Option<Tag>,
}

fn main() {}
