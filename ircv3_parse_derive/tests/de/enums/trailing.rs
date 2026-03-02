use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn basic() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(trailing)]
    enum Message {
        Hello,
        Bye,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hello").unwrap();
    assert_eq!(Message::Hello, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hello", output);

    let err = ircv3_parse::from_str::<Message>("PRIVMSG #channel :Hello").unwrap_err();
    assert!(err.is_not_found_trailing());
}

#[test]
fn value() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(trailing)]
    enum Message {
        #[irc(value = "Hello")]
        Hello,
        #[irc(value = "Bye")]
        Bye,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :Hello").unwrap();
    assert_eq!(Message::Hello, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :Hello", output);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :Bye").unwrap();
    assert_eq!(Message::Bye, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :Bye", output);
}

#[test]
fn multiple_values() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(trailing)]
    enum Message {
        #[irc(value = "hello", pick)]
        #[irc(value = "hi")]
        Hello,
        #[irc(value = "bye")]
        #[irc(value = "goodbye", pick)]
        Bye,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hello").unwrap();
    assert_eq!(Message::Hello, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hello", output);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Message::Hello, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hello", output);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :bye").unwrap();
    assert_eq!(Message::Bye, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :goodbye", output);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :goodbye").unwrap();
    assert_eq!(Message::Bye, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :goodbye", output);

    let err = ircv3_parse::from_str::<Message>("PRIVMSG #channel :hey").unwrap_err();
    assert!(err.is_not_found_trailing());
}

#[test]
fn default() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(trailing, default = "Other")]
    enum Message {
        Hello,
        Other,
    }

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :hello").unwrap();
    assert_eq!(Message::Hello, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :hello", output);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :bye").unwrap();
    assert_eq!(Message::Other, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :other", output);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel :").unwrap();
    assert_eq!(Message::Other, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :other", output);

    let msg: Message = ircv3_parse::from_str("PRIVMSG #channel").unwrap();
    assert_eq!(Message::Other, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" :other", output);
}
