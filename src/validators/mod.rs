use crate::compat::ToString;

use crate::{
    components::Source,
    error::{CommandError, ParamError, SourceError, TagError},
    rfc1123::RFC1123,
    COLON, CR, EQ, HYPEN, LF, NUL, SEMICOLON, SPACE,
};

const NICK_SPECIAL_CHARS: &[u8] = b"-[]\\`^{}";

pub fn command(input: &str) -> Result<(), CommandError> {
    if input.is_empty() {
        return Err(CommandError::Empty);
    }

    let bytes = input.as_bytes();

    let first_str = bytes[0];
    if !first_str.is_ascii_alphanumeric() {
        return Err(CommandError::InvalidFirstChar {
            char: first_str as char,
        });
    }

    if first_str.is_ascii_digit() {
        let mut cur = 1;
        let mut count = 1;

        for c in &bytes[1..] {
            if c.is_ascii_digit() {
                count += 1;
                cur += 1;
                continue;
            }
            break;
        }
        if cur < bytes.len() {
            return Err(CommandError::InvalidCommand {
                input: input.to_string(),
                position: cur,
            });
        }
        if count == 3 {
            Ok(())
        } else {
            Err(CommandError::WrongDigitCount { actual: count })
        }
    } else {
        for (i, c) in bytes.iter().enumerate() {
            if !c.is_ascii_alphabetic() && i < bytes.len() {
                return Err(CommandError::InvalidCommand {
                    input: input.to_string(),
                    position: i,
                });
            }
        }

        Ok(())
    }
}

pub fn source(input: &str) -> Result<(), SourceError> {
    let source = Source::parse(input);
    if source.user.is_none() && source.host.is_none() {
        if RFC1123::new().validate(source.name).is_err() {
            nick(source.name)?;
        }
        return Ok(());
    }

    nick(source.name)?;

    if let Some(user_str) = source.user {
        user(user_str)?;
    }

    if let Some(host) = source.host {
        RFC1123::new().validate(host)?
    }

    Ok(())
}

#[inline]
pub fn nick(input: &str) -> Result<(), SourceError> {
    if input.is_empty() {
        return Err(SourceError::EmptyNick);
    }

    let bytes = input.as_bytes();

    if !bytes[0].is_ascii_alphabetic() {
        return Err(SourceError::InvalidNickFirstChar {
            char: bytes[0] as char,
        });
    }

    for (i, c) in bytes.iter().enumerate().skip(1) {
        if !(c.is_ascii_alphanumeric() || NICK_SPECIAL_CHARS.contains(c)) {
            return Err(SourceError::InvalidNickChar {
                char: *c as char,
                position: i,
            });
        }
    }

    Ok(())
}

#[inline]
pub fn user(input: &str) -> Result<(), SourceError> {
    if input.is_empty() {
        return Err(SourceError::EmptyUser);
    }

    for (i, &c) in input.as_bytes().iter().enumerate() {
        if matches!(c, SPACE | NUL | CR | LF) {
            return Err(SourceError::InvalidUserChar {
                char: c as char,
                position: i,
            });
        }
    }
    Ok(())
}

pub fn host(input: &str) -> Result<(), SourceError> {
    RFC1123::new().validate(input)?;
    Ok(())
}

pub fn tags(input: &str) -> Result<(), TagError> {
    if input.is_empty() {
        return Err(TagError::Empty);
    }

    for tag in input.split(SEMICOLON as char) {
        if let Some((key, value)) = tag.split_once(EQ as char) {
            tag_key(key)?;
            tag_value(value)?;
        } else {
            tag_key(tag)?;
        }
    }
    Ok(())
}

pub fn tag_key(input: &str) -> Result<(), TagError> {
    if input.is_empty() {
        return Err(TagError::EmptyKey);
    }

    let mut pos = 0;

    if input.starts_with('+') {
        pos += 1;
    }

    if let Some((vendor, key)) = input.rsplit_once('/') {
        RFC1123::new().validate(&vendor[pos..])?;
        tag_key_part(key)?;
    } else if input.contains('.') {
        RFC1123::new().validate(&input[pos..])?;
    } else {
        tag_key_part(&input[pos..])?;
    }

    Ok(())
}

