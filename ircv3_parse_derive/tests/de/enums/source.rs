use ircv3_parse_derive::FromMessage;

#[test]
fn basic() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(source)]
    enum Server {
        Irc,
        Local,
    }

    let msg: Server = ircv3_parse::from_str(":irc PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Irc, msg);

    let msg: Server = ircv3_parse::from_str(":local PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Local, msg);

    let err = ircv3_parse::from_str::<Server>(":nick PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_source());

    let err = ircv3_parse::from_str::<Server>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_source_component_not_found());
}

#[test]
fn value() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(source)]
    enum Server {
        #[irc(value = "irc.example.com")]
        Example,
        #[irc(value = "irc.local")]
        Local,
    }

    let msg: Server = ircv3_parse::from_str(":irc.example.com PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Example, msg);

    let msg: Server = ircv3_parse::from_str(":irc.local PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Local, msg);
}

#[test]
fn multiple_values() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(source)]
    enum Server {
        #[irc(value = "irc.example.com")]
        #[irc(value = "irc.example.org")]
        Example,
        #[irc(value = "irc.local")]
        Local,
    }

    let msg: Server = ircv3_parse::from_str(":irc.example.com PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Example, msg);

    let msg: Server = ircv3_parse::from_str(":irc.example.org PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Example, msg);

    let msg: Server = ircv3_parse::from_str(":irc.local PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Local, msg);

    let err = ircv3_parse::from_str::<Server>(":irc.other PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_source());
}

#[test]
fn default() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(source, default = "Unknown")]
    enum Server {
        Irc,
        Unknown,
    }

    let msg: Server = ircv3_parse::from_str(":irc PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Irc, msg);

    let msg: Server = ircv3_parse::from_str(":nick!user@example.com PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Unknown, msg);

    let msg: Server = ircv3_parse::from_str("PRIVMSG #channel :hi").unwrap();
    assert_eq!(Server::Unknown, msg);
}
