use ircv3_parse_derive::FromMessage;

#[test]
fn basic() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(command)]
    enum Command {
        PrivMsg,
        Join,
        Part,
    }

    let msg: Command = ircv3_parse::from_str("PRIVMSG").unwrap();
    assert_eq!(Command::PrivMsg, msg);

    let msg: Command = ircv3_parse::from_str("JOIN").unwrap();
    assert_eq!(Command::Join, msg);

    let msg: Command = ircv3_parse::from_str("PART").unwrap();
    assert_eq!(Command::Part, msg);

    let err = ircv3_parse::from_str::<Command>("CAP").unwrap_err();
    assert!(err.is_not_found_command());
}

#[test]
fn value() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(command)]
    enum Command {
        #[irc(value = "PRIVMSG")]
        PrivMsg,
        #[irc(value = "JOIN")]
        Join,
        #[irc(value = "PART")]
        Part,
    }

    let msg: Command = ircv3_parse::from_str("privmsg").unwrap();
    assert_eq!(Command::PrivMsg, msg);

    let msg: Command = ircv3_parse::from_str("join").unwrap();
    assert_eq!(Command::Join, msg);

    let msg: Command = ircv3_parse::from_str("part").unwrap();
    assert_eq!(Command::Part, msg);
}

#[test]
fn multiple_values() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(command)]
    enum Command {
        #[irc(value = "PRIVMSG")]
        #[irc(value = "NOTICE")]
        Message,
        #[irc(value = "JOIN")]
        #[irc(value = "PART")]
        Channel,
    }

    let msg: Command = ircv3_parse::from_str("PRIVMSG").unwrap();
    assert_eq!(Command::Message, msg);

    let msg: Command = ircv3_parse::from_str("NOTICE").unwrap();
    assert_eq!(Command::Message, msg);

    let msg: Command = ircv3_parse::from_str("JOIN").unwrap();
    assert_eq!(Command::Channel, msg);

    let msg: Command = ircv3_parse::from_str("PART").unwrap();
    assert_eq!(Command::Channel, msg);

    let err = ircv3_parse::from_str::<Command>("CAP").unwrap_err();
    assert!(err.is_not_found_command());
}

#[test]
fn default() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(command, default = "Unknown")]
    enum Command {
        #[irc(value = "PRIVMSG")]
        PrivMsg,
        Unknown,
    }

    let msg: Command = ircv3_parse::from_str("PRIVMSG").unwrap();
    assert_eq!(Command::PrivMsg, msg);

    let msg: Command = ircv3_parse::from_str("NOTICE").unwrap();
    assert_eq!(Command::Unknown, msg);
}

#[test]
fn named_fields() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(command)]
    enum Command {
        PrivMsg {
            #[irc(param)]
            channel: String,
            #[irc(trailing)]
            message: String,
        },
        Join,
        Part,
    }

    let msg: Command = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(
        Command::PrivMsg {
            channel: "#channel".to_string(),
            message: "hi".to_string()
        },
        msg
    );

    let msg: Command = ircv3_parse::from_str("JOIN #channel :hi").unwrap();
    assert_eq!(Command::Join, msg);
}