#[inline]
fn tag_key_part(key: &str) -> Result<(), TagError> {
    for (i, &c) in key.as_bytes().iter().enumerate() {
        if !(c.is_ascii_alphanumeric() || c == HYPEN) {
            return Err(TagError::InvalidKeyChar {
                char: c as char,
                position: i,
            });
        }
    }

    Ok(())
}

#[inline]
pub fn tag_value(input: &str) -> Result<(), TagError> {
    if input.is_empty() {
        return Ok(());
    }

    let bytes = input.as_bytes();

    for (i, c) in bytes.iter().enumerate() {
        if matches!(*c, SPACE | NUL | CR | LF | SEMICOLON) {
            return Err(TagError::InvalidValueChar {
                char: *c as char,
                position: i,
            });
        }
    }

    Ok(())
}

#[inline]
pub fn param(input: &str) -> Result<(), ParamError> {
    if input.is_empty() {
        return Err(ParamError::EmptyMiddle);
    }

    for (i, &c) in input.as_bytes().iter().enumerate() {
        if matches!(c, SPACE | NUL | CR | LF | COLON) {
            return Err(ParamError::InvalidMiddleChar {
                char: c as char,
                position: i,
            });
        }
    }
    Ok(())
}

#[inline]
pub fn params(input: &str) -> Result<(), ParamError> {
    if input.is_empty() {
        return Ok(());
    }

    for p in input.split_whitespace() {
        param(p)?;
    }
    Ok(())
}

