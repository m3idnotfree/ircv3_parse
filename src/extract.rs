/// Extract custom data structures from IRC messages.
///
/// This trait allows you to convert IRC messages into your own types,
/// providing type-safe access to message fields.
///
/// ```rust
/// use ircv3_parse::{extract::FromMessage, ExtractError};
///
/// struct BasicMessage<'a> {
///     color: Option<&'a str>,
///     nick: &'a str,
///     message: &'a str,
/// }
///
/// impl<'a> FromMessage<'a> for BasicMessage<'a> {
///     fn from_message(msg: &ircv3_parse::Message<'a>) -> Result<Self, ircv3_parse::ExtractError> {
///         let command = msg.command();
///         if !command.is_privmsg() {
///             return Err(ExtractError::invalid_command("PRIVMSG", command.as_str()));
///         }
///
///         let tags = msg.tags().ok_or(ExtractError::missing_tags())?;
///         let color = tags.get("color").map(|v| v.as_str());
///
///         let source = msg.source().ok_or(ExtractError::missing_source())?;
///         let nick = source.name;
///
///         let params = msg.params();
///         let message = params.trailing.as_str();
///
///         Ok(Self {
///             color,
///             nick,
///             message
///         })
///     }
/// }
///
/// # fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let input = "@color=#FF0000 :nick!user@host PRIVMSG #rust :Hello!";
/// let msg = BasicMessage::from_str(input)?;
///
/// // or
/// let msg: BasicMessage = ircv3_parse::from_str(input)?;
///
/// assert_eq!("nick", msg.nick);
/// assert_eq!(Some("#FF0000"), msg.color);
///
/// # Ok(())
/// # }
/// ```
pub trait FromMessage<'a>: Sized {
    fn from_str(s: &'a str) -> Result<Self, crate::ExtractError> {
        let msg = crate::parse(s)?;
        Self::from_message(&msg)
    }

    fn from_message(msg: &crate::Message<'a>) -> Result<Self, crate::ExtractError>;
}

#[cfg(test)]
mod tests {
    use crate::{extract::FromMessage, ExtractError};

    #[derive(Debug)]
    struct TestMessage<'a> {
        aaa: Option<&'a str>,
        ccc: bool,
        nick: &'a str,
        message: &'a str,
    }

    impl<'a> FromMessage<'a> for TestMessage<'a> {
        fn from_message(msg: &crate::Message<'a>) -> Result<Self, ExtractError> {
            let command = msg.command();
            if !command.is_privmsg() {
                return Err(ExtractError::invalid_command("PRIVMSG", command.as_str()));
            }

            let tags = msg.tags().ok_or(ExtractError::missing_tags())?;
            let aaa = tags.get("aaa").map(|e| e.as_str());
            let ccc = tags.get_flag("ccc");

            let source = msg.source().ok_or(ExtractError::missing_source())?;
            let nick = source.name;

            let params = msg.params();
            let message = params.trailing;

            Ok(Self {
                aaa,
                ccc,
                nick,
                message: message.as_str(),
            })
        }
    }

    #[test]
    pub fn from_message_with_tags() {
        let input = "@aaa=bbb;ccc;example.com/ddd=eee;+fff=;ggg=hello\\sworld :nick!user@host.com PRIVMSG #channel :Hello World!";

        let test_message: TestMessage = crate::from_str(input).unwrap();

        assert_eq!(Some("bbb"), test_message.aaa);
        assert!(test_message.ccc);
        assert_eq!("nick", test_message.nick);
        assert_eq!("Hello World!", test_message.message);
    }

    #[test]
    pub fn from_str_method() {
        let input = "@aaa=bbb;ccc;example.com/ddd=eee;+fff=;ggg=hello\\sworld :nick!user@host.com PRIVMSG #channel :Hello World!";

        let test_message = TestMessage::from_str(input).unwrap();

        assert_eq!(Some("bbb"), test_message.aaa);
        assert!(test_message.ccc);
        assert_eq!("nick", test_message.nick);
        assert_eq!("Hello World!", test_message.message);
    }

    #[test]
    fn invalid_command_error() {
        let input = ":nick JOIN #channel";
        let error = TestMessage::from_str(input).expect_err("Expected invalid command error");

        assert!(error.is_invalid_command());
    }

    #[test]
    fn missing_source_error() {
        let input = "@aaa=bbb PRIVMSG #channel :Hello";
        let error = TestMessage::from_str(input).expect_err("Expected missing source error");

        assert!(error.is_missing_source());
    }
}
