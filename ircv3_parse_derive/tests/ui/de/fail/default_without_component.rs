use ircv3_parse_derive::FromMessage;

#[derive(FromMessage)]
struct M1 {
    #[irc(default)]
    field: String,
}

fn default_fn() -> String {
    "default-fn".to_string()
}

#[derive(FromMessage)]
struct M2 {
    #[irc(default = "default_fn")]
    field: String,
}

fn main() {}
