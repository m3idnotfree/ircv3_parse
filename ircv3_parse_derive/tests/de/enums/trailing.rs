use ircv3_parse_derive::FromMessage;

#[test]
fn basic() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(trailing)]
    enum Message {
        Hello,
        Bye,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hello").unwrap();
    assert_eq!(Message::Hello, msg);

    let err = ircv3_parse::from_str::<Message>("PRIVMSG #channel :Hello").unwrap_err();
    assert!(err.is_not_found_trailing());
}

#[test]
fn value() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(trailing)]
    enum Message {
        #[irc(value = "Hello")]
        Hello,
        #[irc(value = "Bye")]
        Bye,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :Hello").unwrap();
    assert_eq!(Message::Hello, msg);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :Bye").unwrap();
    assert_eq!(Message::Bye, msg);
}

#[test]
fn multiple_values() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(trailing)]
    enum Message {
        #[irc(value = "hello")]
        #[irc(value = "hi")]
        Hello,
        #[irc(value = "bye")]
        #[irc(value = "goodbye")]
        Bye,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hello").unwrap();
    assert_eq!(Message::Hello, msg);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Message::Hello, msg);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :bye").unwrap();
    assert_eq!(Message::Bye, msg);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :goodbye").unwrap();
    assert_eq!(Message::Bye, msg);

    let err = ircv3_parse::from_str::<Message>("PRIVMSG #channel :hey").unwrap_err();
    assert!(err.is_not_found_trailing());
}

#[test]
fn default() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(trailing, default = "Other")]
    enum Message {
        Hello,
        Other,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hello").unwrap();
    assert_eq!(Message::Hello, msg);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :bye").unwrap();
    assert_eq!(Message::Other, msg);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :").unwrap();
    assert_eq!(Message::Other, msg);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel").unwrap();
    assert_eq!(Message::Other, msg);
}
