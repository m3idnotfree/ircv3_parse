use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn str() {
    #[derive(FromMessage, ToMessage)]
    struct Message<'a> {
        #[irc(trailing)]
        msg: &'a str,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn string() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing)]
        msg: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn optional() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing)]
        msg: Option<String>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("hi".to_string()), msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn optional_missing_returns_none() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing)]
        content: Option<String>,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.content);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn empty_returns_none() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing)]
        content: Option<String>,
    }

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.content);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn unnamed() {
    #[derive(FromMessage, ToMessage)]
    struct Message<'a>(#[irc(trailing)] &'a str);

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn unnamed_string() {
    #[derive(FromMessage, ToMessage)]
    struct Message(#[irc(trailing)] String);

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn unnamed_optional() {
    #[derive(FromMessage, ToMessage)]
    struct Message(#[irc(trailing)] Option<String>);

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("hi".to_string()), msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn unnamed_optional_missing_returns_none() {
    #[derive(FromMessage, ToMessage)]
    struct Message(#[irc(trailing)] Option<String>);

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn unnamed_empty_returns_none() {
    #[derive(FromMessage, ToMessage)]
    struct Message(#[irc(trailing)] Option<String>);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn nested_trailing_empty_returns_none() {
    #[derive(FromMessage, ToMessage)]
    struct Content(#[irc(trailing)] Option<String>);

    #[derive(FromMessage, ToMessage)]
    struct Message(Content);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0 .0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn nested_str() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Content<'a>(#[irc(trailing)] &'a str);

    #[derive(FromMessage, ToMessage)]
    struct Message<'a> {
        content: Content<'a>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Content("hi"), msg.content);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn nested_optional() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Content(#[irc(trailing)] String);

    #[derive(FromMessage, ToMessage)]
    struct Message {
        content: Option<Content>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Content("hi".to_string())), msg.content);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn duplication() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Content<'a>(#[irc(trailing)] &'a str);

    #[derive(FromMessage, ToMessage)]
    struct Message<'a> {
        #[irc(trailing)]
        content: Content<'a>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Content("hi"), msg.content);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn unit_struct() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(trailing, value = "hi")]
    struct Content;

    let input = "PRIVMSG #channel :hi";
    let msg: Content = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Content, msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn default_trait_no_component() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: String,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :", output);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :", output);
}

#[test]
fn default_fn_no_component() {
    fn default_message() -> String {
        "welcome".to_string()
    }

    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing, default = "default_message")]
        msg: String,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("welcome", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :welcome", output);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("welcome", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :welcome", output);
}

#[test]
fn default_trait_present() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn default_fn_present() {
    fn default_message() -> String {
        "welcome".to_string()
    }

    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing, default = "default_message")]
        msg: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}

#[test]
fn optional_with_default() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: Option<String>,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn optional_with_default_present() {
    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: Option<String>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("hi".to_string()), msg.msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hi", output);
}
