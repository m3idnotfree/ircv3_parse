#![allow(unused)]
#[allow(unused_imports)]
use ircv3_parse::de::FromMessage as _;
use ircv3_parse::ser::ToMessage as _;
use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn privmsg() {
    #[derive(FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg<'a>(
        #[irc(source = "name")] &'a str,
        #[irc(param = 0)] &'a str,
        #[irc(trailing)] &'a str,
    );

    let input = ":nick!user@host PRIVMSG #channel :Hello";
    let msg: PrivMsg = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.0);
    assert_eq!("#channel", msg.1);
    assert_eq!("Hello", msg.2);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":nick PRIVMSG #channel :Hello", output);
}

#[test]
fn with_tags() {
    #[derive(FromMessage, ToMessage)]
    struct Tag<'a>(
        #[irc(tag = "msgid")] Option<String>,
        #[irc(tag_flag = "m-1")] bool,
        #[irc(trailing)] &'a str,
    );

    let input = "@msgid=123;m-1 :nick!user@host PRIVMSG #channel :Hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("123".to_string()), msg.0);
    assert!(msg.1);
    assert_eq!("Hello", msg.2);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("@msgid=123;m-1  :Hello", output);
}

#[test]
fn with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage, ToMessage)]
    struct WithNum(#[irc(param = 0, with = "parse_num")] u32);

    let input = "TEST 42";
    let msg: WithNum = ircv3_parse::from_str(input).unwrap();
    assert_eq!(42, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 42", output);
}

#[test]
fn multiple_params() {
    #[derive(FromMessage, ToMessage)]
    struct Params<'a>(
        #[irc(param = 0)] &'a str,
        #[irc(param = 1)] &'a str,
        #[irc(param = 2)] &'a str,
    );

    let input = "CMD arg1 arg2 arg3";
    let msg: Params = ircv3_parse::from_str(input).unwrap();
    assert_eq!("arg1", msg.0);
    assert_eq!("arg2", msg.1);
    assert_eq!("arg3", msg.2);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" arg1 arg2 arg3", output);
}

#[test]
fn command_check() {
    #[allow(unused)]
    #[derive(FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct CommandCheck<'a>(#[irc(trailing)] &'a str);

    let input = "PRIVMSG #channel :hello";
    assert!(ircv3_parse::from_str::<CommandCheck>(input).is_ok());

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<CommandCheck>(input).is_err());
}

#[test]
fn command_check_with_extraction() {
    #[allow(unused)]
    #[derive(FromMessage, ToMessage)]
    #[irc(command = "PRIVMSG")]
    struct CommandCheck<'a>(#[irc(command)] &'a str);

    let input = "PRIVMSG #channel :hello";
    let msg = ircv3_parse::from_str::<CommandCheck>(input).unwrap();
    assert_eq!("PRIVMSG", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<CommandCheck>(input).is_err());
}

#[test]
fn command_string() {
    #[derive(FromMessage, ToMessage)]
    struct Command(#[irc(command)] String);

    let input = "PRIVMSG";
    let msg: Command = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn command_commands() {
    use ircv3_parse::Commands;
    #[derive(FromMessage, ToMessage)]
    struct Command<'a>(#[irc(command)] Commands<'a>);

    let input = "PRIVMSG";
    let msg: Command = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Commands::PRIVMSG, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("PRIVMSG", output);
}

#[test]
fn source_empty_attribute_value_return_name() {
    #[derive(FromMessage, ToMessage)]
    struct Source(#[irc(source)] String);

    let input = "@msgid=1;field2 :nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(":nick ", output);
}

#[test]
fn param_empty_attribute_value_return_first() {
    #[derive(FromMessage, ToMessage)]
    struct Param(#[irc(param)] String);

    let input = "@msgid=1;field2 :nick!user@example.com PRIVMSG #channel param2 :hi";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}
