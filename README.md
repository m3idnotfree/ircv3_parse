# IRCv3 Parse

[![crates.io](https://img.shields.io/crates/v/ircv3_parse.svg)](https://crates.io/crates/ircv3_parse)
[![Documentation](https://docs.rs/ircv3_parse/badge.svg)](https://docs.rs/ircv3_parse)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/m3idnotfree/irc_parse/blob/main/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://github.com/m3idnotfree/irc_parse/blob/main/LICENSE-APACHE)

A **blazingly fast**, **zero-copy** IRC v3 message parser

[Documentation](https://docs.rs/ircv3_parse)

> **Notice:** Each component parses first special character and follows the rule. If you want to use it strictly, use validation of each component.
>
> - **Tags**: Start with `@`, separated by `;` and followed by a ` `(space)
> - **Source**: Start with `:`, format `name!user@example.com` or `example.com` and followed by a ` `(space)
> - **Command**: No prefix, must be letters or 3-digit number
> - **Middle Parameters**: Start with ` ` (space), separated by spaces
> - **Trailing Parameters**: Start with ` :` (space + colon), can contain any text

## Installation

```toml
[dependencies]
ircv3_parse = "2"
```

## Quick Start

```rust
use ircv3_parse::components::{Commands, TagValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = ircv3_parse::parse("PRIVMSG #channel :Hello everyone!")?;

    assert_eq!("PRIVMSG", message.command().as_str());
    assert_eq!("#channel", message.params().middles.first().unwrap());
    assert_eq!("Hello everyone!", message.params().trailing.as_str());

    Ok(())
}
```

## Features

- **`serde`** - Serialization support for all types(deserialization not support)

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
