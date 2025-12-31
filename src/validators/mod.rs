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

pub fn user(input: &str) -> Result<(), SourceError> {
    for (i, c) in input.as_bytes().iter().enumerate() {
        if matches!(*c, SPACE | NUL | CR | LF) {
            return Err(SourceError::InvalidUserChar {
                char: *c as char,
                position: i,
            });
        }
    }
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

        let bytes = key.as_bytes();
        for (i, c) in bytes.iter().enumerate() {
            if !(c.is_ascii_alphanumeric() || *c == HYPEN) {
                return Err(TagError::InvalidKeyChar {
                    char: *c as char,
                    position: i,
                });
            }
        }
    } else if input.contains('.') {
        RFC1123::new().validate(&input[pos..])?;
    } else {
        let bytes = input.as_bytes();
        for (i, c) in bytes[pos..].iter().enumerate() {
            if !(c.is_ascii_alphanumeric() || *c == HYPEN) {
                return Err(TagError::InvalidKeyChar {
                    char: *c as char,
                    position: i,
                });
            }
        }
    }

    Ok(())
}

#[inline]
pub fn tag_value(input: &str) -> Result<(), TagError> {
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

pub fn params(input: &str) -> Result<(), ParamError> {
    for param in input.split_whitespace() {
        for c in param.as_bytes() {
            if matches!(*c, SPACE | NUL | CR | LF | COLON) {
                return Err(ParamError::ContainsControlChar);
            }
        }
    }

    Ok(())
}
