#![allow(unused)]
#[allow(unused_imports)]
use ircv3_parse::message::de::FromMessage as _;
use ircv3_parse_derive::FromMessage;

#[test]
fn privmsg() {
    #[derive(FromMessage)]
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
}

#[test]
fn with_tags() {
    #[derive(FromMessage)]
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
}

#[test]
fn with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage)]
    struct WithNum(#[irc(param = 0, with = "parse_num")] u32);

    let input = "TEST 42";
    let msg: WithNum = ircv3_parse::from_str(input).unwrap();
    assert_eq!(42, msg.0);
}

#[test]
fn multiple_params() {
    #[derive(FromMessage)]
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
}

#[test]
fn command_check() {
    #[allow(unused)]
    #[derive(FromMessage)]
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
    #[derive(FromMessage)]
    #[irc(command = "PRIVMSG")]
    struct CommandCheck<'a>(#[irc(command)] &'a str);

    let input = "PRIVMSG #channel :hello";
    let result = ircv3_parse::from_str::<CommandCheck>(input).unwrap();
    assert_eq!("PRIVMSG", result.0);

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<CommandCheck>(input).is_err());
}

#[test]
fn command_string() {
    #[derive(FromMessage)]
    struct Command(#[irc(command)] String);

    let input = "PRIVMSG";
    let msg: Command = ircv3_parse::from_str(input).unwrap();
    assert_eq!("PRIVMSG", msg.0);
}

#[test]
fn command_commands() {
    use ircv3_parse::Commands;
    #[derive(FromMessage)]
    struct Command<'a>(#[irc(command)] Commands<'a>);

    let input = "PRIVMSG";
    let msg: Command = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Commands::PRIVMSG, msg.0);
}

#[test]
fn source_empty_attribute_value_return_name() {
    #[derive(FromMessage)]
    struct Source(#[irc(source)] String);

    let input = "@msgid=1;field2 :nick!user@example.com PRIVMSG #channel :hi";
    let msg: Source = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.0);
}

#[test]
fn param_empty_attribute_value_return_first() {
    #[derive(FromMessage)]
    struct Param(#[irc(param)] String);

    let input = "@msgid=1;field2 :nick!user@example.com PRIVMSG #channel param2 :hi";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.0);
}
