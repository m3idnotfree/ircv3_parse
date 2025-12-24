use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
enum M1 {
    Field,
}

#[derive(FromMessage)]
struct M2(String);

#[derive(FromMessage)]
struct M3;

fn main() {}
