#![allow(unused)]
#[allow(unused_imports)]
use ircv3_parse::message::de::FromMessage as _;
use ircv3_parse_derive::FromMessage;

#[test]
fn privmsg() {
    #[derive(FromMessage)]
    #[irc(command = "PRIVMSG")]
    struct PrivMsg<'a> {
        #[irc(source = "name")]
        nick: &'a str,
        #[irc(param = 0)]
        channel: &'a str,
        #[irc(trailing)]
        message: &'a str,
    }

    let input = ":nick!user@host PRIVMSG #channel :Hello";
    let msg: PrivMsg = ircv3_parse::from_str(input).unwrap();
    assert_eq!("nick", msg.nick);
    assert_eq!("#channel", msg.channel);
    assert_eq!("Hello", msg.message);
}

#[test]
fn with_tags() {
    #[derive(FromMessage)]
    struct Tag<'a> {
        #[irc(tag = "msgid")]
        msg_id: Option<String>,
        #[irc(tag_flag = "m-1")]
        m_1: bool,
        #[irc(trailing)]
        content: &'a str,
    }

    let input = "@msgid=123;m-1 :nick!user@host PRIVMSG #channel :Hello";
    let msg: Tag = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("123".to_string()), msg.msg_id);
    assert!(msg.m_1);
    assert_eq!("Hello", msg.content);
}

#[test]
fn with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage)]
    struct WithNum {
        #[irc(param = 0, with = "parse_num")]
        count: u32,
    }

    let input = "TEST 42";
    let msg: WithNum = ircv3_parse::from_str(input).unwrap();
    assert_eq!(42, msg.count);
}

#[test]
fn multiple_params() {
    #[derive(FromMessage)]
    struct Params<'a> {
        #[irc(param = 0)]
        first: &'a str,
        #[irc(param = 1)]
        second: &'a str,
        #[irc(param = 2)]
        third: &'a str,
    }

    let input = "CMD arg1 arg2 arg3";
    let msg: Params = ircv3_parse::from_str(input).unwrap();

    assert_eq!("arg1", msg.first);
    assert_eq!("arg2", msg.second);
    assert_eq!("arg3", msg.third);
}

#[test]
fn command_check() {
    #[allow(unused)]
    #[derive(FromMessage)]
    #[irc(command = "PRIVMSG")]
    struct CommandCheck<'a> {
        #[irc(trailing)]
        content: &'a str,
    }

    let input = "PRIVMSG #channel :hello";
    assert!(ircv3_parse::from_str::<CommandCheck>(input).is_ok());

    let input = "NOTICE #channel :hello";
    assert!(ircv3_parse::from_str::<CommandCheck>(input).is_err());
}

#[test]
fn command_string() {
    #[derive(FromMessage)]
    struct Command {
        #[irc(command)]
        command: String,
    }

    let input = "PRIVMSG";

    let msg: Command = ircv3_parse::from_str(input).unwrap();

    assert_eq!("PRIVMSG", msg.command);
}

#[test]
fn command_commands() {
    use ircv3_parse::Commands;
    #[derive(FromMessage)]
    struct Command<'a> {
        #[irc(command)]
        command: Commands<'a>,
    }

    let input = "PRIVMSG";

    let msg: Command = ircv3_parse::from_str(input).unwrap();

    assert_eq!(Commands::PRIVMSG, msg.command);
}
