use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct A {
    #[irc(default)]
    field: String,
}

fn default_fn() -> String {
    "default-fn".to_string()
}

#[derive(FromMessage)]
struct B {
    #[irc(default = "default_fn")]
    field: String,
}

#[derive(FromMessage)]
struct C {
    #[irc(tag = "key", default = "nonexistent_fn")]
    field: String,
}

#[derive(FromMessage)]
#[irc(tag = "tier", default = "Unknown")]
enum D {
    Bronze,
    Unknown(String),
}

fn main() {}