#[inline]
pub fn trailing(input: &str) -> Result<(), ParamError> {
    if input.is_empty() {
        return Ok(());
    }

    for (i, &c) in input.as_bytes().iter().enumerate() {
        if matches!(c, NUL | CR | LF) {
            return Err(ParamError::InvalidMiddleChar {
                char: c as char,
                position: i,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::validators::{
        command, host, nick, param, params, source, tag_key, tag_value, tags, trailing, user,
    };

    #[test]
    fn command_valid() {
        assert!(command("PRIVMSG").is_ok());
        assert!(command("JOIN").is_ok());
        assert!(command("001").is_ok());
        assert!(command("999").is_ok());
    }

    #[test]
    fn command_empty() {
        assert!(command("").is_err());
    }

    #[test]
    fn command_invalid() {
        assert!(command("12").is_err());
        assert!(command("1234").is_err());
        assert!(command("PRIV MSG").is_err());
        assert!(command("123a").is_err());
    }

    #[test]
    fn source_valid() {
        assert!(source("nick").is_ok());
        assert!(source("nick!user").is_ok());
        assert!(source("nick@example.com").is_ok());
        assert!(source("nick!user@example.com").is_ok());
        assert!(source("irc.example.com").is_ok());
    }

    #[test]
    fn source_invalid() {
        assert!(source("nick!user name").is_err());
        assert!(source("nick@-example.com").is_err());
    }

    #[test]
    fn nick_valid() {
        assert!(nick("nick").is_ok());
        assert!(nick("Nick").is_ok());
        assert!(nick("nick123").is_ok());
        assert!(nick("nick-name").is_ok());
        assert!(nick("nick[name]").is_ok());
        assert!(nick("nick{name}").is_ok());
        assert!(nick("escaped\\nick").is_ok());
        assert!(nick("nick`name").is_ok());
        assert!(nick("nick^name").is_ok());
    }

    #[test]
    fn nick_empty() {
        assert!(nick("").is_err());
    }

    #[test]
    fn nick_invalid() {
        assert!(nick("123user").is_err());
        assert!(nick("user@example.com").is_err());
        assert!(nick("user!test").is_err());
    }

    #[test]
    fn user_valid() {
        assert!(user("user").is_ok());
        assert!(user("~user").is_ok());
        assert!(user("user123").is_ok());
        assert!(user("user-name").is_ok());
    }

    #[test]
    fn user_empty() {
        assert!(user("").is_err());
    }

    #[test]
    fn user_invalid() {
        assert!(user("user user").is_err());
        assert!(user("user\0").is_err());
        assert!(user("user\r").is_err());
        assert!(user("user\n").is_err());
    }

    #[test]
    fn hostname_valid() {
        assert!(host("example.com").is_ok());
        assert!(host("irc.example.com").is_ok());
        assert!(host("example.example.example.com").is_ok());
    }

    #[test]
    fn hostname_invalid() {
        assert!(host("").is_err());
        assert!(host("-example.com").is_err());
        assert!(host("example-.com").is_err());
    }

    #[test]
    fn tags_valid() {
        assert!(tags("key").is_ok());
        assert!(tags("key=").is_ok());
        assert!(tags("key=value").is_ok());
        assert!(tags("key=value;key2").is_ok());
        assert!(tags("key=value;key2=").is_ok());
        assert!(tags("key=value;key2=value2").is_ok());
        assert!(tags("+client-tag=value").is_ok());
        assert!(tags("vendor.com/key=value").is_ok());
    }

    #[test]
    fn tags_empty() {
        assert!(tags("").is_err());
    }

    #[test]
    fn tags_invalid() {
        assert!(tags("key=value space").is_err());
    }

    #[test]
    fn tag_key_valid() {
        assert!(tag_key("key").is_ok());
        assert!(tag_key("key-name").is_ok());
        assert!(tag_key("+client-tag").is_ok());
        assert!(tag_key("vendor.com/key").is_ok());
        assert!(tag_key("example.com").is_ok());
    }

    #[test]
    fn tag_key_empty() {
        assert!(tag_key("").is_err());
    }

    #[test]
    fn tag_key_invalid() {
        assert!(tag_key("key space").is_err());
        assert!(tag_key("key=value").is_err());
    }

    #[test]
    fn tag_value_valid() {
        assert!(tag_value("").is_ok());
        assert!(tag_value("value").is_ok());
        assert!(tag_value("hello-world").is_ok());
        assert!(tag_value("123").is_ok());
        assert!(tag_value("escaped\\svalue").is_ok());
    }

    #[test]
    fn tag_value_invalid() {
        assert!(tag_value("space space").is_err());
        assert!(tag_value("CR\rCR").is_err());
        assert!(tag_value("LF\nLF").is_err());
        assert!(tag_value("semi;semi").is_err());
        assert!(tag_value("NUL\0NUL").is_err());
    }

    #[test]
    fn param_valid() {
        assert!(param("#channel").is_ok());
        assert!(param("target").is_ok());
        assert!(param("nick!user@example.com").is_ok());
        assert!(param("hype-hype").is_ok());
    }

    #[test]
    fn param_empty() {
        assert!(param("").is_err());
    }

    #[test]
    fn param_invalid() {
        assert!(param("space space").is_err());
        assert!(param("colon:colon").is_err());
        assert!(param("NUL\nNUL").is_err());
        assert!(param("CR\rCR").is_err());
        assert!(param("LF\nLF").is_err());
    }

    #[test]
    fn params_multiple() {
        assert!(params("#channel target").is_ok());
        assert!(params("").is_ok());
        assert!(params("param1 param2 param3").is_ok());
    }

    #[test]
    fn params_invalid() {
        assert!(params("param1 colon:colon").is_err());
        assert!(params("param1 NUL\0NUL").is_err());
    }

    #[test]
    fn trailing_valid() {
        assert!(trailing("").is_ok());
        assert!(trailing("Hello World!").is_ok());
        assert!(trailing(":colon allowed").is_ok());
        assert!(trailing("TAB\tTAB").is_ok());
    }

    #[test]
    fn trailing_invalid() {
        assert!(trailing("CR\rCR").is_err());
        assert!(trailing("LF\nLF").is_err());
        assert!(trailing("NUL\0NUL").is_err());
        assert!(trailing("line1\nline2").is_err());
    }
}
