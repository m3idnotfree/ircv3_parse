#![allow(dead_code)]
use ircv3_parse::Commands;
use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn string() {
    #[derive(FromMessage, ToMessage)]
    struct Cmd {
        #[irc(command)]
        command: String,
    }

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.command);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn str() {
    #[derive(FromMessage, ToMessage)]
    struct Cmd<'a> {
        #[irc(command)]
        command: &'a str,
    }

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.command);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn commands_enum() {
    #[derive(FromMessage, ToMessage)]
    struct Cmd<'a> {
        #[irc(command)]
        command: Commands<'a>,
    }

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Commands::PRIVMSG, msg.command);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
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
fn validation_and_extraction() {
    #[derive(FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct Cmd<'a> {
        #[irc(command)]
        command: &'a str,
    }

    let input = "PRIVMSG #channel :hello";
    let msg = ircv3_parse::from_str::<Cmd>(input).unwrap();
    assert_eq!(msg.command, "PRIVMSG");

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<Cmd>(input).is_err());
}

#[test]
fn unnamed() {
    #[derive(FromMessage, ToMessage)]
    struct Cmd(#[irc(command)] String);

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn unnamed_str() {
    #[derive(FromMessage, ToMessage)]
    struct Cmd<'a>(#[irc(command)] &'a str);

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn unnamed_commands_enum() {
    #[derive(FromMessage, ToMessage)]
    struct Cmd<'a>(#[irc(command)] Commands<'a>);

    let input = "PRIVMSG";
    let msg: Cmd = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Commands::PRIVMSG, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn unnamed_validation() {
    #[derive(FromMessage, ToMessage, Debug, PartialEq)]
    #[irc(command = "PRIVMSG")]
    struct Message<'a>(#[irc(trailing)] &'a str);

    let input = "PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Message("hello"), msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG :hello", output);

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_err());
}

#[test]
fn unnamed_validation_and_extraction() {
    #[derive(FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct Message<'a>(#[irc(command)] &'a str);

    let input = "PRIVMSG #channel :hello";
    let msg = ircv3_parse::from_str::<Message>(input).unwrap();
    assert_eq!(msg.0, "PRIVMSG");

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<Message>(input).is_err());
}

#[test]
fn nested_command() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Cmd(#[irc(command)] String);

    #[derive(FromMessage, ToMessage)]
    struct Message {
        cmd: Cmd,
    }

    let input = "PRIVMSG";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Cmd("PRIVMSG".to_string()), msg.cmd);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn nested_command_optional() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Cmd(#[irc(command)] String);

    #[derive(FromMessage, ToMessage)]
    struct Message {
        cmd: Option<Cmd>,
    }

    let input = "NOTICE";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Cmd("NOTICE".to_string())), msg.cmd);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("NOTICE", output);
}

#[test]
fn nested_command_with_validation() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg(#[irc(command)] String);

    #[derive(FromMessage, ToMessage)]
    struct Message {
        cmd: PrivMsg,
    }

    let input = "PRIVMSG";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(PrivMsg("PRIVMSG".to_string()), msg.cmd);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);

    assert!(ircv3_parse::from_str::<Message>("NOTICE").is_err());
}

#[test]
fn field_attribute_ignored_for_nested_type() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg(#[irc(command)] String);

    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(command)]
        cmd: PrivMsg,
    }

    let input = "PRIVMSG";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(PrivMsg("PRIVMSG".to_string()), msg.cmd);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);

    assert!(ircv3_parse::from_str::<Message>("NOTICE").is_err());
}

#[test]
fn unit_struct() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg;

    let input = "PRIVMSG";
    let msg: PrivMsg = ircv3_parse::from_str(input).unwrap();
    assert_eq!(PrivMsg, msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);

    assert!(ircv3_parse::from_str::<PrivMsg>("NOTICE").is_err());
}
