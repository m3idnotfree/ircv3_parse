use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct A(String);

#[derive(FromMessage)]
struct B;

#[derive(FromMessage)]
enum C {
    A,
    B,
}

fn main() {}
