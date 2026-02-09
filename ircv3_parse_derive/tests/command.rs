#![allow(dead_code)]
use ircv3_parse_derive::FromMessage;

#[test]
fn string() {
    #[derive(FromMessage)]
    struct Cmd {
        #[irc(command)]
        command: String,
    }

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.command);
}

#[test]
fn str() {
    #[derive(FromMessage)]
    struct Cmd<'a> {
        #[irc(command)]
        command: &'a str,
    }

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.command);
}

#[test]
fn commands_enum() {
    use ircv3_parse::Commands;

    #[derive(FromMessage)]
    struct Cmd<'a> {
        #[irc(command)]
        command: Commands<'a>,
    }

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Commands::PRIVMSG, msg.command);
}

#[test]
fn validation() {
    #[derive(FromMessage)]
    #[irc(command = "PRIVMSG")]
    struct Message<'a> {
        #[irc(trailing)]
        message: &'a str,
    }

    let input = "PRIVMSG #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_ok());

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_err());
}

#[test]
fn overrides_validation() {
    #[derive(FromMessage)]
    #[irc(command = "PRIVMSG")]
    struct Cmd<'a> {
        #[irc(command = "NOTICE")]
        command: &'a str,
    }

    let input = "PRIVMSG #channel :hello";
    assert!(ircv3_parse::from_str::<Cmd>(input).is_err());

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<Cmd>(input).is_ok());
}

#[test]
fn unnamed() {
    #[derive(FromMessage)]
    struct Cmd(#[irc(command)] String);

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.0);
}

#[test]
fn unnamed_str() {
    #[derive(FromMessage)]
    struct Cmd<'a>(#[irc(command)] &'a str);

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.0);
}

#[test]
fn unnamed_commands_enum() {
    use ircv3_parse::Commands;

    #[derive(FromMessage)]
    struct Cmd<'a>(#[irc(command)] Commands<'a>);

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Commands::PRIVMSG, msg.0);
}

#[test]
fn unnamed_validation() {
    #[derive(FromMessage)]
    #[irc(command = "PRIVMSG")]
    struct Message<'a>(#[irc(trailing)] &'a str);

    let input = "PRIVMSG #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_ok());

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_err());
}

#[test]
fn unnamed_overrides_validation() {
    #[derive(FromMessage)]
    #[irc(command = "PRIVMSG")]
    struct Message<'a>(#[irc(command = "NOTICE")] &'a str);

    let input = "PRIVMSG #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_err());

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_ok());
}

#[test]
fn nested_command() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Cmd(#[irc(command)] String);

    #[derive(FromMessage)]
    struct Message {
        cmd: Cmd,
    }

    let input = "PRIVMSG";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Cmd("PRIVMSG".to_string()), msg.cmd);
}

#[test]
fn nested_command_optional() {
    #[derive(FromMessage, Debug, PartialEq)]
    struct Cmd(#[irc(command)] String);

    #[derive(FromMessage)]
    struct Message {
        cmd: Option<Cmd>,
    }

    let input = "NOTICE";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Cmd("NOTICE".to_string())), msg.cmd);
}

#[test]
fn nested_command_with_validation() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg(#[irc(command)] String);

    #[derive(FromMessage)]
    struct Message {
        cmd: PrivMsg,
    }

    let input = "PRIVMSG";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(PrivMsg("PRIVMSG".to_string()), msg.cmd);

    assert!(ircv3_parse::from_str::<Message>("NOTICE").is_err());
}

#[test]
fn nested_outer_attribute_ignored() {
    #[derive(FromMessage, Debug, PartialEq)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg(#[irc(command)] String);

    #[derive(FromMessage)]
    struct Message {
        #[irc(command = "NOTICE")]
        cmd: PrivMsg,
    }

    let input = "PRIVMSG";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(PrivMsg("PRIVMSG".to_string()), msg.cmd);

    assert!(ircv3_parse::from_str::<Message>("NOTICE").is_err());
}
