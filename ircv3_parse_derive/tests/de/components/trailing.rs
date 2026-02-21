use ircv3_parse_derive::FromMessage;

#[test]
fn str() {
    #[derive(FromMessage)]
    struct Message<'a> {
        #[irc(trailing)]
        msg: &'a str,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);
}

#[test]
fn string() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing)]
        msg: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);
}

#[test]
fn optional() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing)]
        msg: Option<String>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("hi".to_string()), msg.msg);
}

#[test]
fn optional_missing_returns_none() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing)]
        content: Option<String>,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.content);
}

#[test]
fn empty_returns_none() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing)]
        content: Option<String>,
    }

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.content);
}

#[test]
fn unnamed() {
    #[derive(FromMessage)]
    struct Message<'a>(#[irc(trailing)] &'a str);

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.0);
}

#[test]
fn unnamed_string() {
    #[derive(FromMessage)]
    struct Message(#[irc(trailing)] String);

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.0);
}

#[test]
fn unnamed_optional() {
    #[derive(FromMessage)]
    struct Message(#[irc(trailing)] Option<String>);

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("hi".to_string()), msg.0);
}

#[test]
fn unnamed_optional_missing_returns_none() {
    #[derive(FromMessage)]
    struct Message(#[irc(trailing)] Option<String>);

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);
}

#[test]
fn unnamed_empty_returns_none() {
    #[derive(FromMessage)]
    struct Message(#[irc(trailing)] Option<String>);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);
}

#[test]
fn nested_trailing_empty_returns_none() {
    #[derive(FromMessage)]
    struct Content(#[irc(trailing)] Option<String>);

    #[derive(FromMessage)]
    struct Message(Content);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0 .0);
}

#[test]
fn nested_str() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Content<'a>(#[irc(trailing)] &'a str);

    #[derive(FromMessage)]
    struct Message<'a> {
        content: Content<'a>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Content("hi"), msg.content);
}

#[test]
fn nested_optional() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Content(#[irc(trailing)] String);

    #[derive(FromMessage)]
    struct Message {
        content: Option<Content>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Content("hi".to_string())), msg.content);
}

#[test]
fn duplication() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Content<'a>(#[irc(trailing)] &'a str);

    #[derive(FromMessage)]
    struct Message<'a> {
        #[irc(trailing)]
        content: Content<'a>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Content("hi"), msg.content);
}

#[test]
fn unit_struct() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(trailing)]
    struct Content;

    let input = "PRIVMSG #channel :hi";
    let msg: Content = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Content, msg);
}

#[test]
fn default_trait_no_component() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: String,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.msg);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.msg);
}

#[test]
fn default_fn_no_component() {
    fn default_message() -> String {
        "welcome".to_string()
    }

    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing, default = "default_message")]
        msg: String,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("welcome", msg.msg);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("welcome", msg.msg);
}

#[test]
fn default_trait_present() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);
}

#[test]
fn default_fn_present() {
    fn default_message() -> String {
        "welcome".to_string()
    }

    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing, default = "default_message")]
        msg: String,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!("hi", msg.msg);
}

#[test]
fn optional_with_default() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: Option<String>,
    }

    let input = "PRIVMSG #channel";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg);

    let input = "PRIVMSG #channel :";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.msg);
}

#[test]
fn optional_with_default_present() {
    #[derive(FromMessage)]
    struct Message {
        #[irc(trailing, default)]
        msg: Option<String>,
    }

    let input = "PRIVMSG #channel :hi";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("hi".to_string()), msg.msg);
}
