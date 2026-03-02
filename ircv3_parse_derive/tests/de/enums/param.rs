use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn basic() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(param)]
    enum Target {
        Channel,
        Server,
    }

    let msg: Target = ircv3_parse::from_str("PRIVMSG channel :hi").unwrap();
    assert_eq!(Target::Channel, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" channel", output);

    let msg: Target = ircv3_parse::from_str("PRIVMSG server :hi").unwrap();
    assert_eq!(Target::Server, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" server", output);

    let err = ircv3_parse::from_str::<Target>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_param());

    let err = ircv3_parse::from_str::<Target>("PRIVMSG :hi").unwrap_err();
    assert!(err.is_not_found_param());
}

#[test]
fn value() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(param = 1)]
    enum Mode {
        #[irc(value = "+b")]
        Ban,
        #[irc(value = "-b")]
        Unban,
    }

    let msg: Mode = ircv3_parse::from_str("MODE #channel +b").unwrap();
    assert_eq!(Mode::Ban, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" +b", output);

    let msg: Mode = ircv3_parse::from_str("MODE #channel -b").unwrap();
    assert_eq!(Mode::Unban, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" -b", output);

    let err = ircv3_parse::from_str::<Mode>("PRIVMSG #channel :hi").unwrap_err();
    assert!(err.is_not_found_param());

    let err = ircv3_parse::from_str::<Mode>("PRIVMSG :hi").unwrap_err();
    assert!(err.is_not_found_param());
}

#[test]
fn multiple_values() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(param = 1)]
    enum Mode {
        #[irc(value = "+b", pick)]
        #[irc(value = "+B")]
        Ban,
        #[irc(value = "-b", pick)]
        #[irc(value = "-B")]
        Unban,
    }

    let msg: Mode = ircv3_parse::from_str("MODE #channel +b nick").unwrap();
    assert_eq!(Mode::Ban, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" +b", output);

    let msg: Mode = ircv3_parse::from_str("MODE #channel +B nick").unwrap();
    assert_eq!(Mode::Ban, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" +b", output);

    let msg: Mode = ircv3_parse::from_str("MODE #channel -b nick").unwrap();
    assert_eq!(Mode::Unban, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" -b", output);

    let msg: Mode = ircv3_parse::from_str("MODE #channel -B nick").unwrap();
    assert_eq!(Mode::Unban, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" -b", output);

    let err = ircv3_parse::from_str::<Mode>("MODE #channel +o nick").unwrap_err();
    assert!(err.is_not_found_param());
}

#[test]
fn rename_uppercase() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(param = 1, rename = "UPPERCASE")]
    enum Command {
        Join,
        Part,
    }

    let msg: Command = ircv3_parse::from_str("PRIVMSG #channel JOIN").unwrap();
    assert_eq!(Command::Join, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" JOIN", output);
}

#[test]
fn default() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(param, default = "Unknown")]
    enum Target {
        Channel,
        Unknown,
    }

    let msg: Target = ircv3_parse::from_str("PRIVMSG channel :hi").unwrap();
    assert_eq!(Target::Channel, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" channel", output);

    let msg: Target = ircv3_parse::from_str("PRIVMSG other :hi").unwrap();
    assert_eq!(Target::Unknown, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" unknown", output);

    let msg: Target = ircv3_parse::from_str("PRIVMSG :hi").unwrap();
    assert_eq!(Target::Unknown, msg);
    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" unknown", output);
}
