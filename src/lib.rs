/*!
# Tags
```
use ircv3_parse::components::TagValue;

let input = "@aaa=bbb;ccc;example.com/ddd=eee;+fff=;ggg=hello\\sworld :nick!user@host.com PRIVMSG #channel :Hello World!";
let message = ircv3_parse::parse(input)?;

if let Some(tags) = message.tags() {
    assert_eq!("aaa=bbb;ccc;example.com/ddd=eee;+fff=;ggg=hello\\sworld", tags.as_str());
    assert_eq!(5, tags.count());
    assert!(tags.contains("aaa"));
    assert!(!tags.is_empty());
    assert!(tags.validate().is_ok());

    assert_eq!(Some(TagValue::Value("bbb")), tags.get("aaa"));

    assert_eq!(Some(TagValue::Flag), tags.get("ccc"));
    assert!(tags.get_flag("ccc"));

    assert_eq!(Some(TagValue::Empty), tags.get("+fff"));

    assert_eq!(Some(TagValue::Value("hello\\sworld")), tags.get("ggg"));
    assert_eq!(Some("hello world".to_string()), tags.get_escaped("ggg"));
}

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Source
```
let input = ":nick!user@example.com PRIVMSG #channel :Hello World!";
let message = ircv3_parse::parse(input)?;

if let Some(source) = message.source() {
    assert!(source.validate().is_ok());

    assert_eq!("nick!user@example.com", source.as_str());
    assert_eq!("nick", source.name);
    assert_eq!(Some("user"), source.user);
    assert_eq!(Some("example.com"), source.host);
}

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Command
```
use ircv3_parse::components::Commands;

let message = ircv3_parse::parse("PRIVMSG")?;
let command = message.command();
assert_eq!(Commands::PRIVMSG, command);
assert_eq!("PRIVMSG", command.as_str());
assert_eq!(b"PRIVMSG", command.as_bytes());

assert!(command.is_privmsg());
assert!(command.validate().is_ok());

assert_eq!(Commands::PRIVMSG, Commands::from("PRIVMSG"));

assert!(!command.is_ping());
assert!(!command.is_pong());
assert!(!command.is_notice());
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Parameters
```
let input = "PRIVMSG #channel :Hello World!";
let message = ircv3_parse::parse(input)?;

let params = message.params();

assert_eq!("#channel :Hello World!", params.as_str());
assert_eq!(" #channel :Hello World!", params.message());

assert_eq!("#channel", params.middles.as_str());
assert_eq!(1, params.middles.count());
assert_eq!(Some("#channel"), params.middles.first());
assert_eq!(None, params.middles.second());
assert!(!params.middles.is_empty());
assert!(params.middles.validate().is_ok());

let trailing = params.trailing;
assert!(trailing.is_some());
assert_eq!("Hello World!", trailing.as_str());
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Building Messages

> **Notice:** Order is important due to single byte operations.
>
> Order: `tags(optional) -> source(optional) -> command(required) -> parameters(optional)`
```
use ircv3_parse::{builder::MessageBuilder, components::Commands};

let message = MessageBuilder::new(Commands::PRIVMSG)
    .with_tags(|tags| {
        tags.add("tag-key", Some("value"))?
            .add_flag("flag")
    })?
    .with_source("nick", |source| {
        source.with_user("user")?.with_host("example.com")
    })?
    .with_params(|params| params.add("#channel"))?
    .with_trailing("Hello World!")?
    .finish();

assert_eq!(
    "@tag-key=value;flag :nick!user@example.com PRIVMSG #channel :Hello World!\r\n",
    String::from_utf8_lossy(message.as_slice())
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

*/

pub mod builder;
pub mod components;
pub mod validators;

mod error;
mod escape;
mod rfc1123;
mod scanner;

pub use error::IRCError;
pub use escape::unescaped_to_escaped;

use components::Message;
use scanner::Scanner;

pub(crate) const NUL: u8 = b'\0';
pub(crate) const SPACE: u8 = b' ';
pub(crate) const CR: u8 = b'\r';
pub(crate) const LF: u8 = b'\n';
pub(crate) const HYPEN: u8 = b'-';
pub(crate) const COLON: u8 = b':';
pub(crate) const SEMICOLON: u8 = b';';
pub(crate) const AT: u8 = b'@';

pub fn parse(input: &str) -> Result<Message, IRCError> {
    if input.is_empty() {
        return Err(IRCError::EmptyInput);
    }

    let scanner = Scanner::new(input)?;
    Ok(Message::new(input, scanner))
}
