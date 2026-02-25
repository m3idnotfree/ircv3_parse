use ircv3_parse_derive::FromMessage;

#[test]
fn basic() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(param)]
    enum Target {
        Channel,
        Server,
    }

    let msg: Target = ircv3_parse::from_str("PRIVMSG channel :hi").unwrap();
    assert_eq!(Target::Channel, msg);

    let msg: Target = ircv3_parse::from_str("PRIVMSG server :hi").unwrap();
    assert_eq!(Target::Server, msg);

    let err = ircv3_parse::from_str::<Target>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_param());

    let err = ircv3_parse::from_str::<Target>("PRIVMSG :hi").unwrap_err();
    assert!(err.is_not_found_param());
}

#[test]
fn value() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(param = 1)]
    enum Mode {
        #[irc(value = "+b")]
        Ban,
        #[irc(value = "-b")]
        Unban,
    }

    let msg: Mode = ircv3_parse::from_str("MODE #channel +b").unwrap();
    assert_eq!(Mode::Ban, msg);

    let msg: Mode = ircv3_parse::from_str("MODE #channel -b").unwrap();
    assert_eq!(Mode::Unban, msg);

    let err = ircv3_parse::from_str::<Mode>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_param());

    let err = ircv3_parse::from_str::<Mode>("PRIVMSG :hi").unwrap_err();
    assert!(err.is_not_found_param());
}

#[test]
fn multiple_values() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(param = 1)]
    enum Mode {
        #[irc(value = "+b")]
        #[irc(value = "+B")]
        Ban,
        #[irc(value = "-b")]
        #[irc(value = "-B")]
        Unban,
    }

    let msg: Mode = ircv3_parse::from_str("MODE #channel +b nick").unwrap();
    assert_eq!(Mode::Ban, msg);

    let msg: Mode = ircv3_parse::from_str("MODE #channel +B nick").unwrap();
    assert_eq!(Mode::Ban, msg);

    let msg: Mode = ircv3_parse::from_str("MODE #channel -b nick").unwrap();
    assert_eq!(Mode::Unban, msg);

    let msg: Mode = ircv3_parse::from_str("MODE #channel -B nick").unwrap();
    assert_eq!(Mode::Unban, msg);

    let err = ircv3_parse::from_str::<Mode>("MODE #channel +o nick").unwrap_err();
    assert!(err.is_not_found_param());
}

#[test]
fn rename_uppercase() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(param = 1, rename = "UPPERCASE")]
    enum Command {
        Join,
        Part,
    }

    let msg: Command = ircv3_parse::from_str("PRIVMSG #channel JOIN").unwrap();
    assert_eq!(Command::Join, msg);
}

#[test]
fn default() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(param, default = "Unknown")]
    enum Target {
        Channel,
        Unknown,
    }

    let msg: Target = ircv3_parse::from_str("PRIVMSG channel :hi").unwrap();
    assert_eq!(Target::Channel, msg);

    let msg: Target = ircv3_parse::from_str("PRIVMSG other :hi").unwrap();
    assert_eq!(Target::Unknown, msg);

    let msg: Target = ircv3_parse::from_str("PRIVMSG :hi").unwrap();
    assert_eq!(Target::Unknown, msg);
}
