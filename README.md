# IRCv3 Parse

[![crates.io](https://img.shields.io/crates/v/ircv3_parse.svg)](https://crates.io/crates/ircv3_parse)
[![Documentation](https://docs.rs/ircv3_parse/badge.svg)](https://docs.rs/ircv3_parse)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/m3idnotfree/irc_parse/blob/main/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://github.com/m3idnotfree/irc_parse/blob/main/LICENSE-APACHE)

A **zero-copy** IRC message parser with **IRCv3** support

[Documentation](https://docs.rs/ircv3_parse)

## Features

- Zero-copy parsing for performance
- IRCv3 message tags support
- Derive macro for easy message extraction
- `no_std` compatible (with `alloc`)

## Quick Start

```toml
[dependencies]
ircv3_parse = { version = "3", features = ["derive"] }
```

```rust
use ircv3_parse::FromMessage;

#[derive(FromMessage)]
#[irc(command = "PRIVMSG")]
struct PrivMsg<'a> {
    #[irc(source = "name")]
    nick: &'a str,
    #[irc(trailing)]
    message: &'a str
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = ":nick!user@example.com PRIVMSG #channel :hi";
    let msg: PrivMsg = ircv3_parse::from_str(input)?;

    println!("From: {}", msg.nick);
    println!("Message: {}",msg.message);

    Ok(())
}
```

## Feature Flags

- **`derive`** - Enables the `FromMessage` derive macro (recommended)
- **`serde`** - Enables `Serialize` implementation for `Message`

## `no_std` Support

```toml
[dependencies]
ircv3_parse = { version = "3", default-features = false, features = ["derive"] }
```

## Parsing Rules

> **Notice:** Each component parses first special character and follows the rule. Use validation methods for strict parsing.

- **Tags**: Start with `@`, separated by `;`, followed by space
- **Source**: Start with `:`, format `name!user@host` or `host`, followed by space
- **Command**: Letters or 3-digit number
- **Middle Parameters**: Separated by spaces
- **Trailing Parameters**: Start with ` :` (space + colon)

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
