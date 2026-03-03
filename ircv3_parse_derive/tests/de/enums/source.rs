use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn basic() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(source)]
    enum Server {
        Irc,
        Local,
    }

    let msg: Server = ircv3_parse::from_str(":irc PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Irc, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":irc ", output);

    let msg: Server = ircv3_parse::from_str(":local PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Local, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":local ", output);

    let err = ircv3_parse::from_str::<Server>(":nick PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_source());

    let err = ircv3_parse::from_str::<Server>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_source_component_not_found());
}

#[test]
fn value() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(source)]
    enum Server {
        #[irc(value = "irc.example.com")]
        Example,
        #[irc(value = "irc.local")]
        Local,
    }

    let msg: Server = ircv3_parse::from_str(":irc.example.com PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Example, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":irc.example.com ", output);

    let msg: Server = ircv3_parse::from_str(":irc.local PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Local, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":irc.local ", output);
}

#[test]
fn multiple_values() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(source)]
    enum Server {
        #[irc(value = "irc.example.com")]
        #[irc(value = "irc.example.org", pick)]
        Example,
        #[irc(value = "irc.local")]
        Local,
    }

    let msg: Server = ircv3_parse::from_str(":irc.example.com PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Example, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":irc.example.org ", output);

    let msg: Server = ircv3_parse::from_str(":irc.example.org PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Example, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":irc.example.org ", output);

    let msg: Server = ircv3_parse::from_str(":irc.local PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Local, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":irc.local ", output);

    let err = ircv3_parse::from_str::<Server>(":irc.other PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_source());
}

#[test]
fn default() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(source, default = "Unknown")]
    enum Server {
        Irc,
        #[irc(skip)]
        Unknown,
    }

    let msg: Server = ircv3_parse::from_str(":irc PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Irc, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":irc ", output);

    let msg: Server = ircv3_parse::from_str(":nick!user@example.com PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Unknown, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let msg: Server = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Unknown, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}
