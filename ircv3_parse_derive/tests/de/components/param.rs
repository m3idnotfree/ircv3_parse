use ircv3_parse::{error::ParamError, SerError};
use ircv3_parse_derive::{FromMessage, ToMessage};

#[test]
fn single() {
    #[derive(FromMessage, ToMessage)]
    struct Param<'a> {
        #[irc(param = 0)]
        channel: &'a str,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.channel);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}

#[test]
fn multiple() {
    #[derive(FromMessage, ToMessage)]
    struct Params<'a> {
        #[irc(param = 0)]
        first: &'a str,
        #[irc(param = 1)]
        second: &'a str,
        #[irc(param = 2)]
        third: &'a str,
    }

    let input = "PRIVMSG param1 param2 param3";
    let msg: Params = ircv3_parse::from_str(input).unwrap();
    assert_eq!("param1", msg.first);
    assert_eq!("param2", msg.second);
    assert_eq!("param3", msg.third);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" param1 param2 param3", output);
}

#[test]
fn default() {
    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param)]
        param: String,
    }

    let input = "PRIVMSG #channel param2 :hi";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.param);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}

#[test]
fn optional() {
    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param = 1)]
        param: Option<String>,
    }

    let input = "PRIVMSG param1 param2";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("param2".to_string()), msg.param);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" param2", output);
}

#[test]
fn optional_missing() {
    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param = 1)]
        param: Option<String>,
    }

    let input = "PRIVMSG param";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.param);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param = 0, with = "parse_num")]
        count: u32,
    }

    let input = "PRIVMSG 42";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(42, msg.count);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 42", output);
}

#[test]
fn with_function_missing() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param = 0, with = "parse_num")]
        count: u32,
    }

    let input = "PRIVMSG";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(0, msg.count);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 0", output);
}

#[test]
fn unnamed_single() {
    #[derive(FromMessage, ToMessage)]
    struct Param<'a>(#[irc(param = 0)] &'a str);

    let input = "PRIVMSG #channel :hello";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}

#[test]
fn unnamed_multiple() {
    #[derive(FromMessage, ToMessage)]
    struct Params<'a>(
        #[irc(param = 0)] &'a str,
        #[irc(param = 1)] &'a str,
        #[irc(param = 2)] &'a str,
    );

    let input = "PRIVMSG param1 param2 param3";
    let msg: Params = ircv3_parse::from_str(input).unwrap();
    assert_eq!("param1", msg.0);
    assert_eq!("param2", msg.1);
    assert_eq!("param3", msg.2);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" param1 param2 param3", output);
}

#[test]
fn unnamed_default() {
    #[derive(FromMessage, ToMessage)]
    struct Param(#[irc(param)] String);

    let input = "PRIVMSG #channel param2 :hi";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}

#[test]
fn unnamed_optional() {
    #[derive(FromMessage, ToMessage)]
    struct Param(#[irc(param = 1)] Option<String>);

    let input = "PRIVMSG param1 param2";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("param2".to_string()), msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" param2", output);
}

#[test]
fn unnamed_optional_missing() {
    #[derive(FromMessage, ToMessage)]
    struct Param(#[irc(param = 1)] Option<String>);

    let input = "PRIVMSG param";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn unnamed_with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage, ToMessage)]
    struct Param(#[irc(param = 0, with = "parse_num")] u32);

    let input = "PRIVMSG 42";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(42, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 42", output);
}

#[test]
fn unnamed_with_function_missing() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(FromMessage, ToMessage)]
    struct Param(#[irc(param = 0, with = "parse_num")] u32);

    let input = "PRIVMSG";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(0, msg.0);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 0", output);
}

#[test]
fn nested_param() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Channel<'a>(#[irc(param = 0)] &'a str);

    #[derive(FromMessage, ToMessage)]
    struct Message<'a> {
        ch: Channel<'a>,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Channel("#channel"), msg.ch);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}

#[test]
fn nested_param_optional() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Channel<'a>(#[irc(param = 0)] &'a str);

    #[derive(FromMessage, ToMessage)]
    struct Message<'a> {
        ch: Option<Channel<'a>>,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some(Channel("#channel")), msg.ch);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}

#[test]
fn nested_param_with_function() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Count(#[irc(param = 0, with = "parse_num")] u32);

    #[derive(FromMessage, ToMessage)]
    struct Message {
        count: Count,
    }

    let input = "PRIVMSG 42";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Count(42), msg.count);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 42", output);
}

#[test]
fn nested_outer_attribute_ignored() {
    fn parse_num(s: Option<&str>) -> u32 {
        s.and_then(|x| x.parse().ok()).unwrap_or(0)
    }

    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    struct Count(#[irc(param = 0, with = "parse_num")] u32);

    #[derive(FromMessage, ToMessage)]
    struct Message {
        #[irc(param = 2)]
        count: Count,
    }

    let input = "PRIVMSG 42";
    let msg: Message = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Count(42), msg.count);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 42", output);
}

#[test]
fn unit_struct() {
    #[derive(Debug, PartialEq, FromMessage, ToMessage)]
    #[irc(param, value = "42")]
    struct Count;

    let input = "PRIVMSG 42";
    let msg: Count = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Count, msg);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" 42", output);
}

#[test]
fn default_trait_no_component() {
    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param, default)]
        second: String,
    }

    let input = "PRIVMSG";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.second);

    // parameter middle ctannot be empty
    let err = ircv3_parse::to_message(&msg).unwrap_err();
    assert_eq!(SerError::Param(ParamError::EmptyMiddle), err);
}

#[test]
fn default_fn_no_component() {
    fn default_channel() -> String {
        "#official".to_string()
    }

    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param, default = "default_channel")]
        channel: String,
    }

    let input = "PRIVMSG :hello";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#official", msg.channel);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #official", output);
}

#[test]
fn default_trait_present() {
    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param = 1, default)]
        second: String,
    }

    let input = "PRIVMSG param1 param2";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("param2", msg.second);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" param2", output);
}

#[test]
fn default_fn_present() {
    fn default_channel() -> String {
        "#official".to_string()
    }

    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param, default = "default_channel")]
        channel: String,
    }

    let input = "PRIVMSG #channel :hello";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("#channel", msg.channel);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" #channel", output);
}

#[test]
fn optional_with_default() {
    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param = 1, default)]
        second: Option<String>,
    }

    let input = "PRIVMSG param1";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.second);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);

    let input = "PRIVMSG";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(None, msg.second);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!("", output);
}

#[test]
fn optional_with_default_present() {
    #[derive(FromMessage, ToMessage)]
    struct Param {
        #[irc(param = 1, default)]
        second: Option<String>,
    }

    let input = "PRIVMSG param1 param2";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!(Some("param2".to_string()), msg.second);

    let output = ircv3_parse::to_message(&msg).unwrap();
    assert_eq!(" param2", output);
}

#[test]
fn unnamed_default_trait() {
    #[derive(FromMessage, ToMessage)]
    struct Param(#[irc(param, default)] String);

    let input = "PRIVMSG :hello";
    let msg: Param = ircv3_parse::from_str(input).unwrap();
    assert_eq!("", msg.0);

    // parameter middle ctannot be empty
    let err = ircv3_parse::to_message(&msg).unwrap_err();
    assert_eq!(SerError::Param(ParamError::EmptyMiddle), err);
}
