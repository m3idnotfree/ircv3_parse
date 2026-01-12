# IRCv3 Parse

[![CI](https://github.com/m3idnotfree/ircv3_parse/workflows/CI/badge.svg)](https://github.com/m3idnotfree/ircv3_parse/actions)
[![crates.io](https://img.shields.io/crates/v/ircv3_parse.svg)](https://crates.io/crates/ircv3_parse)
[![Documentation](https://docs.rs/ircv3_parse/badge.svg)](https://docs.rs/ircv3_parse)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/m3idnotfree/irc_parse/blob/main/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://github.com/m3idnotfree/irc_parse/blob/main/LICENSE-APACHE)

A **zero-copy** IRC message parser with **IRCv3** support

[Documentation](https://docs.rs/ircv3_parse)

## Features

- Zero-copy parsing for performance
- IRCv3 message tags support
- **Derive macros** (`FromMessage`, `ToMessage`) for easy message extraction and generation
- `no_std` compatible (with `alloc`)

## Quick Start

```toml
[dependencies]
ircv3_parse = { version = "3", features = ["derive"] }
```

### Parsing Messages (FromMessage)

Extract IRC message components into your custom types with the `FromMessage` derive macro:

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

    // Output: From: nick
    println!("From: {}", msg.nick);
    // Output: Message: hi
    println!("Message: {}", msg.message);

    Ok(())
}
```

### Building Messages (ToMessage)

Generate IRC messages from your custom types with the `ToMessage` derive macro:

```rust
use ircv3_parse::ToMessage;

#[derive(ToMessage)]
#[irc(command = "PRIVMSG", crlf)]
struct PrivMsg<'a> {
    #[irc(tag)]
    msgid: &'a str,
    #[irc(param)]
    channel: &'a str,
    #[irc(trailing)]
    message: &'a str
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let msg = PrivMsg {
        msgid: "123",
        channel: "#channel",
        message: "hi"
    };

    let output = ircv3_parse::to_message(&msg)?;
    // Output: @msgid=123 PRIVMSG #channel :hi\r\n
    println!("{}", output);

    Ok(())
}
```

## Feature Flags

- **`std`** (default) - Standard library support
- **`derive`** - Enables `FromMessage` and `ToMessage` derive macros (recommended)
- **`serde`** - Enables `Serialize` implementation for `Message`

## `no_std` Support

```toml
[dependencies]
ircv3_parse = { version = "3", default-features = false, features = ["derive"] }
```

## Minimum Supported Rust Version (MSRV)

This crate requires **Rust 1.78 or later**.

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
